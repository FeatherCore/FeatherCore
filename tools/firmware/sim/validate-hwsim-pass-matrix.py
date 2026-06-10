#!/usr/bin/env python3
############################################################################
# tools/firmware/sim/validate-hwsim-pass-matrix.py
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

"""Run repeatable NuttX hwsim AP+STA validation cases.

This runner intentionally covers the common AP+STA1 PASS rows that share the
same hostapd/wpa_supplicant shape. Feature flows that require extra control
commands, such as P2P, WPS, DPP, FT roam, WNM action exchange, TWT, and PS
testmode, remain separate manual/specialized validations.
"""

from __future__ import annotations

import argparse
import json
import os
import shutil
import signal
import subprocess
import sys
import time
from dataclasses import dataclass, asdict
from pathlib import Path


@dataclass(frozen=True)
class Case:
    name: str
    ap_config: str
    sta_config: str
    timeout: int = 70
    note: str = ""


CASES: list[Case] = [
    Case("open", "hostapd-hwsim.conf", "wpa_supplicant-hwsim.conf"),
    Case("wpa2-psk-ccmp", "hostapd-hwsim-wpa2-psk-ccmp.conf",
         "wpa_supplicant-hwsim-wpa2-psk-ccmp.conf"),
    Case("wpa2-psk-hex", "hostapd-hwsim-wpa2-psk-hex.conf",
         "wpa_supplicant-hwsim-wpa2-psk-hex.conf"),
    Case("wpa2-psk-tkip", "hostapd-hwsim-wpa2-psk-tkip.conf",
         "wpa_supplicant-hwsim-wpa2-psk-tkip.conf"),
    Case("wpa-mixed", "hostapd-hwsim-wpa-mixed.conf",
         "wpa_supplicant-hwsim-wpa-mixed.conf"),
    Case("wpa2-pmf-optional", "hostapd-hwsim-wpa2-psk-pmf-optional.conf",
         "wpa_supplicant-hwsim-wpa2-psk-pmf-optional.conf"),
    Case("wpa2-pmf-required", "hostapd-hwsim-wpa2-psk-pmf.conf",
         "wpa_supplicant-hwsim-wpa2-psk-pmf.conf"),
    Case("wpa2-ocv", "hostapd-hwsim-wpa2-psk-ocv.conf",
         "wpa_supplicant-hwsim-wpa2-psk-ocv.conf"),
    Case("fils-sha256", "hostapd-hwsim-fils-sha256.conf",
         "wpa_supplicant-hwsim-fils-sha256.conf", timeout=100),
    Case("wpa3-sae", "hostapd-hwsim-wpa3-sae.conf",
         "wpa_supplicant-hwsim-wpa3-sae.conf", timeout=100),
    Case("wpa3-sae-h2e", "hostapd-hwsim-wpa3-sae-h2e.conf",
         "wpa_supplicant-hwsim-wpa3-sae-h2e.conf", timeout=100),
    Case("wpa3-sae-pk", "hostapd-hwsim-wpa3-sae-pk.conf",
         "wpa_supplicant-hwsim-wpa3-sae-pk.conf", timeout=100),
    Case("transition-sae", "hostapd-hwsim-wpa-transition.conf",
         "wpa_supplicant-hwsim-wpa-transition.conf", timeout=100),
    Case("transition-psk", "hostapd-hwsim-wpa-transition.conf",
         "wpa_supplicant-hwsim-wpa-transition-psk.conf", timeout=90),
    Case("owe-g19", "hostapd-hwsim-owe.conf",
         "wpa_supplicant-hwsim-owe.conf", timeout=100),
    Case("owe-g20", "hostapd-hwsim-owe-g20.conf",
         "wpa-owe-g20.conf", timeout=100),
    Case("owe-g21", "hostapd-hwsim-owe-g21.conf",
         "wpa-owe-g21.conf", timeout=100),
    Case("eap-psk", "hostapd-hwsim-wpa2-eap-psk.conf",
         "wpa_supplicant-hwsim-wpa2-eap-psk.conf", timeout=100),
    Case("eap-tls", "hostapd-hwsim-wpa2-eap-tls.conf",
         "wpa_supplicant-hwsim-wpa2-eap-tls.conf", timeout=140),
    Case("eap-peap", "hostapd-hwsim-wpa2-eap-peap.conf",
         "wpa_supplicant-hwsim-wpa2-eap-peap.conf", timeout=140),
    Case("eap-ttls", "hostapd-hwsim-wpa2-eap-ttls.conf",
         "wpa_supplicant-hwsim-wpa2-eap-ttls.conf", timeout=140),
    Case("eap-ttls-chap", "hostapd-hwsim-wpa2-eap-ttls-chap.conf",
         "wpa_supplicant-hwsim-wpa2-eap-ttls-chap.conf", timeout=140),
    Case("eap-ttls-mschap", "hostapd-hwsim-wpa2-eap-ttls-mschap.conf",
         "wpa_supplicant-hwsim-wpa2-eap-ttls-mschap.conf", timeout=140),
    Case("eap-ttls-mschapv2", "hostapd-hwsim-wpa2-eap-ttls-mschapv2.conf",
         "wpa_supplicant-hwsim-wpa2-eap-ttls-mschapv2.conf", timeout=140),
    Case("suiteb192", "hostapd-hwsim-suiteb192-eap-tls.conf",
         "wpa_supplicant-hwsim-suiteb192-eap-tls.conf", timeout=160),
    Case("hs20", "hostapd-hwsim-hs20.conf",
         "wpa_supplicant-hwsim-hs20.conf", timeout=120),
    Case("ft-psk", "hostapd-hwsim-ft-psk.conf",
         "wpa_supplicant-hwsim-ft-psk.conf", timeout=100),
    Case("wnm-base", "hostapd-hwsim-wnm.conf",
         "wpa_supplicant-hwsim-wnm.conf", timeout=90,
         note="Association/data smoke only; WNM action exchange is a separate flow."),
    Case("11b", "hostapd-b.conf", "wpa-b.conf"),
    Case("11g", "hostapd-g.conf", "wpa-g.conf"),
    Case("11a", "hostapd-a.conf", "wpa-a.conf"),
    Case("11n-ht20", "hostapd-n.conf", "wpa-n.conf"),
    Case("11n-ht40", "hostapd-n-ht40.conf", "wpa-n-ht40.conf"),
    Case("11ac-vht20", "hostapd-ac.conf", "wpa-ac.conf"),
    Case("11ac-vht80", "hostapd-ac-vht80.conf", "wpa-ac-vht80.conf"),
    Case("11ac-vht160", "hostapd-ac-vht160.conf", "wpa-ac-vht160.conf"),
    Case("11ac-vht80p80", "hostapd-ac-vht80p80.conf", "wpa-ac-vht80p80.conf"),
    Case("11ax-24g", "hostapd-ax.conf", "wpa-ax.conf"),
    Case("11ax-5g", "hostapd-ax-a.conf", "wpa-ax-a.conf"),
    Case("11ax-6g-sae", "hostapd-ax6-sae.conf", "wpa-ax6-sae.conf",
         timeout=110),
    Case("11ax-6g-owe", "hostapd-ax6-owe.conf", "wpa-ax6-owe.conf",
         timeout=110),
    Case("s1g", "hostapd-hwsim-s1g.conf",
         "wpa_supplicant-hwsim-s1g.conf", timeout=90),
]


