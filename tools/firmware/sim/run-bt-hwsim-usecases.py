#!/usr/bin/env python3
############################################################################
# tools/firmware/sim/run-bt-hwsim-usecases.py
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

"""Run BT/BLE hwsim usecases against NuttX sim binaries.

The target test model remains one sim instance per role.  This runner starts
one process for each role needed by a case, feeds that role's generated nsh
command file through stdin, captures per-role logs, then invokes the log
validator for PASS/FAIL reporting.
"""

from __future__ import annotations

import argparse
import json
import os
import select
import signal
import subprocess
import sys
import time
from dataclasses import dataclass
from pathlib import Path


SCRIPT_DIR = Path(__file__).resolve().parent
ROOT = SCRIPT_DIR.parents[2]
DEFAULT_OUT = ROOT / "build" / "bt-hwsim-usecases"
DEFAULT_MEDIUM_DIR = Path("/tmp/nuttx-bthwsim")
DEFAULT_CASE_TIMEOUT = 600.0
COMMAND_WRITE_TIMEOUT = 10.0


@dataclass(frozen=True)
class Case:
    name: str
    roles: tuple[str, ...]


CASES: tuple[Case, ...] = (
    Case("bt-basic", ("bt1", "bt2")),
    Case("ble-basic", ("ble1", "ble2")),
    Case("ble-ip-ping", ("ble1", "ble2")),
    Case("ble-ip-reconnect-stress", ("ble1", "ble2")),
    Case("ble-ip-iperf-tcp", ("ble1", "ble2")),
    Case("ble-ip-iperf-tcp-reverse", ("ble2", "ble1")),
    Case("ble-ip-iperf-udp", ("ble1", "ble2")),
    Case("ble-ip-iperf-udp-reverse", ("ble2", "ble1")),
    Case("ble-ip-closeout-full", ("ble1", "ble2")),
    Case("bluez-ipsp-closeout-full", ("ble1", "ble2")),
    Case("bluez-daemon-ipsp-closeout-full", ("ble1", "ble2")),
    Case("bluez-net-current-complete-closeout", ("bt1", "bt2", "ble1", "ble2")),
    Case("bluez-net-upstream-convergence-closeout", ("bt1", "bt2", "ble1", "ble2")),
    Case("bluez-current-functional-closeout", ("bt1", "bt2", "ble1", "ble2")),
    Case("hci-bredr-medium", ("bt1", "bt2")),
    Case("l2cap-native-basic", ("bt1", "bt2")),
    Case("hci-le-lifecycle", ("ble1",)),
    Case("hci-le-reconnect-stress", ("ble1",)),
    Case("hci-le-medium", ("ble1", "ble2")),
    Case("hci-le-pairing", ("ble1", "ble2")),
    Case("mgmt-control", ("ble1",)),
    Case("bluez-mgmt-control", ("ble1",)),
    Case("bluez-mgmt-pair-noio", ("ble1",)),
    Case("bluez-mgmt-user-confirm", ("ble1",)),
    Case("bluez-mgmt-user-confirm-neg", ("ble1",)),
    Case("bluez-mgmt-passkey", ("ble1",)),
    Case("bluez-mgmt-passkey-neg", ("ble1",)),
    Case("bluez-mgmt-cancel-pair", ("ble1",)),
    Case("bluez-mgmt-cancel-pair-pending", ("ble1",)),
    Case("bluez-mgmt-daemon-bootstrap", ("ble1",)),
    Case("bluez-mgmt-pair-unpair", ("ble1",)),
    Case("bluez-mgmt-lifecycle", ("ble1",)),
    Case("bluez-mgmt-reconnect-stress", ("ble1",)),
    Case("bluez-mgmt-error-path", ("ble1",)),
    Case("bluez-daemon-smoke", ("ble1",)),
    Case("bluez-daemon-reconnect-stress", ("ble1",)),
    Case("bluez-daemon-device-policy", ("ble1",)),
    Case("bluez-daemon-discovery-peer", ("ble2", "ble1")),
    Case("bluez-daemon-pairing-matrix", ("ble1",)),
    Case("bluez-daemon-mgmt-full-lifecycle", ("ble2", "ble1")),
    Case("bluez-btmon-monitor", ("ble1",)),
    Case("bluez-hciioctl-basic", ("ble1",)),
    Case("bluez-hciraw-command", ("ble1",)),
    Case("bluez-hciuser-command", ("ble1",)),
    Case("bluez-hciuser-monitor", ("ble1",)),
    Case("bluez-hciuser-sequence-monitor", ("ble1",)),
    Case("bluez-hciuser-error-monitor", ("ble1",)),
    Case("bluez-hciuser-init-sequence-monitor", ("ble1",)),
    Case("bluez-hciuser-full-abi", ("ble1",)),
    Case("bluez-hciuser-adv-scan-medium", ("ble1", "ble2")),
    Case("bluez-hci-mgmt-socket-closeout-full", ("ble1", "ble2")),
    Case("mgmt-noio", ("ble1",)),
    Case("mgmt-confirm", ("ble1",)),
    Case("mgmt-passkey", ("ble1",)),
    Case("bnep-session", ("bt1",)),
    Case("bneptest-fd-probe", ("bt1",)),
    Case("bluez-bneptest-fd-handoff", ("bt1",)),
    Case("bluez-bneptest-ping", ("bt1", "bt2")),
    Case("bluez-network-ping", ("bt1", "bt2")),
    Case("bluez-network-daemon-profile", ("bt1", "bt2")),
    Case("bluez-network-daemon-role-matrix", ("bt1", "bt2")),
    Case("bluez-network-daemon-full-lifecycle", ("bt1", "bt2")),
    Case("bluez-network-closeout-full", ("bt1", "bt2")),
    Case("bluez-network-error-path", ("bt1",)),
    Case("bluez-network-iperf-tcp", ("bt1", "bt2")),
    Case("bluez-network-iperf-tcp-reverse", ("bt2", "bt1")),
    Case("bluez-network-iperf-udp", ("bt1", "bt2")),
    Case("bluez-network-iperf-udp-reverse", ("bt2", "bt1")),
    Case("bluez-network-iperf-tcp-soak", ("bt1", "bt2")),
    Case("bluez-network-iperf-matrix", ("bt1", "bt2")),
    Case("bluez-network-frag-ping", ("bt1", "bt2")),
    Case("bluez-network-jumbo-ping", ("bt1", "bt2")),
    Case("bluez-network-mtu-ping", ("bt1", "bt2")),
    Case("bluez-network-mtu-soak", ("bt1", "bt2")),
    Case("bluez-network-mtu-reconnect-stress", ("bt1", "bt2")),
    Case("bluez-network-reconnect", ("bt1", "bt2")),
    Case("bluez-network-reconnect-stress", ("bt1", "bt2")),
    Case("bluez-bneptest-iperf-tcp", ("bt1", "bt2")),
    Case("bluez-bneptest-iperf-tcp-reverse", ("bt2", "bt1")),
    Case("bluez-bneptest-iperf-udp", ("bt1", "bt2")),
    Case("bluez-bneptest-iperf-udp-reverse", ("bt2", "bt1")),
    Case("bluez-bneptest-reconnect", ("bt1", "bt2")),
    Case("bluez-bneptest-reconnect-stress", ("bt1", "bt2")),
    Case("bneptest-ping", ("bt1", "bt2")),
    Case("bneptest-reconnect", ("bt1", "bt2")),
    Case("bneptest-reconnect-stress", ("bt1", "bt2")),
    Case("bnep-ping", ("bt1", "bt2")),
    Case("bnep-iperf-tcp", ("bt1", "bt2")),
    Case("bnep-iperf-tcp-reverse", ("bt2", "bt1")),
    Case("bnep-iperf-udp", ("bt1", "bt2")),
    Case("bnep-iperf-udp-reverse", ("bt2", "bt1")),
    Case("a2dp", ("bt1", "bt2")),
    Case("a2dp-extended", ("bt1", "bt2")),
    Case("bluez-a2dp-signaling", ("bt1", "bt2")),
    Case("bluez-a2dp-signaling-native", ("bt1", "bt2")),
    Case("bluez-a2dp-transaction", ("bt1", "bt2")),
    Case("bluez-a2dp-transaction-native", ("bt1", "bt2")),
    Case("bluez-a2dp-transaction-reconnect-native", ("bt1", "bt2")),
    Case("bluez-a2dp-extended", ("bt1", "bt2")),
    Case("a2dp-media", ("bt1", "bt2")),
    Case("bluez-a2dp-media", ("bt1", "bt2")),
    Case("bluez-a2dp-profile", ("bt1", "bt2")),
    Case("bluez-a2dp-profile-reconnect", ("bt1", "bt2")),
    Case("bluez-a2dp-extended-profile", ("bt1", "bt2")),
    Case("bluez-a2dp-transport", ("bt1", "bt2")),
    Case("bluez-a2dp-endpoint-transport", ("bt1", "bt2")),
    Case("bluez-a2dp-sbc-codec-transport", ("bt1", "bt2")),
    Case("bluez-a2dp-sbc-codec-extended", ("bt1", "bt2")),
    Case("bluez-a2dp-sbc-codec-abort", ("bt1", "bt2")),
    Case("bluez-a2dp-sbc-codec-reconnect", ("bt1", "bt2")),
    Case("bluez-a2dp-sbc-codec-concurrent", ("bt1", "bt2")),
    Case("bluez-a2dp-transport-reconnect", ("bt1", "bt2")),
    Case("bluez-a2dp-transport-bidir", ("bt1", "bt2")),
    Case("bluez-a2dp-transport-bidir-teardown", ("bt1", "bt2")),
    Case("bluez-a2dp-avrcp-control", ("bt1", "bt2")),
    Case("bluez-a2dp-avrcp-browsing", ("bt1", "bt2")),
    Case("bluez-a2dp-avrcp-notification", ("bt1", "bt2")),
    Case("bluez-a2dp-avrcp-absolute-volume", ("bt1", "bt2")),
    Case("bluez-a2dp-avrcp-metadata", ("bt1", "bt2")),
    Case("bluez-a2dp-avrcp-player-settings-list", ("bt1", "bt2")),
    Case("bluez-a2dp-avrcp-player-settings-values", ("bt1", "bt2")),
    Case("bluez-a2dp-avrcp-player-settings-value-text", ("bt1", "bt2")),
    Case("bluez-a2dp-avrcp-player-settings-notification", ("bt1", "bt2")),
    Case("bluez-a2dp-avrcp-player-settings-error", ("bt1", "bt2")),
    Case("bluez-a2dp-avrcp-addressed-player", ("bt1", "bt2")),
    Case("bluez-a2dp-avrcp-player-settings", ("bt1", "bt2")),
    Case("bluez-a2dp-avrcp-player-settings-set", ("bt1", "bt2")),
    Case("bluez-daemon-a2dp-full", ("bt1", "bt2")),
    Case("bluez-daemon-a2dp-reconnect-full", ("bt1", "bt2")),
    Case("bluez-daemon-a2dp-dbus-client-full", ("bt1", "bt2")),
    Case("bluez-daemon-a2dp-dbus-client-busy", ("bt1", "bt2")),
    Case("bluez-daemon-a2dp-full-concurrent", ("bt1", "bt2")),
    Case("bluez-daemon-a2dp-full-concurrent-reconnect", ("bt1", "bt2")),
    Case("bluez-daemon-a2dp-full-concurrent-soak", ("bt1", "bt2")),
    Case("bluez-daemon-a2dp-integrated-profile", ("bt1", "bt2")),
    Case("bluez-daemon-a2dp-integrated-reconnect", ("bt1", "bt2")),
    Case("bluez-daemon-a2dp-session-ownership", ("bt1", "bt2")),
    Case("bluez-daemon-a2dp-error-policy", ("bt1", "bt2")),
    Case("bluez-daemon-a2dp-upstream-session", ("bt1", "bt2")),
    Case("bluez-daemon-a2dp-upstream-reconnect", ("bt1", "bt2")),
    Case("bluez-daemon-a2dp-upstream-transactions", ("bt1", "bt2")),
    Case("bluez-daemon-a2dp-media-transport-fd", ("bt1", "bt2")),
    Case("bluez-daemon-a2dp-codec-policy", ("bt1", "bt2")),
    Case("bluez-daemon-a2dp-closeout-full", ("bt1", "bt2")),
    Case("bluez-a2dp-current-complete-closeout", ("bt1", "bt2")),
    Case("bluez-a2dp-upstream-convergence-closeout", ("bt1", "bt2")),
    Case("bluez-basic-mgmt-flow", ("ble1", "ble2")),
    Case("bluez-basic-scan-connect-auth-flow", ("ble2", "ble1")),
    Case("bluez-basic-upstream-convergence-closeout", ("bt1", "bt2", "ble2", "ble1")),
    Case("bluez-hid-hogp-profile-closeout", ("bt1", "bt2", "ble2", "ble1")),
    Case("bluez-hfp-hsp-profile-closeout", ("bt1", "bt2")),
    Case("bluez-obex-pbap-opp-profile-closeout", ("bt1", "bt2")),
    Case("bluez-obex-map-mns-profile-closeout", ("bt1", "bt2")),
    Case("bluez-obex-ftp-sync-profile-closeout", ("bt1", "bt2")),
    Case("bluez-mesh-profile-closeout", ("ble1", "ble2")),
    Case("bluez-gatt-profile-closeout", ("ble1", "ble2")),
    Case("bluez-asha-profile-closeout", ("ble1", "ble2")),
    Case("bluez-obex-bip-profile-closeout", ("bt1", "bt2")),
    Case("bluez-print-profile-closeout", ("bt1", "bt2")),
    Case("bluez-iap-profile-closeout", ("bt1", "bt2")),
    Case("bluez-midi-profile-closeout", ("ble1", "ble2")),
    Case("bluez-ranging-profile-closeout", ("ble1", "ble2")),
    Case("le-audio", ("ble1", "ble2")),
    Case("bluez-le-audio", ("ble1", "ble2")),
    Case("bluez-le-audio-profile", ("ble1", "ble2")),
    Case("bluez-le-audio-broadcast-restart", ("ble1", "ble2")),
    Case("bluez-le-audio-unicast-profile", ("ble1", "ble2")),
    Case("bluez-le-audio-transport", ("ble1", "ble2")),
    Case("bluez-le-audio-transport-reconnect", ("ble1", "ble2")),
    Case("bluez-le-audio-transport-bidir", ("ble1", "ble2")),
    Case("bluez-le-audio-transport-bidir-disable", ("ble1", "ble2")),
    Case("bluez-le-audio-transport-bidir-qos-update", ("ble1", "ble2")),
    Case("bluez-le-audio-transport-bidir-qos-reject", ("ble1", "ble2")),
    Case("bluez-le-audio-transport-bidir-qos-cancel", ("ble1", "ble2")),
    Case("bluez-le-audio-transport-bidir-release-reconfig", ("ble1", "ble2")),
    Case("bluez-le-audio-transport-bidir-reconnect", ("ble1", "ble2")),
    Case("bluez-le-audio-full-lifecycle", ("ble1", "ble2")),
    Case("bluez-le-audio-daemon-full-lifecycle", ("ble1", "ble2")),
    Case("bluez-le-audio-dbus-client-full", ("ble1", "ble2")),
    Case("bluez-le-audio-lc3-codec-transport", ("ble1", "ble2")),
    Case("bluez-le-audio-iso-dataplane-soak", ("ble1", "ble2")),
    Case("bluez-le-audio-controller-iso-dataplane-soak", ("ble1", "ble2")),
    Case("bluez-le-audio-daemon-profile-flow", ("ble1", "ble2")),
    Case("bluez-le-audio-controller-daemon-profile-flow", ("ble1", "ble2")),
    Case("bluez-le-audio-controller-daemon-profile-reconnect", ("ble1", "ble2")),
    Case("bluez-le-audio-controller-daemon-error-recovery", ("ble1", "ble2")),
    Case("bluez-le-audio-controller-bap-ascs-error-matrix", ("ble1", "ble2")),
    Case("bluez-le-audio-controller-daemon-dbus-ownership", ("ble1", "ble2")),
    Case("bluez-le-audio-bap-ascs-dbus-owner-recovery", ("ble1", "ble2")),
    Case("bluez-le-audio-controller-dbus-bap-ascs-reconnect", ("ble1", "ble2")),
    Case("bluez-le-audio-controller-daemon-full-stack", ("ble1", "ble2")),
    Case("bluez-le-audio-controller-daemon-full-stack-reconnect", ("ble1", "ble2")),
    Case("bluez-le-audio-controller-daemon-mainloop-cleanup", ("ble1", "ble2")),
    Case("bluez-le-audio-daemon-broadcast-profile-flow", ("ble1", "ble2")),
    Case("bluez-le-audio-controller-daemon-broadcast-profile-flow", ("ble1", "ble2")),
    Case("bluez-le-audio-controller-daemon-broadcast-reconnect", ("ble1", "ble2")),
    Case("bluez-le-audio-controller-lc3-bidir", ("ble1", "ble2")),
    Case("bluez-le-audio-coordinated-services", ("ble1", "ble2")),
    Case("bluez-le-audio-cap-csip-group", ("ble1", "ble2")),
    Case("bluez-le-audio-tmap-mcp-ccp-flow", ("ble1", "ble2")),
    Case("bluez-le-audio-broadcast-multibis", ("ble1", "ble2")),
    Case("bluez-le-audio-broadcast-multibis-reconnect", ("ble1", "ble2")),
    Case("bluez-le-audio-bass-scan-delegator", ("ble1", "ble2")),
    Case("bluez-le-audio-daemon-integrated-profile", ("ble1", "ble2")),
    Case("bluez-le-audio-daemon-integrated-profile-reconnect", ("ble1", "ble2")),
    Case("bluez-le-audio-bap-pacs-ascs-session", ("ble1", "ble2")),
    Case("bluez-le-audio-bap-pacs-ascs-reconnect-recovery", ("ble1", "ble2")),
    Case("bluez-le-audio-bap-pacs-ascs-metadata-reconfig", ("ble1", "ble2")),
    Case("bluez-le-audio-codec-qos-policy-matrix", ("ble1", "ble2")),
    Case("bluez-le-audio-role-soak", ("ble1", "ble2")),
    Case("bluez-le-audio-umbrella", ("ble1", "ble2")),
    Case("bluez-le-audio-controller-setup", ("ble1",)),
    Case("bluez-le-audio-controller-reconnect", ("ble1",)),
    Case("bluez-hid-upstream-convergence-closeout",
         ("bt1", "bt2", "ble1", "ble2")),
    Case("bluez-gatt-upstream-convergence-closeout", ("ble1", "ble2")),
)


