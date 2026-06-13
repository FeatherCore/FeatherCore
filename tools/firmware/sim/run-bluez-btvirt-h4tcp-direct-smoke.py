#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Directly connect NuttX SIM btvhcid H4 client mode to BlueZ btvirt TCP.

This is the no-/dev/vhci BlueZ userspace emulator path:
    btvhcid --h4-in connect:<port> --h4-out connect:<port>
        <-> BlueZ emulator/btvirt -t<port>
"""

from __future__ import annotations

import argparse
import os
import pty
import select
import subprocess
import sys
import time
from pathlib import Path


def read_until(fd: int, needle: str, timeout: float, transcript: list[str]) -> bool:
    end = time.monotonic() + timeout
    buf = ""
    while time.monotonic() < end:
        ready, _, _ = select.select([fd], [], [], 0.05)
        if not ready:
            continue
        try:
            chunk = os.read(fd, 4096)
        except OSError:
            return False
        if not chunk:
            return False
        text = chunk.decode("utf-8", errors="replace")
        transcript.append(text)
        buf += text
        if needle in buf:
            return True
    return False


def drain_pty(fd: int, seconds: float, transcript: list[str]) -> None:
    end = time.monotonic() + seconds
    while time.monotonic() < end:
        ready, _, _ = select.select([fd], [], [], 0.05)
        if not ready:
            continue
        try:
            chunk = os.read(fd, 4096)
        except OSError:
            return
        if not chunk:
            return
        transcript.append(chunk.decode("utf-8", errors="replace"))


def send_cmd(fd: int, cmd: str) -> None:
    os.write(fd, (cmd + "\n").encode("utf-8"))


def start_btvirt(btvirt: Path, port: int) -> subprocess.Popen[str]:
    return subprocess.Popen(
        [str(btvirt), "-t" + str(port), "-d"],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        errors="replace",
    )


def collect_proc(proc: subprocess.Popen[str]) -> str:
    if proc.stdout is None:
        return ""
    out: list[str] = []
    while True:
        ready, _, _ = select.select([proc.stdout], [], [], 0)
        if not ready:
            break
        line = proc.stdout.readline()
        if not line:
            break
        out.append(line)
    return "".join(out)


def run(args: argparse.Namespace) -> int:
    sim = Path(args.sim).resolve()
    btvirt = Path(args.btvirt).resolve()
    transcript: list[str] = []
    btvirt_out = ""

    if not sim.exists():
        print(f"missing SIM binary: {sim}", file=sys.stderr)
        return 2
    if not btvirt.exists():
        print(f"missing BlueZ btvirt binary: {btvirt}", file=sys.stderr)
        return 2

    btvirt_proc = start_btvirt(btvirt, args.btvirt_port)
    time.sleep(0.3)
    btvirt_out += collect_proc(btvirt_proc)

    master_fd, slave_fd = pty.openpty()
    sim_proc = subprocess.Popen(
        [str(sim)],
        stdin=slave_fd,
        stdout=slave_fd,
        stderr=slave_fd,
        cwd=str(Path.cwd()),
        close_fds=True,
    )
    os.close(slave_fd)

    try:
        if not read_until(master_fd, "nsh>", args.boot_timeout, transcript):
            print("timeout waiting for nsh prompt", file=sys.stderr)
            return 1

        send_cmd(
            master_fd,
            f"btvhcid --rounds {args.rounds} --h4-in connect:{args.btvirt_port} --h4-out connect:{args.btvirt_port}",
        )

        read_until(master_fd, "nsh>", args.command_timeout, transcript)
        drain_pty(master_fd, 0.5, transcript)
        btvirt_out += collect_proc(btvirt_proc)

        text = "".join(transcript)
        log_text = text + "\n--- btvirt ---\n" + btvirt_out
        if args.log:
            Path(args.log).write_text(log_text, encoding="utf-8")

        required = [
            f"bthwsim: H4 TCP connected to 127.0.0.1:{args.btvirt_port}",
            "bthwsim: H4 TCP send bytes=",
            "bthwsim: H4 TCP recv bytes=",
            "btvhcid: h4-in packet=",
            "Bluetooth: hci0 Event packet",
            "Listening TCP on 127.0.0.1:",
        ]
        missing = [item for item in required if item not in log_text]
        if missing:
            print("BlueZ btvirt H4 TCP direct smoke: FAIL", file=sys.stderr)
            print("missing markers:", file=sys.stderr)
            for item in missing:
                print(f"  {item}", file=sys.stderr)
            print("\n--- transcript ---", file=sys.stderr)
            print(log_text, file=sys.stderr)
            return 1

        print("BlueZ btvirt H4 TCP direct smoke: PASS")
        return 0
    finally:
        try:
            send_cmd(master_fd, "poweroff")
            drain_pty(master_fd, 0.5, transcript)
        except Exception:
            pass
        sim_proc.terminate()
        try:
            sim_proc.wait(timeout=2.0)
        except subprocess.TimeoutExpired:
            sim_proc.kill()
            sim_proc.wait(timeout=2.0)
        os.close(master_fd)
        btvirt_proc.terminate()
        try:
            btvirt_proc.wait(timeout=2.0)
        except subprocess.TimeoutExpired:
            btvirt_proc.kill()
            btvirt_proc.wait(timeout=2.0)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--sim", default="build/nuttx-sim-bt1")
    parser.add_argument("--btvirt", default="/home/uan/Feather-develop-BT/third/bluez/emulator/btvirt")
    parser.add_argument("--btvirt-port", type=int, default=45552)
    parser.add_argument("--rounds", type=int, default=80)
    parser.add_argument("--boot-timeout", type=float, default=10.0)
    parser.add_argument("--command-timeout", type=float, default=10.0)
    parser.add_argument("--log", default="build/logs/run-bluez-btvirt-h4tcp-direct-smoke.log")
    return run(parser.parse_args())


if __name__ == "__main__":
    raise SystemExit(main())