def read_text(path: Path) -> str:
    try:
        return path.read_text(errors="ignore")
    except FileNotFoundError:
        return ""


def send(proc: subprocess.Popen[bytes], command: str, delay: float = 0.4) -> None:
    if proc.poll() is not None:
        raise RuntimeError(f"process exited before command: {command}")

    assert proc.stdin is not None
    proc.stdin.write((command + "\n").encode())
    proc.stdin.flush()
    time.sleep(delay)


def wait_for(path: Path, needle: str, timeout: int) -> bool:
    end = time.time() + timeout
    while time.time() < end:
        if needle in read_text(path):
            return True
        time.sleep(0.5)
    return False


def terminate(procs: list[subprocess.Popen[bytes]]) -> None:
    for proc in procs:
        if proc.poll() is None:
            try:
                os.killpg(os.getpgid(proc.pid), signal.SIGTERM)
            except ProcessLookupError:
                pass

    time.sleep(1)

    for proc in procs:
        if proc.poll() is None:
            try:
                os.killpg(os.getpgid(proc.pid), signal.SIGKILL)
            except ProcessLookupError:
                pass


def prepare_hostfs(sim_dir: Path, run_dir: Path, case: Case) -> None:
    if run_dir.exists():
        shutil.rmtree(run_dir)

    run_dir.mkdir(parents=True)

    for item in sim_dir.iterdir():
        target = run_dir / item.name
        try:
            target.symlink_to(item)
        except FileExistsError:
            pass

    (run_dir / "ha.conf").unlink(missing_ok=True)
    (run_dir / "ws.conf").unlink(missing_ok=True)
    (run_dir / "ha.conf").symlink_to(sim_dir / case.ap_config)
    (run_dir / "ws.conf").symlink_to(sim_dir / case.sta_config)

    for medium in ("hwsim-bss.bin", "hwsim-frames.bin"):
        path = run_dir / medium
        path.unlink(missing_ok=True)