ROLE_BINARIES: dict[str, str] = {
    "bt1": "nuttx-sim-bt1",
    "bt2": "nuttx-sim-bt2",
    "ble1": "nuttx-sim-ble1",
    "ble2": "nuttx-sim-ble2",
}

CASE_MIN_SETTLE_DELAYS: dict[str, float] = {
    "ble-ip-reconnect-stress": 55.0,
    "ble-ip-closeout-full": 65.0,
    "bluez-ipsp-closeout-full": 65.0,
    "bluez-daemon-ipsp-closeout-full": 65.0,
    "bluez-net-current-complete-closeout": 95.0,
    "bluez-net-upstream-convergence-closeout": 95.0,
    "bluez-current-functional-closeout": 240.0,
    "bluez-hci-mgmt-socket-closeout-full": 120.0,
    "bluez-network-iperf-matrix": 45.0,
    "bluez-network-frag-ping": 22.0,
    "bluez-network-jumbo-ping": 22.0,
    "bluez-network-mtu-ping": 18.0,
    "bluez-network-mtu-soak": 34.0,
    "bluez-network-mtu-reconnect-stress": 38.0,
    "bluez-network-closeout-full": 55.0,
    "bluez-a2dp-current-complete-closeout": 70.0,
    "bluez-a2dp-upstream-convergence-closeout": 70.0,
}


