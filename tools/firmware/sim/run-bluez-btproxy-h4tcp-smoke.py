#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Smoke-test BlueZ btproxy against the NuttX SIM btvhcid H4 TCP endpoint.

This does not claim full BlueZ integration by itself.  It verifies the first
real BlueZ userspace tool boundary: btproxy connects to btvhcid's tcp:<port>
H4 stream and then attempts to open the host Linux /dev/vhci side.
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


def run_btproxy(btproxy: Path, port: int, timeout: float) -> tuple[int | None, str]:
    cmd = [str(btproxy), "--connect", "127.0.0.1", "--port", str(port), "--debug"]
    proc = subprocess.Popen(
        cmd,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        errors="replace",
    )

    try:
        out, _ = proc.communicate(timeout=timeout)
        return proc.returncode, out
    except subprocess.TimeoutExpired:
        proc.terminate()
        try:
            out, _ = proc.communicate(timeout=2.0)
        except subprocess.TimeoutExpired:
            proc.kill()
            out, _ = proc.communicate(timeout=2.0)
        return None, out


def run_preflight(preflight: Path) -> tuple[int | None, str]:
    if not preflight.exists():
        return 127, f"missing preflight script: {preflight}\n"

    proc = subprocess.Popen(
        [str(preflight)],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        errors="replace",
    )

    out, _ = proc.communicate()
    return proc.returncode, out


def run(args: argparse.Namespace) -> int:
    sim = Path(args.sim).resolve()
    btproxy = Path(args.btproxy).resolve()
    preflight = Path(args.preflight).resolve()
    transcript: list[str] = []

    if not sim.exists():
        print(f"missing SIM binary: {sim}", file=sys.stderr)
        return 2
    if not btproxy.exists():
        print(f"missing BlueZ btproxy binary: {btproxy}", file=sys.stderr)
        return 2

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

    btproxy_rc: int | None = None
    btproxy_out = ""
    preflight_rc: int | None = None
    preflight_out = ""

    try:
        preflight_rc, preflight_out = run_preflight(preflight)

        if not read_until(master_fd, "nsh>", args.boot_timeout, transcript):
            print("timeout waiting for nsh prompt", file=sys.stderr)
            return 1

        send_cmd(master_fd,
                 f"btvhcid --rounds {args.rounds} --h4-in tcp:{args.port} --h4-out tcp:{args.port}")

        if not read_until(master_fd, f"H4 TCP listening on 127.0.0.1:{args.port}",
                          args.listen_timeout, transcript):
            print("timeout waiting for H4 TCP listener", file=sys.stderr)
            return 1

        btproxy_rc, btproxy_out = run_btproxy(btproxy, args.port,
                                             args.btproxy_timeout)
        drain_pty(master_fd, args.post_btproxy_drain, transcript)
        text = "".join(transcript)

        log_text = (
            text + "\n--- preflight ---\n" + preflight_out +
            f"preflight_rc={preflight_rc}\n" +
            "\n--- btproxy ---\n" + btproxy_out +
            f"\n--- result ---\nbtproxy_rc={btproxy_rc}\n"
        )
        if args.log:
            Path(args.log).write_text(log_text, encoding="utf-8")

        connected = "bthwsim: H4 TCP client accepted" in text
        attempted_vhci = "Opening virtual device" in btproxy_out
        vhci_missing = "Failed to open /dev/vhci device" in btproxy_out

        if connected and attempted_vhci:
            if vhci_missing:
                print("BlueZ btproxy H4 TCP smoke: PARTIAL")
                print("btproxy connected to NuttX H4 TCP, but host /dev/vhci is unavailable")
                if preflight_rc not in (0, None):
                    print(f"preflight_rc={preflight_rc}")
                return 0

            print("BlueZ btproxy H4 TCP smoke: PASS")
            if btproxy_rc is None:
                print("btproxy stayed running until timeout and was terminated by the smoke harness")
            else:
                print(f"btproxy exited rc={btproxy_rc}")
            return 0

        print("BlueZ btproxy H4 TCP smoke: FAIL", file=sys.stderr)
        if not connected:
            print("missing NuttX TCP accept marker", file=sys.stderr)
        if not attempted_vhci:
            print("btproxy did not reach /dev/vhci open path", file=sys.stderr)
        print("\n--- transcript ---", file=sys.stderr)
        print(log_text, file=sys.stderr)
        return 1
    finally:
        try:
            send_cmd(master_fd, "poweroff")
            drain_pty(master_fd, 0.5, transcript)
        except OSError:
            pass
        sim_proc.terminate()
        try:
            sim_proc.wait(timeout=2.0)
        except subprocess.TimeoutExpired:
            sim_proc.kill()
            sim_proc.wait(timeout=2.0)
        os.close(master_fd)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--sim", default="build/nuttx-sim-bt1")
    parser.add_argument("--btproxy", default="/home/uan/Feather-develop-BT/third/bluez/tools/btproxy")
    parser.add_argument("--preflight", default="tools/firmware/sim/preflight-bluez-vhci.sh")
    parser.add_argument("--port", type=int, default=9103)
    parser.add_argument("--rounds", type=int, default=40)
    parser.add_argument("--boot-timeout", type=float, default=10.0)
    parser.add_argument("--listen-timeout", type=float, default=5.0)
    parser.add_argument("--btproxy-timeout", type=float, default=5.0)
    parser.add_argument("--post-btproxy-drain", type=float, default=1.0)
    parser.add_argument("--log", default="build/logs/run-bluez-btproxy-h4tcp-smoke.log")
    return run(parser.parse_args())


if __name__ == "__main__":
    raise SystemExit(main())