def summarize_iperf(log: str) -> str:
    lines = [line.strip() for line in log.splitlines()]
    summaries = [line for line in lines if " sec " in line and
                 ("Mbits/sec" in line or "Kbits/sec" in line)]
    return summaries[-1] if summaries else ""


def run_case(args: argparse.Namespace, case: Case, index: int) -> dict[str, object]:
    root = Path(args.root).resolve()
    sim_dir = root / "tools/firmware/sim"
    build_dir = root / "build"
    ap_bin = build_dir / "nuttx-sim-ap"
    sta_bin = build_dir / "nuttx-sim-sta1"
    out_dir = Path(args.out_dir).resolve()
    run_dir = Path(args.hostfs_dir).resolve()
    out_dir.mkdir(parents=True, exist_ok=True)

    prepare_hostfs(sim_dir, run_dir, case)

    prefix = out_dir / f"{index:03d}-{case.name}"
    ap_log = prefix.with_suffix(".ap.log")
    sta_log = prefix.with_suffix(".sta.log")
    result_json = prefix.with_suffix(".json")

    ap_log.unlink(missing_ok=True)
    sta_log.unlink(missing_ok=True)

    ap_out = ap_log.open("wb", buffering=0)
    sta_out = sta_log.open("wb", buffering=0)
    procs: list[subprocess.Popen[bytes]] = []

    base_octet = 40 + (index % 160)
    ap_ip = f"192.168.{base_octet}.1"
    sta_ip = f"192.168.{base_octet}.2"
    port_a = 6100 + index * 2
    port_b = port_a + 1

    result: dict[str, object] = {
        "case": asdict(case),
        "ap_log": str(ap_log),
        "sta_log": str(sta_log),
        "ap_ready": False,
        "connected": False,
        "sta_ping_ap": False,
        "ap_ping_sta": False,
        "sta_iperf_client": False,
        "ap_iperf_client": False,
        "passed": False,
    }

    try:
        ap = subprocess.Popen(
            [str(ap_bin)], stdin=subprocess.PIPE, stdout=ap_out,
            stderr=subprocess.STDOUT, cwd=str(root), preexec_fn=os.setsid)
        procs.append(ap)
        time.sleep(2)
        send(ap, f"mount -t hostfs -o fs={run_dir} /h")
        send(ap, f"ifconfig wlan0 {ap_ip} netmask 255.255.255.0")
        send(ap, "hostapd -dd /h/ha.conf &")
        result["ap_ready"] = wait_for(ap_log, "AP-ENABLED", args.ap_timeout)

        sta = subprocess.Popen(
            [str(sta_bin)], stdin=subprocess.PIPE, stdout=sta_out,
            stderr=subprocess.STDOUT, cwd=str(root), preexec_fn=os.setsid)
        procs.append(sta)
        time.sleep(2)
        send(sta, f"mount -t hostfs -o fs={run_dir} /h")
        send(sta, f"ifconfig wlan0 {sta_ip} netmask 255.255.255.0")
        send(sta, "wpa_supplicant -dd -Dnl80211 -iwlan0 -c/h/ws.conf &")
        result["connected"] = wait_for(sta_log, "CTRL-EVENT-CONNECTED",
                                       case.timeout)

        if result["connected"]:
            send(sta, f"ping -c {args.ping_count} {ap_ip}",
                 args.ping_count + 2)
            send(ap, f"ping -c {args.ping_count} {sta_ip}",
                 args.ping_count + 2)
            send(ap, f"iperf -s -p {port_a} &", 0.8)
            send(sta, f"iperf -c {ap_ip} -p {port_a} -t {args.iperf_time}",
                 args.iperf_time + args.iperf_grace)
            send(sta, f"iperf -s -p {port_b} &", 0.8)
            send(ap, f"iperf -c {sta_ip} -p {port_b} -t {args.iperf_time}",
                 args.iperf_time + args.iperf_grace)

        ap_text = read_text(ap_log)
        sta_text = read_text(sta_log)
        result["sta_ping_ap"] = "0% packet loss" in sta_text
        result["ap_ping_sta"] = "0% packet loss" in ap_text
        result["sta_iperf_summary"] = summarize_iperf(sta_text)
        result["ap_iperf_summary"] = summarize_iperf(ap_text)
        result["sta_iperf_client"] = bool(result["sta_iperf_summary"])
        result["ap_iperf_client"] = bool(result["ap_iperf_summary"])
        result["passed"] = all(result[key] for key in (
            "ap_ready", "connected", "sta_ping_ap", "ap_ping_sta",
            "sta_iperf_client", "ap_iperf_client"))
        return result
    except Exception as exc:  # noqa: BLE001
        result["error"] = str(exc)
        return result
    finally:
        terminate(procs)
        ap_out.close()
        sta_out.close()
        result_json.write_text(json.dumps(result, indent=2, sort_keys=True))


