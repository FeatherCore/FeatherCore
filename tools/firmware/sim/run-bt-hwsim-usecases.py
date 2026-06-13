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
    Case("bluez-network-error-path", ("bt1",)),
    Case("bluez-network-iperf-tcp", ("bt1", "bt2")),
    Case("bluez-network-iperf-tcp-reverse", ("bt2", "bt1")),
    Case("bluez-network-iperf-udp", ("bt1", "bt2")),
    Case("bluez-network-iperf-udp-reverse", ("bt2", "bt1")),
    Case("bluez-network-iperf-tcp-soak", ("bt1", "bt2")),
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
)


ROLE_BINARIES: dict[str, str] = {
    "bt1": "nuttx-sim-bt1",
    "bt2": "nuttx-sim-bt2",
    "ble1": "nuttx-sim-ble1",
    "ble2": "nuttx-sim-ble2",
}

CASE_MIN_SETTLE_DELAYS: dict[str, float] = {
    "ble-ip-reconnect-stress": 55.0,
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


def terminate(procs: list[subprocess.Popen[bytes]]) -> None:
    for proc in procs:
        if proc.poll() is None:
            try:
                os.killpg(os.getpgid(proc.pid), signal.SIGTERM)
            except ProcessLookupError:
                pass

    time.sleep(0.5)

    for proc in procs:
        if proc.poll() is None:
            try:
                os.killpg(os.getpgid(proc.pid), signal.SIGKILL)
            except ProcessLookupError:
                pass


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

    proc.stdin.write((line + "\n").encode())
    proc.stdin.flush()


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


def run_case(case: Case, out_dir: Path, startup_delay: float,
             line_delay: float, settle_delay: float) -> dict[str, object]:
    procs: list[subprocess.Popen[bytes]] = []
    logs: dict[str, Path] = {}
    result: dict[str, object] = {
        "case": case.name,
        "roles": list(case.roles),
        "started": [],
        "logs": {},
        "run_error": "",
    }

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

        time.sleep(settle_delay)
    except Exception as exc:  # noqa: BLE001 - report and still clean up
        result["run_error"] = str(exc)
    finally:
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
        result = run_case(case, out_dir, args.startup_delay,
                          args.line_delay, settle_delay)
        run_results.append(result)
        if result["run_error"]:
            run_failed = True
            print(f"RUN-ERROR {case.name}: {result['run_error']}",
                  file=sys.stderr)
        else:
            print(f"RUN-DONE {case.name}")

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
    print(f"RUN-MANIFEST {manifest_path}")

    return 1 if run_failed or validate_rc != 0 else 0


if __name__ == "__main__":
    sys.exit(main())
