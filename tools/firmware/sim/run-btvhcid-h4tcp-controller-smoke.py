#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Exercise btvhcid tcp:<port> as a bidirectional H4 controller peer.

This is a small host-side controller-side smoke test.  It starts one NuttX SIM,
runs btvhcid from nsh, connects to its tcp:<port> H4 endpoint, reads HCI command
frames drained from the upstream VHCI path, and sends matching Command Complete
events back over the same H4 stream.
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


def h4_frame_length(buf: bytearray) -> int | None:
    if not buf:
        return None

    ptype = buf[0]
    if ptype == 0x01:
        if len(buf) < 4:
            return None
        return 4 + buf[3]
    if ptype == 0x02:
        if len(buf) < 5:
            return None
        return 5 + buf[3] + (buf[4] << 8)
    if ptype == 0x03:
        if len(buf) < 4:
            return None
        return 4 + buf[3]
    if ptype == 0x04:
        if len(buf) < 3:
            return None
        return 3 + buf[2]
    if ptype == 0x05:
        if len(buf) < 5:
            return None
        return 5 + buf[3] + (buf[4] << 8)

    del buf[0]
    return None


def command_complete(opcode: int, status: int = 0) -> bytes:
    return bytes([
        0x04,
        0x0e,
        0x04,
        0x01,
        opcode & 0xff,
        (opcode >> 8) & 0xff,
        status & 0xff,
    ])


def run_controller(port: int, timeout: float, max_replies: int) -> tuple[list[int], int, int, int]:
    deadline = time.monotonic() + timeout
    commands: list[int] = []
    replies = 0
    raw_bytes = 0
    drops = 0
    rx = bytearray()

    with socket.create_connection(("127.0.0.1", port), timeout=5.0) as sock:
        sock.setblocking(False)
        while time.monotonic() < deadline and replies < max_replies:
            ready, _, _ = select.select([sock], [], [], 0.05)
            if not ready:
                continue

            try:
                chunk = sock.recv(4096)
            except BlockingIOError:
                continue

            if not chunk:
                break

            raw_bytes += len(chunk)
            rx.extend(chunk)
            while True:
                before = len(rx)
                frame_len = h4_frame_length(rx)
                if len(rx) < before:
                    drops += before - len(rx)
                if frame_len is None or len(rx) < frame_len:
                    break

                frame = bytes(rx[:frame_len])
                del rx[:frame_len]

                if frame[0] != 0x01 or len(frame) < 4:
                    continue

                opcode = frame[1] | (frame[2] << 8)
                commands.append(opcode)
                sock.sendall(command_complete(opcode))
                replies += 1

    return commands, replies, raw_bytes, drops


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
        cwd=str(Path.cwd()),
        close_fds=True,
    )
    os.close(slave_fd)

    commands: list[int] = []
    replies = 0
    raw_bytes = 0
    drops = 0

    def save_log(extra: str = "") -> None:
        if args.log:
            Path(args.log).write_text(
                "".join(transcript) + extra,
                encoding="utf-8",
            )

    try:
        if not read_until(master_fd, "nsh>", args.boot_timeout, transcript):
            print("timeout waiting for nsh prompt", file=sys.stderr)
            save_log()
            return 1

        send_cmd(
            master_fd,
            f"btvhcid --rounds {args.rounds} "
            f"--h4-in tcp:{args.port} --h4-out tcp:{args.port}",
        )

        if not read_until(master_fd, f"H4 TCP listening on 127.0.0.1:{args.port}",
                          args.listen_timeout, transcript):
            print("timeout waiting for H4 TCP listener", file=sys.stderr)
            save_log()
            print("\n--- transcript ---", file=sys.stderr)
            print("".join(transcript), file=sys.stderr)
            return 1

        commands, replies, raw_bytes, drops = \
            run_controller(args.port, args.controller_timeout,
                           args.max_replies)
        read_until(master_fd, "nsh>", args.command_timeout, transcript)
        text = "".join(transcript)

        if args.log:
            save_log("\n--- controller ---\n" +
                     f"commands={[hex(op) for op in commands]}\n" +
                     f"replies={replies}\n" +
                     f"raw_bytes={raw_bytes}\n" +
                     f"drops={drops}\n")

        if replies <= 0:
            print("btvhcid H4 TCP controller smoke: FAIL", file=sys.stderr)
            print("no HCI command was read from the TCP H4 stream", file=sys.stderr)
            print(f"raw_bytes={raw_bytes} drops={drops}", file=sys.stderr)
            print("\n--- transcript ---", file=sys.stderr)
            print(text, file=sys.stderr)
            return 1

        if "Bluetooth: hci0 Event packet" not in text:
            print("btvhcid H4 TCP controller smoke: FAIL", file=sys.stderr)
            print("missing upstream HCI event marker", file=sys.stderr)
            print("\n--- transcript ---", file=sys.stderr)
            print(text, file=sys.stderr)
            return 1

        print("btvhcid H4 TCP controller smoke: PASS")
        print("commands=" + ",".join(f"0x{op:04x}" for op in commands))
        print(f"replies={replies}")
        return 0
    finally:
        try:
            send_cmd(master_fd, "poweroff")
            drain_pty(master_fd, 0.5, transcript)
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
    parser.add_argument("--port", type=int, default=9102)
    parser.add_argument("--rounds", type=int, default=30)
    parser.add_argument("--max-replies", type=int, default=8)
    parser.add_argument("--boot-timeout", type=float, default=10.0)
    parser.add_argument("--listen-timeout", type=float, default=5.0)
    parser.add_argument("--controller-timeout", type=float, default=5.0)
    parser.add_argument("--command-timeout", type=float, default=8.0)
    parser.add_argument("--log", default="build/logs/run-btvhcid-h4tcp-controller-smoke.log")
    return run(parser.parse_args())


if __name__ == "__main__":
    raise SystemExit(main())