def write_summary(out_dir: Path, results: list[dict[str, object]]) -> Path:
    summary = out_dir / "summary.md"
    passed = sum(1 for result in results if result.get("passed"))
    lines = [
        "# HWSIM PASS Matrix Rerun Summary",
        "",
        f"- Total: {len(results)}",
        f"- Passed: {passed}",
        f"- Failed: {len(results) - passed}",
        "",
        "| Case | Result | STA iperf | AP iperf | Logs |",
        "| --- | --- | --- | --- | --- |",
    ]

    for result in results:
        case = result["case"]["name"]  # type: ignore[index]
        status = "PASS" if result.get("passed") else "FAIL"
        sta_iperf = result.get("sta_iperf_summary", "")
        ap_iperf = result.get("ap_iperf_summary", "")
        logs = f"{result.get('ap_log')} / {result.get('sta_log')}"
        lines.append(f"| {case} | {status} | `{sta_iperf}` | `{ap_iperf}` | `{logs}` |")

    summary.write_text("\n".join(lines) + "\n")
    return summary


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", default="/home/uan/Feather-develop-WIFI/FeatherCore_ESP")
    parser.add_argument("--out-dir", default="/tmp/hwsim-pass-matrix")
    parser.add_argument("--hostfs-dir", default="/tmp/hwsim-pass-hostfs")
    parser.add_argument("--case", action="append", dest="cases",
                        help="Run only named case(s). Can be repeated.")
    parser.add_argument("--list", action="store_true", help="List cases and exit.")
    parser.add_argument("--ap-timeout", type=int, default=30)
    parser.add_argument("--ping-count", type=int, default=3)
    parser.add_argument("--iperf-time", type=int, default=4)
    parser.add_argument("--iperf-grace", type=int, default=8)
    parser.add_argument("--stop-on-fail", action="store_true")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    selected = CASES

    if args.cases:
        wanted = set(args.cases)
        known = {case.name for case in CASES}
        missing = sorted(wanted - known)
        if missing:
            print(f"unknown case(s): {', '.join(missing)}", file=sys.stderr)
            return 2
        selected = [case for case in CASES if case.name in wanted]

    if args.list:
        for case in selected:
            print(case.name)
        return 0

    results: list[dict[str, object]] = []
    for index, case in enumerate(selected, 1):
        print(f"==> [{index}/{len(selected)}] {case.name}", flush=True)
        result = run_case(args, case, index)
        results.append(result)
        status = "PASS" if result.get("passed") else "FAIL"
        print(f"    {status} ap={result.get('ap_ready')} connected={result.get('connected')} "
              f"sta_ping={result.get('sta_ping_ap')} ap_ping={result.get('ap_ping_sta')} "
              f"sta_iperf={result.get('sta_iperf_client')} ap_iperf={result.get('ap_iperf_client')}",
              flush=True)
        if args.stop_on_fail and not result.get("passed"):
            break

    summary = write_summary(Path(args.out_dir).resolve(), results)
    print(f"summary: {summary}")
    return 0 if all(result.get("passed") for result in results) else 1


if __name__ == "__main__":
    raise SystemExit(main())
