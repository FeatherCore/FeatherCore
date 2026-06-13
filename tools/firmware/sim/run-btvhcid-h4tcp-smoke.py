#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Smoke-test the NuttX SIM btvhcid tcp:<port> H4 bridge.

The test starts one SIM image, launches btvhcid from nsh, injects a small H4
Command Complete event from the Ubuntu host side, then verifies that the event
reaches the upstream Linux Bluetooth HCI event path inside the SIM.
"""

from __future__ import annotations

import argparse
import os
import pty
import select
import socket
import subprocess
import sys
import time
from pathlib import Path

H4_RESET_COMPLETE = bytes([0x04, 0x0e, 0x04, 0x01, 0x03, 0x0c, 0x00])


def read_until(fd: int, needle: str, timeout: float, transcript: list[str]) -> bool:
    end = time.monotonic() + timeout
    buf = ""

    while time.monotonic() < end:
        ready, _, _ = select.select([fd], [], [], 0.1)
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


def drain(fd: int, seconds: float, transcript: list[str]) -> None:
    end = time.monotonic() + seconds
    while time.monotonic() < end:
        ready, _, _ = select.select([fd], [], [], 0.1)
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


def inject_h4(port: int, payload: bytes) -> None:
    with socket.create_connection(("127.0.0.1", port), timeout=5.0) as sock:
        sock.sendall(payload)
        time.sleep(0.2)


def run(args: argparse.Namespace) -> int:
    sim = Path(args.sim).resolve()
    transcript: list[str] = []

    if not sim.exists():
        print(f"missing SIM binary: {sim}", file=sys.stderr)
        return 2

    master_fd, slave_fd = pty.openpty()
    proc = subprocess.Popen(
        [str(sim)],
        stdin=slave_fd,
        stdout=slave_fd,
        stderr=slave_fd,
        cwd=str(sim.parent.parent if sim.parent.name == "build" else Path.cwd()),
        close_fds=True,
    )
    os.close(slave_fd)

    try:
        if not read_until(master_fd, "nsh>", args.boot_timeout, transcript):
            print("timeout waiting for nsh prompt", file=sys.stderr)
            return 1

        cmd = (
            f"btvhcid --rounds {args.rounds} --delay-ms {args.delay_ms} "
            f"--h4-in tcp:{args.port} --h4-out tcp:{args.port}"
        )
        send_cmd(master_fd, cmd)

        if not read_until(master_fd, f"H4 TCP listening on 127.0.0.1:{args.port}",
                          args.listen_timeout, transcript):
            print("timeout waiting for H4 TCP listener", file=sys.stderr)
            return 1

        inject_h4(args.port, H4_RESET_COMPLETE)
        read_until(master_fd, "nsh>", args.command_timeout, transcript)

        text = "".join(transcript)
        if "btvhcid: h4-in packet=" not in text:
            # The SIM host pthread may receive after the first btvhcid command
            # exits.  Run one short pump to consume the retained H4 rx buffer.
            send_cmd(master_fd,
                     f"btvhcid --rounds 1 --h4-in tcp:{args.port} --h4-out tcp:{args.port}")
            read_until(master_fd, "nsh>", args.command_timeout, transcript)
            text = "".join(transcript)

        required = [
            "bthwsim: H4 TCP listening",
            "bthwsim: H4 TCP recv bytes=7",
            "Bluetooth: hci0 Event packet",
            "Bluetooth: event 0x0e",
            "btvhcid: h4-in packet=",
            "total-h4-in=1",
        ]

        missing = [item for item in required if item not in text]
        if missing:
            print("btvhcid H4 TCP smoke: FAIL", file=sys.stderr)
            print("missing markers:", file=sys.stderr)
            for item in missing:
                print(f"  {item}", file=sys.stderr)
            print("\n--- transcript ---", file=sys.stderr)
            print(text, file=sys.stderr)
            return 1

        if args.log:
            Path(args.log).write_text(text, encoding="utf-8")

        print("btvhcid H4 TCP smoke: PASS")
        return 0
    finally:
        try:
            send_cmd(master_fd, "poweroff")
            drain(master_fd, 0.5, transcript)
        except OSError:
            pass
        proc.terminate()
        try:
            proc.wait(timeout=2.0)
        except subprocess.TimeoutExpired:
            proc.kill()
            proc.wait(timeout=2.0)
        os.close(master_fd)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--sim", default="build/nuttx-sim-bt1")
    parser.add_argument("--port", type=int, default=9101)
    parser.add_argument("--rounds", type=int, default=20)
    parser.add_argument("--delay-ms", type=int, default=100)
    parser.add_argument("--boot-timeout", type=float, default=10.0)
    parser.add_argument("--listen-timeout", type=float, default=5.0)
    parser.add_argument("--command-timeout", type=float, default=8.0)
    parser.add_argument("--log", default="build/logs/run-btvhcid-h4tcp-smoke.log")
    return run(parser.parse_args())


if __name__ == "__main__":
    raise SystemExit(main())