def case_by_name(name: str) -> Case | None:
    for case in CASES:
        if case.name == name:
            return case
    return None


def ensure_command_files(out_dir: Path) -> None:
    subprocess.run(
        [str(SCRIPT_DIR / "test-bt-hwsim-usecases.sh"), "write",
         str(out_dir)],
        cwd=str(ROOT),
        check=True,
    )


def role_binary(role: str) -> Path:
    return ROOT / "build" / ROLE_BINARIES[role]


def close_stdin(procs: list[subprocess.Popen[bytes]]) -> None:
    for proc in procs:
        stdin = proc.stdin
        if stdin is not None and not stdin.closed:
            try:
                os.close(stdin.fileno())
            except (BrokenPipeError, OSError):
                pass
            proc.stdin = None


def wait_procs(procs: list[subprocess.Popen[bytes]], timeout: float) -> None:
    deadline = time.monotonic() + timeout

    for proc in procs:
        remaining = max(0.0, deadline - time.monotonic())
        if proc.poll() is None:
            try:
                proc.wait(timeout=remaining)
            except subprocess.TimeoutExpired:
                pass


def terminate(procs: list[subprocess.Popen[bytes]]) -> None:
    close_stdin(procs)
    wait_procs(procs, 0.5)

    for proc in procs:
        if proc.poll() is None:
            try:
                os.killpg(os.getpgid(proc.pid), signal.SIGTERM)
            except ProcessLookupError:
                pass

    wait_procs(procs, 0.5)

    for proc in procs:
        if proc.poll() is None:
            try:
                os.killpg(os.getpgid(proc.pid), signal.SIGKILL)
            except ProcessLookupError:
                pass

    wait_procs(procs, 1.0)


