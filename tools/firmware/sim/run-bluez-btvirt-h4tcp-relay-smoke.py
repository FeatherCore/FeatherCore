#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Relay BlueZ btvirt TCP H4 server to NuttX SIM btvhcid tcp:<port>.

BlueZ btvirt's TCP mode is a server backed by BlueZ emulator/btdev and does not
require host /dev/vhci.  NuttX btvhcid tcp:<port> is also a server.  This smoke
starts both and bridges the two TCP streams, proving that rebuilt BlueZ btdev can
act as the controller-side H4 peer for the NuttX upstream Linux Bluetooth path.
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


def wait_tcp(port: int, timeout: float) -> socket.socket:
    deadline = time.monotonic() + timeout
    last_error: OSError | None = None
    while time.monotonic() < deadline:
        try:
            sock = socket.create_connection(("127.0.0.1", port), timeout=1.0)
            sock.setblocking(False)
            return sock
        except OSError as exc:
            last_error = exc
            time.sleep(0.05)
    raise TimeoutError(f"timeout connecting to 127.0.0.1:{port}: {last_error}")


def relay(left: socket.socket, right: socket.socket, seconds: float) -> tuple[int, int]:
    deadline = time.monotonic() + seconds
    left_to_right = 0
    right_to_left = 0
    sockets = [left, right]

    while time.monotonic() < deadline:
        readable, _, _ = select.select(sockets, [], [], 0.05)
        for sock in readable:
            try:
                data = sock.recv(4096)
            except BlockingIOError:
                continue
            except OSError:
                return left_to_right, right_to_left

            if not data:
                return left_to_right, right_to_left

            peer = right if sock is left else left
            try:
                peer.sendall(data)
            except OSError:
                return left_to_right, right_to_left

            if sock is left:
                left_to_right += len(data)
            else:
                right_to_left += len(data)

    return left_to_right, right_to_left


def collect_proc(proc: subprocess.Popen[str]) -> str:
    if proc.stdout is None:
        return ""
    out = []
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
    relay_lr = 0
    relay_rl = 0

    if not sim.exists():
        print(f"missing SIM binary: {sim}", file=sys.stderr)
        return 2
    if not btvirt.exists():
        print(f"missing BlueZ btvirt binary: {btvirt}", file=sys.stderr)
        return 2

    btvirt_proc = start_btvirt(btvirt, args.btvirt_port)
    time.sleep(0.2)
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

    left = None
    right = None

    try:
        if not read_until(master_fd, "nsh>", args.boot_timeout, transcript):
            print("timeout waiting for nsh prompt", file=sys.stderr)
            return 1

        send_cmd(master_fd,
                 f"btvhcid --rounds {args.rounds} --h4-in tcp:{args.nuttx_port} --h4-out tcp:{args.nuttx_port}")

        if not read_until(master_fd, f"H4 TCP listening on 127.0.0.1:{args.nuttx_port}",
                          args.listen_timeout, transcript):
            print("timeout waiting for NuttX H4 TCP listener", file=sys.stderr)
            return 1

        left = wait_tcp(args.nuttx_port, args.connect_timeout)
        right = wait_tcp(args.btvirt_port, args.connect_timeout)
        relay_lr, relay_rl = relay(left, right, args.relay_seconds)
        read_until(master_fd, "nsh>", args.command_timeout, transcript)
        drain_pty(master_fd, 0.5, transcript)
        btvirt_out += collect_proc(btvirt_proc)

        text = "".join(transcript)
        log_text = (
            text + "\n--- btvirt ---\n" + btvirt_out +
            "\n--- relay ---\n" +
            f"nuttx_to_btvirt={relay_lr}\n" +
            f"btvirt_to_nuttx={relay_rl}\n"
        )
        if args.log:
            Path(args.log).write_text(log_text, encoding="utf-8")

        if relay_lr <= 0 or relay_rl <= 0:
            print("BlueZ btvirt H4 TCP relay smoke: FAIL", file=sys.stderr)
            print(f"nuttx_to_btvirt={relay_lr} btvirt_to_nuttx={relay_rl}", file=sys.stderr)
            print("\n--- transcript ---", file=sys.stderr)
            print(log_text, file=sys.stderr)
            return 1

        if "Bluetooth: hci0 Event packet" not in text:
            print("BlueZ btvirt H4 TCP relay smoke: FAIL", file=sys.stderr)
            print("missing NuttX upstream HCI event marker", file=sys.stderr)
            return 1

        print("BlueZ btvirt H4 TCP relay smoke: PASS")
        print(f"nuttx_to_btvirt={relay_lr}")
        print(f"btvirt_to_nuttx={relay_rl}")
        return 0
    finally:
        for sock in (left, right):
            if sock is not None:
                try:
                    sock.close()
                except OSError:
                    pass
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
    parser.add_argument("--nuttx-port", type=int, default=9104)
    parser.add_argument("--btvirt-port", type=int, default=45551)
    parser.add_argument("--rounds", type=int, default=80)
    parser.add_argument("--boot-timeout", type=float, default=10.0)
    parser.add_argument("--listen-timeout", type=float, default=5.0)
    parser.add_argument("--connect-timeout", type=float, default=5.0)
    parser.add_argument("--relay-seconds", type=float, default=3.0)
    parser.add_argument("--command-timeout", type=float, default=8.0)
    parser.add_argument("--log", default="build/logs/run-bluez-btvirt-h4tcp-relay-smoke.log")
    return run(parser.parse_args())


if __name__ == "__main__":
    raise SystemExit(main())