def reset_medium(medium_dir: Path = DEFAULT_MEDIUM_DIR) -> None:
    medium_dir.mkdir(parents=True, exist_ok=True)

    for pattern in (
        "bt-hwsim-*.bin",
        "bt-hwsim-*.bin.*.off",
        "bt-hwsim-*.bin.raw.*.off",
        "bt-h4-in.*.off",
    ):
        for path in medium_dir.glob(pattern):
            if path.is_file():
                path.unlink()


def load_commands(path: Path) -> list[str]:
    commands: list[str] = []

    for raw in path.read_text().splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue

        commands.append(line)

    return commands


def send_command(proc: subprocess.Popen[bytes], line: str) -> None:
    if proc.stdin is None:
        raise RuntimeError("sim process has no stdin")

    if proc.poll() is not None:
        raise RuntimeError(f"sim exited before command: {line}")

    fd = proc.stdin.fileno()
    data = (line + "\n").encode()
    deadline = time.monotonic() + COMMAND_WRITE_TIMEOUT

    while data:
        if proc.poll() is not None:
            raise RuntimeError(f"sim exited before command: {line}")

        remaining = deadline - time.monotonic()
        if remaining <= 0.0:
            raise TimeoutError(f"timed out writing command: {line}")

        _, writable, _ = select.select([], [fd], [], remaining)
        if not writable:
            raise TimeoutError(f"timed out writing command: {line}")

        try:
            written = os.write(fd, data)
        except BrokenPipeError as exc:
            raise RuntimeError(
                f"sim closed stdin before command: {line}") from exc

        data = data[written:]


def send_commands(procs: list[subprocess.Popen[bytes]],
                  command_files: list[Path], line_delay: float) -> None:
    per_role = [load_commands(path) for path in command_files]
    max_lines = max((len(commands) for commands in per_role), default=0)

    for line_index in range(max_lines):
        for proc, commands in zip(procs, per_role):
            if line_index >= len(commands):
                continue

            line = commands[line_index]

            send_command(proc, line)
            time.sleep(line_delay)


def case_timeout_handler(_signum: int, _frame: object) -> None:
    raise TimeoutError("case timed out")


def run_case(case: Case, out_dir: Path, startup_delay: float,
             line_delay: float, settle_delay: float,
             case_timeout: float) -> dict[str, object]:
    procs: list[subprocess.Popen[bytes]] = []
    logs: dict[str, Path] = {}
    result: dict[str, object] = {
        "case": case.name,
        "roles": list(case.roles),
        "started": [],
        "logs": {},
        "run_error": "",
    }

    old_handler = signal.signal(signal.SIGALRM, case_timeout_handler)
    signal.setitimer(signal.ITIMER_REAL, case_timeout)

    try:
        reset_medium()

        for role in case.roles:
            binary = role_binary(role)
            command_file = out_dir / f"{case.name}.{role}.nsh"
            log_file = out_dir / f"{case.name}.{role}.log"

            if not binary.exists():
                raise FileNotFoundError(f"missing sim binary: {binary}")
            if not command_file.exists():
                raise FileNotFoundError(
                    f"missing command file: {command_file}")

            log = log_file.open("wb", buffering=0)
            proc = subprocess.Popen(
                [str(binary)],
                stdin=subprocess.PIPE,
                stdout=log,
                stderr=subprocess.STDOUT,
                cwd=str(ROOT),
                bufsize=0,
                preexec_fn=os.setsid,
            )
            procs.append(proc)
            logs[role] = log_file
            result["started"].append(role)
            result["logs"][role] = str(log_file)
            time.sleep(startup_delay)

        command_files = [
            out_dir / f"{case.name}.{role}.nsh"
            for role in case.roles
        ]
        send_commands(procs, command_files, line_delay)
        close_stdin(procs)

        time.sleep(settle_delay)
    except Exception as exc:  # noqa: BLE001 - report and still clean up
        result["run_error"] = str(exc)
    finally:
        signal.setitimer(signal.ITIMER_REAL, 0.0)
        signal.signal(signal.SIGALRM, old_handler)
        terminate(procs)

    return result


def validate(out_dir: Path, names: list[str]) -> int:
    cmd = [
        str(SCRIPT_DIR / "validate-bt-hwsim-usecases.py"),
        "--log-dir",
        str(out_dir),
    ]
    for name in names:
        cmd.extend(["--case", name])

    return subprocess.run(cmd, cwd=str(ROOT)).returncode


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Run BT/BLE hwsim usecases against sim binaries")
    parser.add_argument(
        "--out-dir",
        default=str(DEFAULT_OUT),
        help="Directory for generated nsh files and captured logs")
    parser.add_argument(
        "--case",
        action="append",
        choices=[case.name for case in CASES],
        help="Case to run. May be passed more than once.")
    parser.add_argument(
        "--skip-generate",
        action="store_true",
        help="Use existing .nsh files instead of regenerating them")
    parser.add_argument(
        "--no-validate",
        action="store_true",
        help="Do not run validate-bt-hwsim-usecases.py after execution")
    parser.add_argument(
        "--startup-delay",
        type=float,
        default=4.0,
        help="Seconds to wait after starting each sim role")
    parser.add_argument(
        "--line-delay",
        type=float,
        default=0.7,
        help="Seconds to wait after each nsh command")
    parser.add_argument(
        "--settle-delay",
        type=float,
        default=10.0,
        help="Seconds to wait before terminating sim roles")
    parser.add_argument(
        "--case-timeout",
        type=float,
        default=DEFAULT_CASE_TIMEOUT,
        help="Maximum seconds to allow one usecase before cleanup")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    out_dir = Path(args.out_dir).resolve()
    out_dir.mkdir(parents=True, exist_ok=True)

    names = args.case if args.case else [case.name for case in CASES]
    cases = [case_by_name(name) for name in names]
    if any(case is None for case in cases):
        print("unknown case requested", file=sys.stderr)
        return 2

    if not args.skip_generate:
        ensure_command_files(out_dir)

    run_failed = False
    run_results: list[dict[str, object]] = []
    for case in cases:
        assert case is not None
        settle_delay = max(args.settle_delay,
                           CASE_MIN_SETTLE_DELAYS.get(case.name, 0.0))
        started = time.monotonic()
        print(f"RUN-START {case.name}", flush=True)
        result = run_case(case, out_dir, args.startup_delay,
                          args.line_delay, settle_delay,
                          args.case_timeout)
        elapsed = time.monotonic() - started
        result["elapsed"] = round(elapsed, 3)
        run_results.append(result)
        if result["run_error"]:
            run_failed = True
            print(f"RUN-ERROR {case.name} elapsed={elapsed:.1f}s: "
                  f"{result['run_error']}",
                  file=sys.stderr, flush=True)
        else:
            print(f"RUN-DONE {case.name} elapsed={elapsed:.1f}s", flush=True)

    validate_rc = 0
    if not args.no_validate:
        validate_rc = validate(out_dir, names)

    manifest = {
        "out_dir": str(out_dir),
        "cases": names,
        "run_failed": run_failed,
        "validate_rc": validate_rc,
        "passed": not run_failed and validate_rc == 0,
        "results": run_results,
    }
    manifest_path = out_dir / "run-results.json"
    manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True))
    print(f"RUN-MANIFEST {manifest_path}", flush=True)

    return 1 if run_failed or validate_rc != 0 else 0


if __name__ == "__main__":
    sys.exit(main())
