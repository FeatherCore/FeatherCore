#!/usr/bin/env python3
############################################################################
# tools/firmware/sim/validate-bt-hwsim-usecases.py
#
# SPDX-License-Identifier: Apache-2.0
#
############################################################################

"""Validate BT/BLE hwsim per-terminal nsh logs.

This validator intentionally checks user-visible command/log evidence instead
of trying to infer success from internal files.  It pairs with
test-bt-hwsim-usecases.sh, which writes the command files that operators paste
into the bt1/bt2/ble1/ble2 sim terminals.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import dataclass, asdict
from pathlib import Path


@dataclass(frozen=True)
class RoleCheck:
    role: str
    required: tuple[str, ...]


@dataclass(frozen=True)
class CaseCheck:
    name: str
    roles: tuple[RoleCheck, ...]


BNEP_NATIVE_DATAPATH_REQUIRED: tuple[str, ...] = (
    "bnep-native-active=1",
    "bnep-native-netdev-register=1",
    "bnep-native-session-create=1",
    "re:bnep-native-netdev-xmit=[1-9]",
    "re:bnep-native-tx-frame=[1-9]",
    "re:bnep-native-tx-frame-ok=[1-9]",
    "re:bnep-native-l2cap-rx=[1-9]",
    "re:bnep-native-l2cap-delivered=[1-9]",
    "re:bnep-native-rx-frame=[1-9]",
    "re:bnep-native-rx-frame-ok=[1-9]",
    "re:bnep-native-netif-rx=[1-9]",
)


BNEP_NATIVE_TEARDOWN_REQUIRED: tuple[str, ...] = (
    "bnep-ioctl-conndel=1",
    "bnep-native-session-terminate=1",
    "bnep-native-active=0",
    "bnep-native-netdev-unregister=1",
    "bnep-native-session-start=1",
    "bnep-native-session-stop=1",
)


BNEP_NATIVE_RECONNECT_REQUIRED: tuple[str, ...] = (
    "bnep-ioctl-connadd=2",
    "bnep-ioctl-conndel=2",
    "bnep-native-session-create=2",
    "bnep-native-session-start=2",
    "bnep-native-session-terminate=2",
    "bnep-native-session-stop=2",
    "bnep-native-netdev-register=2",
    "bnep-native-netdev-unregister=2",
    "bnep-native-active=0",
    "re:bnep-native-netdev-xmit=[1-9]",
    "re:bnep-native-tx-frame-ok=[1-9]",
    "re:bnep-native-l2cap-delivered=[1-9]",
    "re:bnep-native-rx-frame-ok=[1-9]",
    "re:bnep-native-netif-rx=[1-9]",
)


BNEP_NATIVE_RECONNECT_STRESS_REQUIRED: tuple[str, ...] = (
    "bnep-ioctl-connadd=3",
    "bnep-ioctl-conndel=3",
    "bnep-native-session-create=3",
    "bnep-native-session-start=3",
    "bnep-native-session-terminate=3",
    "bnep-native-session-stop=3",
    "bnep-native-netdev-register=3",
    "bnep-native-netdev-unregister=3",
    "bnep-native-active=0",
    "re:bnep-native-netdev-xmit=[1-9]",
    "re:bnep-native-tx-frame-ok=[1-9]",
    "re:bnep-native-l2cap-delivered=[1-9]",
    "re:bnep-native-rx-frame-ok=[1-9]",
    "re:bnep-native-netif-rx=[1-9]",
)


BNEP_IPERF_THROUGHPUT_REQUIRED: tuple[str, ...] = (
    r"re:[1-9][0-9]* Bytes\s+[0-9]+\.[0-9]*[1-9] Mbits/sec",
)


BLE_6LOWPAN_RECONNECT_STRESS_REQUIRED: tuple[str, ...] = (
    "linux-bt-6lowpan: registered=1 ifname=bt0",
    "ipsp-open=1",
    "ipsp-open-ret=0",
    "ipsp-psm=0x0023",
    "ipsp-cid=0x0040",
    "ipsp-state=open",
    "upstream-core-init=1",
    "upstream-core-ret=0",
    "upstream-bt6lowpan-init=1",
    "upstream-bt6lowpan-ret=0",
    "upstream-owner=bridge-ipsp",
    "last-tx-dispatch=0x7b last-rx-dispatch=0x7b",
    "tx-fallback=0",
    "lifecycle-register=3",
    "lifecycle-unregister=3",
    "lifecycle-ifup=3",
    "lifecycle-ifdown=3",
    "count>=3:btctl: upstream 6lowpan-down complete",
    "count>=3:linux-bt-6lowpan: registered=0 ifname=-",
    "count>=3:ipsp-state=closed",
    "linux-bt-6lowpan: registered=0 ifname=-",
)


BLE_6LOWPAN_DATAPATH_REQUIRED: tuple[str, ...] = (
    "linux-bt-6lowpan: registered=1 ifname=bt0",
    "ipsp-open=1",
    "ipsp-open-ret=0",
    "ipsp-psm=0x0023",
    "ipsp-cid=0x0040",
    "ipsp-state=open",
    "upstream-core-init=1",
    "upstream-core-ret=0",
    "upstream-bt6lowpan-init=1",
    "upstream-bt6lowpan-ret=0",
    "upstream-owner=bridge-ipsp",
    "re:last-tx-dispatch=0x7[ab] last-rx-dispatch=0x7[ab]",
    "tx-fallback=0",
    "btctl: upstream 6lowpan-down complete",
)


BLE_6LOWPAN_TX_FRAGMENT_REQUIRED: tuple[str, ...] = (
    "re:tx-frag-dgrams=[1-9]",
    "re:tx-frag-frames=[1-9]",
    "rx-frag-drop=0",
)


BLE_6LOWPAN_RX_FRAGMENT_REQUIRED: tuple[str, ...] = (
    "re:rx-frag-dgrams=[1-9]",
    "re:rx-frag-frames=[1-9]",
    "rx-frag-drop=0",
)


CASES: tuple[CaseCheck, ...] = (
    CaseCheck(
        "bt-basic",
        (
            RoleCheck("bt1", (
                "btctl: mgmt power on",
                "btctl: mgmt connectable on",
                "btctl: mgmt discoverable on",
                "btctl: state",
            )),
            RoleCheck("bt2", (
                "btctl: mgmt power on",
                "btctl: hwsim records=",
                "btctl: state",
            )),
        ),
    ),
    CaseCheck(
        "ble-basic",
        (
            RoleCheck("ble1", (
                "btctl: mgmt power on",
                "btctl: mgmt le on",
                "btctl: state",
            )),
            RoleCheck("ble2", (
                "btctl: mgmt power on",
                "btctl: mgmt le on",
                "btctl: state",
            )),
        ),
    ),
    CaseCheck(
        "ble-ip-ping",
        (
            RoleCheck("ble1", (
                "btctl: upstream 6lowpan-up ifname=bt0",
                "linux-bt-6lowpan: registered=1 ifname=bt0",
                "ipsp-open=1",
                "ipsp-open-ret=0",
                "ipsp-psm=0x0023",
                "ipsp-cid=0x0040",
                "ipsp-state=open",
                "upstream-core-init=1",
                "upstream-core-ret=0",
                "upstream-bt6lowpan-init=1",
                "upstream-bt6lowpan-ret=0",
                "upstream-owner=bridge-ipsp",
                "PING6 fc00::2:",
                "2 packets transmitted, 2 received",
                "last-tx-dispatch=0x7b last-rx-dispatch=0x7b",
                "tx-fallback=0",
                "btctl: upstream 6lowpan-down complete",
                "linux-bt-6lowpan: registered=0 ifname=-",
                "ipsp-state=closed",
            )),
            RoleCheck("ble2", (
                "btctl: upstream 6lowpan-up ifname=bt0",
                "linux-bt-6lowpan: registered=1 ifname=bt0",
                "ipsp-open=1",
                "ipsp-open-ret=0",
                "ipsp-psm=0x0023",
                "ipsp-cid=0x0040",
                "ipsp-state=open",
                "upstream-core-init=1",
                "upstream-core-ret=0",
                "upstream-bt6lowpan-init=1",
                "upstream-bt6lowpan-ret=0",
                "upstream-owner=bridge-ipsp",
                "PING6 fc00::1:",
                "2 packets transmitted, 2 received",
                "last-tx-dispatch=0x7b last-rx-dispatch=0x7b",
                "tx-fallback=0",
                "btctl: upstream 6lowpan-down complete",
                "linux-bt-6lowpan: registered=0 ifname=-",
                "ipsp-state=closed",
            )),
        ),
    ),
    CaseCheck(
        "ble-ip-reconnect-stress",
        (
            RoleCheck("ble1", (
                "btctl: upstream 6lowpan-up ifname=bt0",
                "PING6 fc00::2:",
                "2 packets transmitted, 2 received",
                "btctl: upstream 6lowpan-down complete",
                *BLE_6LOWPAN_RECONNECT_STRESS_REQUIRED,
            )),
            RoleCheck("ble2", (
                "btctl: upstream 6lowpan-up ifname=bt0",
                "PING6 fc00::1:",
                "2 packets transmitted, 2 received",
                "btctl: upstream 6lowpan-down complete",
                *BLE_6LOWPAN_RECONNECT_STRESS_REQUIRED,
            )),
        ),
    ),
    CaseCheck(
        "ble-ip-iperf-tcp",
        (
            RoleCheck("ble1", (
                "mode=tcp-server",
                "sip=fc00::1:5001",
                "accept: fc00::2:",
                "Mbits/sec",
                *BLE_6LOWPAN_DATAPATH_REQUIRED,
            )),
            RoleCheck("ble2", (
                "mode=tcp-client",
                "dip=fc00::1:5001",
                "PING6 fc00::1:",
                "1 packets transmitted, 1 received",
                "Mbits/sec",
                "iperf exit",
                *BLE_6LOWPAN_DATAPATH_REQUIRED,
            )),
        ),
    ),
    CaseCheck(
        "ble-ip-iperf-tcp-reverse",
        (
            RoleCheck("ble2", (
                "mode=tcp-server",
                "sip=fc00::2:5001",
                "accept: fc00::1:",
                "Mbits/sec",
                *BLE_6LOWPAN_DATAPATH_REQUIRED,
            )),
            RoleCheck("ble1", (
                "mode=tcp-client",
                "dip=fc00::2:5001",
                "PING6 fc00::2:",
                "1 packets transmitted, 1 received",
                "Mbits/sec",
                "iperf exit",
                *BLE_6LOWPAN_DATAPATH_REQUIRED,
            )),
        ),
    ),
    CaseCheck(
        "ble-ip-iperf-udp",
        (
            RoleCheck("ble1", (
                "mode=udp-server",
                "sip=fc00::1:5001",
                "accept: fc00::2:",
                "Mbits/sec",
                *BLE_6LOWPAN_DATAPATH_REQUIRED,
                *BLE_6LOWPAN_RX_FRAGMENT_REQUIRED,
            )),
            RoleCheck("ble2", (
                "mode=udp-client",
                "dip=fc00::1:5001",
                "PING6 fc00::1:",
                "1 packets transmitted, 1 received",
                "Mbits/sec",
                "iperf exit",
                *BLE_6LOWPAN_DATAPATH_REQUIRED,
                *BLE_6LOWPAN_TX_FRAGMENT_REQUIRED,
            )),
        ),
    ),
    CaseCheck(
        "ble-ip-iperf-udp-reverse",
        (
            RoleCheck("ble2", (
                "mode=udp-server",
                "sip=fc00::2:5001",
                "accept: fc00::1:",
                "Mbits/sec",
                *BLE_6LOWPAN_DATAPATH_REQUIRED,
                *BLE_6LOWPAN_RX_FRAGMENT_REQUIRED,
            )),
            RoleCheck("ble1", (
                "mode=udp-client",
                "dip=fc00::2:5001",
                "PING6 fc00::2:",
                "1 packets transmitted, 1 received",
                "Mbits/sec",
                "iperf exit",
                *BLE_6LOWPAN_DATAPATH_REQUIRED,
                *BLE_6LOWPAN_TX_FRAGMENT_REQUIRED,
            )),
        ),
    ),
    CaseCheck(
        "mgmt-noio",
        (
            RoleCheck("ble1", (
                "upstream-mgmt-listen: create-ret=0 bind-ret=0",
                "upstream-mgmt-send: opcode=0x0018",
                "upstream-mgmt-send: opcode=0x0019",
                "upstream-mgmt-read: recv-ret=",
                "payload=0a 00",
                "payload=01 00 00 00 0a 00 19 00 00",
                "upstream-mgmt-close: released",
            )),
        ),
    ),
    CaseCheck(
        "hci-bredr-medium",
        (
            RoleCheck("bt1", (
                "btctl: mgmt power on",
                "btctl: mgmt bredr on",
                "upstream-mgmt-listen: create-ret=0 bind-ret=0",
                "upstream-hci-connect-br: peer=2 handle=0x0052 state=1",
                "command-path=1 event-path=1 fallback=0",
                "upstream-hci-conn=1",
                "btctl: upstream hci-connect-br peer=2",
                "payload=0b 00",
                "sim-conn br=1 peer=2 role=0 handle=0x0052 upstream-hci-conn=1",
                "conn-hash acl=1 sco=0 le=0",
                "conn[0] type=1 role=0 state=1 out=1 handle=0x0052",
                "btctl: upstream hci-disconnect-br peer=2",
                "upstream-hci-disconnect-br: peer=2 handle=0x0052",
                "payload=0c 00",
                "conn-hash acl=0 sco=0 le=0",
                "upstream-mgmt-close: released",
            )),
            RoleCheck("bt2", (
                "btctl: mgmt power on",
                "btctl: mgmt bredr on",
                "upstream-mgmt-listen: create-ret=0 bind-ret=0",
                "HCI_CMD_CONNECT src=1 dst=2 transport=bredr handle=0x0052",
                "btctl: scan records=",
                "HCI_EVT_CONN_COMPLETE status=0 peer=1",
                "role=acceptor",
                "payload=0b 00",
                "sim-conn br=1 peer=1 role=1 handle=0x0052 upstream-hci-conn=1",
                "conn-hash acl=1 sco=0 le=0",
                "conn[0] type=1 role=1 state=1 out=0 handle=0x0052",
                "HCI_CMD_DISCONNECT src=1 dst=2 transport=bredr handle=0x0052 reason=0x13",
                "HCI_EVT_DISCONN_COMPLETE status=0 peer=1",
                "reason=0x13",
                "payload=0c 00",
                "conn-hash acl=0 sco=0 le=0",
                "upstream-mgmt-close: released",
                "btctl: state",
            )),
        ),
    ),
    CaseCheck(
        "l2cap-native-basic",
        (
            RoleCheck("bt1", (
                "btctl: mgmt power on",
                "btctl: mgmt bredr on",
                "upstream-mgmt-listen: create-ret=0 bind-ret=0",
                "upstream-hci-connect-br: peer=2 handle=0x0052 state=1",
                "command-path=1 event-path=1 fallback=0",
                "payload=0b 00",
                "upstream-l2cap-bind: psm=0x1001 cid=0x0040 handle=0x0052 create-ret=0 bind-ret=0",
                "upstream-l2cap-connect: psm=0x1001 cid=0x0040 connect-ret=0",
                "upstream-l2cap-write: payload-len=8 send-ret=8",
                "native-ret=8 attach-ret=0 fallback-ret=-95",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "upstream-l2cap-close: released",
                "btctl: upstream hci-disconnect-br peer=2",
                "payload=0c 00",
                "conn-hash acl=0 sco=0 le=0",
                "upstream-mgmt-close: released",
            )),
            RoleCheck("bt2", (
                "btctl: mgmt power on",
                "btctl: mgmt bredr on",
                "upstream-mgmt-listen: create-ret=0 bind-ret=0",
                "upstream-l2cap-bind: psm=0x1001 cid=0x0040 handle=0x0052 create-ret=0 bind-ret=0",
                "upstream-l2cap-listen: backlog=1 listen-ret=0",
                "HCI_CMD_CONNECT src=1 dst=2 transport=bredr handle=0x0052",
                "HCI_EVT_CONN_COMPLETE status=0 peer=1",
                "payload=0b 00",
                "upstream-l2cap-recv:",
                "payload=01 02 03 04 05 06 07 08",
                "native-ret=8",
                "l2cap-socket-bind=1",
                "l2cap-socket-listen=1",
                "re:l2cap-socket-recv=[1-9]",
                "upstream-l2cap-close: released",
                "HCI_CMD_DISCONNECT src=1 dst=2 transport=bredr handle=0x0052 reason=0x13",
                "HCI_EVT_DISCONN_COMPLETE status=0 peer=1",
                "payload=0c 00",
                "conn-hash acl=0 sco=0 le=0",
                "upstream-mgmt-close: released",
            )),
        ),
    ),
    CaseCheck(
        "hci-le-lifecycle",
        (
            RoleCheck("ble1", (
                "btctl: mgmt power on",
                "btctl: mgmt le on",
                "conn-hash acl=0 sco=0 le=0",
                "upstream-hci-connect-le: peer=2 handle=0x0102 state=1",
                "command-path=1 event-path=1 fallback=0",
                "upstream-hci-conn=1",
                "btctl: upstream hci-connect-le peer=2",
                "sim-conn le=1 peer=2 role=0 handle=0x0102 upstream-hci-conn=1",
                "conn-hash acl=0 sco=0 le=1",
                "conn[0] type=128 role=0 state=1 out=1 handle=0x0102",
                "btctl: upstream hci-disconnect-le peer=2",
                "upstream-hci-disconnect-le: peer=2 handle=0x0102",
                "command-path=1 event-path=1 fallback=0",
                "conn-hash acl=0 sco=0 le=0",
            )),
        ),
    ),
    CaseCheck(
        "hci-le-reconnect-stress",
        (
            RoleCheck("ble1", (
                "btctl: mgmt power on",
                "btctl: mgmt le on",
                "count>=4:conn-hash acl=0 sco=0 le=0",
                "count>=3:upstream-hci-connect-le: peer=2 handle=0x0102 state=1",
                "count>=3:command-path=1 event-path=1 fallback=0",
                "count>=3:upstream-hci-conn=1",
                "count>=3:btctl: upstream hci-connect-le peer=2",
                "count>=3:sim-conn le=1 peer=2 role=0 handle=0x0102 upstream-hci-conn=1",
                "count>=3:conn-hash acl=0 sco=0 le=1",
                "count>=3:conn[0] type=128 role=0 state=1 out=1 handle=0x0102",
                "count>=3:btctl: upstream hci-disconnect-le peer=2",
                "count>=3:upstream-hci-disconnect-le: peer=2 handle=0x0102",
            )),
        ),
    ),
    CaseCheck(
        "hci-le-medium",
        (
            RoleCheck("ble1", (
                "btctl: mgmt power on",
                "btctl: mgmt le on",
                "upstream-mgmt-listen: create-ret=0 bind-ret=0",
                "upstream-hci-connect-le: peer=4 handle=0x0104 state=1",
                "command-path=1 event-path=1 fallback=0",
                "upstream-hci-conn=1",
                "btctl: upstream hci-connect-le peer=4",
                "payload=0b 00",
                "sim-conn le=1 peer=4 role=0 handle=0x0104 upstream-hci-conn=1",
                "conn-hash acl=0 sco=0 le=1",
                "conn[0] type=128 role=0 state=1 out=1 handle=0x0104",
                "btctl: upstream hci-disconnect-le peer=4",
                "upstream-hci-disconnect-le: peer=4 handle=0x0104",
                "payload=0c 00",
                "conn-hash acl=0 sco=0 le=0",
                "upstream-mgmt-close: released",
            )),
            RoleCheck("ble2", (
                "btctl: mgmt power on",
                "btctl: mgmt le on",
                "upstream-mgmt-listen: create-ret=0 bind-ret=0",
                "HCI_CMD_CONNECT src=3 dst=4 transport=le handle=0x0104",
                "btctl: scan records=",
                "HCI_EVT_CONN_COMPLETE status=0 peer=3",
                "role=acceptor",
                "payload=0b 00",
                "sim-conn le=1 peer=3 role=1 handle=0x0104 upstream-hci-conn=1",
                "conn-hash acl=0 sco=0 le=1 le-peripheral=1",
                "conn[0] type=128 role=1 state=1 out=0 handle=0x0104",
                "HCI_CMD_DISCONNECT src=3 dst=4 transport=le handle=0x0104 reason=0x13",
                "HCI_EVT_DISCONN_COMPLETE status=0 peer=3",
                "reason=0x13",
                "payload=0c 00",
                "conn-hash acl=0 sco=0 le=0 le-peripheral=0",
                "upstream-mgmt-close: released",
                "btctl: state",
            )),
        ),
    ),
    CaseCheck(
        "hci-le-pairing",
        (
            RoleCheck("ble1", (
                "btctl: mgmt power on",
                "btctl: mgmt le on",
                "upstream-mgmt-listen: create-ret=0 bind-ret=0",
                "upstream-hci-connect-le: peer=4 handle=0x0104 state=1",
                "command-path=1 event-path=1 fallback=0",
                "upstream-hci-conn=1",
                "payload=0b 00",
                "upstream-mgmt-send: opcode=0x0018",
                "payload=01 00 00 00 03 00 18 00 00",
                "upstream-mgmt-send: opcode=0x0019",
                "payload=0a 00",
                "payload=01 00 00 00 0a 00 19 00 00",
                "sim-conn le=1 peer=4 role=0 handle=0x0104 upstream-hci-conn=1",
                "conn-hash acl=0 sco=0 le=1",
                "btctl: upstream hci-disconnect-le peer=4",
                "payload=0c 00",
                "conn-hash acl=0 sco=0 le=0",
                "upstream-mgmt-close: released",
            )),
            RoleCheck("ble2", (
                "btctl: mgmt power on",
                "btctl: mgmt le on",
                "upstream-mgmt-listen: create-ret=0 bind-ret=0",
                "HCI_CMD_CONNECT src=3 dst=4 transport=le handle=0x0104",
                "HCI_EVT_CONN_COMPLETE status=0 peer=3",
                "payload=0b 00",
                "upstream-mgmt-send: opcode=0x0018",
                "payload=01 00 00 00 03 00 18 00 00",
                "upstream-mgmt-send: opcode=0x0019",
                "payload=0a 00",
                "payload=01 00 00 00 0a 00 19 00 00",
                "sim-conn le=1 peer=3 role=1 handle=0x0104 upstream-hci-conn=1",
                "conn-hash acl=0 sco=0 le=1 le-peripheral=1",
                "HCI_CMD_DISCONNECT src=3 dst=4 transport=le handle=0x0104 reason=0x13",
                "HCI_EVT_DISCONN_COMPLETE status=0 peer=3",
                "payload=0c 00",
                "conn-hash acl=0 sco=0 le=0 le-peripheral=0",
                "upstream-mgmt-close: released",
            )),
        ),
    ),
    CaseCheck(
        "mgmt-confirm",
        (
            RoleCheck("ble1", (
                "upstream-mgmt-listen: create-ret=0 bind-ret=0",
                "upstream-mgmt-send: opcode=0x0018",
                "upstream-mgmt-send: opcode=0x0019",
                "payload=0f 00",
                "upstream-mgmt-send: opcode=0x001c",
                "payload=01 00 00 00 0a 00 1c 00 00",
                "payload=0a 00",
                "upstream-mgmt-close: released",
            )),
        ),
    ),
    CaseCheck(
        "mgmt-control",
        (
            RoleCheck("ble1", (
                "upstream-mgmt-listen: create-ret=0 bind-ret=0",
                "upstream-mgmt-send: opcode=0x0001 index=0xffff",
                "upstream-mgmt-send: opcode=0x0002 index=0xffff",
                "upstream-mgmt-send: opcode=0x0003 index=0xffff",
                "upstream-mgmt-send: opcode=0x0004 index=0x0000",
                "upstream-mgmt-send: opcode=0x0005 index=0x0000 param=1",
                "upstream-mgmt-send: opcode=0x0007 index=0x0000 param=1",
                "upstream-mgmt-send: opcode=0x0006 index=0x0000 param=1",
                "upstream-mgmt-send: opcode=0x0009 index=0x0000 param=1",
                "upstream-mgmt-send: opcode=0x000d index=0x0000 param=1",
                "upstream-mgmt-send: opcode=0x0029 index=0x0000 param=1",
                "upstream-mgmt-send: opcode=0x002a index=0x0000 param=1",
                "upstream-mgmt-send: opcode=0x0023 index=0x0000 param=1",
                "upstream-mgmt-send: opcode=0x0024 index=0x0000 param=1",
                "count>=13:upstream-mgmt-read: recv-ret=",
                "upstream-mgmt-poll-discovery:",
                "re:hci-mgmt-socket-cmd=1[0-9]",
                "re:hci-mgmt-socket-recv=[1-9][0-9]",
                "upstream-mgmt-close: released",
            )),
        ),
    ),
    CaseCheck(
        "bluez-mgmt-control",
        (
            RoleCheck("ble1", (
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=control",
                "bluez-mgmt: hci-socket fd=",
                "bluez-mgmt: hci-bind-control ret=0",
                "bluez-mgmt: send opcode=0x0001 index=0xffff",
                "bluez-mgmt: send opcode=0x0002 index=0xffff",
                "bluez-mgmt: send opcode=0x0003 index=0xffff",
                "bluez-mgmt: send opcode=0x0004 index=0x0000",
                "bluez-mgmt: send opcode=0x0005 index=0x0000",
                "bluez-mgmt: send opcode=0x0007 index=0x0000",
                "bluez-mgmt: send opcode=0x0006 index=0x0000",
                "bluez-mgmt: send opcode=0x0009 index=0x0000",
                "bluez-mgmt: send opcode=0x000d index=0x0000",
                "bluez-mgmt: send opcode=0x0029 index=0x0000",
                "bluez-mgmt: send opcode=0x002a index=0x0000",
                "bluez-mgmt: send opcode=0x0023 index=0x0000",
                "bluez-mgmt: send opcode=0x0024 index=0x0000",
                "count>=13:bluez-mgmt: recv ret=",
                "bluez-mgmt: control complete",
                "re:hci-mgmt-socket-cmd=1[0-9]",
                "re:hci-mgmt-socket-recv=[1-9][0-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-mgmt-pair-noio",
        (
            RoleCheck("ble1", (
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=pair-noio",
                "bluez-mgmt: hci-socket fd=",
                "bluez-mgmt: hci-bind-control ret=0",
                "bluez-mgmt: send opcode=0x0018 index=0x0000",
                "bluez-mgmt: send opcode=0x0019 index=0x0000",
                "recv-until label=pair-noio-connected",
                "opcode=0x0019 status=0x00 expect-event=0x000b expect-opcode=0x0019",
                "bluez-mgmt: pair-noio complete",
                "re:hci-mgmt-socket-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-mgmt-cancel-pair",
        (
            RoleCheck("ble1", (
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=cancel-pair",
                "bluez-mgmt: hci-socket fd=",
                "bluez-mgmt: hci-bind-control ret=0",
                "bluez-mgmt: send opcode=0x001a index=0x0000 len=7",
                "bluez-mgmt: recv-status ret=",
                "opcode=0x001a status=0x0d expect=0x001a expect-status=0x0d",
                "bluez-mgmt: cancel-pair complete",
            )),
        ),
    ),
    CaseCheck(
        "bluez-mgmt-passkey",
        (
            RoleCheck("ble1", (
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=passkey",
                "bluez-mgmt: hci-socket fd=",
                "bluez-mgmt: hci-bind-control ret=0",
                "bluez-mgmt: send opcode=0x0018 index=0x0000 len=1",
                "bluez-mgmt: send opcode=0x0019 index=0x0000 len=8",
                "bluez-mgmt: recv ret=",
                "event=0x0010",
                "bluez-mgmt: send opcode=0x001e index=0x0000 len=11",
                "opcode=0x001e status=0x00 expect=0x001e expect-status=0x00",
                "event=0x000a",
                "recv-until label=passkey-connected",
                "opcode=0x0019 status=0x00 expect-event=0x000b expect-opcode=0x0019",
                "bluez-mgmt: passkey complete",
            )),
        ),
    ),
    CaseCheck(
        "bluez-mgmt-user-confirm-neg",
        (
            RoleCheck("ble1", (
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=user-confirm-neg",
                "bluez-mgmt: hci-socket fd=",
                "bluez-mgmt: hci-bind-control ret=0",
                "bluez-mgmt: send opcode=0x0018 index=0x0000 len=1",
                "bluez-mgmt: send opcode=0x0019 index=0x0000 len=8",
                "bluez-mgmt: recv ret=",
                "event=0x000f",
                "bluez-mgmt: send opcode=0x001d index=0x0000 len=7",
                "opcode=0x001d status=0x00 expect=0x001d expect-status=0x00",
                "opcode=0x0019 status=0x03 expect=0x0019 expect-status=0x03",
                "bluez-mgmt: user-confirm-neg complete",
            )),
        ),
    ),
    CaseCheck(
        "bluez-mgmt-passkey-neg",
        (
            RoleCheck("ble1", (
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=passkey-neg",
                "bluez-mgmt: hci-socket fd=",
                "bluez-mgmt: hci-bind-control ret=0",
                "bluez-mgmt: send opcode=0x0018 index=0x0000 len=1",
                "bluez-mgmt: send opcode=0x0019 index=0x0000 len=8",
                "bluez-mgmt: recv ret=",
                "event=0x0010",
                "bluez-mgmt: send opcode=0x001f index=0x0000 len=7",
                "opcode=0x001f status=0x00 expect=0x001f expect-status=0x00",
                "opcode=0x0019 status=0x03 expect=0x0019 expect-status=0x03",
                "bluez-mgmt: passkey-neg complete",
            )),
        ),
    ),
    CaseCheck(
        "bluez-mgmt-user-confirm",
        (
            RoleCheck("ble1", (
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=user-confirm",
                "bluez-mgmt: hci-socket fd=",
                "bluez-mgmt: hci-bind-control ret=0",
                "bluez-mgmt: send opcode=0x0018 index=0x0000 len=1",
                "bluez-mgmt: send opcode=0x0019 index=0x0000 len=8",
                "bluez-mgmt: recv ret=",
                "event=0x000f",
                "bluez-mgmt: send opcode=0x001c index=0x0000 len=7",
                "opcode=0x001c status=0x00 expect=0x001c expect-status=0x00",
                "event=0x000a",
                "recv-until label=user-confirm-connected",
                "opcode=0x0019 status=0x00 expect-event=0x000b expect-opcode=0x0019",
                "bluez-mgmt: user-confirm complete",
            )),
        ),
    ),
    CaseCheck(
        "bluez-mgmt-cancel-pair-pending",
        (
            RoleCheck("ble1", (
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=cancel-pair-pending",
                "bluez-mgmt: hci-socket fd=",
                "bluez-mgmt: hci-bind-control ret=0",
                "bluez-mgmt: send opcode=0x0018 index=0x0000 len=1",
                "bluez-mgmt: send opcode=0x0019 index=0x0000 len=8",
                "bluez-mgmt: recv ret=",
                "event=0x000f",
                "bluez-mgmt: send opcode=0x001a index=0x0000 len=7",
                "opcode=0x0019 status=0x10 expect=0x0019 expect-status=0x10",
                "opcode=0x001a status=0x00 expect=0x001a expect-status=0x00",
                "bluez-mgmt: cancel-pair-pending complete",
            )),
        ),
    ),
    CaseCheck(
        "bluez-mgmt-lifecycle",
        (
            RoleCheck("ble1", (
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=lifecycle",
                "bluez-mgmt: hci-socket fd=",
                "bluez-mgmt: hci-bind-control ret=0",
                "bluez-mgmt: send opcode=0x0018 index=0x0000",
                "bluez-mgmt: send opcode=0x0019 index=0x0000",
                "bluez-mgmt: recv-until label=pair-connected",
                "event=0x000b",
                "opcode=0x0019 status=0x00",
                "bluez-mgmt: send opcode=0x0031 index=0x0000",
                "opcode=0x0031 status=0x00 expect=0x0031",
                "bluez-mgmt: send opcode=0x0014 index=0x0000",
                "bluez-mgmt: recv-until label=disconnect",
                "event=0x000c",
                "opcode=0x0014 status=0x00",
                "bluez-mgmt: lifecycle complete",
                "re:hci-mgmt-socket-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-mgmt-pair-unpair",
        (
            RoleCheck("ble1", (
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=pair-unpair",
                "bluez-mgmt: hci-socket fd=",
                "bluez-mgmt: hci-bind-control ret=0",
                "bluez-mgmt: send opcode=0x0018 index=0x0000",
                "bluez-mgmt: send opcode=0x0019 index=0x0000",
                "bluez-mgmt: recv-until label=pair-unpair-connected",
                "event=0x000b",
                "opcode=0x0019 status=0x00",
                "bluez-mgmt: send opcode=0x001b index=0x0000",
                "opcode=0x001b status=0x00 expect=0x001b expect-status=0x00",
                "bluez-mgmt: recv-event label=unpair-observer",
                "event=0x0016",
                "bluez-mgmt: hci-close-observer ret=0",
                "bluez-mgmt: pair-unpair complete",
                "re:hci-mgmt-socket-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-mgmt-reconnect-stress",
        (
            RoleCheck("ble1", (
                "bluez-mgmt: source=third/bluez/tools/btmgmt style "
                "mode=reconnect-stress rounds=3",
                "bluez-mgmt: hci-socket fd=",
                "bluez-mgmt: hci-bind-control ret=0",
                "count>=3:bluez-mgmt: send opcode=0x0018 index=0x0000",
                "count>=3:bluez-mgmt: send opcode=0x0019 index=0x0000",
                "count>=3:bluez-mgmt: send opcode=0x0031 index=0x0000",
                "count>=3:bluez-mgmt: send opcode=0x0014 index=0x0000",
                "bluez-mgmt: recv-until label=round1-connected",
                "bluez-mgmt: recv-until label=round1-disconnected",
                "bluez-mgmt: recv-until label=round2-connected",
                "bluez-mgmt: recv-until label=round2-disconnected",
                "bluez-mgmt: recv-until label=round3-connected",
                "bluez-mgmt: recv-until label=round3-disconnected",
                "count>=3:event=0x000b",
                "count>=3:event=0x000c",
                "count>=3:opcode=0x0019 status=0x00",
                "count>=3:opcode=0x0031 status=0x00 expect=0x0031",
                "count>=3:opcode=0x0014 status=0x00",
                "bluez-mgmt: reconnect round=1 complete",
                "bluez-mgmt: reconnect round=2 complete",
                "bluez-mgmt: reconnect round=3 complete",
                "bluez-mgmt: reconnect-stress complete rounds=3",
                "re:hci-mgmt-socket-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-mgmt-error-path",
        (
            RoleCheck("ble1", (
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=error-path",
                "bluez-mgmt: hci-socket fd=",
                "bluez-mgmt: hci-bind-control ret=0",
                "bluez-mgmt: send opcode=0x0018 index=0x0000 len=1",
                "bluez-mgmt: recv-status ret=",
                "opcode=0x0018 status=0x0d expect=0x0018 expect-status=0x0d",
                "bluez-mgmt: error-path complete",
                "re:hci-mgmt-socket-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-smoke",
        (
            RoleCheck("ble1", (
                "bluez-daemon: source=third/bluez/src/main.c+src/adapter.c style mode=smoke",
                "bluez-daemon: hci-socket fd=",
                "bluez-daemon: hci-bind-control ret=0",
                "bluez-daemon: command opcode=0x0001 index=0xffff",
                "bluez-daemon: command opcode=0x0002 index=0xffff",
                "bluez-daemon: command opcode=0x0003 index=0xffff",
                "bluez-daemon: command opcode=0x0004 index=0x0000",
                "bluez-daemon: command opcode=0x0005 index=0x0000",
                "bluez-daemon: command opcode=0x0007 index=0x0000",
                "bluez-daemon: command opcode=0x0006 index=0x0000",
                "bluez-daemon: command opcode=0x0009 index=0x0000",
                "bluez-daemon: command opcode=0x000d index=0x0000",
                "bluez-daemon: command opcode=0x0029 index=0x0000",
                "bluez-daemon: command opcode=0x002a index=0x0000",
                "bluez-daemon: command opcode=0x0023 index=0x0000",
                "bluez-daemon: event-loop label=discovery-start",
                "event=0x0013",
                "opcode=0x0023 status=0x00",
                "bluez-daemon: command opcode=0x0024 index=0x0000",
                "bluez-daemon: event-loop label=discovery-stop",
                "opcode=0x0024 status=0x00",
                "bluez-daemon: command opcode=0x0018 index=0x0000",
                "bluez-daemon: command opcode=0x0019 index=0x0000",
                "bluez-daemon: event-loop label=pair-connected",
                "event=0x000b",
                "opcode=0x0019 status=0x00",
                "bluez-daemon: command opcode=0x0031 index=0x0000",
                "opcode=0x0031 status=0x00 expect=0x0031",
                "bluez-daemon: command opcode=0x0014 index=0x0000",
                "bluez-daemon: event-loop label=disconnect-complete",
                "event=0x000c",
                "opcode=0x0014 status=0x00",
                "bluez-daemon: smoke complete",
                "re:hci-mgmt-socket-cmd=1[0-9]",
                "re:hci-mgmt-socket-recv=[1-9][0-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-reconnect-stress",
        (
            RoleCheck("ble1", (
                "bluez-daemon: source=third/bluez/src/main.c+src/adapter.c style mode=reconnect-stress rounds=3",
                "bluez-daemon: hci-socket fd=",
                "bluez-daemon: hci-bind-control ret=0",
                "bluez-daemon: command opcode=0x0001 index=0xffff",
                "bluez-daemon: command opcode=0x0002 index=0xffff",
                "bluez-daemon: command opcode=0x0003 index=0xffff",
                "bluez-daemon: command opcode=0x0004 index=0x0000",
                "bluez-daemon: command opcode=0x0005 index=0x0000",
                "bluez-daemon: command opcode=0x0007 index=0x0000",
                "bluez-daemon: command opcode=0x0006 index=0x0000",
                "bluez-daemon: command opcode=0x0009 index=0x0000",
                "bluez-daemon: command opcode=0x000d index=0x0000",
                "bluez-daemon: command opcode=0x0029 index=0x0000",
                "bluez-daemon: command opcode=0x002a index=0x0000",
                "count>=3:bluez-daemon: command opcode=0x0018 index=0x0000",
                "count>=3:bluez-daemon: command opcode=0x0019 index=0x0000",
                "count>=3:bluez-daemon: command opcode=0x0031 index=0x0000",
                "count>=3:bluez-daemon: command opcode=0x0014 index=0x0000",
                "bluez-daemon: event-loop label=round1-connected",
                "bluez-daemon: event-loop label=round2-connected",
                "bluez-daemon: event-loop label=round3-connected",
                "bluez-daemon: event-loop label=round1-disconnected",
                "bluez-daemon: event-loop label=round2-disconnected",
                "bluez-daemon: event-loop label=round3-disconnected",
                "count>=3:event=0x000b",
                "count>=3:event=0x000c",
                "bluez-daemon: reconnect round=1 complete",
                "bluez-daemon: reconnect round=2 complete",
                "bluez-daemon: reconnect round=3 complete",
                "bluez-daemon: reconnect-stress complete rounds=3",
                "conn-hash acl=0 sco=0 le=0",
                "re:hci-mgmt-socket-cmd=2[0-9]",
                "re:hci-mgmt-socket-recv=[3-9][0-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-device-policy",
        (
            RoleCheck("ble1", (
                "bluez-daemon: source=third/bluez/src/adapter.c+src/device.c style mode=device-policy",
                "bluez-daemon: hci-socket fd=",
                "bluez-daemon: hci-bind-control ret=0",
                "bluez-daemon: command opcode=0x0001 index=0xffff",
                "bluez-daemon: command opcode=0x0002 index=0xffff",
                "bluez-daemon: command opcode=0x0003 index=0xffff",
                "bluez-daemon: command opcode=0x0004 index=0x0000",
                "bluez-daemon: command opcode=0x0005 index=0x0000",
                "bluez-daemon: command opcode=0x0007 index=0x0000",
                "bluez-daemon: command opcode=0x0006 index=0x0000",
                "bluez-daemon: command opcode=0x0009 index=0x0000",
                "bluez-daemon: command opcode=0x000d index=0x0000",
                "bluez-daemon: command opcode=0x0029 index=0x0000",
                "bluez-daemon: command opcode=0x0033 index=0x0000",
                "bluez-daemon: complete-loop label=add-device",
                "opcode=0x0033 status=0x00 expect=0x0033",
                "bluez-daemon: command opcode=0x004f index=0x0000",
                "bluez-daemon: complete-loop label=get-device-flags-initial",
                "opcode=0x004f status=0x00 expect=0x004f",
                "bluez-daemon: command opcode=0x0050 index=0x0000",
                "bluez-daemon: complete-loop label=set-device-flags",
                "opcode=0x0050 status=0x00 expect=0x0050",
                "bluez-daemon: complete-loop label=get-device-flags-set",
                "bluez-daemon: command opcode=0x0026 index=0x0000",
                "bluez-daemon: complete-loop label=block-device",
                "opcode=0x0026 status=0x00 expect=0x0026",
                "bluez-daemon: command opcode=0x0027 index=0x0000",
                "bluez-daemon: complete-loop label=unblock-device",
                "opcode=0x0027 status=0x00 expect=0x0027",
                "bluez-daemon: command opcode=0x0018 index=0x0000",
                "bluez-daemon: command opcode=0x0019 index=0x0000",
                "bluez-daemon: event-loop label=policy-pair-connected",
                "event=0x000b",
                "opcode=0x0019 status=0x00",
                "bluez-daemon: command opcode=0x001b index=0x0000",
                "bluez-daemon: complete-loop label=unpair-device",
                "opcode=0x001b status=0x00 expect=0x001b",
                "bluez-daemon: command opcode=0x0034 index=0x0000",
                "event=0x0016",
                "bluez-daemon: complete-loop label=remove-device",
                "opcode=0x0034 status=0x00 expect=0x0034",
                "bluez-daemon: device-policy complete",
                "re:hci-mgmt-socket-cmd=[1-9][0-9]",
                "re:hci-mgmt-socket-recv=[2-9][0-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-discovery-peer",
        (
            RoleCheck("ble2", (
                "btctl: mgmt power on",
                "btctl: mgmt le on",
                "btctl: advertise start",
                "btctl: state",
            )),
            RoleCheck("ble1", (
                "bluez-daemon: source=third/bluez/src/adapter.c+src/device.c style mode=discovery-peer",
                "bluez-daemon: hci-socket fd=",
                "bluez-daemon: hci-bind-control ret=0",
                "bluez-daemon: command opcode=0x0001 index=0xffff",
                "bluez-daemon: command opcode=0x0002 index=0xffff",
                "bluez-daemon: command opcode=0x0003 index=0xffff",
                "bluez-daemon: command opcode=0x0004 index=0x0000",
                "bluez-daemon: command opcode=0x0005 index=0x0000",
                "bluez-daemon: command opcode=0x0007 index=0x0000",
                "bluez-daemon: command opcode=0x0006 index=0x0000",
                "bluez-daemon: command opcode=0x0009 index=0x0000",
                "bluez-daemon: command opcode=0x000d index=0x0000",
                "bluez-daemon: command opcode=0x0023 index=0x0000",
                "bluez-daemon: event-loop label=discovery-start",
                "event=0x0013",
                "opcode=0x0023 status=0x00",
                "bluez-daemon: event-only label=device-found",
                "event=0x0012",
                "bluez-daemon: command opcode=0x0024 index=0x0000",
                "bluez-daemon: event-loop label=discovery-stop",
                "opcode=0x0024 status=0x00",
                "bluez-daemon: discovery-peer complete",
                "Bluetooth: sock ",
                "hci-mgmt-socket-recv=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-pairing-matrix",
        (
            RoleCheck("ble1", (
                "bluez-daemon: source=third/bluez/src/adapter.c+src/agent.c style mode=pairing-matrix",
                "bluez-daemon: hci-socket fd=",
                "bluez-daemon: hci-bind-control ret=0",
                "bluez-daemon: command opcode=0x0001 index=0xffff",
                "bluez-daemon: command opcode=0x0002 index=0xffff",
                "bluez-daemon: command opcode=0x0003 index=0xffff",
                "bluez-daemon: command opcode=0x0004 index=0x0000",
                "bluez-daemon: command opcode=0x0005 index=0x0000",
                "bluez-daemon: command opcode=0x0009 index=0x0000",
                "bluez-daemon: command opcode=0x000d index=0x0000",
                "count>=5:bluez-daemon: command opcode=0x0018 index=0x0000",
                "count>=5:bluez-daemon: command opcode=0x0019 index=0x0000",
                "count>=3:event=0x000f",
                "count>=2:event=0x0010",
                "count>=2:event=0x000a",
                "bluez-daemon: command opcode=0x001c index=0x0000 len=7",
                "bluez-daemon: command opcode=0x001d index=0x0000 len=7",
                "bluez-daemon: command opcode=0x001e index=0x0000 len=11",
                "bluez-daemon: command opcode=0x001f index=0x0000 len=7",
                "bluez-daemon: command opcode=0x001a index=0x0000 len=7",
                "opcode=0x001c status=0x00",
                "opcode=0x001d status=0x00",
                "opcode=0x001e status=0x00",
                "opcode=0x001f status=0x00",
                "count>=2:opcode=0x0019 status=0x00",
                "count>=2:opcode=0x0019 status=0x03",
                "opcode=0x0019 status=0x10",
                "opcode=0x001a status=0x00",
                "bluez-daemon: pairing-matrix step=confirm-accept complete",
                "bluez-daemon: pairing-matrix step=confirm-reject complete",
                "bluez-daemon: pairing-matrix step=passkey-accept complete",
                "bluez-daemon: pairing-matrix step=passkey-reject complete",
                "bluez-daemon: pairing-matrix step=cancel-pending complete",
                "bluez-daemon: pairing-matrix complete",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-mgmt-full-lifecycle",
        (
            RoleCheck("ble2", (
                "btctl: mgmt power on",
                "btctl: mgmt le on",
                "btctl: advertise start",
                "btctl: state",
            )),
            RoleCheck("ble1", (
                "bluez-daemon: source=third/bluez/src/adapter.c+src/device.c style mode=discovery-peer",
                "bluez-daemon: event-only label=device-found",
                "event=0x0012",
                "bluez-daemon: discovery-peer complete",
                "bluez-daemon: source=third/bluez/src/main.c+src/adapter.c style mode=smoke",
                "bluez-daemon: event-loop label=discovery-start",
                "bluez-daemon: event-loop label=pair-connected",
                "bluez-daemon: event-loop label=disconnect-complete",
                "bluez-daemon: smoke complete",
                "bluez-daemon: source=third/bluez/src/adapter.c+src/device.c style mode=device-policy",
                "bluez-daemon: complete-loop label=add-device",
                "bluez-daemon: complete-loop label=set-device-flags",
                "bluez-daemon: complete-loop label=block-device",
                "bluez-daemon: complete-loop label=unblock-device",
                "bluez-daemon: complete-loop label=unpair-device",
                "bluez-daemon: complete-loop label=remove-device",
                "bluez-daemon: device-policy complete",
                "bluez-daemon: source=third/bluez/src/adapter.c+src/agent.c style mode=pairing-matrix",
                "bluez-daemon: pairing-matrix step=confirm-accept complete",
                "bluez-daemon: pairing-matrix step=confirm-reject complete",
                "bluez-daemon: pairing-matrix step=passkey-accept complete",
                "bluez-daemon: pairing-matrix step=passkey-reject complete",
                "bluez-daemon: pairing-matrix step=cancel-pending complete",
                "bluez-daemon: pairing-matrix complete",
                "bluez-daemon: source=third/bluez/src/main.c+src/adapter.c style mode=reconnect-stress rounds=3",
                "bluez-daemon: reconnect round=1 complete",
                "bluez-daemon: reconnect round=2 complete",
                "bluez-daemon: reconnect round=3 complete",
                "bluez-daemon: reconnect-stress complete rounds=3",
                "conn-hash acl=0 sco=0 le=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-btmon-monitor",
        (
            RoleCheck("ble1", (
                "bluez-btmon: source=third/bluez/monitor/bt.h style mode=control",
                "bluez-btmon: hci-socket-monitor fd=",
                "bluez-btmon: hci-bind-monitor ret=0",
                "bluez-btmon: hci-socket-control fd=",
                "bluez-btmon: hci-bind-control ret=0",
                "bluez-btmon: control-send opcode=0x0001 index=0xffff",
                "bluez-btmon: control-send opcode=0x0005 index=0x0000",
                "count>=4:bluez-btmon: monitor-recv ret=",
                "re:bluez-btmon: monitor-count=([4-9]|1[0-2])",
                "bluez-btmon: control complete",
                "Bluetooth: sock ",
                "hci-mgmt-socket-cmd=0",
                "hci-monitor-event=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-hciioctl-basic",
        (
            RoleCheck("ble1", (
                "bluez-hciioctl: source=third/bluez/tools/hciconfig style mode=basic",
                "bluez-hciioctl: hci-socket fd=",
                "bluez-hciioctl: ioctl-getdevlist ret=0",
                "bluez-hciioctl: devlist num=",
                "bluez-hciioctl: ioctl-getdevinfo ret=0",
                "bluez-hciioctl: devinfo id=0 name=hci0",
                "bluez-hciioctl: ioctl-devup",
                "bluez-hciioctl: ioctl-devrestat ret=0",
                "bluez-hciioctl: ioctl-devreset ret=0",
                "bluez-hciioctl: ioctl-devdown ret=0",
                "bluez-hciioctl: ioctl-devup-final ret=0",
                "bluez-hciioctl: basic complete",
                "Bluetooth: cmd 800448d2",
                "Bluetooth: cmd 800448d3",
                "Bluetooth: cmd 400448c9",
                "Bluetooth: cmd 400448cc",
                "Bluetooth: cmd 400448cb",
                "Bluetooth: cmd 400448ca",
            )),
        ),
    ),
    CaseCheck(
        "bluez-hciraw-command",
        (
            RoleCheck("ble1", (
                "bluez-hciraw: source=third/bluez/tools/hcitool style mode=command",
                "bluez-hciraw: hci-socket fd=",
                "bluez-hciraw: hci-bind-raw ret=0",
                "bluez-hciraw: setsockopt-filter ret=0",
                "bluez-hciraw: getsockopt-filter ret=0",
                "bluez-hciraw: send-command opcode=0x1001 ret=4",
                "bluez-hciraw: recv-event ret=",
                "event=0x0e opcode=0x1001 status=0x00",
                "bluez-hciraw: command complete",
                "Bluetooth: hci0 cmd_cnt 1 cmd queued 1",
                "hci-data-socket-rx=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-hciuser-command",
        (
            RoleCheck("ble1", (
                "bluez-hciraw: source=third/bluez/tools/hcitool style mode=user-command",
                "bluez-hciraw: hci-socket fd=",
                "bluez-hciraw: hci-bind-user ret=0",
                "bluez-hciraw: send-command opcode=0x0c03 ret=4",
                "bluez-hciraw: recv-event ret=",
                "event=0x0e opcode=0x0c03 status=0x00",
                "bluez-hciraw: user-command complete",
                "Bluetooth: hci0 cmd_cnt 1 cmd queued 1",
                "promisc=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-hciuser-monitor",
        (
            RoleCheck("ble1", (
                "bluez-hciraw: source=third/bluez/tools/hcitool style mode=user-command-monitor",
                "bluez-hciraw: hci-socket-monitor fd=",
                "bluez-hciraw: hci-bind-monitor ret=0",
                "bluez-hciraw: source=third/bluez/tools/hcitool style mode=user-command",
                "bluez-hciraw: hci-bind-user ret=0",
                "bluez-hciraw: send-command opcode=0x0c03 ret=4",
                "event=0x0e opcode=0x0c03 status=0x00",
                "bluez-hciraw: monitor-recv ret=",
                "mon-event=0x0003",
                "hci-event=0x0e opcode=0x0c03 status=0x00",
                "bluez-hciraw: monitor-count=",
                "event-seen=1",
                "bluez-hciraw: user-command-monitor complete",
                "promisc=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-hciuser-sequence-monitor",
        (
            RoleCheck("ble1", (
                "bluez-hciraw: source=third/bluez/tools/hcitool style mode=user-command-sequence-monitor",
                "bluez-hciraw: hci-socket-monitor fd=",
                "bluez-hciraw: hci-bind-monitor ret=0",
                "bluez-hciraw: hci-socket-user fd=",
                "bluez-hciraw: hci-bind-user ret=0",
                "bluez-hciraw: sequence-send tag=reset opcode=0x0c03 ret=4",
                "bluez-hciraw: sequence-recv tag=reset ret=7",
                "event=0x0e opcode=0x0c03 status=0x00",
                "bluez-hciraw: sequence-send tag=read-local-version opcode=0x1001 ret=4",
                "bluez-hciraw: sequence-recv tag=read-local-version ret=15",
                "event=0x0e opcode=0x1001 status=0x00",
                "hci-ver=0x09 manufacturer=0xffff lmp-subver=0x0001",
                "bluez-hciraw: sequence-send tag=read-local-commands opcode=0x1002 ret=4",
                "bluez-hciraw: sequence-recv tag=read-local-commands ret=71",
                "event=0x0e opcode=0x1002 status=0x00",
                "supported-len=64",
                "bluez-hciraw: sequence-send tag=read-local-features opcode=0x1003 ret=4",
                "bluez-hciraw: sequence-recv tag=read-local-features ret=15",
                "event=0x0e opcode=0x1003 status=0x00",
                "features-len=8",
                "bluez-hciraw: sequence-send tag=read-buffer-size opcode=0x1005 ret=4",
                "bluez-hciraw: sequence-recv tag=read-buffer-size ret=14",
                "event=0x0e opcode=0x1005 status=0x00",
                "acl-mtu=1024 acl-pkts=10 sco-mtu=0 sco-pkts=0",
                "bluez-hciraw: sequence-send tag=le-read-buffer-size opcode=0x2002 ret=4",
                "bluez-hciraw: sequence-recv tag=le-read-buffer-size ret=10",
                "event=0x0e opcode=0x2002 status=0x00",
                "le-mtu=251 le-pkts=10",
                "bluez-hciraw: sequence-send tag=le-read-local-features opcode=0x2003 ret=4",
                "bluez-hciraw: sequence-recv tag=le-read-local-features ret=15",
                "event=0x0e opcode=0x2003 status=0x00",
                "le-features-len=8",
                "bluez-hciraw: sequence-send tag=le-read-supported-states opcode=0x201c ret=4",
                "bluez-hciraw: sequence-recv tag=le-read-supported-states ret=15",
                "event=0x0e opcode=0x201c status=0x00",
                "le-states-len=8",
                "bluez-hciraw: sequence-send tag=le-read-accept-list-size opcode=0x200f ret=4",
                "bluez-hciraw: sequence-recv tag=le-read-accept-list-size ret=8",
                "event=0x0e opcode=0x200f status=0x00",
                "accept-list-size=8",
                "bluez-hciraw: sequence-send tag=le-read-resolv-list-size opcode=0x202a ret=4",
                "bluez-hciraw: sequence-recv tag=le-read-resolv-list-size ret=8",
                "event=0x0e opcode=0x202a status=0x00",
                "resolv-list-size=8",
                "bluez-hciraw: sequence-send tag=le-read-num-adv-sets opcode=0x203b ret=4",
                "bluez-hciraw: sequence-recv tag=le-read-num-adv-sets ret=8",
                "event=0x0e opcode=0x203b status=0x00",
                "adv-sets=1",
                "bluez-hciraw: sequence-send tag=read-bd-addr opcode=0x1009 ret=4",
                "bluez-hciraw: sequence-recv tag=read-bd-addr ret=13",
                "event=0x0e opcode=0x1009 status=0x00",
                "bdaddr=02:fe:00:00:00:03",
                "bluez-hciraw: sequence-monitor-recv ret=",
                "hci-event=0x0e opcode=0x0c03 status=0x00",
                "bluez-hciraw: sequence-monitor-recv ret=20",
                "hci-event=0x0e opcode=0x1001 status=0x00",
                "len=14 hci-event=0x0e opcode=0x1001 status=0x00 hci-ver=0x09 manufacturer=0xffff lmp-subver=0x0001",
                "bluez-hciraw: sequence-monitor-recv ret=76",
                "len=70 hci-event=0x0e opcode=0x1002 status=0x00",
                "supported-len=64",
                "bluez-hciraw: sequence-monitor-recv ret=20",
                "len=14 hci-event=0x0e opcode=0x1003 status=0x00",
                "features-len=8",
                "bluez-hciraw: sequence-monitor-recv ret=19",
                "len=13 hci-event=0x0e opcode=0x1005 status=0x00",
                "acl-mtu=1024 acl-pkts=10 sco-mtu=0 sco-pkts=0",
                "bluez-hciraw: sequence-monitor-recv ret=15",
                "len=9 hci-event=0x0e opcode=0x2002 status=0x00",
                "le-mtu=251 le-pkts=10",
                "len=14 hci-event=0x0e opcode=0x2003 status=0x00",
                "le-features-len=8",
                "len=14 hci-event=0x0e opcode=0x201c status=0x00",
                "le-states-len=8",
                "len=7 hci-event=0x0e opcode=0x200f status=0x00",
                "accept-list-size=8",
                "len=7 hci-event=0x0e opcode=0x202a status=0x00",
                "resolv-list-size=8",
                "len=7 hci-event=0x0e opcode=0x203b status=0x00",
                "adv-sets=1",
                "bluez-hciraw: sequence-monitor-recv ret=18",
                "len=12 hci-event=0x0e opcode=0x1009 status=0x00",
                "bdaddr=02:fe:00:00:00:03",
                "bluez-hciraw: sequence-monitor-count=",
                "first-seen=1 second-seen=1 third-seen=1 fourth-seen=1 fifth-seen=1 sixth-seen=1 seventh-seen=1 eighth-seen=1 ninth-seen=1 tenth-seen=1 eleventh-seen=1 twelfth-seen=1",
                "bluez-hciraw: user-command-sequence-monitor complete",
                "promisc=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-hciuser-error-monitor",
        (
            RoleCheck("ble1", (
                "bluez-hciraw: source=third/bluez/tools/hcitool style mode=user-command-error-monitor",
                "bluez-hciraw: hci-socket-monitor fd=",
                "bluez-hciraw: hci-bind-monitor ret=0",
                "bluez-hciraw: hci-socket-user fd=",
                "bluez-hciraw: hci-bind-user ret=0",
                "bluez-hciraw: sequence-send tag=unknown opcode=0xffff ret=4",
                "bluez-hciraw: sequence-recv tag=unknown ret=7",
                "event=0x0e opcode=0xffff status=0x01",
                "bluez-hciraw: monitor-recv ret=12",
                "mon-event=0x0003",
                "len=6 hci-event=0x0e opcode=0xffff status=0x01",
                "bluez-hciraw: monitor-count=",
                "event-seen=1",
                "bluez-hciraw: user-command-error-monitor complete",
                "promisc=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-hciuser-init-sequence-monitor",
        (
            RoleCheck("ble1", (
                "bluez-hciraw: source=third/bluez/tools/hcitool style mode=user-command-init-sequence-monitor",
                "bluez-hciraw: hci-socket-monitor fd=",
                "bluez-hciraw: hci-bind-monitor ret=0",
                "bluez-hciraw: hci-socket-user fd=",
                "bluez-hciraw: hci-bind-user ret=0",
                "bluez-hciraw: init-send tag=set-event-mask opcode=0x0c01 plen=8 ret=12",
                "bluez-hciraw: init-recv tag=set-event-mask ret=7",
                "event=0x0e opcode=0x0c01 status=0x00",
                "bluez-hciraw: init-send tag=le-set-event-mask opcode=0x2001 plen=8 ret=12",
                "bluez-hciraw: init-recv tag=le-set-event-mask ret=7",
                "event=0x0e opcode=0x2001 status=0x00",
                "bluez-hciraw: init-send tag=write-le-host-supported opcode=0x0c6d plen=2 ret=6",
                "bluez-hciraw: init-recv tag=write-le-host-supported ret=7",
                "event=0x0e opcode=0x0c6d status=0x00",
                "bluez-hciraw: init-send tag=read-local-ext-features opcode=0x1004 plen=1 ret=5",
                "bluez-hciraw: init-recv tag=read-local-ext-features ret=17",
                "event=0x0e opcode=0x1004 status=0x00",
                "bluez-hciraw: init-send tag=read-local-name opcode=0x0c14 plen=0 ret=4",
                "bluez-hciraw: init-recv tag=read-local-name ret=255",
                "event=0x0e opcode=0x0c14 status=0x00",
                "bluez-hciraw: init-send tag=le-read-adv-tx-power opcode=0x2007 plen=0 ret=4",
                "bluez-hciraw: init-recv tag=le-read-adv-tx-power ret=8",
                "event=0x0e opcode=0x2007 status=0x00",
                "bluez-hciraw: init-send tag=le-read-def-data-len opcode=0x2023 plen=0 ret=4",
                "bluez-hciraw: init-recv tag=le-read-def-data-len ret=11",
                "event=0x0e opcode=0x2023 status=0x00",
                "bluez-hciraw: init-send tag=le-read-max-data-len opcode=0x202f plen=0 ret=4",
                "bluez-hciraw: init-recv tag=le-read-max-data-len ret=15",
                "event=0x0e opcode=0x202f status=0x00",
                "bluez-hciraw: init-send tag=le-read-transmit-power opcode=0x204b plen=0 ret=4",
                "bluez-hciraw: init-recv tag=le-read-transmit-power ret=9",
                "event=0x0e opcode=0x204b status=0x00",
                "bluez-hciraw: init-send tag=set-event-mask-page-2 opcode=0x0c63 plen=8 ret=12",
                "bluez-hciraw: init-recv tag=set-event-mask-page-2 ret=7",
                "event=0x0e opcode=0x0c63 status=0x00",
                "bluez-hciraw: init-send tag=le-set-random-addr opcode=0x2005 plen=6 ret=10",
                "bluez-hciraw: init-recv tag=le-set-random-addr ret=7",
                "event=0x0e opcode=0x2005 status=0x00",
                "bluez-hciraw: init-send tag=le-set-adv-param opcode=0x2006 plen=15 ret=19",
                "bluez-hciraw: init-recv tag=le-set-adv-param ret=7",
                "event=0x0e opcode=0x2006 status=0x00",
                "bluez-hciraw: init-send tag=le-set-adv-data opcode=0x2008 plen=32 ret=36",
                "bluez-hciraw: init-recv tag=le-set-adv-data ret=7",
                "event=0x0e opcode=0x2008 status=0x00",
                "bluez-hciraw: init-send tag=le-set-scan-rsp-data opcode=0x2009 plen=32 ret=36",
                "bluez-hciraw: init-recv tag=le-set-scan-rsp-data ret=7",
                "event=0x0e opcode=0x2009 status=0x00",
                "bluez-hciraw: init-send tag=le-set-adv-enable opcode=0x200a plen=1 ret=5",
                "bluez-hciraw: init-recv tag=le-set-adv-enable ret=7",
                "event=0x0e opcode=0x200a status=0x00",
                "bluez-hciraw: init-send tag=le-set-scan-param opcode=0x200b plen=7 ret=11",
                "bluez-hciraw: init-recv tag=le-set-scan-param ret=7",
                "event=0x0e opcode=0x200b status=0x00",
                "bluez-hciraw: init-send tag=le-set-scan-enable opcode=0x200c plen=2 ret=6",
                "bluez-hciraw: init-recv tag=le-set-scan-enable ret=7",
                "event=0x0e opcode=0x200c status=0x00",
                "bluez-hciraw: init-send tag=le-clear-accept-list opcode=0x2010 plen=0 ret=4",
                "bluez-hciraw: init-recv tag=le-clear-accept-list ret=7",
                "event=0x0e opcode=0x2010 status=0x00",
                "bluez-hciraw: init-send tag=le-add-to-accept-list opcode=0x2011 plen=7 ret=11",
                "bluez-hciraw: init-recv tag=le-add-to-accept-list ret=7",
                "event=0x0e opcode=0x2011 status=0x00",
                "bluez-hciraw: init-send tag=le-del-from-accept-list opcode=0x2012 plen=7 ret=11",
                "bluez-hciraw: init-recv tag=le-del-from-accept-list ret=7",
                "event=0x0e opcode=0x2012 status=0x00",
                "bluez-hciraw: init-send tag=le-clear-resolv-list opcode=0x2029 plen=0 ret=4",
                "bluez-hciraw: init-recv tag=le-clear-resolv-list ret=7",
                "event=0x0e opcode=0x2029 status=0x00",
                "bluez-hciraw: init-send tag=le-set-addr-resolv-enable opcode=0x202d plen=1 ret=5",
                "bluez-hciraw: init-recv tag=le-set-addr-resolv-enable ret=7",
                "event=0x0e opcode=0x202d status=0x00",
                "bluez-hciraw: init-send tag=le-set-rpa-timeout opcode=0x202e plen=2 ret=6",
                "bluez-hciraw: init-recv tag=le-set-rpa-timeout ret=7",
                "event=0x0e opcode=0x202e status=0x00",
                "bluez-hciraw: init-monitor-recv ret=",
                "hci-event=0x0e opcode=0x0c01 status=0x00",
                "hci-event=0x0e opcode=0x2001 status=0x00",
                "hci-event=0x0e opcode=0x0c6d status=0x00",
                "hci-event=0x0e opcode=0x1004 status=0x00",
                "hci-event=0x0e opcode=0x0c14 status=0x00",
                "hci-event=0x0e opcode=0x2007 status=0x00",
                "hci-event=0x0e opcode=0x2023 status=0x00",
                "hci-event=0x0e opcode=0x202f status=0x00",
                "hci-event=0x0e opcode=0x204b status=0x00",
                "hci-event=0x0e opcode=0x0c63 status=0x00",
                "hci-event=0x0e opcode=0x2005 status=0x00",
                "hci-event=0x0e opcode=0x2006 status=0x00",
                "hci-event=0x0e opcode=0x2008 status=0x00",
                "hci-event=0x0e opcode=0x2009 status=0x00",
                "hci-event=0x0e opcode=0x200a status=0x00",
                "hci-event=0x0e opcode=0x200b status=0x00",
                "hci-event=0x0e opcode=0x200c status=0x00",
                "hci-event=0x0e opcode=0x2010 status=0x00",
                "hci-event=0x0e opcode=0x2011 status=0x00",
                "hci-event=0x0e opcode=0x2012 status=0x00",
                "hci-event=0x0e opcode=0x2029 status=0x00",
                "hci-event=0x0e opcode=0x202d status=0x00",
                "hci-event=0x0e opcode=0x202e status=0x00",
                "bluez-hciraw: init-monitor-count=",
                "seen1=1 seen2=1 seen3=1 seen4=1 seen5=1 seen6=1 seen7=1 seen8=1 seen9=1 seen10=1 seen11=1 seen12=1 seen13=1 seen14=1 seen15=1 seen16=1 seen17=1 seen18=1 seen19=1 seen20=1 seen21=1 seen22=1 seen23=1",
                "bluez-hciraw: user-command-init-sequence-monitor complete",
                "hci-user-random-addr-valid=1",
                "hci-user-random-addr=02:fe:00:00:00:c3",
                "hci-user-adv-enable=1",
                "hci-user-adv-type=0",
                "hci-user-adv-own-addr-type=0",
                "hci-user-adv-filter-policy=0",
                "hci-user-adv-interval-min=160",
                "hci-user-adv-interval-max=160",
                "hci-user-adv-data-len=0",
                "hci-user-scan-rsp-len=0",
                "hci-user-scan-enable=1",
                "hci-user-scan-filter-dup=0",
                "hci-user-scan-type=0",
                "hci-user-scan-own-addr-type=0",
                "hci-user-scan-filter-policy=0",
                "hci-user-scan-interval=16",
                "hci-user-scan-window=16",
                "hci-user-accept-list-count=0",
                "hci-user-resolv-list-count=0",
                "hci-user-addr-resolv-enable=1",
                "hci-user-rpa-timeout=900",
                "hci-user-event-mask-page2-set=1",
                "hci-user-random-addr-set=1",
                "hci-user-adv-param-set=1",
                "hci-user-adv-data-set=1",
                "hci-user-scan-rsp-set=1",
                "hci-user-adv-enable-set=1",
                "hci-user-scan-param-set=1",
                "hci-user-scan-enable-set=1",
                "hci-user-accept-list-clear=1",
                "hci-user-accept-list-add=1",
                "hci-user-accept-list-del=1",
                "hci-user-resolv-list-clear=1",
                "hci-user-addr-resolv-set=1",
                "hci-user-rpa-timeout-set=1",
                "promisc=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-hciuser-full-abi",
        (
            RoleCheck("ble1", (
                "bluez-hciraw: source=third/bluez/tools/hcitool style mode=user-command-sequence-monitor",
                "bluez-hciraw: sequence-send tag=reset opcode=0x0c03 ret=4",
                "bluez-hciraw: sequence-recv tag=read-local-version ret=15",
                "hci-ver=0x09 manufacturer=0xffff lmp-subver=0x0001",
                "bluez-hciraw: sequence-recv tag=read-local-commands ret=71",
                "supported-len=64",
                "bluez-hciraw: sequence-recv tag=le-read-buffer-size ret=10",
                "le-mtu=251 le-pkts=10",
                "accept-list-size=8",
                "resolv-list-size=8",
                "adv-sets=1",
                "bdaddr=02:fe:00:00:00:03",
                "bluez-hciraw: sequence-monitor-count=",
                "first-seen=1 second-seen=1 third-seen=1 fourth-seen=1 fifth-seen=1 sixth-seen=1 seventh-seen=1 eighth-seen=1 ninth-seen=1 tenth-seen=1 eleventh-seen=1 twelfth-seen=1",
                "bluez-hciraw: user-command-sequence-monitor complete",
                "bluez-hciraw: source=third/bluez/tools/hcitool style mode=user-command-error-monitor",
                "bluez-hciraw: sequence-send tag=unknown opcode=0xffff ret=4",
                "event=0x0e opcode=0xffff status=0x01",
                "len=6 hci-event=0x0e opcode=0xffff status=0x01",
                "event-seen=1",
                "bluez-hciraw: user-command-error-monitor complete",
                "bluez-hciraw: source=third/bluez/tools/hcitool style mode=user-command-init-sequence-monitor",
                "bluez-hciraw: init-send tag=set-event-mask opcode=0x0c01 plen=8 ret=12",
                "bluez-hciraw: init-send tag=le-set-event-mask opcode=0x2001 plen=8 ret=12",
                "bluez-hciraw: init-send tag=le-set-random-addr opcode=0x2005 plen=6 ret=10",
                "bluez-hciraw: init-send tag=le-set-adv-enable opcode=0x200a plen=1 ret=5",
                "bluez-hciraw: init-send tag=le-set-scan-enable opcode=0x200c plen=2 ret=6",
                "bluez-hciraw: init-send tag=le-set-addr-resolv-enable opcode=0x202d plen=1 ret=5",
                "bluez-hciraw: init-monitor-count=",
                "seen1=1 seen2=1 seen3=1 seen4=1 seen5=1 seen6=1 seen7=1 seen8=1 seen9=1 seen10=1 seen11=1 seen12=1 seen13=1 seen14=1 seen15=1 seen16=1 seen17=1 seen18=1 seen19=1 seen20=1 seen21=1 seen22=1 seen23=1",
                "bluez-hciraw: user-command-init-sequence-monitor complete",
                "hci-user-random-addr-valid=1",
                "hci-user-random-addr=02:fe:00:00:00:c3",
                "hci-user-adv-enable=1",
                "hci-user-scan-enable=1",
                "hci-user-accept-list-count=0",
                "hci-user-resolv-list-count=0",
                "hci-user-addr-resolv-enable=1",
                "hci-user-rpa-timeout=900",
                "hci-user-event-mask-page2-set=1",
                "hci-user-random-addr-set=1",
                "hci-user-adv-param-set=1",
                "hci-user-adv-data-set=1",
                "hci-user-scan-rsp-set=1",
                "hci-user-adv-enable-set=1",
                "hci-user-scan-param-set=1",
                "hci-user-scan-enable-set=1",
                "hci-user-accept-list-clear=1",
                "hci-user-accept-list-add=1",
                "hci-user-accept-list-del=1",
                "hci-user-resolv-list-clear=1",
                "hci-user-addr-resolv-set=1",
                "hci-user-rpa-timeout-set=1",
                "promisc=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-hciuser-adv-scan-medium",
        (
            RoleCheck("ble1", (
                "bluez-hciraw: source=third/bluez/tools/hcitool style mode=user-advertise-enable",
                "bluez-hciraw: hci-socket-user fd=",
                "bluez-hciraw: hci-bind-user ret=0",
                "bluez-hciraw: init-send tag=le-set-adv-param opcode=0x2006 plen=15 ret=19",
                "bluez-hciraw: init-recv tag=le-set-adv-param ret=7",
                "event=0x0e opcode=0x2006 status=0x00",
                "bluez-hciraw: init-send tag=le-set-adv-data opcode=0x2008 plen=32 ret=36",
                "bluez-hciraw: init-recv tag=le-set-adv-data ret=7",
                "event=0x0e opcode=0x2008 status=0x00",
                "bluez-hciraw: init-send tag=le-set-adv-enable opcode=0x200a plen=1 ret=5",
                "bluez-hciraw: init-recv tag=le-set-adv-enable ret=7",
                "event=0x0e opcode=0x200a status=0x00",
                "bluez-hciraw: user-advertise-enable complete",
                "hci-user-adv-enable=1",
                "hci-user-adv-param-set=1",
                "hci-user-adv-data-set=1",
                "hci-user-adv-enable-set=1",
                "hci-user-adv-hwsim-tx=1",
                "promisc=0",
            )),
            RoleCheck("ble2", (
                "bluez-hciraw: source=third/bluez/tools/hcitool style mode=user-scan-report",
                "bluez-hciraw: hci-socket-user fd=",
                "bluez-hciraw: hci-bind-user ret=0",
                "bluez-hciraw: init-send tag=le-set-scan-param opcode=0x200b plen=7 ret=11",
                "bluez-hciraw: init-recv tag=le-set-scan-param ret=7",
                "event=0x0e opcode=0x200b status=0x00",
                "bluez-hciraw: init-send tag=le-set-scan-enable opcode=0x200c plen=2 ret=6",
                "bluez-hciraw: init-recv tag=le-set-scan-enable ret=7",
                "event=0x0e opcode=0x200c status=0x00",
                "bluez-hciraw: scan-report-recv ret=",
                "event=0x3e subevent=0x02 reports=1 adv-type=0 addr-type=0",
                "addr=02:fe:00:00:00:03",
                "bluez-hciraw: user-scan-report complete",
                "hci-user-scan-enable=1",
                "hci-user-scan-param-set=1",
                "hci-user-scan-enable-set=1",
                "hci-user-scan-hwsim-poll=1",
                "hci-user-scan-hwsim-report=1",
                "promisc=0",
            )),
        ),
    ),
    CaseCheck(
        "mgmt-passkey",
        (
            RoleCheck("ble1", (
                "upstream-mgmt-listen: create-ret=0 bind-ret=0",
                "upstream-mgmt-send: opcode=0x0018",
                "upstream-mgmt-send: opcode=0x0019",
                "payload=10 00",
                "upstream-mgmt-send: opcode=0x001e",
                "payload=01 00 00 00 0a 00 1e 00 00",
                "payload=0a 00",
                "upstream-mgmt-close: released",
            )),
        ),
    ),
    CaseCheck(
        "a2dp",
        (
            RoleCheck("bt1", (
                "btctl: upstream avdtp-discover peer=2",
                "re:upstream-l2cap-send: psm=0x0019 cid=0x0040 handle=0x0052 payload-len=2[^\\n]*send-ret=2 native-ret=2 attach-ret=0 fallback-ret=-95",
                "btctl: upstream avdtp-getcap peer=2",
                "re:upstream-l2cap-send: psm=0x0019 cid=0x0040 handle=0x0052 payload-len=3[^\\n]*send-ret=3 native-ret=3 attach-ret=0 fallback-ret=-95",
                "btctl: upstream avdtp-setconfig peer=2",
                "re:upstream-l2cap-send: psm=0x0019 cid=0x0040 handle=0x0052 payload-len=14[^\\n]*send-ret=14 native-ret=14 attach-ret=0 fallback-ret=-95",
                "btctl: upstream avdtp-open peer=2",
                "btctl: upstream avdtp-start peer=2",
                "btctl: upstream avdtp-suspend peer=2",
                "btctl: upstream avdtp-close-stream peer=2",
                "count>=4:upstream-l2cap-send: psm=0x0019 cid=0x0040 handle=0x0052 payload-len=3",
                "count>=5:send-ret=3 native-ret=3 attach-ret=0 fallback-ret=-95",
            )),
            RoleCheck("bt2", (
                "btctl: upstream avdtp signaling listening",
                "re:upstream-l2cap-send: psm=0x0019 cid=0x0040 handle=0x0052 payload-len=4[^\\n]*send-ret=4 native-ret=4 attach-ret=0 fallback-ret=-95",
                "btctl: upstream avdtp-auto-rsp signal=0x01",
                "state=IDLE->DISCOVERED",
                "re:upstream-l2cap-send: psm=0x0019 cid=0x0040 handle=0x0052 payload-len=12[^\\n]*send-ret=12 native-ret=12 attach-ret=0 fallback-ret=-95",
                "btctl: upstream avdtp-auto-rsp signal=0x02",
                "count>=5:send-ret=2 native-ret=2 attach-ret=0 fallback-ret=-95",
                "btctl: upstream avdtp-auto-rsp signal=0x03",
                "state=DISCOVERED->CONFIGURED",
                "btctl: upstream avdtp-auto-rsp signal=0x06",
                "state=CONFIGURED->OPEN",
                "btctl: upstream avdtp-auto-rsp signal=0x07",
                "state=OPEN->STREAMING",
                "btctl: upstream avdtp-auto-rsp signal=0x09",
                "state=STREAMING->OPEN",
                "btctl: upstream avdtp-auto-rsp signal=0x08",
                "state=OPEN->IDLE",
                "btctl: upstream avdtp-auto-rsp-loop complete count=7",
                "upstream-l2cap-close: released",
            )),
        ),
    ),
    CaseCheck(
        "a2dp-media",
        (
            RoleCheck("bt1", (
                "btctl: upstream hci-connect-br peer=2",
                "upstream-l2cap-connect:",
                "upstream-l2cap-send: psm=0x0019 cid=0x0041 handle=0x0052 payload-len=24",
                "send-ret=24 native-ret=24 attach-ret=0 fallback-ret=-95",
                "btaudio: upstream a2dp source queued",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "upstream-l2cap-close: released",
            )),
            RoleCheck("bt2", (
                "btaudio: upstream a2dp sink listening",
                "upstream-l2cap-recv-handle: psm=0x0019 cid=0x0041 handle=0x0052 recv-ret=24",
                "payload=41 32 44 50 3a 53 42 43 3a 73 79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-listen=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
                "upstream-l2cap-close: released",
                "btaudio: upstream a2dp sink stopped",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-signaling",
        (
            RoleCheck("bt1", (
                "btctl: upstream hci-connect-br peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=discover peer=2 "
                "handle=0x0052 len=2",
                "re:upstream-l2cap-send: psm=0x0019 cid=0x0040 "
                "handle=0x0052 payload-len=2[^\\n]*send-ret=2 "
                "native-ret=2 attach-ret=0 fallback-ret=-95",
                "bluez-audio: a2dp signaling discover peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=getcap peer=2 "
                "handle=0x0052 len=3",
                "bluez-audio: a2dp signaling getcap peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=setconfig peer=2 "
                "handle=0x0052 len=14",
                "re:upstream-l2cap-send: psm=0x0019 cid=0x0040 "
                "handle=0x0052 payload-len=14[^\\n]*send-ret=14 "
                "native-ret=14 attach-ret=0 fallback-ret=-95",
                "bluez-audio: a2dp signaling setconfig peer=2",
                "bluez-audio: a2dp signaling open peer=2",
                "bluez-audio: a2dp signaling start peer=2",
                "bluez-audio: a2dp signaling suspend peer=2",
                "bluez-audio: a2dp signaling close-stream peer=2",
                "count>=4:upstream-l2cap-send: psm=0x0019 cid=0x0040 "
                "handle=0x0052 payload-len=3",
                "count>=5:send-ret=3 native-ret=3 attach-ret=0 "
                "fallback-ret=-95",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=listen-native "
                "handle=0x0052",
                "bluez-audio: a2dp signaling listening handle=0x0052",
                "re:upstream-l2cap-send: psm=0x0019 cid=0x0040 "
                "handle=0x0052 payload-len=4[^\\n]*send-ret=4 "
                "native-ret=4 attach-ret=0 fallback-ret=-95",
                "bluez-audio: a2dp auto-rsp signal=0x01",
                "state=IDLE->DISCOVERED",
                "re:upstream-l2cap-send: psm=0x0019 cid=0x0040 "
                "handle=0x0052 payload-len=12[^\\n]*send-ret=12 "
                "native-ret=12 attach-ret=0 fallback-ret=-95",
                "bluez-audio: a2dp auto-rsp signal=0x02",
                "count>=5:send-ret=2 native-ret=2 attach-ret=0 "
                "fallback-ret=-95",
                "bluez-audio: a2dp auto-rsp signal=0x03",
                "state=DISCOVERED->CONFIGURED",
                "bluez-audio: a2dp auto-rsp signal=0x06",
                "state=CONFIGURED->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x07",
                "state=OPEN->STREAMING",
                "bluez-audio: a2dp auto-rsp signal=0x09",
                "state=STREAMING->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x08",
                "state=OPEN->IDLE",
                "bluez-audio: a2dp auto-rsp-loop complete count=7",
                "upstream-l2cap-close: released",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-signaling-native",
        (
            RoleCheck("bt1", (
                "btctl: upstream hci-connect-br peer=2",
                "upstream-l2cap-bind: psm=0x0019 cid=0x0040 "
                "handle=0x0052",
                "upstream-l2cap-connect: psm=0x0019 cid=0x0040",
                "bluez-audio: a2dp signaling discover peer=2",
                "bluez-audio: a2dp signaling getcap peer=2",
                "bluez-audio: a2dp signaling setconfig peer=2",
                "bluez-audio: a2dp signaling open peer=2",
                "bluez-audio: a2dp signaling start peer=2",
                "bluez-audio: a2dp signaling suspend peer=2",
                "bluez-audio: a2dp signaling close-stream peer=2",
                "count>=7:upstream-l2cap-send: psm=0x0019 cid=0x0040 "
                "handle=0x0052 payload-len=",
                "re:l2cap-socket-send=[1-9]",
                "upstream-l2cap-close: released",
            )),
            RoleCheck("bt2", (
                "style profile=a2dp-signaling command=listen-native "
                "handle=0x0052",
                "upstream-l2cap-native-control: enabled=1",
                "bluez-audio: a2dp signaling listening handle=0x0052 "
                "native-control=1",
                "bluez-audio: a2dp auto-rsp signal=0x01",
                "state=IDLE->DISCOVERED",
                "bluez-audio: a2dp auto-rsp signal=0x02",
                "bluez-audio: a2dp auto-rsp signal=0x03",
                "state=DISCOVERED->CONFIGURED",
                "bluez-audio: a2dp auto-rsp signal=0x06",
                "state=CONFIGURED->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x07",
                "state=OPEN->STREAMING",
                "bluez-audio: a2dp auto-rsp signal=0x09",
                "state=STREAMING->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x08",
                "state=OPEN->IDLE",
                "bluez-audio: a2dp auto-rsp-loop complete count=7",
                "re:l2cap-socket-native-recv=[1-9]",
                "l2cap-socket-native-recv-fail=0",
                "upstream-l2cap-close: released",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-transaction",
        (
            RoleCheck("bt1", (
                "btctl: upstream hci-connect-br peer=2",
                "upstream-l2cap-bind: psm=0x0019 cid=0x0040 "
                "handle=0x0052",
                "upstream-l2cap-connect: psm=0x0019 cid=0x0040",
                "style profile=a2dp-signaling command=source-transaction "
                "peer=2 count=7",
                "bluez-audio: a2dp transaction response command=discover "
                "signal=0x01",
                "bluez-audio: a2dp transaction response command=getcap "
                "signal=0x02",
                "bluez-audio: a2dp transaction response command=setconfig "
                "signal=0x03",
                "bluez-audio: a2dp transaction response command=open "
                "signal=0x06",
                "bluez-audio: a2dp transaction response command=start "
                "signal=0x07",
                "bluez-audio: a2dp transaction response command=suspend "
                "signal=0x09",
                "bluez-audio: a2dp transaction response command=close-stream "
                "signal=0x08",
                "bluez-audio: a2dp transaction complete peer=2 count=7",
                "upstream-l2cap-close: released",
            )),
            RoleCheck("bt2", (
                "style profile=a2dp-signaling command=listen "
                "handle=0x0052",
                "bluez-audio: a2dp signaling listening handle=0x0052 "
                "native-control=0",
                "bluez-audio: a2dp auto-rsp signal=0x01",
                "state=IDLE->DISCOVERED",
                "bluez-audio: a2dp auto-rsp signal=0x02",
                "bluez-audio: a2dp auto-rsp signal=0x03",
                "state=DISCOVERED->CONFIGURED",
                "bluez-audio: a2dp auto-rsp signal=0x06",
                "state=CONFIGURED->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x07",
                "state=OPEN->STREAMING",
                "bluez-audio: a2dp auto-rsp signal=0x09",
                "state=STREAMING->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x08",
                "state=OPEN->IDLE",
                "bluez-audio: a2dp auto-rsp-loop complete count=7",
                "re:l2cap-socket-recv=[1-9]",
                "upstream-l2cap-close: released",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-transaction-native",
        (
            RoleCheck("bt1", (
                "btctl: upstream hci-connect-br peer=2",
                "upstream-l2cap-bind: psm=0x0019 cid=0x0040 "
                "handle=0x0052",
                "upstream-l2cap-connect: psm=0x0019 cid=0x0040",
                "style profile=a2dp-signaling command=source-transaction "
                "peer=2 count=7",
                "bluez-audio: a2dp transaction response command=discover "
                "signal=0x01",
                "bluez-audio: a2dp transaction response command=getcap "
                "signal=0x02",
                "bluez-audio: a2dp transaction response command=setconfig "
                "signal=0x03",
                "bluez-audio: a2dp transaction response command=open "
                "signal=0x06",
                "bluez-audio: a2dp transaction response command=start "
                "signal=0x07",
                "bluez-audio: a2dp transaction response command=suspend "
                "signal=0x09",
                "bluez-audio: a2dp transaction response command=close-stream "
                "signal=0x08",
                "bluez-audio: a2dp transaction complete peer=2 count=7",
                "upstream-l2cap-close: released",
            )),
            RoleCheck("bt2", (
                "style profile=a2dp-signaling command=listen-native "
                "handle=0x0052",
                "upstream-l2cap-native-control: enabled=1",
                "bluez-audio: a2dp signaling listening handle=0x0052 "
                "native-control=1",
                "bluez-audio: a2dp auto-rsp signal=0x01",
                "state=IDLE->DISCOVERED",
                "bluez-audio: a2dp auto-rsp signal=0x02",
                "bluez-audio: a2dp auto-rsp signal=0x03",
                "state=DISCOVERED->CONFIGURED",
                "bluez-audio: a2dp auto-rsp signal=0x06",
                "state=CONFIGURED->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x07",
                "state=OPEN->STREAMING",
                "bluez-audio: a2dp auto-rsp signal=0x09",
                "state=STREAMING->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x08",
                "state=OPEN->IDLE",
                "bluez-audio: a2dp auto-rsp-loop complete count=7",
                "re:l2cap-socket-native-recv=[1-9]",
                "l2cap-socket-native-recv-fail=0",
                "upstream-l2cap-close: released",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-transaction-reconnect-native",
        (
            RoleCheck("bt1", (
                "btctl: upstream hci-connect-br peer=2",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=source-transaction "
                "peer=2 count=7",
                "count>=2:bluez-audio: a2dp transaction response command=discover "
                "signal=0x01",
                "count>=2:bluez-audio: a2dp transaction response command=getcap "
                "signal=0x02",
                "count>=2:bluez-audio: a2dp transaction response command=setconfig "
                "signal=0x03",
                "count>=2:bluez-audio: a2dp transaction response command=open "
                "signal=0x06",
                "count>=2:bluez-audio: a2dp transaction response command=start "
                "signal=0x07",
                "count>=2:bluez-audio: a2dp transaction response command=suspend "
                "signal=0x09",
                "count>=2:bluez-audio: a2dp transaction response command=close-stream "
                "signal=0x08",
                "count>=2:bluez-audio: a2dp transaction complete peer=2 count=7",
                "re:l2cap-socket-bind=[2-9]",
                "re:l2cap-socket-connect=[2-9]",
                "re:l2cap-socket-send=1[4-9]",
                "upstream-l2cap-close: released",
            )),
            RoleCheck("bt2", (
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=listen-native "
                "handle=0x0052",
                "count>=2:bluez-audio: a2dp signaling listening handle=0x0052 "
                "native-control=1",
                "count>=2:bluez-audio: a2dp auto-rsp signal=0x01",
                "count>=2:state=IDLE->DISCOVERED",
                "count>=2:bluez-audio: a2dp auto-rsp signal=0x02",
                "count>=2:bluez-audio: a2dp auto-rsp signal=0x03",
                "count>=2:state=DISCOVERED->CONFIGURED",
                "count>=2:bluez-audio: a2dp auto-rsp signal=0x06",
                "count>=2:state=CONFIGURED->OPEN",
                "count>=2:bluez-audio: a2dp auto-rsp signal=0x07",
                "count>=2:state=OPEN->STREAMING",
                "count>=2:bluez-audio: a2dp auto-rsp signal=0x09",
                "count>=2:state=STREAMING->OPEN",
                "count>=2:bluez-audio: a2dp auto-rsp signal=0x08",
                "count>=2:state=OPEN->IDLE",
                "count>=2:bluez-audio: a2dp auto-rsp-loop complete count=7",
                "re:l2cap-socket-bind=[2-9]",
                "re:l2cap-socket-listen=[2-9]",
                "re:l2cap-socket-send=1[4-9]",
                "re:l2cap-socket-native-recv=[2-9]",
                "upstream-l2cap-close: released",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-extended",
        (
            RoleCheck("bt1", (
                "btctl: upstream hci-connect-br peer=2",
                "bluez-audio: a2dp signaling discover peer=2",
                "bluez-audio: a2dp signaling getallcap peer=2",
                "bluez-audio: a2dp signaling getcap peer=2",
                "bluez-audio: a2dp signaling setconfig peer=2",
                "bluez-audio: a2dp signaling getconfig peer=2",
                "bluez-audio: a2dp signaling open peer=2",
                "bluez-audio: a2dp signaling reconfigure peer=2",
                "bluez-audio: a2dp signaling delay-report peer=2",
                "bluez-audio: a2dp signaling security-control peer=2",
                "bluez-audio: a2dp signaling start peer=2",
                "bluez-audio: a2dp signaling suspend peer=2",
                "bluez-audio: a2dp signaling abort peer=2",
                "count>=12:upstream-l2cap-send: psm=0x0019 cid=0x0040 "
                "handle=0x0052",
                "count>=12:attach-ret=0 fallback-ret=-95",
                "count>=12:upstream-l2cap-close: released",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=listen "
                "handle=0x0052",
                "bluez-audio: a2dp signaling listening handle=0x0052",
                "bluez-audio: a2dp auto-rsp signal=0x01",
                "state=IDLE->DISCOVERED",
                "bluez-audio: a2dp auto-rsp signal=0x0c",
                "state=DISCOVERED->DISCOVERED",
                "bluez-audio: a2dp auto-rsp signal=0x02",
                "bluez-audio: a2dp auto-rsp signal=0x03",
                "state=DISCOVERED->CONFIGURED",
                "bluez-audio: a2dp auto-rsp signal=0x04",
                "state=CONFIGURED->CONFIGURED",
                "bluez-audio: a2dp auto-rsp signal=0x06",
                "state=CONFIGURED->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x05",
                "state=OPEN->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x0d",
                "bluez-audio: a2dp auto-rsp signal=0x0b",
                "bluez-audio: a2dp auto-rsp signal=0x07",
                "state=OPEN->STREAMING",
                "bluez-audio: a2dp auto-rsp signal=0x09",
                "state=STREAMING->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x0a",
                "state=OPEN->IDLE",
                "bluez-audio: a2dp auto-rsp-loop complete count=12",
                "count>=12:attach-ret=0 fallback-ret=-95",
                "upstream-l2cap-close: released",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-media",
        (
            RoleCheck("bt1", (
                "btctl: upstream hci-connect-br peer=2",
                "upstream-l2cap-connect:",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp.c "
                "style profile=a2dp-source command=start peer=2 "
                "handle=0x0052",
                "upstream-l2cap-send: psm=0x0019 cid=0x0041 "
                "handle=0x0052 payload-len=24",
                "send-ret=24 native-ret=24 attach-ret=0 fallback-ret=-95",
                "bluez-audio: a2dp source queued media payload len=24",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "upstream-l2cap-close: released",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=a2dp-sink command=start peer=0 "
                "handle=0x0000",
                "bluez-audio: a2dp sink transport listening",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=a2dp-sink command=read",
                "upstream-l2cap-recv-handle: psm=0x0019 cid=0x0041 handle=0x0052 recv-ret=24",
                "payload=41 32 44 50 3a 53 42 43 3a 73 79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "bluez-audio: a2dp sink media payload received",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-listen=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
                "re:l2cap-socket-native-recv=[1-9]",
                "upstream-l2cap-close: released",
                "bluez-audio: a2dp sink transport stopped",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-profile",
        (
            RoleCheck("bt1", (
                "btctl: upstream hci-connect-br peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=discover peer=2 "
                "handle=0x0052 len=2",
                "bluez-audio: a2dp signaling discover peer=2",
                "bluez-audio: a2dp signaling getcap peer=2",
                "bluez-audio: a2dp signaling setconfig peer=2",
                "bluez-audio: a2dp signaling open peer=2",
                "bluez-audio: a2dp signaling start peer=2",
                "bluez-audio: a2dp signaling suspend peer=2",
                "bluez-audio: a2dp signaling close-stream peer=2",
                "count>=7:upstream-l2cap-send: psm=0x0019 cid=0x0040 "
                "handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp.c "
                "style profile=a2dp-source command=start peer=2 "
                "handle=0x0052",
                "upstream-l2cap-send: psm=0x0019 cid=0x0041 "
                "handle=0x0052 payload-len=24",
                "send-ret=24 native-ret=24 attach-ret=0 fallback-ret=-95",
                "bluez-audio: a2dp source queued media payload len=24",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "upstream-l2cap-close: released",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=listen "
                "handle=0x0052",
                "bluez-audio: a2dp signaling listening handle=0x0052",
                "bluez-audio: a2dp auto-rsp signal=0x01",
                "state=IDLE->DISCOVERED",
                "bluez-audio: a2dp auto-rsp signal=0x02",
                "bluez-audio: a2dp auto-rsp signal=0x03",
                "state=DISCOVERED->CONFIGURED",
                "bluez-audio: a2dp auto-rsp signal=0x06",
                "state=CONFIGURED->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x07",
                "state=OPEN->STREAMING",
                "bluez-audio: a2dp auto-rsp signal=0x09",
                "state=STREAMING->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x08",
                "state=OPEN->IDLE",
                "bluez-audio: a2dp auto-rsp-loop complete count=7",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=a2dp-sink command=start peer=0 "
                "handle=0x0000",
                "bluez-audio: a2dp sink transport listening",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=a2dp-sink command=read",
                "upstream-l2cap-recv-handle: psm=0x0019 cid=0x0041 handle=0x0052 recv-ret=24",
                "payload=41 32 44 50 3a 53 42 43 3a 73 79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "bluez-audio: a2dp sink media payload received",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-listen=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
                "re:l2cap-socket-native-recv=[1-9]",
                "upstream-l2cap-close: released",
                "bluez-audio: a2dp sink transport stopped",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-profile-reconnect",
        (
            RoleCheck("bt1", (
                "btctl: upstream hci-connect-br peer=2",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=source-transaction "
                "peer=2 count=7",
                "count>=2:bluez-audio: a2dp transaction complete peer=2 count=7",
                "count>=14:upstream-l2cap-send: psm=0x0019",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/a2dp.c "
                "style profile=a2dp-source command=start peer=2 "
                "handle=0x0052",
                "count>=2:send-ret=24 native-ret=24 attach-ret=0 fallback-ret=-95",
                "count>=2:bluez-audio: a2dp source queued media payload len=24",
                "re:l2cap-socket-bind=[2-9]",
                "re:l2cap-socket-connect=[2-9]",
                "re:l2cap-socket-send=([2-9]|[1-9][0-9]+)",
                "upstream-l2cap-close: released",
            )),
            RoleCheck("bt2", (
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=listen-native "
                "handle=0x0052",
                "count>=2:bluez-audio: a2dp signaling listening handle=0x0052 native-control=1",
                "count>=2:bluez-audio: a2dp auto-rsp-loop complete count=7",
                "count>=2:state=IDLE->DISCOVERED",
                "count>=2:state=DISCOVERED->CONFIGURED",
                "count>=2:state=CONFIGURED->OPEN",
                "count>=2:state=OPEN->STREAMING",
                "count>=2:state=STREAMING->OPEN",
                "count>=2:state=OPEN->IDLE",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=a2dp-sink command=start peer=1 "
                "handle=0x0052",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=a2dp-sink command=read",
                "count>=2:upstream-l2cap-recv-handle: psm=0x0019 cid=0x0041 handle=0x0052 recv-ret=24",
                "count>=2:bluez-audio: a2dp sink media payload received",
                "re:l2cap-socket-bind=[2-9]",
                "re:l2cap-socket-listen=[2-9]",
                "re:l2cap-socket-recv=([2-9]|[1-9][0-9]+)",
                "re:l2cap-socket-native-recv=[2-9]",
                "upstream-l2cap-close: released",
                "count>=2:bluez-audio: a2dp sink transport stopped",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-extended-profile",
        (
            RoleCheck("bt1", (
                "btctl: upstream hci-connect-br peer=2",
                "bluez-audio: a2dp signaling discover peer=2",
                "bluez-audio: a2dp signaling getallcap peer=2",
                "bluez-audio: a2dp signaling getcap peer=2",
                "bluez-audio: a2dp signaling setconfig peer=2",
                "bluez-audio: a2dp signaling getconfig peer=2",
                "bluez-audio: a2dp signaling open peer=2",
                "bluez-audio: a2dp signaling reconfigure peer=2",
                "bluez-audio: a2dp signaling delay-report peer=2",
                "bluez-audio: a2dp signaling security-control peer=2",
                "bluez-audio: a2dp signaling start peer=2",
                "bluez-audio: a2dp signaling suspend peer=2",
                "bluez-audio: a2dp signaling abort peer=2",
                "count>=12:upstream-l2cap-send: psm=0x0019 cid=0x0040 "
                "handle=0x0052",
                "count>=12:attach-ret=0 fallback-ret=-95",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp.c "
                "style profile=a2dp-source command=start peer=2 "
                "handle=0x0052",
                "upstream-l2cap-send: psm=0x0019 cid=0x0041 "
                "handle=0x0052 payload-len=24",
                "send-ret=24 native-ret=24 attach-ret=0 fallback-ret=-95",
                "bluez-audio: a2dp source queued media payload len=24",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "upstream-l2cap-close: released",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=listen "
                "handle=0x0052",
                "bluez-audio: a2dp signaling listening handle=0x0052",
                "bluez-audio: a2dp auto-rsp signal=0x01",
                "state=IDLE->DISCOVERED",
                "bluez-audio: a2dp auto-rsp signal=0x0c",
                "state=DISCOVERED->DISCOVERED",
                "bluez-audio: a2dp auto-rsp signal=0x02",
                "bluez-audio: a2dp auto-rsp signal=0x03",
                "state=DISCOVERED->CONFIGURED",
                "bluez-audio: a2dp auto-rsp signal=0x04",
                "state=CONFIGURED->CONFIGURED",
                "bluez-audio: a2dp auto-rsp signal=0x06",
                "state=CONFIGURED->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x05",
                "state=OPEN->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x0d",
                "bluez-audio: a2dp auto-rsp signal=0x0b",
                "bluez-audio: a2dp auto-rsp signal=0x07",
                "state=OPEN->STREAMING",
                "bluez-audio: a2dp auto-rsp signal=0x09",
                "state=STREAMING->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x0a",
                "state=OPEN->IDLE",
                "bluez-audio: a2dp auto-rsp-loop complete count=12",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=a2dp-sink command=start peer=0 "
                "handle=0x0000",
                "bluez-audio: a2dp sink transport listening",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=a2dp-sink command=read",
                "upstream-l2cap-recv-handle: psm=0x0019 cid=0x0041 handle=0x0052 recv-ret=24",
                "payload=41 32 44 50 3a 53 42 43 3a 73 79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "bluez-audio: a2dp sink media payload received",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-listen=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
                "re:l2cap-socket-native-recv=[1-9]",
                "upstream-l2cap-close: released",
                "bluez-audio: a2dp sink transport stopped",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-transport",
        (
            RoleCheck("bt1", (
                "bluez-audio: a2dp signaling discover peer=2",
                "bluez-audio: a2dp signaling getcap peer=2",
                "bluez-audio: a2dp signaling setconfig peer=2",
                "bluez-audio: a2dp signaling open peer=2",
                "bluez-audio: a2dp signaling start peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=a2dp-source-acquire-write-release peer=2 "
                "handle=0x0052",
                "upstream-l2cap-bind: psm=0x0019 cid=0x0041 "
                "handle=0x0052",
                "upstream-l2cap-connect: psm=0x0019 cid=0x0041",
                "bluez-audio: media transport acquire fd=l2cap",
                "upstream-l2cap-write: payload-len=24 send-ret=24",
                "bluez-audio: media transport write len=24 "
                "payload=A2DP:SBC:synthetic-frame",
                "upstream-l2cap-close: released",
                "bluez-audio: media transport release complete role=source",
                "bluez-audio: a2dp signaling suspend peer=2",
                "bluez-audio: a2dp signaling close-stream peer=2",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=listen "
                "handle=0x0052",
                "bluez-audio: a2dp signaling listening handle=0x0052",
                "bluez-audio: a2dp auto-rsp signal=0x01",
                "state=IDLE->DISCOVERED",
                "bluez-audio: a2dp auto-rsp signal=0x02",
                "bluez-audio: a2dp auto-rsp signal=0x03",
                "state=DISCOVERED->CONFIGURED",
                "bluez-audio: a2dp auto-rsp signal=0x06",
                "state=CONFIGURED->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x07",
                "state=OPEN->STREAMING",
                "bluez-audio: a2dp auto-rsp-loop complete count=5",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=a2dp-sink-acquire-read-release peer=1 "
                "handle=0x0052",
                "upstream-l2cap-bind: psm=0x0019 cid=0x0041",
                "bluez-audio: media transport acquire fd=l2cap",
                "upstream-l2cap-recv-handle: psm=0x0019 cid=0x0041 handle=0x0052 recv-ret=24",
                "payload=41 32 44 50 3a 53 42 43 3a 73 79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "bluez-audio: media transport read complete "
                "payload=A2DP:SBC:synthetic-frame",
                "upstream-l2cap-close: released",
                "bluez-audio: media transport release complete role=sink",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-listen=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
                "re:l2cap-socket-native-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-endpoint-transport",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=source peer=2",
                "bluez-audio: media endpoint register endpoint=/org/bluez/hci0/dev_feather/sep/source0",
                "uuid=0000110a-0000-1000-8000-00805f9b34fb",
                "codec=sbc codec-id=0x00",
                "bluez-audio: media endpoint capabilities media-type=audio codec=sbc caps=ff ff 02 35",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp.c select-config role=source",
                "sampling=44100",
                "channel-mode=joint-stereo",
                "configuration=21 15 02 35",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c transport created",
                "bluez-audio: media endpoint lifecycle complete role=source",
                "bluez-audio: a2dp signaling discover peer=2",
                "bluez-audio: a2dp signaling getcap peer=2",
                "bluez-audio: a2dp signaling setconfig peer=2",
                "bluez-audio: a2dp signaling open peer=2",
                "bluez-audio: a2dp signaling start peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=a2dp-source-acquire-write-release peer=2 "
                "handle=0x0052",
                "bluez-audio: media transport acquire fd=l2cap",
                "upstream-l2cap-write: payload-len=24 send-ret=24",
                "bluez-audio: media transport write len=24 payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: media transport release complete role=source",
                "bluez-audio: a2dp signaling suspend peer=2",
                "bluez-audio: a2dp signaling close-stream peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=clear role=source peer=2",
                "bluez-audio: media endpoint unregister endpoint=/org/bluez/hci0/dev_feather/sep/source0",
                "bluez-audio: media endpoint clear complete role=source",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=sink peer=1",
                "bluez-audio: media endpoint register endpoint=/org/bluez/hci0/dev_feather/sep/sink0",
                "uuid=0000110b-0000-1000-8000-00805f9b34fb",
                "codec=sbc codec-id=0x00",
                "bluez-audio: media endpoint capabilities media-type=audio codec=sbc caps=ff ff 02 35",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp.c select-config role=sink",
                "configuration=21 15 02 35",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c transport created",
                "bluez-audio: media endpoint lifecycle complete role=sink",
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=listen "
                "handle=0x0052",
                "bluez-audio: a2dp auto-rsp signal=0x01",
                "state=IDLE->DISCOVERED",
                "bluez-audio: a2dp auto-rsp signal=0x03",
                "state=DISCOVERED->CONFIGURED",
                "bluez-audio: a2dp auto-rsp signal=0x07",
                "state=OPEN->STREAMING",
                "bluez-audio: a2dp auto-rsp-loop complete count=5",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=a2dp-sink-acquire-read-release peer=1 "
                "handle=0x0052",
                "bluez-audio: media transport acquire fd=l2cap",
                "upstream-l2cap-recv-handle: psm=0x0019 cid=0x0041 handle=0x0052 recv-ret=24",
                "bluez-audio: media transport read complete "
                "payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: media transport release complete role=sink",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=clear role=sink peer=1",
                "bluez-audio: media endpoint unregister endpoint=/org/bluez/hci0/dev_feather/sep/sink0",
                "bluez-audio: media endpoint clear complete role=sink",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-listen=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
                "re:l2cap-socket-native-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-sbc-codec-transport",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=source peer=2",
                "bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus object-manager path=/ "
                "interface=org.freedesktop.DBus.ObjectManager "
                "method=GetManagedObjects adapter=/org/bluez/hci0 "
                "device=/org/bluez/hci0/dev_feather role=source",
                "bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus signal=InterfacesAdded path=/org/bluez/hci0/dev_feather "
                "interface=org.bluez.MediaControl1 role=source",
                "bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus signal=InterfacesAdded "
                "path=/org/bluez/hci0/dev_feather/sep/source0 "
                "interface=org.bluez.MediaEndpoint1 "
                "uuid=0000110a-0000-1000-8000-00805f9b34fb "
                "codec=0x00 role=source",
                "bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus signal=InterfacesAdded "
                "path=/org/bluez/hci0/dev_feather/fd/source0 "
                "interface=org.bluez.MediaTransport1 state=idle "
                "role=source",
                "bluez-audio: source=third/bluez/profiles/audio/source.c "
                "sdp register service=AudioSource "
                "uuid=0000110a-0000-1000-8000-00805f9b34fb psm=0x0019 "
                "features=delay-report,media-transport",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "control register psm=0x0017 browsing-psm=0x001b "
                "role=source state=idle",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "sdp register controller uuid=0000110e-0000-1000-8000-00805f9b34fb "
                "target uuid=0000110c-0000-1000-8000-00805f9b34fb "
                "role=source",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1 "
                "path=/org/bluez/hci0/dev_feather control=avrcp role=source",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "dbus owner=:1.feather adapter=/org/bluez/hci0 "
                "interface=org.bluez.Media1 method=RegisterEndpoint "
                "endpoint=/org/bluez/hci0/dev_feather/sep/source0 role=source",
                "bluez-audio: source=third/bluez/client/player.c "
                "dbus provider=bluezaudio interface=org.bluez.MediaEndpoint1 "
                "methods=SelectConfiguration,SetConfiguration,ClearConfiguration,Release",
                "codec=sbc codec-id=0x00",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "dbus endpoint owner=:1.feather "
                "endpoint=/org/bluez/hci0/dev_feather/sep/source0 "
                "uuid=0000110a-0000-1000-8000-00805f9b34fb codec=0x00",
                "bluez-audio: media endpoint capabilities media-type=audio codec=sbc caps=ff ff 02 35",
                "bluez-audio: source=third/bluez/client/player.c "
                "preset=a2dp_src/sbc codec=sbc codec-id=0x00 "
                "caps=ff ff 02 40 preset-count=6",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp.c select-config role=source",
                "configuration=21 15 02 35",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "dbus export interface=org.bluez.MediaTransport1 "
                "path=/org/bluez/hci0/dev_feather/fd/source0 "
                "owner=:1.feather state=idle codec=sbc",
                "bluez-audio: a2dp signaling discover peer=2",
                "bluez-audio: a2dp signaling getcap peer=2",
                "bluez-audio: a2dp signaling setconfig peer=2",
                "bluez-audio: a2dp signaling open peer=2",
                "bluez-audio: a2dp signaling start peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=source-session-open "
                "peer=2 handle=0x0052",
                "upstream-l2cap-bind: psm=0x0019 cid=0x0040 "
                "handle=0x0052",
                "upstream-l2cap-connect: psm=0x0019 cid=0x0040 "
                "connect-ret=0",
                "bluez-audio: a2dp source session opened "
                "peer=2 handle=0x0052",
                "upstream-l2cap-write: payload-len=2 send-ret=2 "
                "native-ret=2 attach-ret=0 fallback-ret=-95",
                "count>=5:upstream-l2cap-write: payload-len=3 send-ret=3 "
                "native-ret=3 attach-ret=0 fallback-ret=-95",
                "upstream-l2cap-write: payload-len=14 send-ret=14 "
                "native-ret=14 attach-ret=0 fallback-ret=-95",
                "bluez-audio: a2dp source session closed "
                "peer=2 handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp.c "
                "style profile=a2dp-codec role=source "
                "command=source-sbc-encode-write-release peer=2 "
                "handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "pcm input format=s16le rate=44100 channels=2 frames=128",
                "bluez-audio: codec-backend profile=a2dp codec=sbc "
                "backend=libsbc status=source-built "
                "required-source=third/sbc-2.0",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp-codecs.h "
                "selected-config=21 15 02 35 codec-id=0x00",
                "re:bluez-audio: source=third/bluez/profiles/audio/a2dp-codecs\\.h "
                "sbc encode syncword=0x9c frame-header=bd [0-9a-f]{2} [0-9a-f]{2} "
                "samplerate=44100 channels=2 channel-mode=joint-stereo "
                "blocks=16 subbands=8 allocation=loudness bitpool=[1-9][0-9]* "
                "frame-len=[1-9][0-9]*",
                "bluez-audio: media transport acquire fd=l2cap",
                "bluez-audio: media transport handle open role=source "
                "psm=0x0019 cid=0x0041 handle=0x0052 ret=0",
                "bluez-audio: media transport handle connect role=source "
                "psm=0x0019 cid=0x0041 handle=0x0052 ret=0",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "dbus method=Acquire interface=org.bluez.MediaTransport1 "
                "path=/org/bluez/hci0/dev_feather/fd/source0 fd=l2cap "
                "role=source",
                "codec=sbc",
                "re:upstream-l2cap-write-handle: psm=0x0019 cid=0x0041 "
                "handle=0x0052 payload-len=[1-9][0-9]* "
                "send-ret=[1-9][0-9]* native-ret=[1-9][0-9]* "
                "attach-ret=0 fallback-ret=(-95|0|[1-9][0-9]*)",
                "re:bluez-audio: a2dp codec source wrote sbc frame len=[1-9][0-9]*",
                "bluez-audio: media transport handle close role=source "
                "psm=0x0019 cid=0x0041 handle=0x0052 ret=0",
                "bluez-audio: media transport release complete role=source codec=sbc",
                "bluez-audio: a2dp signaling suspend peer=2",
                "bluez-audio: a2dp signaling close-stream peer=2",
                "bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus signal=InterfacesRemoved "
                "path=/org/bluez/hci0/dev_feather/fd/source0 "
                "interface=org.bluez.MediaTransport1 role=source",
                "bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus signal=InterfacesRemoved "
                "path=/org/bluez/hci0/dev_feather/sep/source0 "
                "interface=org.bluez.MediaEndpoint1 role=source",
                "bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus signal=InterfacesRemoved path=/org/bluez/hci0/dev_feather "
                "interface=org.bluez.MediaControl1 role=source",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus unexport interface=org.bluez.MediaControl1 "
                "path=/org/bluez/hci0/dev_feather role=source",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "sdp unregister controller uuid=0000110e-0000-1000-8000-00805f9b34fb "
                "target uuid=0000110c-0000-1000-8000-00805f9b34fb "
                "role=source",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "control unregister psm=0x0017 browsing-psm=0x001b "
                "role=source",
                "bluez-audio: source=third/bluez/profiles/audio/source.c "
                "sdp unregister service=AudioSource "
                "uuid=0000110a-0000-1000-8000-00805f9b34fb psm=0x0019",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "dbus method=ClearConfiguration owner=:1.feather "
                "endpoint=/org/bluez/hci0/dev_feather/sep/source0 "
                "transport=/org/bluez/hci0/dev_feather/fd/source0",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "dbus unexport interface=org.bluez.MediaTransport1 "
                "path=/org/bluez/hci0/dev_feather/fd/source0",
                "bluez-audio: media endpoint clear complete role=source",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=sink peer=1",
                "bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus object-manager path=/ "
                "interface=org.freedesktop.DBus.ObjectManager "
                "method=GetManagedObjects adapter=/org/bluez/hci0 "
                "device=/org/bluez/hci0/dev_feather role=sink",
                "bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus signal=InterfacesAdded path=/org/bluez/hci0/dev_feather "
                "interface=org.bluez.MediaControl1 role=sink",
                "bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus signal=InterfacesAdded "
                "path=/org/bluez/hci0/dev_feather/sep/sink0 "
                "interface=org.bluez.MediaEndpoint1 "
                "uuid=0000110b-0000-1000-8000-00805f9b34fb "
                "codec=0x00 role=sink",
                "bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus signal=InterfacesAdded "
                "path=/org/bluez/hci0/dev_feather/fd/sink0 "
                "interface=org.bluez.MediaTransport1 state=idle "
                "role=sink",
                "bluez-audio: source=third/bluez/profiles/audio/sink.c "
                "sdp register service=AudioSink "
                "uuid=0000110b-0000-1000-8000-00805f9b34fb psm=0x0019 "
                "features=delay-report,media-transport",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "control register psm=0x0017 browsing-psm=0x001b "
                "role=sink state=idle",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "sdp register controller uuid=0000110e-0000-1000-8000-00805f9b34fb "
                "target uuid=0000110c-0000-1000-8000-00805f9b34fb "
                "role=sink",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1 "
                "path=/org/bluez/hci0/dev_feather control=avrcp role=sink",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "dbus owner=:1.feather adapter=/org/bluez/hci0 "
                "interface=org.bluez.Media1 method=RegisterEndpoint "
                "endpoint=/org/bluez/hci0/dev_feather/sep/sink0 role=sink",
                "bluez-audio: source=third/bluez/client/player.c "
                "dbus provider=bluezaudio interface=org.bluez.MediaEndpoint1 "
                "methods=SelectConfiguration,SetConfiguration,ClearConfiguration,Release",
                "codec=sbc codec-id=0x00",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "dbus endpoint owner=:1.feather "
                "endpoint=/org/bluez/hci0/dev_feather/sep/sink0 "
                "uuid=0000110b-0000-1000-8000-00805f9b34fb codec=0x00",
                "bluez-audio: media endpoint capabilities media-type=audio codec=sbc caps=ff ff 02 35",
                "bluez-audio: source=third/bluez/client/player.c "
                "preset=a2dp_sink/sbc codec=sbc codec-id=0x00 "
                "caps=ff ff 02 40 preset-count=6",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp.c select-config role=sink",
                "configuration=21 15 02 35",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "dbus export interface=org.bluez.MediaTransport1 "
                "path=/org/bluez/hci0/dev_feather/fd/sink0 "
                "owner=:1.feather state=idle codec=sbc",
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=listen-native "
                "handle=0x0052",
                "upstream-l2cap-native-control: enabled=1",
                "bluez-audio: a2dp signaling listening handle=0x0052 "
                "native-control=1",
                "bluez-audio: a2dp auto-rsp signal=0x01",
                "state=IDLE->DISCOVERED",
                "bluez-audio: a2dp auto-rsp signal=0x07",
                "state=OPEN->STREAMING",
                "bluez-audio: a2dp auto-rsp-loop complete count=5",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp.c "
                "style profile=a2dp-codec role=sink "
                "command=sink-sbc-recv-decode-release peer=1 "
                "handle=0x0052",
                "bluez-audio: media transport acquire fd=l2cap",
                "bluez-audio: media transport handle open role=sink "
                "psm=0x0019 cid=0x0041 handle=0x0052 ret=0",
                "bluez-audio: media transport handle connect role=sink "
                "psm=0x0019 cid=0x0041 handle=0x0052 ret=0",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "dbus method=Acquire interface=org.bluez.MediaTransport1 "
                "path=/org/bluez/hci0/dev_feather/fd/sink0 fd=l2cap "
                "role=sink",
                "codec=sbc",
                "re:upstream-l2cap-recv-handle: psm=0x0019 cid=0x0041 "
                "handle=0x0052 recv-ret=[1-9][0-9]* "
                "flags=0x0 payload=9c bd [0-9a-f]{2}",
                "bluez-audio: codec-backend profile=a2dp codec=sbc "
                "backend=libsbc status=source-built "
                "required-source=third/sbc-2.0",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp-codecs.h "
                "selected-config=21 15 02 35 codec-id=0x00",
                "re:bluez-audio: source=third/bluez/profiles/audio/a2dp-codecs\\.h "
                "sbc decode syncword=0x9c frame-header=bd [0-9a-f]{2} [0-9a-f]{2} "
                "samplerate=44100 channels=2 channel-mode=joint-stereo "
                "blocks=16 subbands=8 allocation=loudness bitpool=[1-9][0-9]* "
                "frame-len=[1-9][0-9]*",
                "re:bluez-audio: source=third/sbc-2\\.0/sbc/sbc\\.c "
                "sbc_decode pcm-bytes=[1-9][0-9]* codesize=[1-9][0-9]* "
                "checksum=0x[0-9a-f]{8}",
                "re:bluez-audio: source=third/bluez/profiles/audio/media\\.c "
                "pcm output format=s16le rate=44100 channels=2 "
                "pcm-bytes=[1-9][0-9]* checksum=0x[0-9a-f]{8}",
                "re:bluez-audio: a2dp codec sink decoded pcm checksum=0x[0-9a-f]{8}",
                "bluez-audio: media transport handle close role=sink "
                "psm=0x0019 cid=0x0041 handle=0x0052 ret=0",
                "bluez-audio: media transport release complete role=sink codec=sbc",
                "bluez-audio: a2dp auto-rsp-loop initial-state=STREAMING",
                "bluez-audio: a2dp auto-rsp signal=0x09 msg-type=0x00 "
                "pkt-type=0x00 rsp-len=2 err=0x00 state=STREAMING->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x08 msg-type=0x00 "
                "pkt-type=0x00 rsp-len=2 err=0x00 state=OPEN->IDLE",
                "bluez-audio: a2dp auto-rsp-loop complete count=2",
                "bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus signal=InterfacesRemoved "
                "path=/org/bluez/hci0/dev_feather/fd/sink0 "
                "interface=org.bluez.MediaTransport1 role=sink",
                "bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus signal=InterfacesRemoved "
                "path=/org/bluez/hci0/dev_feather/sep/sink0 "
                "interface=org.bluez.MediaEndpoint1 role=sink",
                "bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus signal=InterfacesRemoved path=/org/bluez/hci0/dev_feather "
                "interface=org.bluez.MediaControl1 role=sink",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus unexport interface=org.bluez.MediaControl1 "
                "path=/org/bluez/hci0/dev_feather role=sink",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "sdp unregister controller uuid=0000110e-0000-1000-8000-00805f9b34fb "
                "target uuid=0000110c-0000-1000-8000-00805f9b34fb "
                "role=sink",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "control unregister psm=0x0017 browsing-psm=0x001b "
                "role=sink",
                "bluez-audio: source=third/bluez/profiles/audio/sink.c "
                "sdp unregister service=AudioSink "
                "uuid=0000110b-0000-1000-8000-00805f9b34fb psm=0x0019",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "dbus method=ClearConfiguration owner=:1.feather "
                "endpoint=/org/bluez/hci0/dev_feather/sep/sink0 "
                "transport=/org/bluez/hci0/dev_feather/fd/sink0",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "dbus unexport interface=org.bluez.MediaTransport1 "
                "path=/org/bluez/hci0/dev_feather/fd/sink0",
                "bluez-audio: media endpoint clear complete role=sink",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-listen=[1-9]",
                "re:l2cap-socket-recv=[0-9]+",
                "re:l2cap-socket-native-recv=([8-9]|[1-9][0-9]+)",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-sbc-codec-extended",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=source peer=2",
                "bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus object-manager path=/ "
                "interface=org.freedesktop.DBus.ObjectManager "
                "method=GetManagedObjects adapter=/org/bluez/hci0 "
                "device=/org/bluez/hci0/dev_feather role=source",
                "bluez-audio: a2dp signaling discover peer=2",
                "bluez-audio: a2dp signaling getallcap peer=2",
                "bluez-audio: a2dp signaling getcap peer=2",
                "bluez-audio: a2dp signaling setconfig peer=2",
                "bluez-audio: a2dp signaling getconfig peer=2",
                "bluez-audio: a2dp signaling reconfigure peer=2",
                "bluez-audio: a2dp signaling delay-report peer=2",
                "bluez-audio: a2dp signaling security-control peer=2",
                "bluez-audio: a2dp signaling open peer=2",
                "bluez-audio: a2dp signaling start peer=2",
                "bluez-audio: a2dp signaling suspend peer=2",
                "bluez-audio: a2dp signaling close-stream peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=source-session-open "
                "peer=2 handle=0x0052",
                "upstream-l2cap-bind: psm=0x0019 cid=0x0040 "
                "handle=0x0052",
                "upstream-l2cap-connect: psm=0x0019 cid=0x0040 "
                "connect-ret=0",
                "upstream-l2cap-write: payload-len=2 send-ret=2 "
                "native-ret=2 attach-ret=0 fallback-ret=-95",
                "count>=7:upstream-l2cap-write: payload-len=3 send-ret=3 "
                "native-ret=3 attach-ret=0 fallback-ret=-95",
                "count>=2:upstream-l2cap-write: payload-len=5 send-ret=5 "
                "native-ret=5 attach-ret=0 fallback-ret=-95",
                "upstream-l2cap-write: payload-len=11 send-ret=11 "
                "native-ret=11 attach-ret=0 fallback-ret=-95",
                "upstream-l2cap-write: payload-len=14 send-ret=14 "
                "native-ret=14 attach-ret=0 fallback-ret=-95",
                "bluez-audio: a2dp source session closed "
                "peer=2 handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp.c "
                "style profile=a2dp-codec role=source "
                "command=source-sbc-encode-write-release peer=2 "
                "handle=0x0052",
                "bluez-audio: codec-backend profile=a2dp codec=sbc "
                "backend=libsbc status=source-built "
                "required-source=third/sbc-2.0",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp-codecs.h "
                "selected-config=21 15 02 35 codec-id=0x00",
                "bluez-audio: media transport release complete role=source codec=sbc",
                "bluez-audio: media endpoint clear complete role=source",
                "re:l2cap-socket-send=[1-9]",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=sink peer=1",
                "bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus object-manager path=/ "
                "interface=org.freedesktop.DBus.ObjectManager "
                "method=GetManagedObjects adapter=/org/bluez/hci0 "
                "device=/org/bluez/hci0/dev_feather role=sink",
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=listen-native "
                "handle=0x0052",
                "upstream-l2cap-native-control: enabled=1",
                "bluez-audio: a2dp signaling listening handle=0x0052 "
                "native-control=1",
                "bluez-audio: a2dp auto-rsp-loop complete count=10",
                "bluez-audio: a2dp auto-rsp-loop initial-state=STREAMING",
                "bluez-audio: a2dp auto-rsp signal=0x09 msg-type=0x00 "
                "pkt-type=0x00 rsp-len=2 err=0x00 state=STREAMING->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x08 msg-type=0x00 "
                "pkt-type=0x00 rsp-len=2 err=0x00 state=OPEN->IDLE",
                "bluez-audio: a2dp auto-rsp-loop complete count=2",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp.c "
                "style profile=a2dp-codec role=sink "
                "command=sink-sbc-recv-decode-release peer=1 "
                "handle=0x0052",
                "bluez-audio: codec-backend profile=a2dp codec=sbc "
                "backend=libsbc status=source-built "
                "required-source=third/sbc-2.0",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp-codecs.h "
                "selected-config=21 15 02 35 codec-id=0x00",
                "bluez-audio: media transport release complete role=sink codec=sbc",
                "bluez-audio: media endpoint clear complete role=sink",
                "re:bluez-audio: source=third/sbc-2\\.0/sbc/sbc\\.c "
                "sbc_decode pcm-bytes=[1-9][0-9]* codesize=[1-9][0-9]* "
                "checksum=0x[0-9a-f]{8}",
                "re:l2cap-socket-native-recv=(1[3-9]|[2-9][0-9]+)",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-sbc-codec-abort",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=source peer=2",
                "bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus object-manager path=/ "
                "interface=org.freedesktop.DBus.ObjectManager "
                "method=GetManagedObjects adapter=/org/bluez/hci0 "
                "device=/org/bluez/hci0/dev_feather role=source",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "dbus owner=:1.feather adapter=/org/bluez/hci0 "
                "interface=org.bluez.Media1 method=RegisterEndpoint "
                "endpoint=/org/bluez/hci0/dev_feather/sep/source0 role=source",
                "bluez-audio: source=third/bluez/client/player.c "
                "preset=a2dp_src/sbc codec=sbc codec-id=0x00 "
                "caps=ff ff 02 40 preset-count=6",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp.c select-config role=source",
                "configuration=21 15 02 35",
                "bluez-audio: a2dp signaling discover peer=2",
                "bluez-audio: a2dp signaling getcap peer=2",
                "bluez-audio: a2dp signaling setconfig peer=2",
                "bluez-audio: a2dp signaling open peer=2",
                "bluez-audio: a2dp signaling abort peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=source-session-open "
                "peer=2 handle=0x0052",
                "upstream-l2cap-bind: psm=0x0019 cid=0x0040 "
                "handle=0x0052",
                "upstream-l2cap-connect: psm=0x0019 cid=0x0040 "
                "connect-ret=0",
                "upstream-l2cap-write: payload-len=2 send-ret=2 "
                "native-ret=2 attach-ret=0 fallback-ret=-95",
                "count>=3:upstream-l2cap-write: payload-len=3 send-ret=3 "
                "native-ret=3 attach-ret=0 fallback-ret=-95",
                "upstream-l2cap-write: payload-len=14 send-ret=14 "
                "native-ret=14 attach-ret=0 fallback-ret=-95",
                "bluez-audio: a2dp source session closed peer=2 "
                "handle=0x0052",
                "bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus signal=InterfacesRemoved "
                "path=/org/bluez/hci0/dev_feather/sep/source0 "
                "interface=org.bluez.MediaEndpoint1 role=source",
                "bluez-audio: media endpoint clear complete role=source",
                "re:l2cap-socket-send=[0-9]+",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=sink peer=1",
                "bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus object-manager path=/ "
                "interface=org.freedesktop.DBus.ObjectManager "
                "method=GetManagedObjects adapter=/org/bluez/hci0 "
                "device=/org/bluez/hci0/dev_feather role=sink",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "dbus owner=:1.feather adapter=/org/bluez/hci0 "
                "interface=org.bluez.Media1 method=RegisterEndpoint "
                "endpoint=/org/bluez/hci0/dev_feather/sep/sink0 role=sink",
                "bluez-audio: source=third/bluez/client/player.c "
                "preset=a2dp_sink/sbc codec=sbc codec-id=0x00 "
                "caps=ff ff 02 40 preset-count=6",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp.c select-config role=sink",
                "configuration=21 15 02 35",
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=listen-native "
                "handle=0x0052",
                "upstream-l2cap-native-control: enabled=1",
                "bluez-audio: a2dp signaling listening handle=0x0052 "
                "native-control=1",
                "bluez-audio: a2dp auto-rsp signal=0x01",
                "state=IDLE->DISCOVERED",
                "bluez-audio: a2dp auto-rsp signal=0x02",
                "bluez-audio: a2dp auto-rsp signal=0x03",
                "state=DISCOVERED->CONFIGURED",
                "bluez-audio: a2dp auto-rsp signal=0x06",
                "state=CONFIGURED->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x0a",
                "state=OPEN->IDLE",
                "bluez-audio: a2dp auto-rsp-loop complete count=5",
                "bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus signal=InterfacesRemoved "
                "path=/org/bluez/hci0/dev_feather/sep/sink0 "
                "interface=org.bluez.MediaEndpoint1 role=sink",
                "bluez-audio: media endpoint clear complete role=sink",
                "re:l2cap-socket-native-recv=([5-9]|[1-9][0-9]+)",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-sbc-codec-reconnect",
        (
            RoleCheck("bt1", (
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=source peer=2",
                "count>=2:bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus object-manager path=/ "
                "interface=org.freedesktop.DBus.ObjectManager "
                "method=GetManagedObjects adapter=/org/bluez/hci0 "
                "device=/org/bluez/hci0/dev_feather role=source",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=source-session-open "
                "peer=2 handle=0x0052",
                "count>=2:upstream-l2cap-bind: psm=0x0019 cid=0x0040 "
                "handle=0x0052",
                "count>=2:upstream-l2cap-connect: psm=0x0019 cid=0x0040 "
                "connect-ret=0",
                "count>=2:upstream-l2cap-write: payload-len=2 send-ret=2 "
                "native-ret=2 attach-ret=0 fallback-ret=-95",
                "count>=10:upstream-l2cap-write: payload-len=3 send-ret=3 "
                "native-ret=3 attach-ret=0 fallback-ret=-95",
                "count>=2:upstream-l2cap-write: payload-len=14 send-ret=14 "
                "native-ret=14 attach-ret=0 fallback-ret=-95",
                "count>=2:bluez-audio: a2dp source session closed "
                "peer=2 handle=0x0052",
                "count>=2:bluez-audio: a2dp signaling suspend peer=2",
                "count>=2:bluez-audio: a2dp signaling close-stream peer=2",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/a2dp.c "
                "style profile=a2dp-codec role=source "
                "command=source-sbc-encode-write-release peer=2 "
                "handle=0x0052",
                "count>=2:bluez-audio: codec-backend profile=a2dp codec=sbc "
                "backend=libsbc status=source-built "
                "required-source=third/sbc-2.0",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/a2dp-codecs.h "
                "selected-config=21 15 02 35 codec-id=0x00",
                "count>=2:bluez-audio: media transport release complete role=source codec=sbc",
                "count>=2:bluez-audio: media endpoint clear complete role=source",
                "re:l2cap-socket-send=([2-9]|[1-9][0-9]+)",
            )),
            RoleCheck("bt2", (
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=sink peer=1",
                "count>=2:bluez-audio: source=third/bluez/gdbus/object.c "
                "dbus object-manager path=/ "
                "interface=org.freedesktop.DBus.ObjectManager "
                "method=GetManagedObjects adapter=/org/bluez/hci0 "
                "device=/org/bluez/hci0/dev_feather role=sink",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=listen-native "
                "handle=0x0052",
                "count>=2:upstream-l2cap-native-control: enabled=1",
                "count>=2:bluez-audio: a2dp signaling listening handle=0x0052 "
                "native-control=1",
                "count>=2:bluez-audio: a2dp auto-rsp-loop complete count=5",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/a2dp.c "
                "style profile=a2dp-codec role=sink "
                "command=sink-sbc-recv-decode-release peer=1 "
                "handle=0x0052",
                "count>=2:bluez-audio: codec-backend profile=a2dp codec=sbc "
                "backend=libsbc status=source-built "
                "required-source=third/sbc-2.0",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/a2dp-codecs.h "
                "selected-config=21 15 02 35 codec-id=0x00",
                "count>=2:bluez-audio: media transport release complete role=sink codec=sbc",
                "count>=2:bluez-audio: a2dp auto-rsp-loop initial-state=STREAMING",
                "count>=2:bluez-audio: a2dp auto-rsp signal=0x09 msg-type=0x00 "
                "pkt-type=0x00 rsp-len=2 err=0x00 state=STREAMING->OPEN",
                "count>=2:bluez-audio: a2dp auto-rsp signal=0x08 msg-type=0x00 "
                "pkt-type=0x00 rsp-len=2 err=0x00 state=OPEN->IDLE",
                "count>=2:bluez-audio: a2dp auto-rsp-loop complete count=2",
                "count>=2:bluez-audio: media endpoint clear complete role=sink",
                "re:bluez-audio: source=third/sbc-2\\.0/sbc/sbc\\.c "
                "sbc_decode pcm-bytes=[1-9][0-9]* codesize=[1-9][0-9]* "
                "checksum=0x[0-9a-f]{8}",
                "re:l2cap-socket-native-recv=(1[6-9]|[2-9][0-9]+)",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-transport-reconnect",
        (
            RoleCheck("bt1", (
                "count>=2:bluez-audio: a2dp signaling discover peer=2",
                "count>=2:bluez-audio: a2dp signaling getcap peer=2",
                "count>=2:bluez-audio: a2dp signaling setconfig peer=2",
                "count>=2:bluez-audio: a2dp signaling open peer=2",
                "count>=2:bluez-audio: a2dp signaling start peer=2",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=a2dp-source-acquire-write-release peer=2 "
                "handle=0x0052",
                "count>=2:upstream-l2cap-bind: psm=0x0019 cid=0x0041 "
                "handle=0x0052",
                "count>=2:upstream-l2cap-connect: psm=0x0019 cid=0x0041",
                "count>=2:bluez-audio: media transport acquire fd=l2cap",
                "count>=2:upstream-l2cap-write: payload-len=24 send-ret=24",
                "count>=2:bluez-audio: media transport write len=24 "
                "payload=A2DP:SBC:synthetic-frame",
                "count>=2:upstream-l2cap-close: released",
                "count>=2:bluez-audio: media transport release complete role=source",
                "count>=2:bluez-audio: a2dp signaling suspend peer=2",
                "count>=2:bluez-audio: a2dp signaling close-stream peer=2",
                "re:l2cap-socket-bind=([2-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=([2-9]|[1-9][0-9]+)",
                "re:l2cap-socket-send=([2-9]|[1-9][0-9]+)",
            )),
            RoleCheck("bt2", (
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=listen "
                "handle=0x0052",
                "count>=2:bluez-audio: a2dp signaling listening handle=0x0052",
                "count>=2:bluez-audio: a2dp auto-rsp signal=0x01",
                "count>=2:state=IDLE->DISCOVERED",
                "count>=2:bluez-audio: a2dp auto-rsp signal=0x02",
                "count>=2:bluez-audio: a2dp auto-rsp signal=0x03",
                "count>=2:state=DISCOVERED->CONFIGURED",
                "count>=2:bluez-audio: a2dp auto-rsp signal=0x06",
                "count>=2:state=CONFIGURED->OPEN",
                "count>=2:bluez-audio: a2dp auto-rsp signal=0x07",
                "count>=2:state=OPEN->STREAMING",
                "count>=2:bluez-audio: a2dp auto-rsp-loop complete count=5",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=a2dp-sink-acquire-read-release peer=1 "
                "handle=0x0052",
                "count>=2:upstream-l2cap-bind: psm=0x0019 cid=0x0041",
                "count>=2:bluez-audio: media transport acquire fd=l2cap",
                "count>=2:upstream-l2cap-recv-handle: psm=0x0019 cid=0x0041 handle=0x0052 recv-ret=24",
                "payload=41 32 44 50 3a 53 42 43 3a 73 79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "count>=2:bluez-audio: media transport read complete "
                "payload=A2DP:SBC:synthetic-frame",
                "count>=2:upstream-l2cap-close: released",
                "count>=2:bluez-audio: media transport release complete role=sink",
                "re:l2cap-socket-bind=([2-9]|[1-9][0-9]+)",
                "re:l2cap-socket-listen=([2-9]|[1-9][0-9]+)",
                "re:l2cap-socket-recv=([2-9]|[1-9][0-9]+)",
                "re:l2cap-socket-native-recv=([2-9]|[1-9][0-9]+)",
            )),
        ),
    ),
    CaseCheck(
        "a2dp-extended",
        (
            RoleCheck("bt1", (
                "btctl: upstream hci-connect-br peer=2",
                "btctl: upstream avdtp-discover peer=2",
                "btctl: upstream avdtp-getallcap peer=2",
                "btctl: upstream avdtp-getcap peer=2",
                "btctl: upstream avdtp-setconfig peer=2",
                "btctl: upstream avdtp-getconfig peer=2",
                "btctl: upstream avdtp-open peer=2",
                "btctl: upstream avdtp-reconfigure peer=2",
                "btctl: upstream avdtp-delay-report peer=2",
                "btctl: upstream avdtp-security-control peer=2",
                "btctl: upstream avdtp-start peer=2",
                "btctl: upstream avdtp-suspend peer=2",
                "btctl: upstream avdtp-abort peer=2",
                "count>=12:upstream-l2cap-send: psm=0x0019 cid=0x0040 handle=0x0052",
                "count>=12:attach-ret=0 fallback-ret=-95",
                "count>=12:upstream-l2cap-close: released",
            )),
            RoleCheck("bt2", (
                "btctl: upstream avdtp signaling listening",
                "btctl: upstream avdtp-auto-rsp signal=0x01",
                "state=IDLE->DISCOVERED",
                "btctl: upstream avdtp-auto-rsp signal=0x0c",
                "state=DISCOVERED->DISCOVERED",
                "btctl: upstream avdtp-auto-rsp signal=0x02",
                "btctl: upstream avdtp-auto-rsp signal=0x03",
                "state=DISCOVERED->CONFIGURED",
                "btctl: upstream avdtp-auto-rsp signal=0x04",
                "state=CONFIGURED->CONFIGURED",
                "btctl: upstream avdtp-auto-rsp signal=0x06",
                "state=CONFIGURED->OPEN",
                "btctl: upstream avdtp-auto-rsp signal=0x05",
                "state=OPEN->OPEN",
                "btctl: upstream avdtp-auto-rsp signal=0x0d",
                "btctl: upstream avdtp-auto-rsp signal=0x0b",
                "btctl: upstream avdtp-auto-rsp signal=0x07",
                "state=OPEN->STREAMING",
                "btctl: upstream avdtp-auto-rsp signal=0x09",
                "state=STREAMING->OPEN",
                "btctl: upstream avdtp-auto-rsp signal=0x0a",
                "state=OPEN->IDLE",
                "btctl: upstream avdtp-auto-rsp-loop complete count=12",
                "count>=12:attach-ret=0 fallback-ret=-95",
                "upstream-l2cap-close: released",
            )),
        ),
    ),
    CaseCheck(
        "bnep-session",
        (
            RoleCheck("bt1", (
                "upstream-l2cap-bind:",
                "upstream-l2cap-connect:",
                "name=BNEPGETSUPPFEAT",
                "supp-feat=0x00000001",
                "name=BNEPCONNADD",
                "ioctl-ret=0",
                "connadd-device=btn0",
                "name=BNEPGETCONNLIST",
                "connlist-cnum=1",
                "connlist0-state=0x0001",
                "connlist0-device=btn0",
                "name=BNEPGETCONNINFO",
                "conninfo-state=0x0001",
                "conninfo-device=btn0",
                "bnep-native-active=1",
                "bnep-native-netdev-register=1",
                "bnep-native-session-create=1",
                "bnep-native-session-start=1",
                "name=BNEPCONNDEL",
                "bnep-native-active=0",
                "bnep-native-netdev-unregister=1",
                "bnep-native-session-stop=1",
                "bnep-native-session-terminate=1",
                "upstream-l2cap-close: already-released",
            )),
        ),
    ),
    CaseCheck(
        "bnep-ping",
        (
            RoleCheck("bt1", (
                "connadd-device=btn0",
                "PING 10.77.0.2",
                "2 packets transmitted, 2 received, 0% packet loss",
                "upstream-l2cap-close: released",
            )),
            RoleCheck("bt2", (
                "connadd-device=btn0",
                "PING 10.77.0.1",
                "2 packets transmitted, 2 received, 0% packet loss",
                "upstream-l2cap-close: released",
            )),
        ),
    ),
    CaseCheck(
        "bneptest-fd-probe",
        (
            RoleCheck("bt1", (
                "btbneptest: fd-probe",
                "btbneptest: fd-probe l2cap-socket fd=",
                "btbneptest: fd-probe l2cap-bind ret=0",
                "btbneptest: fd-probe l2cap-connect ret=0",
                "btbneptest: fd-probe bnep-socket fd=",
                "btbneptest: fd-probe bnep-suppfeat ret=0",
                "features=0x00000001",
                "btbneptest: fd-probe bnep-connlist-empty ret=0",
                "cnum=0",
                "btbneptest: fd-probe bnep-conndel-missing ret=-1",
                "btbneptest: fd-probe bnep-conninfo-missing ret=-1",
                "btbneptest: fd-probe bnep-connadd-invalid ret=-1",
                "btbneptest: fd-probe bnep-connadd ret=0",
                "device=btn0",
                "btbneptest: fd-probe bnep-connlist-postadd ret=0",
                "cnum=1 state=0x0001 device=btn0",
                "btbneptest: fd-probe bnep-conninfo-postadd ret=0",
                "state=0x0001 device=btn0",
                "btbneptest: fd-probe bnep-conndel-postadd ret=0",
                "btbneptest: fd-probe bnep-connlist-postdel ret=0",
                "cnum=0",
                "bnep-native-active=0",
                "bnep-native-netdev-register=1",
                "bnep-native-netdev-unregister=1",
                "bnep-native-session-create=1",
                "bnep-native-session-start=1",
                "bnep-native-session-stop=1",
                "bnep-native-session-terminate=1",
            )),
        ),
    ),
    CaseCheck(
        "bluez-bneptest-fd-handoff",
        (
            RoleCheck("bt1", (
                "bluez-bneptest: source=third/bluez/tools/bneptest.c",
                "mode=fd-handoff",
                "bluez-bneptest: l2cap-socket fd=",
                "bluez-bneptest: l2cap-bind ret=0",
                "bluez-bneptest: l2cap-connect ret=0",
                "bluez-bneptest: bnep-socket fd=",
                "bluez-bneptest: bnep-suppfeat ret=0",
                "features=0x00000001",
                "bluez-bneptest: bnep-connadd ret=0",
                "device=btn0",
                "bluez-bneptest: bnep-connlist ret=0",
                "cnum=1 state=0x0001 device=btn0",
                "bluez-bneptest: bnep-conninfo ret=0",
                "state=0x0001 device=btn0",
                "bluez-bneptest: bnep-conndel ret=0",
                "bluez-bneptest: bnep-connlist-postdel ret=0",
                "bluez-bneptest: fd-handoff complete",
                "bnep-ioctl-connadd=1",
                "bnep-ioctl-conndel=2",
                "bnep-native-active=0",
                "bnep-native-netdev-register=1",
                "bnep-native-netdev-unregister=1",
                "bnep-native-session-create=1",
                "bnep-native-session-start=1",
                "bnep-native-session-stop=1",
                "bnep-native-session-terminate=1",
            )),
        ),
    ),
    CaseCheck(
        "bluez-bneptest-ping",
        (
            RoleCheck("bt1", (
                "bluez-bneptest: source=third/bluez/tools/bneptest.c",
                "mode=pan-up",
                "bluez-bneptest: l2cap-socket fd=",
                "bluez-bneptest: bnep-socket fd=",
                "bluez-bneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                "bluez-bneptest: pan-up complete",
                "bluez-bneptest: bnep-connlist ret=0",
                "bluez-bneptest: bnep-conninfo-fd ret=0",
                "bnep-ioctl-connlist=1",
                "bnep-ioctl-conninfo=1",
                "bnep-ioctl-connadd=1",
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "PING 10.77.0.2",
                "2 packets transmitted, 2 received, 0% packet loss",
                "bluez-bneptest: bnep-conndel-fd ret=0",
                "bluez-bneptest: bnep-connlist-postdel ret=0",
                "cnum=0",
                "bluez-bneptest: pan-down complete",
                *BNEP_NATIVE_TEARDOWN_REQUIRED,
            )),
            RoleCheck("bt2", (
                "bluez-bneptest: source=third/bluez/tools/bneptest.c",
                "mode=pan-up",
                "bluez-bneptest: l2cap-socket fd=",
                "bluez-bneptest: bnep-socket fd=",
                "bluez-bneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                "bluez-bneptest: pan-up complete",
                "bluez-bneptest: bnep-connlist ret=0",
                "bluez-bneptest: bnep-conninfo-fd ret=0",
                "bnep-ioctl-connlist=1",
                "bnep-ioctl-conninfo=1",
                "bnep-ioctl-connadd=1",
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "PING 10.77.0.1",
                "2 packets transmitted, 2 received, 0% packet loss",
                "bluez-bneptest: bnep-conndel-fd ret=0",
                "bluez-bneptest: bnep-connlist-postdel ret=0",
                "cnum=0",
                "bluez-bneptest: pan-down complete",
                *BNEP_NATIVE_TEARDOWN_REQUIRED,
            )),
        ),
    ),
    CaseCheck(
        "bluez-network-ping",
        (
            RoleCheck("bt1", (
                "bluez-network: source=third/bluez/profiles/network/"
                "connection.c+profiles/network/bnep.c",
                "mode=connect",
                "service=panu",
                "bluez-network: profile interface=org.bluez.Network1 "
                "state=connecting",
                "bluez-network: l2cap-socket fd=",
                "bluez-network: l2cap-bind ret=0",
                "bluez-network: l2cap-connect ret=0",
                "bluez-network: bnep-socket fd=",
                "bluez-network: suppfeat ret=0",
                "features=0x00000001",
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "bluez-network: profile connected interface=btn0",
                "bluez-network: connect complete",
                "bluez-network: mode=status",
                "bluez-network: conninfo ret=0",
                "connected=true",
                "bnep-ioctl-connadd=1",
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "PING 10.77.0.2",
                "2 packets transmitted, 2 received, 0% packet loss",
                "bluez-network: mode=disconnect",
                "bluez-network: bnep-conndel ret=0",
                "bluez-network: connlist-postdel ret=0",
                "cnum=0",
                "bluez-network: profile disconnected",
                "bluez-network: disconnect complete",
                *BNEP_NATIVE_TEARDOWN_REQUIRED,
            )),
            RoleCheck("bt2", (
                "bluez-network: source=third/bluez/profiles/network/"
                "connection.c+profiles/network/bnep.c",
                "mode=connect",
                "service=panu",
                "bluez-network: profile interface=org.bluez.Network1 "
                "state=connecting",
                "bluez-network: l2cap-socket fd=",
                "bluez-network: l2cap-bind ret=0",
                "bluez-network: l2cap-connect ret=0",
                "bluez-network: bnep-socket fd=",
                "bluez-network: suppfeat ret=0",
                "features=0x00000001",
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "bluez-network: profile connected interface=btn0",
                "bluez-network: connect complete",
                "bluez-network: mode=status",
                "bluez-network: conninfo ret=0",
                "connected=true",
                "bnep-ioctl-connadd=1",
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "PING 10.77.0.1",
                "2 packets transmitted, 2 received, 0% packet loss",
                "bluez-network: mode=disconnect",
                "bluez-network: bnep-conndel ret=0",
                "bluez-network: connlist-postdel ret=0",
                "cnum=0",
                "bluez-network: profile disconnected",
                "bluez-network: disconnect complete",
                *BNEP_NATIVE_TEARDOWN_REQUIRED,
            )),
        ),
    ),
    CaseCheck(
        "bluez-network-daemon-role-matrix",
        (
            RoleCheck("bt1", (
                "bluez-network: source=third/bluez/src/main.c+src/profile.c+"
                "profiles/network/manager.c+profiles/network/server.c+"
                "profiles/network/connection.c+profiles/network/bnep.c "
                "mode=daemon-profile action=register",
                "bluez-network: dbus name-owner=org.bluez acquired=true",
                "interface=org.freedesktop.DBus.ObjectManager registered=true",
                "interface=org.bluez.NetworkServer1 service=panu uuid=0x1115",
                "interface=org.bluez.NetworkServer1 service=nap uuid=0x1116",
                "interface=org.bluez.NetworkServer1 service=gn uuid=0x1117",
                "role-policy service=panu uuid=0x1115 allowed=1",
                "role-policy service=nap uuid=0x1116 allowed=1",
                "role-policy service=gn uuid=0x1117 allowed=1",
                "bluez-network: daemon-profile connect complete "
                "service=panu role=0x1115",
                "bluez-network: daemon-profile connect complete "
                "service=nap role=0x1116",
                "bluez-network: daemon-profile connect complete "
                "service=gn role=0x1117",
                "bluez-network: daemon-profile action=status "
                "service=panu role=0x1115",
                "bluez-network: daemon-profile action=status "
                "service=nap role=0x1116",
                "bluez-network: daemon-profile action=status "
                "service=gn role=0x1117",
                "bluez-network: daemon-profile action=disconnect "
                "service=panu role=0x1115",
                "bluez-network: daemon-profile action=disconnect "
                "service=nap role=0x1116",
                "bluez-network: daemon-profile action=disconnect "
                "service=gn role=0x1117",
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "PING 10.77.0.2",
                "2 packets transmitted, 2 received, 0% packet loss",
                "bnep-ioctl-connadd=3",
                "bnep-ioctl-conndel=3",
                "bnep-native-session-create=3",
                "bnep-native-session-start=3",
                "bnep-native-session-terminate=3",
                "bnep-native-session-stop=3",
                "bnep-native-netdev-register=3",
                "bnep-native-netdev-unregister=3",
                "bnep-native-active=0",
            )),
            RoleCheck("bt2", (
                "bluez-network: source=third/bluez/src/main.c+src/profile.c+"
                "profiles/network/manager.c+profiles/network/server.c+"
                "profiles/network/connection.c+profiles/network/bnep.c "
                "mode=daemon-profile action=register",
                "bluez-network: dbus name-owner=org.bluez acquired=true",
                "interface=org.freedesktop.DBus.ObjectManager registered=true",
                "interface=org.bluez.NetworkServer1 service=panu uuid=0x1115",
                "interface=org.bluez.NetworkServer1 service=nap uuid=0x1116",
                "interface=org.bluez.NetworkServer1 service=gn uuid=0x1117",
                "role-policy service=panu uuid=0x1115 allowed=1",
                "role-policy service=nap uuid=0x1116 allowed=1",
                "role-policy service=gn uuid=0x1117 allowed=1",
                "bluez-network: daemon-profile connect complete "
                "service=nap role=0x1116",
                "bluez-network: daemon-profile connect complete "
                "service=gn role=0x1117",
                "bluez-network: daemon-profile connect complete "
                "service=panu role=0x1115",
                "bluez-network: daemon-profile action=status "
                "service=nap role=0x1116",
                "bluez-network: daemon-profile action=status "
                "service=gn role=0x1117",
                "bluez-network: daemon-profile action=status "
                "service=panu role=0x1115",
                "bluez-network: daemon-profile action=disconnect "
                "service=nap role=0x1116",
                "bluez-network: daemon-profile action=disconnect "
                "service=gn role=0x1117",
                "bluez-network: daemon-profile action=disconnect "
                "service=panu role=0x1115",
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "PING 10.77.0.1",
                "2 packets transmitted, 2 received, 0% packet loss",
                "bnep-ioctl-connadd=3",
                "bnep-ioctl-conndel=3",
                "bnep-native-session-create=3",
                "bnep-native-session-start=3",
                "bnep-native-session-terminate=3",
                "bnep-native-session-stop=3",
                "bnep-native-netdev-register=3",
                "bnep-native-netdev-unregister=3",
                "bnep-native-active=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-network-daemon-full-lifecycle",
        (
            RoleCheck("bt1", (
                "bluez-network: source=third/bluez/src/main.c+src/profile.c+"
                "profiles/network/manager.c+profiles/network/server.c+"
                "profiles/network/connection.c+profiles/network/bnep.c "
                "mode=daemon-profile action=register",
                "bluez-network: dbus name-owner=org.bluez acquired=true",
                "interface=org.freedesktop.DBus.ObjectManager registered=true",
                "interface=org.bluez.Adapter1 powered=true",
                "interface=org.bluez.NetworkServer1 service=panu uuid=0x1115",
                "interface=org.bluez.NetworkServer1 service=nap uuid=0x1116",
                "interface=org.bluez.NetworkServer1 service=gn uuid=0x1117",
                "role-policy service=panu uuid=0x1115 allowed=1",
                "role-policy service=nap uuid=0x1116 allowed=1",
                "role-policy service=gn uuid=0x1117 allowed=1",
                "bluez-network: daemon-profile connect complete "
                "service=panu role=0x1115",
                "bluez-network: daemon-profile connect complete "
                "service=nap role=0x1116",
                "bluez-network: daemon-profile connect complete "
                "service=gn role=0x1117",
                "bluez-network: daemon-profile action=status "
                "service=panu role=0x1115",
                "bluez-network: daemon-profile action=status "
                "service=nap role=0x1116",
                "bluez-network: daemon-profile action=status "
                "service=gn role=0x1117",
                "bluez-network: daemon-profile action=disconnect "
                "service=panu role=0x1115",
                "bluez-network: daemon-profile action=disconnect "
                "service=nap role=0x1116",
                "bluez-network: daemon-profile action=disconnect "
                "service=gn role=0x1117",
                "bluez-network: daemon-profile action=error-path "
                "service=gn role=0x1117",
                "missing-connection=org.bluez.Error.NotConnected",
                "duplicate=org.bluez.Error.AlreadyConnected",
                "cancel=org.bluez.Error.Canceled",
                "bluez-network: duplicate-connect rejected",
                "bluez-network: daemon-profile error-path complete",
                "bluez-network: daemon-profile action=unregister",
                "network-server unregister "
                "interface=org.bluez.NetworkServer1 service=panu uuid=0x1115",
                "network-server unregister "
                "interface=org.bluez.NetworkServer1 service=nap uuid=0x1116",
                "network-server unregister "
                "interface=org.bluez.NetworkServer1 service=gn uuid=0x1117",
                "interface=org.freedesktop.DBus.ObjectManager released=true",
                "bluez-network: dbus name-owner=org.bluez released=true",
                "bluez-network: daemon-profile unregister complete",
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "PING 10.77.0.2",
                "2 packets transmitted, 2 received, 0% packet loss",
                "bnep-ioctl-connadd=5",
                "bnep-ioctl-conndel=5",
                "bnep-native-session-create=4",
                "bnep-native-session-start=4",
                "bnep-native-session-terminate=4",
                "bnep-native-session-stop=4",
                "bnep-native-netdev-register=4",
                "bnep-native-netdev-unregister=4",
                "bnep-native-active=0",
            )),
            RoleCheck("bt2", (
                "bluez-network: source=third/bluez/src/main.c+src/profile.c+"
                "profiles/network/manager.c+profiles/network/server.c+"
                "profiles/network/connection.c+profiles/network/bnep.c "
                "mode=daemon-profile action=register",
                "bluez-network: dbus name-owner=org.bluez acquired=true",
                "interface=org.freedesktop.DBus.ObjectManager registered=true",
                "interface=org.bluez.Adapter1 powered=true",
                "interface=org.bluez.NetworkServer1 service=panu uuid=0x1115",
                "interface=org.bluez.NetworkServer1 service=nap uuid=0x1116",
                "interface=org.bluez.NetworkServer1 service=gn uuid=0x1117",
                "role-policy service=panu uuid=0x1115 allowed=1",
                "role-policy service=nap uuid=0x1116 allowed=1",
                "role-policy service=gn uuid=0x1117 allowed=1",
                "bluez-network: daemon-profile connect complete "
                "service=nap role=0x1116",
                "bluez-network: daemon-profile connect complete "
                "service=gn role=0x1117",
                "bluez-network: daemon-profile connect complete "
                "service=panu role=0x1115",
                "bluez-network: daemon-profile action=status "
                "service=nap role=0x1116",
                "bluez-network: daemon-profile action=status "
                "service=gn role=0x1117",
                "bluez-network: daemon-profile action=status "
                "service=panu role=0x1115",
                "bluez-network: daemon-profile action=disconnect "
                "service=nap role=0x1116",
                "bluez-network: daemon-profile action=disconnect "
                "service=gn role=0x1117",
                "bluez-network: daemon-profile action=disconnect "
                "service=panu role=0x1115",
                "bluez-network: daemon-profile action=error-path "
                "service=panu role=0x1115",
                "missing-connection=org.bluez.Error.NotConnected",
                "duplicate=org.bluez.Error.AlreadyConnected",
                "cancel=org.bluez.Error.Canceled",
                "bluez-network: duplicate-connect rejected",
                "bluez-network: daemon-profile error-path complete",
                "bluez-network: daemon-profile action=unregister",
                "network-server unregister "
                "interface=org.bluez.NetworkServer1 service=panu uuid=0x1115",
                "network-server unregister "
                "interface=org.bluez.NetworkServer1 service=nap uuid=0x1116",
                "network-server unregister "
                "interface=org.bluez.NetworkServer1 service=gn uuid=0x1117",
                "interface=org.freedesktop.DBus.ObjectManager released=true",
                "bluez-network: dbus name-owner=org.bluez released=true",
                "bluez-network: daemon-profile unregister complete",
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "PING 10.77.0.1",
                "2 packets transmitted, 2 received, 0% packet loss",
                "bnep-ioctl-connadd=5",
                "bnep-ioctl-conndel=5",
                "bnep-native-session-create=4",
                "bnep-native-session-start=4",
                "bnep-native-session-terminate=4",
                "bnep-native-session-stop=4",
                "bnep-native-netdev-register=4",
                "bnep-native-netdev-unregister=4",
                "bnep-native-active=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-network-error-path",
        (
            RoleCheck("bt1", (
                "bluez-network: source=third/bluez/profiles/network/"
                "connection.c+profiles/network/bnep.c mode=error-path",
                "bluez-network: conninfo-missing ret=-1",
                "connected=false",
                "bluez-network: conndel-missing ret=-1",
                "bluez-network: suppfeat ret=0",
                "features=0x00000001",
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "bluez-network: conninfo-active ret=0",
                "connected=true",
                "bluez-network: duplicate-connect rejected",
                "expected=1",
                "bluez-network: bnep-conndel ret=0",
                "bluez-network: connlist-postdel ret=0",
                "cnum=0",
                "bluez-network: error-path complete",
                "bnep-ioctl-connadd=1",
                "bnep-ioctl-conndel=2",
                "bnep-native-session-create=1",
                "bnep-native-session-start=1",
                "bnep-native-session-terminate=1",
                "bnep-native-session-stop=1",
                "bnep-native-netdev-register=1",
                "bnep-native-netdev-unregister=1",
                "bnep-native-active=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-network-daemon-profile",
        (
            RoleCheck("bt1", (
                "bluez-network: source=third/bluez/src/main.c+src/profile.c+"
                "profiles/network/manager.c+profiles/network/server.c+"
                "profiles/network/connection.c+profiles/network/bnep.c "
                "mode=daemon-profile action=register",
                "bluez-network: dbus name-owner=org.bluez acquired=true",
                "interface=org.freedesktop.DBus.ObjectManager registered=true",
                "interface=org.bluez.Adapter1 powered=true",
                "interface=org.bluez.NetworkServer1 service=panu uuid=0x1115",
                "interface=org.bluez.NetworkServer1 service=nap uuid=0x1116",
                "interface=org.bluez.NetworkServer1 service=gn uuid=0x1117",
                "role-policy service=panu uuid=0x1115 allowed=1",
                "role-policy service=nap uuid=0x1116 allowed=1",
                "role-policy service=gn uuid=0x1117 allowed=1",
                "bluez-network: daemon-profile action=connect "
                "service=panu role=0x1115",
                "bluez-network: dbus method=org.bluez.Network1.Connect",
                "authorization request service=panu uuid=0x1115 result=allowed",
                "service-record resolve service=panu uuid=0x1115 psm=0x000f",
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "bluez-network: dbus signal=PropertiesChanged "
                "interface=org.bluez.Network1 property=Connected value=true",
                "bluez-network: daemon-profile connect complete "
                "service=panu role=0x1115",
                "bluez-network: daemon-profile action=status "
                "service=panu role=0x1115",
                "bluez-network: dbus method=GetManagedObjects",
                "interface=org.bluez.Network1 service=panu uuid=0x1115",
                "bluez-network: conninfo ret=0",
                "connected=true",
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "PING 10.77.0.2",
                "2 packets transmitted, 2 received, 0% packet loss",
                "bluez-network: daemon-profile action=disconnect "
                "service=panu role=0x1115",
                "bluez-network: dbus method=org.bluez.Network1.Disconnect",
                "bluez-network: bnep-conndel ret=0",
                "bluez-network: connlist-postdel ret=0",
                "cnum=0",
                "bluez-network: dbus signal=PropertiesChanged "
                "interface=org.bluez.Network1 property=Connected value=false",
                "bluez-network: dbus signal=InterfacesRemoved",
                "bluez-network: daemon-profile disconnect complete",
                *BNEP_NATIVE_TEARDOWN_REQUIRED,
                "bluez-network: daemon-profile action=error-path "
                "service=panu role=0x1115",
                "missing-connection=org.bluez.Error.NotConnected",
                "duplicate=org.bluez.Error.AlreadyConnected",
                "cancel=org.bluez.Error.Canceled",
                "bluez-network: duplicate-connect rejected",
                "bluez-network: daemon-profile error-path complete",
                "bnep-native-active=0",
            )),
            RoleCheck("bt2", (
                "bluez-network: source=third/bluez/src/main.c+src/profile.c+"
                "profiles/network/manager.c+profiles/network/server.c+"
                "profiles/network/connection.c+profiles/network/bnep.c "
                "mode=daemon-profile action=register",
                "bluez-network: dbus name-owner=org.bluez acquired=true",
                "interface=org.freedesktop.DBus.ObjectManager registered=true",
                "interface=org.bluez.NetworkServer1 service=panu uuid=0x1115",
                "interface=org.bluez.NetworkServer1 service=nap uuid=0x1116",
                "interface=org.bluez.NetworkServer1 service=gn uuid=0x1117",
                "role-policy service=panu uuid=0x1115 allowed=1",
                "role-policy service=nap uuid=0x1116 allowed=1",
                "role-policy service=gn uuid=0x1117 allowed=1",
                "bluez-network: daemon-profile action=connect "
                "service=panu role=0x1115",
                "bluez-network: dbus method=org.bluez.Network1.Connect",
                "authorization request service=panu uuid=0x1115 result=allowed",
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "bluez-network: daemon-profile connect complete "
                "service=panu role=0x1115",
                "bluez-network: daemon-profile action=status "
                "service=panu role=0x1115",
                "bluez-network: conninfo ret=0",
                "connected=true",
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "PING 10.77.0.1",
                "2 packets transmitted, 2 received, 0% packet loss",
                "bluez-network: daemon-profile action=disconnect "
                "service=panu role=0x1115",
                "bluez-network: dbus method=org.bluez.Network1.Disconnect",
                "bluez-network: bnep-conndel ret=0",
                "bluez-network: connlist-postdel ret=0",
                "cnum=0",
                "bluez-network: daemon-profile disconnect complete",
                *BNEP_NATIVE_TEARDOWN_REQUIRED,
            )),
        ),
    ),
    CaseCheck(
        "bluez-network-iperf-tcp",
        (
            RoleCheck("bt1", (
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "mode=tcp-server",
                "accept: 10.77.0.2",
                "Mbits/sec",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
            )),
            RoleCheck("bt2", (
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "mode=tcp-client",
                "dip=10.77.0.1:5001",
                "Mbits/sec",
                "iperf exit",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "bluez-network: disconnect complete",
            )),
        ),
    ),
    CaseCheck(
        "bluez-network-iperf-tcp-reverse",
        (
            RoleCheck("bt2", (
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "mode=tcp-server",
                "accept: 10.77.0.1",
                "Mbits/sec",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
            )),
            RoleCheck("bt1", (
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "mode=tcp-client",
                "dip=10.77.0.2:5001",
                "Mbits/sec",
                "iperf exit",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "bluez-network: disconnect complete",
            )),
        ),
    ),
    CaseCheck(
        "bluez-network-iperf-udp",
        (
            RoleCheck("bt1", (
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "mode=udp-server",
                "accept: 10.77.0.2",
                "Mbits/sec",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
            )),
            RoleCheck("bt2", (
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "mode=udp-client",
                "dip=10.77.0.1:5001",
                "Mbits/sec",
                "iperf exit",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "bluez-network: disconnect complete",
            )),
        ),
    ),
    CaseCheck(
        "bluez-network-iperf-udp-reverse",
        (
            RoleCheck("bt2", (
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "mode=udp-server",
                "accept: 10.77.0.1",
                "Mbits/sec",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
            )),
            RoleCheck("bt1", (
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "mode=udp-client",
                "dip=10.77.0.2:5001",
                "Mbits/sec",
                "iperf exit",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "bluez-network: disconnect complete",
            )),
        ),
    ),
    CaseCheck(
        "bluez-network-iperf-tcp-soak",
        (
            RoleCheck("bt1", (
                "bluez-network: source=third/bluez/profiles/network/"
                "connection.c+profiles/network/bnep.c",
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "mode=tcp-server",
                "accept: 10.77.0.2",
                "Mbits/sec",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "bluez-network: disconnect complete",
                *BNEP_NATIVE_TEARDOWN_REQUIRED,
            )),
            RoleCheck("bt2", (
                "bluez-network: source=third/bluez/profiles/network/"
                "connection.c+profiles/network/bnep.c",
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "mode=tcp-client",
                "dip=10.77.0.1:5001",
                "Mbits/sec",
                "iperf exit",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "bluez-network: disconnect complete",
                *BNEP_NATIVE_TEARDOWN_REQUIRED,
            )),
        ),
    ),
    CaseCheck(
        "bluez-network-reconnect",
        (
            RoleCheck("bt1", (
                "count>=2:bluez-network: source=third/bluez/profiles/network/"
                "connection.c+profiles/network/bnep.c",
                "count>=2:bluez-network: bnep-connadd ret=0",
                "count>=2:role=0x1115",
                "count>=2:flags=0x00000001",
                "count>=2:bluez-network: connect complete",
                "count>=2:bluez-network: disconnect complete",
                "PING 10.77.0.2",
                "2 packets transmitted, 2 received, 0% packet loss",
                *BNEP_NATIVE_RECONNECT_REQUIRED,
            )),
            RoleCheck("bt2", (
                "count>=2:bluez-network: source=third/bluez/profiles/network/"
                "connection.c+profiles/network/bnep.c",
                "count>=2:bluez-network: bnep-connadd ret=0",
                "count>=2:role=0x1115",
                "count>=2:flags=0x00000001",
                "count>=2:bluez-network: connect complete",
                "count>=2:bluez-network: disconnect complete",
                "PING 10.77.0.1",
                "2 packets transmitted, 2 received, 0% packet loss",
                *BNEP_NATIVE_RECONNECT_REQUIRED,
            )),
        ),
    ),
    CaseCheck(
        "bluez-network-reconnect-stress",
        (
            RoleCheck("bt1", (
                "count>=3:bluez-network: source=third/bluez/profiles/network/"
                "connection.c+profiles/network/bnep.c",
                "count>=3:bluez-network: bnep-connadd ret=0",
                "count>=3:role=0x1115",
                "count>=3:flags=0x00000001",
                "count>=3:bluez-network: connect complete",
                "count>=3:bluez-network: disconnect complete",
                "PING 10.77.0.2",
                "2 packets transmitted, 2 received, 0% packet loss",
                *BNEP_NATIVE_RECONNECT_STRESS_REQUIRED,
            )),
            RoleCheck("bt2", (
                "count>=3:bluez-network: source=third/bluez/profiles/network/"
                "connection.c+profiles/network/bnep.c",
                "count>=3:bluez-network: bnep-connadd ret=0",
                "count>=3:role=0x1115",
                "count>=3:flags=0x00000001",
                "count>=3:bluez-network: connect complete",
                "count>=3:bluez-network: disconnect complete",
                "PING 10.77.0.1",
                "2 packets transmitted, 2 received, 0% packet loss",
                *BNEP_NATIVE_RECONNECT_STRESS_REQUIRED,
            )),
        ),
    ),
    CaseCheck(
        "bneptest-ping",
        (
            RoleCheck("bt1", (
                "btbneptest: pan-up",
                "btbneptest: l2cap-socket fd=",
                "btbneptest: bnep-socket fd=",
                "btbneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                "btbneptest: pan-up complete",
                "btbneptest: bnep-connlist ret=0",
                "btbneptest: bnep-conninfo-fd ret=0",
                "bnep-ioctl-connlist=1",
                "bnep-ioctl-conninfo=1",
                "bnep-ioctl-connadd=1",
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "PING 10.77.0.2",
                "2 packets transmitted, 2 received, 0% packet loss",
                "btbneptest: bnep-conndel-fd ret=0",
                "btbneptest: bnep-connlist-postdel ret=0",
                "cnum=0",
                "btbneptest: l2cap-close ret=0",
                "btbneptest: pan-down complete",
                *BNEP_NATIVE_TEARDOWN_REQUIRED,
            )),
            RoleCheck("bt2", (
                "btbneptest: pan-up",
                "btbneptest: l2cap-socket fd=",
                "btbneptest: bnep-socket fd=",
                "btbneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                "btbneptest: pan-up complete",
                "btbneptest: bnep-connlist ret=0",
                "btbneptest: bnep-conninfo-fd ret=0",
                "bnep-ioctl-connlist=1",
                "bnep-ioctl-conninfo=1",
                "bnep-ioctl-connadd=1",
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "PING 10.77.0.1",
                "2 packets transmitted, 2 received, 0% packet loss",
                "btbneptest: bnep-conndel-fd ret=0",
                "btbneptest: bnep-connlist-postdel ret=0",
                "cnum=0",
                "btbneptest: l2cap-close ret=0",
                "btbneptest: pan-down complete",
                *BNEP_NATIVE_TEARDOWN_REQUIRED,
            )),
        ),
    ),
    CaseCheck(
        "bneptest-reconnect",
        (
            RoleCheck("bt1", (
                "btbneptest: pan-up",
                "btbneptest: bnep-connadd-fd ret=0",
                "PING 10.77.0.2",
                "2 packets transmitted, 2 received, 0% packet loss",
                "btbneptest: pan-down complete",
                "count>=2:btbneptest: bnep-connlist-postdel ret=0",
                "count>=2:cnum=0",
                *BNEP_NATIVE_RECONNECT_REQUIRED,
            )),
            RoleCheck("bt2", (
                "btbneptest: pan-up",
                "btbneptest: bnep-connadd-fd ret=0",
                "PING 10.77.0.1",
                "2 packets transmitted, 2 received, 0% packet loss",
                "btbneptest: pan-down complete",
                "count>=2:btbneptest: bnep-connlist-postdel ret=0",
                "count>=2:cnum=0",
                *BNEP_NATIVE_RECONNECT_REQUIRED,
            )),
        ),
    ),
    CaseCheck(
        "bluez-bneptest-reconnect",
        (
            RoleCheck("bt1", (
                "bluez-bneptest: source=third/bluez/tools/bneptest.c",
                "mode=pan-up",
                "bluez-bneptest: bnep-connadd-fd ret=0",
                "PING 10.77.0.2",
                "2 packets transmitted, 2 received, 0% packet loss",
                "bluez-bneptest: pan-down complete",
                "count>=2:bluez-bneptest: bnep-connlist-postdel ret=0",
                "count>=2:cnum=0",
                *BNEP_NATIVE_RECONNECT_REQUIRED,
            )),
            RoleCheck("bt2", (
                "bluez-bneptest: source=third/bluez/tools/bneptest.c",
                "mode=pan-up",
                "bluez-bneptest: bnep-connadd-fd ret=0",
                "PING 10.77.0.1",
                "2 packets transmitted, 2 received, 0% packet loss",
                "bluez-bneptest: pan-down complete",
                "count>=2:bluez-bneptest: bnep-connlist-postdel ret=0",
                "count>=2:cnum=0",
                *BNEP_NATIVE_RECONNECT_REQUIRED,
            )),
        ),
    ),
    CaseCheck(
        "bneptest-reconnect-stress",
        (
            RoleCheck("bt1", (
                "btbneptest: pan-up",
                "btbneptest: bnep-connadd-fd ret=0",
                "PING 10.77.0.2",
                "2 packets transmitted, 2 received, 0% packet loss",
                "btbneptest: pan-down complete",
                "count>=3:btbneptest: bnep-connlist-postdel ret=0",
                "count>=3:cnum=0",
                *BNEP_NATIVE_RECONNECT_STRESS_REQUIRED,
            )),
            RoleCheck("bt2", (
                "btbneptest: pan-up",
                "btbneptest: bnep-connadd-fd ret=0",
                "PING 10.77.0.1",
                "2 packets transmitted, 2 received, 0% packet loss",
                "btbneptest: pan-down complete",
                "count>=3:btbneptest: bnep-connlist-postdel ret=0",
                "count>=3:cnum=0",
                *BNEP_NATIVE_RECONNECT_STRESS_REQUIRED,
            )),
        ),
    ),
    CaseCheck(
        "bluez-bneptest-reconnect-stress",
        (
            RoleCheck("bt1", (
                "bluez-bneptest: source=third/bluez/tools/bneptest.c",
                "mode=pan-up",
                "bluez-bneptest: bnep-connadd-fd ret=0",
                "PING 10.77.0.2",
                "2 packets transmitted, 2 received, 0% packet loss",
                "bluez-bneptest: pan-down complete",
                "count>=3:bluez-bneptest: bnep-connlist-postdel ret=0",
                "count>=3:cnum=0",
                *BNEP_NATIVE_RECONNECT_STRESS_REQUIRED,
            )),
            RoleCheck("bt2", (
                "bluez-bneptest: source=third/bluez/tools/bneptest.c",
                "mode=pan-up",
                "bluez-bneptest: bnep-connadd-fd ret=0",
                "PING 10.77.0.1",
                "2 packets transmitted, 2 received, 0% packet loss",
                "bluez-bneptest: pan-down complete",
                "count>=3:bluez-bneptest: bnep-connlist-postdel ret=0",
                "count>=3:cnum=0",
                *BNEP_NATIVE_RECONNECT_STRESS_REQUIRED,
            )),
        ),
    ),
    CaseCheck(
        "bnep-iperf-tcp",
        (
            RoleCheck("bt1", (
                "btbneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                "mode=tcp-server",
                "accept: 10.77.0.2",
                "Mbits/sec",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
            )),
            RoleCheck("bt2", (
                "btbneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                "mode=tcp-client",
                "dip=10.77.0.1:5001",
                "Mbits/sec",
                "iperf exit",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "btbneptest: pan-down complete",
            )),
        ),
    ),
    CaseCheck(
        "bnep-iperf-tcp-reverse",
        (
            RoleCheck("bt2", (
                "btbneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                "mode=tcp-server",
                "accept: 10.77.0.1",
                "Mbits/sec",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
            )),
            RoleCheck("bt1", (
                "btbneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                "mode=tcp-client",
                "dip=10.77.0.2:5001",
                "Mbits/sec",
                "iperf exit",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "btbneptest: pan-down complete",
            )),
        ),
    ),
    CaseCheck(
        "bnep-iperf-udp",
        (
            RoleCheck("bt1", (
                "btbneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                "mode=udp-server",
                "accept: 10.77.0.2",
                "Mbits/sec",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
            )),
            RoleCheck("bt2", (
                "btbneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                "mode=udp-client",
                "dip=10.77.0.1:5001",
                "Mbits/sec",
                "iperf exit",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "btbneptest: pan-down complete",
            )),
        ),
    ),
    CaseCheck(
        "bnep-iperf-udp-reverse",
        (
            RoleCheck("bt2", (
                "btbneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                "mode=udp-server",
                "accept: 10.77.0.1",
                "Mbits/sec",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
            )),
            RoleCheck("bt1", (
                "btbneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                "mode=udp-client",
                "dip=10.77.0.2:5001",
                "Mbits/sec",
                "iperf exit",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "btbneptest: pan-down complete",
            )),
        ),
    ),
    CaseCheck(
        "bluez-bneptest-iperf-tcp",
        (
            RoleCheck("bt1", (
                "bluez-bneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                "mode=tcp-server",
                "accept: 10.77.0.2",
                "Mbits/sec",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
            )),
            RoleCheck("bt2", (
                "bluez-bneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                "mode=tcp-client",
                "dip=10.77.0.1:5001",
                "Mbits/sec",
                "iperf exit",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "bluez-bneptest: pan-down complete",
            )),
        ),
    ),
    CaseCheck(
        "bluez-bneptest-iperf-tcp-reverse",
        (
            RoleCheck("bt2", (
                "bluez-bneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                "mode=tcp-server",
                "accept: 10.77.0.1",
                "Mbits/sec",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
            )),
            RoleCheck("bt1", (
                "bluez-bneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                "mode=tcp-client",
                "dip=10.77.0.2:5001",
                "Mbits/sec",
                "iperf exit",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "bluez-bneptest: pan-down complete",
            )),
        ),
    ),
    CaseCheck(
        "bluez-bneptest-iperf-udp",
        (
            RoleCheck("bt1", (
                "bluez-bneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                "mode=udp-server",
                "accept: 10.77.0.2",
                "Mbits/sec",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
            )),
            RoleCheck("bt2", (
                "bluez-bneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                "mode=udp-client",
                "dip=10.77.0.1:5001",
                "Mbits/sec",
                "iperf exit",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "bluez-bneptest: pan-down complete",
            )),
        ),
    ),
    CaseCheck(
        "bluez-bneptest-iperf-udp-reverse",
        (
            RoleCheck("bt2", (
                "bluez-bneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                "mode=udp-server",
                "accept: 10.77.0.1",
                "Mbits/sec",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
            )),
            RoleCheck("bt1", (
                "bluez-bneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                "mode=udp-client",
                "dip=10.77.0.2:5001",
                "Mbits/sec",
                "iperf exit",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "bluez-bneptest: pan-down complete",
            )),
        ),
    ),
    CaseCheck(
        "le-audio",
        (
            RoleCheck("ble1", (
                "upstream-iso-connect: addr-type=1 connect-ret=0 "
                "state=1 sim-fastpath=1",
                "re:upstream-iso-send: addr-type=0 handle=0x0101 "
                r"payload-len=28[^\n]*send-ret=28[^\n]*"
                r"fallback-ret=-95[^\n]*sim-fastpath=0[^\n]*"
                r"upstream-iso-attach=1",
                "btaudio: upstream le broadcast source queued iso socket "
                "sample big=0 bis=1 handle=0x0101",
                "iso-socket-bind=1",
                "iso-socket-connect=1",
                "iso-socket-send=1",
                "upstream-iso-close: released",
                "conn-hash acl=0 sco=0 le=0 le-peripheral=0 cis=0 bis=0",
            )),
            RoleCheck("ble2", (
                "upstream-iso-connect: addr-type=1 connect-ret=0 "
                "state=1 sim-fastpath=1",
                "btaudio: upstream le broadcast sink synced big=0 bis=1 "
                "handle=0x0101",
                "btaudio: upstream le broadcast sink polled=1",
                "upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 4c 43 33 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "iso-socket-bind=1",
                "iso-socket-connect=1",
                "re:iso-socket-recv=[1-9]",
                "upstream-iso-close: released",
                "btaudio: upstream le broadcast sink stopped",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-avrcp-browsing",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=source peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "control register psm=0x0017 browsing-psm=0x001b",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-browsing role=controller "
                "command=controller-browse peer=2 handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-browsing role=controller "
                "pdu=get-folder-items pdu-id=0x71",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus method=ListItems interface=org.bluez.MediaControl1",
                "bluez-audio: avrcp browsing handle open role=controller "
                "psm=0x001b cid=0x0043 handle=0x0052 ret=0",
                "bluez-audio: avrcp browsing handle connect "
                "role=controller psm=0x001b cid=0x0043 "
                "handle=0x0052 ret=0",
                "upstream-l2cap-write-handle: psm=0x001b cid=0x0043 "
                "handle=0x0052 payload-len=9 send-ret=9",
                "bluez-audio: avrcp controller sent browsing "
                "pdu=get-folder-items len=9",
                "bluez-audio: avrcp browsing handle open "
                "role=controller-rx psm=0x001b cid=0x0043",
                "upstream-l2cap-recv-handle: psm=0x001b cid=0x0043 "
                "handle=0x0052 recv-ret=9",
                "payload=02 11 0e 09 71 00 00 00 00",
                "bluez-audio: avrcp controller browsing response "
                "status=success pdu=get-folder-items",
                "bluez-audio: avrcp browsing handle close psm=0x001b "
                "cid=0x0043 handle=0x0052 ret=0",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=source",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=sink peer=1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "control register psm=0x0017 browsing-psm=0x001b",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-browsing role=target "
                "command=target-browse-respond peer=1 handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-browsing role=target "
                "pdu=get-folder-items pdu-id=0x71",
                "bluez-audio: avrcp browsing handle open role=target "
                "psm=0x001b cid=0x0043 handle=0x0052 ret=0",
                "bluez-audio: avrcp browsing handle connect role=target "
                "psm=0x001b cid=0x0043 handle=0x0052 ret=0",
                "upstream-l2cap-recv-handle: psm=0x001b cid=0x0043 "
                "handle=0x0052 recv-ret=9",
                "payload=00 11 0e 00 71 00 00 00 01",
                "bluez-audio: avrcp target received browsing "
                "pdu=get-folder-items",
                "bluez-audio: avrcp browsing handle close psm=0x001b "
                "cid=0x0043 handle=0x0052 ret=0",
                "upstream-l2cap-send: psm=0x001b cid=0x0043 "
                "handle=0x0052 payload-len=9",
                "send-ret=9",
                "bluez-audio: avrcp target browsing response "
                "status=success len=9",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=sink",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-avrcp-control",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=source peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=controller "
                "command=controller-play peer=2 handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-control role=controller "
                "operation=play opcode=0x7c opid=0x44",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus method=Play interface=org.bluez.MediaControl1",
                "bluez-audio: avrcp control handle open role=controller "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "bluez-audio: avrcp control handle connect "
                "role=controller psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "upstream-l2cap-write-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=8 send-ret=8",
                "bluez-audio: avrcp controller sent pass-through "
                "operation=play len=8",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=8",
                "payload=02 11 0e 09 48 7c 44 00",
                "bluez-audio: avrcp controller response accepted "
                "operation=play status=accepted",
                "bluez-audio: avrcp control handle close psm=0x0017 "
                "cid=0x0042 handle=0x0052 ret=0",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=source",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=sink peer=1",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=target "
                "command=target-respond peer=1 handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-control role=target "
                "operation=play opcode=0x7c opid=0x44",
                "bluez-audio: avrcp control handle open role=target "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "bluez-audio: avrcp control handle connect role=target "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=8",
                "payload=00 11 0e 00 48 7c 44 00",
                "bluez-audio: avrcp target received pass-through "
                "operation=play",
                "upstream-l2cap-send: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=8",
                "send-ret=8",
                "bluez-audio: avrcp target response accepted "
                "operation=play len=8",
                "bluez-audio: avrcp control handle close psm=0x0017 "
                "cid=0x0042 handle=0x0052 ret=0",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=sink",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-avrcp-notification",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=source peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=controller "
                "command=controller-notify peer=2 handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-notification role=controller "
                "pdu=register-notification pdu-id=0x31 event=playback-status",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus method=RegisterNotification "
                "interface=org.bluez.MediaControl1",
                "bluez-audio: avrcp notify handle open role=controller "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "bluez-audio: avrcp notify handle connect "
                "role=controller psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "upstream-l2cap-write-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=12 send-ret=12",
                "bluez-audio: avrcp controller sent notification "
                "event=playback-status len=12",
                "bluez-audio: avrcp notify handle open "
                "role=controller-rx psm=0x0017 cid=0x0042",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=10",
                "payload=02 11 0e 0f 31 00 00 02 01 01",
                "bluez-audio: avrcp controller notification "
                "interim event=playback-status status=playing",
                "bluez-audio: avrcp notify handle close psm=0x0017 "
                "cid=0x0042 handle=0x0052 ret=0",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=source",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=sink peer=1",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=target "
                "command=target-notify-respond peer=1 handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-notification role=target "
                "pdu=register-notification pdu-id=0x31 event=playback-status",
                "bluez-audio: avrcp notify handle open role=target "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "bluez-audio: avrcp notify handle connect role=target "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=12",
                "payload=00 11 0e 00 31 00 00 05 01 00 00 00",
                "bluez-audio: avrcp target received notification "
                "event=playback-status",
                "bluez-audio: avrcp notify handle close psm=0x0017 "
                "cid=0x0042 handle=0x0052 ret=0",
                "upstream-l2cap-send: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=10",
                "send-ret=10",
                "bluez-audio: avrcp target notification interim "
                "event=playback-status status=playing len=10",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=sink",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-avrcp-absolute-volume",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=source peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=controller "
                "command=controller-volume peer=2 handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-absolute-volume role=controller "
                "pdu=set-absolute-volume pdu-id=0x50 volume=64",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "dbus property=Volume interface=org.bluez.MediaTransport1",
                "bluez-audio: avrcp volume handle open role=controller "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "bluez-audio: avrcp volume handle connect "
                "role=controller psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "upstream-l2cap-write-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=9 send-ret=9",
                "bluez-audio: avrcp controller sent absolute-volume "
                "volume=64 len=9",
                "bluez-audio: avrcp volume handle open "
                "role=controller-rx psm=0x0017 cid=0x0042",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=9",
                "payload=02 11 0e 09 50 00 00 01 40",
                "bluez-audio: avrcp controller absolute-volume "
                "response volume=64 status=accepted",
                "bluez-audio: avrcp volume handle close psm=0x0017 "
                "cid=0x0042 handle=0x0052 ret=0",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=source",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=sink peer=1",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=target "
                "command=target-volume-respond peer=1 handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-absolute-volume role=target "
                "pdu=set-absolute-volume pdu-id=0x50 volume=64",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "dbus property=Volume interface=org.bluez.MediaTransport1",
                "bluez-audio: avrcp volume handle open role=target "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "bluez-audio: avrcp volume handle connect role=target "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=9",
                "payload=00 11 0e 00 50 00 00 01 40",
                "bluez-audio: avrcp target received absolute-volume "
                "volume=64",
                "bluez-audio: avrcp volume handle close psm=0x0017 "
                "cid=0x0042 handle=0x0052 ret=0",
                "upstream-l2cap-send: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=9",
                "send-ret=9",
                "bluez-audio: avrcp target absolute-volume response "
                "volume=64 len=9",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=sink",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-avrcp-metadata",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=source peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=controller "
                "command=controller-metadata peer=2 handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-metadata role=controller "
                "pdu=get-element-attributes pdu-id=0x20",
                "bluez-audio: source=third/bluez/profiles/audio/player.c "
                "dbus property=Track interface=org.bluez.MediaPlayer1",
                "bluez-audio: avrcp metadata handle open role=controller "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "bluez-audio: avrcp metadata handle connect "
                "role=controller psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "upstream-l2cap-write-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=17 send-ret=17",
                "bluez-audio: avrcp controller sent metadata "
                "pdu=get-element-attributes len=17",
                "bluez-audio: avrcp metadata handle open "
                "role=controller-rx psm=0x0017 cid=0x0042",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=24",
                "payload=02 11 0e 09 20 00 00 10 01 00 00 00 01 "
                "00 6a 00 07 46 65 61 74 68 65 72",
                "bluez-audio: avrcp controller metadata response "
                "attribute=title value=Feather status=accepted",
                "bluez-audio: avrcp metadata handle close psm=0x0017 "
                "cid=0x0042 handle=0x0052 ret=0",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=source",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=sink peer=1",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=target "
                "command=target-metadata-respond peer=1 handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-metadata role=target "
                "pdu=get-element-attributes pdu-id=0x20",
                "bluez-audio: source=third/bluez/profiles/audio/player.c "
                "dbus property=Track interface=org.bluez.MediaPlayer1",
                "bluez-audio: avrcp metadata handle open role=target "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "bluez-audio: avrcp metadata handle connect role=target "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=17",
                "payload=00 11 0e 00 20 00 00 09 ff ff ff ff "
                "ff ff ff ff 00",
                "bluez-audio: avrcp target received metadata "
                "pdu=get-element-attributes",
                "bluez-audio: avrcp metadata handle close psm=0x0017 "
                "cid=0x0042 handle=0x0052 ret=0",
                "upstream-l2cap-send: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=24",
                "send-ret=24",
                "bluez-audio: avrcp target metadata response "
                "attribute=title value=Feather len=24",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=sink",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-avrcp-player-settings-list",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=source peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=controller "
                "command=controller-player-settings-list peer=2 "
                "handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-player-settings-list role=controller "
                "pdu=list-player-application-setting-attributes",
                "bluez-audio: source=third/bluez/profiles/audio/player.c "
                "dbus property=SupportedSettings "
                "interface=org.bluez.MediaPlayer1",
                "bluez-audio: avrcp player-settings-list handle open "
                "role=controller psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "bluez-audio: avrcp player-settings-list handle connect "
                "role=controller psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "upstream-l2cap-write-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=8 send-ret=8",
                "bluez-audio: avrcp controller sent "
                "player-settings-list len=8",
                "bluez-audio: avrcp player-settings-list handle open "
                "role=controller-rx psm=0x0017 cid=0x0042",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=11",
                "payload=02 11 0e 09 11 00 00 03 02 02 03",
                "bluez-audio: avrcp controller player-settings-list "
                "response attributes=repeat,shuffle status=accepted",
                "bluez-audio: avrcp player-settings-list handle close "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=source",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=sink peer=1",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=target "
                "command=target-player-settings-list-respond peer=1 "
                "handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-player-settings-list role=target "
                "pdu=list-player-application-setting-attributes",
                "bluez-audio: source=third/bluez/profiles/audio/player.c "
                "dbus property=SupportedSettings "
                "interface=org.bluez.MediaPlayer1",
                "bluez-audio: avrcp player-settings-list handle open "
                "role=target psm=0x0017 cid=0x0042 handle=0x0052 "
                "ret=0",
                "bluez-audio: avrcp player-settings-list handle connect "
                "role=target psm=0x0017 cid=0x0042 handle=0x0052 "
                "ret=0",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=8",
                "payload=00 11 0e 00 11 00 00 00",
                "bluez-audio: avrcp target received "
                "player-settings-list request",
                "bluez-audio: avrcp player-settings-list handle close "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "upstream-l2cap-send: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=11",
                "send-ret=11",
                "bluez-audio: avrcp target player-settings-list "
                "response attributes=repeat,shuffle len=11",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=sink",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-avrcp-player-settings-values",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=source peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=controller "
                "command=controller-player-settings-values peer=2 "
                "handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-player-settings-values role=controller "
                "pdu=list-player-application-setting-values",
                "bluez-audio: source=third/bluez/profiles/audio/player.c "
                "dbus property=SupportedSettingValues "
                "interface=org.bluez.MediaPlayer1",
                "bluez-audio: avrcp player-settings-values handle open "
                "role=controller psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "bluez-audio: avrcp player-settings-values handle connect "
                "role=controller psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "upstream-l2cap-write-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=9 send-ret=9",
                "bluez-audio: avrcp controller sent "
                "player-settings-values attribute=repeat len=9",
                "bluez-audio: avrcp player-settings-values handle open "
                "role=controller-rx psm=0x0017 cid=0x0042",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=13",
                "payload=02 11 0e 09 12 00 00 05 04 01 02 03 04",
                "bluez-audio: avrcp controller player-settings-values "
                "response attribute=repeat "
                "values=off,single-track,all-tracks,group "
                "status=accepted",
                "bluez-audio: avrcp player-settings-values handle close "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=source",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=sink peer=1",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=target "
                "command=target-player-settings-values-respond peer=1 "
                "handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-player-settings-values role=target "
                "pdu=list-player-application-setting-values",
                "bluez-audio: source=third/bluez/profiles/audio/player.c "
                "dbus property=SupportedSettingValues "
                "interface=org.bluez.MediaPlayer1",
                "bluez-audio: avrcp player-settings-values handle open "
                "role=target psm=0x0017 cid=0x0042 handle=0x0052 "
                "ret=0",
                "bluez-audio: avrcp player-settings-values handle connect "
                "role=target psm=0x0017 cid=0x0042 handle=0x0052 "
                "ret=0",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=9",
                "payload=00 11 0e 00 12 00 00 01 02",
                "bluez-audio: avrcp target received "
                "player-settings-values attribute=repeat",
                "bluez-audio: avrcp player-settings-values handle close "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "upstream-l2cap-send: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=13",
                "send-ret=13",
                "bluez-audio: avrcp target player-settings-values "
                "response attribute=repeat "
                "values=off,single-track,all-tracks,group len=13",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=sink",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-avrcp-player-settings-value-text",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=source peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=controller "
                "command=controller-player-settings-value-text peer=2 "
                "handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-player-settings-value-text "
                "role=controller",
                "bluez-audio: source=third/bluez/profiles/audio/player.c "
                "dbus property=SettingValueText "
                "interface=org.bluez.MediaPlayer1",
                "bluez-audio: avrcp player-settings-value-text handle open "
                "role=controller psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "bluez-audio: avrcp player-settings-value-text "
                "handle connect role=controller psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "upstream-l2cap-write-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=12 send-ret=12",
                "bluez-audio: avrcp controller sent "
                "player-settings-value-text attribute=repeat len=12",
                "bluez-audio: avrcp player-settings-value-text "
                "handle open role=controller-rx psm=0x0017 cid=0x0042",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=34",
                "payload=02 11 0e 09 16 00 00 1a 02 01 "
                "00 6a 00 03 4f 66 66 02 00 6a 00 0c "
                "53 69 6e 67 6c 65 20 54 72 61 63 6b",
                "bluez-audio: avrcp controller "
                "player-settings-value-text response "
                "attribute=repeat values=Off,Single Track "
                "status=accepted",
                "bluez-audio: avrcp player-settings-value-text "
                "handle close psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=source",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=sink peer=1",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=target "
                "command=target-player-settings-value-text-respond peer=1 "
                "handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-player-settings-value-text "
                "role=target",
                "bluez-audio: source=third/bluez/profiles/audio/player.c "
                "dbus property=SettingValueText "
                "interface=org.bluez.MediaPlayer1",
                "bluez-audio: avrcp player-settings-value-text handle open "
                "role=target psm=0x0017 cid=0x0042 handle=0x0052 "
                "ret=0",
                "bluez-audio: avrcp player-settings-value-text "
                "handle connect role=target psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=12",
                "payload=00 11 0e 00 16 00 00 04 02 02 01 02",
                "bluez-audio: avrcp target received "
                "player-settings-value-text attribute=repeat "
                "values=off,single-track",
                "bluez-audio: avrcp player-settings-value-text "
                "handle close psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "upstream-l2cap-send: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=34",
                "send-ret=34",
                "bluez-audio: avrcp target player-settings-value-text "
                "response attribute=repeat values=Off,Single Track "
                "len=34",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=sink",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-avrcp-player-settings-notification",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=source peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=controller "
                "command=controller-player-settings-notify peer=2 "
                "handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-player-settings-notification "
                "role=controller pdu=register-notification pdu-id=0x31 "
                "event=player-application-setting-changed",
                "bluez-audio: source=third/bluez/profiles/audio/player.c "
                "dbus signal=PropertiesChanged "
                "interface=org.bluez.MediaPlayer1",
                "bluez-audio: avrcp player-settings-notify handle open "
                "role=controller psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "bluez-audio: avrcp player-settings-notify "
                "handle connect role=controller psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "upstream-l2cap-write-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=12 send-ret=12",
                "bluez-audio: avrcp controller sent "
                "player-settings-notification event=changed len=12",
                "bluez-audio: avrcp player-settings-notify handle open "
                "role=controller-rx psm=0x0017 cid=0x0042",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=14",
                "payload=02 11 0e 0f 31 00 00 06 08 02 02 02 03 02",
                "bluez-audio: avrcp controller "
                "player-settings-notification interim "
                "repeat=single-track shuffle=all-tracks",
                "bluez-audio: avrcp player-settings-notify handle close "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=source",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=sink peer=1",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=target "
                "command=target-player-settings-notify-respond peer=1 "
                "handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-player-settings-notification "
                "role=target pdu=register-notification pdu-id=0x31 "
                "event=player-application-setting-changed",
                "bluez-audio: source=third/bluez/profiles/audio/player.c "
                "dbus signal=PropertiesChanged "
                "interface=org.bluez.MediaPlayer1",
                "bluez-audio: avrcp player-settings-notify handle open "
                "role=target psm=0x0017 cid=0x0042 handle=0x0052 "
                "ret=0",
                "bluez-audio: avrcp player-settings-notify "
                "handle connect role=target psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=12",
                "payload=00 11 0e 00 31 00 00 05 08 00 00 00",
                "bluez-audio: avrcp target received "
                "player-settings-notification event=changed",
                "bluez-audio: avrcp player-settings-notify handle close "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "upstream-l2cap-send: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=14",
                "send-ret=14",
                "bluez-audio: avrcp target player-settings-notification "
                "interim repeat=single-track shuffle=all-tracks len=14",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=sink",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-avrcp-player-settings-error",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=source peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=controller "
                "command=controller-player-settings-error peer=2 "
                "handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-player-settings-error role=controller "
                "pdu=get-current-player-application-setting-value",
                "bluez-audio: source=third/bluez/profiles/audio/player.c "
                "dbus error=InvalidArguments "
                "interface=org.bluez.MediaPlayer1",
                "bluez-audio: avrcp player-settings-error handle open "
                "role=controller psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "bluez-audio: avrcp player-settings-error handle connect "
                "role=controller psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "upstream-l2cap-write-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=10 send-ret=10",
                "bluez-audio: avrcp controller sent "
                "player-settings-error attribute=0x7f len=10",
                "bluez-audio: avrcp player-settings-error handle open "
                "role=controller-rx psm=0x0017 cid=0x0042",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=9",
                "payload=02 11 0e 0a 13 00 00 01 01",
                "bluez-audio: avrcp controller "
                "player-settings-error response status=rejected "
                "error=invalid-parameter",
                "bluez-audio: avrcp player-settings-error handle close "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=source",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=sink peer=1",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=target "
                "command=target-player-settings-error-respond peer=1 "
                "handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-player-settings-error role=target "
                "pdu=get-current-player-application-setting-value",
                "bluez-audio: source=third/bluez/profiles/audio/player.c "
                "dbus error=InvalidArguments "
                "interface=org.bluez.MediaPlayer1",
                "bluez-audio: avrcp player-settings-error handle open "
                "role=target psm=0x0017 cid=0x0042 handle=0x0052 "
                "ret=0",
                "bluez-audio: avrcp player-settings-error handle connect "
                "role=target psm=0x0017 cid=0x0042 handle=0x0052 "
                "ret=0",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=10",
                "payload=00 11 0e 00 13 00 00 02 01 7f",
                "bluez-audio: avrcp target received "
                "player-settings-error attribute=0x7f",
                "bluez-audio: avrcp player-settings-error handle close "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "upstream-l2cap-send: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=9",
                "send-ret=9",
                "bluez-audio: avrcp target player-settings-error "
                "response status=rejected error=invalid-parameter len=9",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=sink",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-avrcp-addressed-player",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=source peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=controller "
                "command=controller-addressed-player peer=2 "
                "handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-addressed-player role=controller "
                "pdu=set-addressed-player pdu-id=0x60 player-id=0x0001",
                "bluez-audio: source=third/bluez/profiles/audio/player.c "
                "dbus property=AddressedPlayer "
                "interface=org.bluez.MediaPlayer1",
                "bluez-audio: avrcp addressed-player handle open "
                "role=controller psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "bluez-audio: avrcp addressed-player handle connect "
                "role=controller psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "upstream-l2cap-write-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=10 send-ret=10",
                "bluez-audio: avrcp controller sent addressed-player "
                "player-id=0x0001 len=10",
                "bluez-audio: avrcp addressed-player handle open "
                "role=controller-rx psm=0x0017 cid=0x0042",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=9",
                "payload=02 11 0e 09 60 00 00 01 04",
                "bluez-audio: avrcp controller addressed-player "
                "response player-id=0x0001 status=success",
                "bluez-audio: avrcp addressed-player handle close "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=source",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=sink peer=1",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=target "
                "command=target-addressed-player-respond peer=1 "
                "handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-addressed-player role=target "
                "pdu=set-addressed-player pdu-id=0x60 player-id=0x0001",
                "bluez-audio: source=third/bluez/profiles/audio/player.c "
                "dbus property=AddressedPlayer "
                "interface=org.bluez.MediaPlayer1",
                "bluez-audio: avrcp addressed-player handle open "
                "role=target psm=0x0017 cid=0x0042 handle=0x0052 "
                "ret=0",
                "bluez-audio: avrcp addressed-player handle connect "
                "role=target psm=0x0017 cid=0x0042 handle=0x0052 "
                "ret=0",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=10",
                "payload=00 11 0e 00 60 00 00 02 00 01",
                "bluez-audio: avrcp target received addressed-player "
                "player-id=0x0001",
                "bluez-audio: avrcp addressed-player handle close "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "upstream-l2cap-send: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=9",
                "send-ret=9",
                "bluez-audio: avrcp target addressed-player response "
                "player-id=0x0001 status=success len=9",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=sink",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-avrcp-player-settings",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=source peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=controller "
                "command=controller-player-settings peer=2 handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-player-settings role=controller "
                "pdu=get-current-player-application-setting-value",
                "bluez-audio: source=third/bluez/profiles/audio/player.c "
                "dbus property=Settings interface=org.bluez.MediaPlayer1",
                "bluez-audio: avrcp player-settings handle open "
                "role=controller psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "bluez-audio: avrcp player-settings handle connect "
                "role=controller psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "upstream-l2cap-write-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=11 send-ret=11",
                "bluez-audio: avrcp controller sent player-settings "
                "attributes=repeat,shuffle len=11",
                "bluez-audio: avrcp player-settings handle open "
                "role=controller-rx psm=0x0017 cid=0x0042",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=13",
                "payload=02 11 0e 09 13 00 00 05 02 02 01 03 01",
                "bluez-audio: avrcp controller player-settings "
                "response repeat=off shuffle=off status=accepted",
                "bluez-audio: avrcp player-settings handle close "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=source",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=sink peer=1",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=target "
                "command=target-player-settings-respond peer=1 handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-player-settings role=target "
                "pdu=get-current-player-application-setting-value",
                "bluez-audio: source=third/bluez/profiles/audio/player.c "
                "dbus property=Settings interface=org.bluez.MediaPlayer1",
                "bluez-audio: avrcp player-settings handle open role=target "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "bluez-audio: avrcp player-settings handle connect "
                "role=target psm=0x0017 cid=0x0042 handle=0x0052 "
                "ret=0",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=11",
                "payload=00 11 0e 00 13 00 00 03 02 02 03",
                "bluez-audio: avrcp target received player-settings "
                "attributes=repeat,shuffle",
                "bluez-audio: avrcp player-settings handle close "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "upstream-l2cap-send: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=13",
                "send-ret=13",
                "bluez-audio: avrcp target player-settings response "
                "repeat=off shuffle=off len=13",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=sink",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-avrcp-player-settings-set",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=source peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=controller "
                "command=controller-player-settings-set peer=2 "
                "handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-player-settings-set role=controller "
                "pdu=set-player-application-setting-value",
                "bluez-audio: source=third/bluez/profiles/audio/player.c "
                "dbus property=Settings interface=org.bluez.MediaPlayer1",
                "bluez-audio: avrcp player-settings-set handle open "
                "role=controller psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "bluez-audio: avrcp player-settings-set handle connect "
                "role=controller psm=0x0017 cid=0x0042 "
                "handle=0x0052 ret=0",
                "upstream-l2cap-write-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=13 send-ret=13",
                "bluez-audio: avrcp controller sent player-settings-set "
                "repeat=single-track shuffle=all-tracks len=13",
                "bluez-audio: avrcp player-settings-set handle open "
                "role=controller-rx psm=0x0017 cid=0x0042",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=8",
                "payload=02 11 0e 09 14 00 00 00",
                "bluez-audio: avrcp controller player-settings-set "
                "response status=accepted",
                "bluez-audio: avrcp player-settings-set handle close "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=source",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=sink peer=1",
                "bluez-audio: source=third/bluez/profiles/audio/control.c "
                "dbus export interface=org.bluez.MediaControl1",
                "bluez-audio: source=third/bluez/profiles/audio/avctp.c "
                "style profile=avctp-control role=target "
                "command=target-player-settings-set-respond peer=1 "
                "handle=0x0052",
                "bluez-audio: source=third/bluez/profiles/audio/avrcp.c "
                "style profile=avrcp-player-settings-set role=target "
                "pdu=set-player-application-setting-value",
                "bluez-audio: source=third/bluez/profiles/audio/player.c "
                "dbus property=Settings interface=org.bluez.MediaPlayer1",
                "bluez-audio: avrcp player-settings-set handle open "
                "role=target psm=0x0017 cid=0x0042 handle=0x0052 "
                "ret=0",
                "bluez-audio: avrcp player-settings-set handle connect "
                "role=target psm=0x0017 cid=0x0042 handle=0x0052 "
                "ret=0",
                "upstream-l2cap-recv-handle: psm=0x0017 cid=0x0042 "
                "handle=0x0052 recv-ret=13",
                "payload=00 11 0e 00 14 00 00 05 02 02 02 03 02",
                "bluez-audio: avrcp target received player-settings-set "
                "repeat=single-track shuffle=all-tracks",
                "bluez-audio: avrcp player-settings-set handle close "
                "psm=0x0017 cid=0x0042 handle=0x0052 ret=0",
                "upstream-l2cap-send: psm=0x0017 cid=0x0042 "
                "handle=0x0052 payload-len=8",
                "send-ret=8",
                "bluez-audio: avrcp target player-settings-set "
                "response status=accepted len=8",
                "bluez-audio: avrcp control complete",
                "bluez-audio: media endpoint clear complete role=sink",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-transport-bidir",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=a2dp-source-acquire-write-release",
                "upstream-l2cap-write: payload-len=24 send-ret=24",
                "bluez-audio: media transport write len=24 "
                "payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: media transport release complete role=source",
                "bluez-audio: a2dp auto-rsp signal=0x07 msg-type=0x00 "
                "pkt-type=0x00 rsp-len=2 err=0x00 state=OPEN->STREAMING",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=a2dp-sink-acquire-read-release",
                "upstream-l2cap-recv-handle: psm=0x0019 cid=0x0041 handle=0x0052 recv-ret=24",
                "bluez-audio: media transport read complete "
                "payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: media transport release complete role=sink",
                "re:l2cap-socket-bind=([2-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-listen=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
                "re:l2cap-socket-native-recv=[1-9]",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=a2dp-sink-acquire-read-release",
                "upstream-l2cap-recv-handle: psm=0x0019 cid=0x0041 handle=0x0052 recv-ret=24",
                "bluez-audio: media transport read complete "
                "payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: media transport release complete role=sink",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=a2dp-source-acquire-write-release",
                "upstream-l2cap-write: payload-len=24 send-ret=24",
                "bluez-audio: media transport write len=24 "
                "payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: media transport release complete role=source",
                "re:l2cap-socket-bind=([2-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-listen=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
                "re:l2cap-socket-native-recv=[1-9]",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-a2dp-full",
        (
            RoleCheck("bt1", (
                "bluez-daemon: source=third/bluez/src/main.c+"
                "profiles/audio/media.c style=bluetoothd-mainloop "
                "mode=audio-a2dp-owner role=source peer=2",
                "bluez-daemon: dbus name-owner org.bluez acquired "
                "bus=system owner=bluezdaemon",
                "bluez-daemon: dbus object-manager GetManagedObjects "
                "root=/org/bluez",
                "bluez-daemon: dbus InterfacesAdded "
                "path=/org/bluez/hci0/dev_02/player0 "
                "interface=org.bluez.MediaPlayer1 owner=bluetoothd",
                "bluez-daemon: dbus InterfacesAdded "
                "path=/org/bluez/hci0/dev_02/sep1 "
                "interface=org.bluez.MediaEndpoint1 codec=sbc role=source",
                "bluez-daemon: dbus InterfacesAdded "
                "path=/org/bluez/hci0/dev_02/fd0 "
                "interface=org.bluez.MediaTransport1 state=idle",
                "bluez-daemon: plugin audio/avrcp mainloop registered "
                "avdtp=owned avctp=owned media=owned",
                "bluez-daemon: controller-driven l2cap policy "
                "setup=connect-on-demand error=fail-fast "
                "teardown=owner-close",
                "bluez-daemon: codec policy a2dp=sbc "
                "channel-mode=joint-stereo frequency=44100 "
                "allocation=loudness avrcp=ct+tg+browsing",
                "bluez-daemon: audio owner source step=avdtp-discover "
                "complete",
                "bluez-daemon: audio owner source step=avdtp-setconfig "
                "complete",
                "bluez-daemon: audio owner source step=avdtp-open complete",
                "bluez-daemon: audio owner source step=avdtp-start complete",
                "bluez-daemon: audio owner source media complete "
                "payload=A2DP:SBC:daemon-frame",
                "bluez-daemon: audio owner source step=avrcp-play complete",
                "bluez-daemon: audio owner source step=avrcp-browse "
                "complete",
                "bluez-daemon: audio owner source step=avrcp-notify "
                "complete",
                "bluez-daemon: audio owner source step=avrcp-volume "
                "complete",
                "bluez-daemon: audio owner source step=avrcp-metadata "
                "complete",
                "bluez-daemon: audio owner source step=avrcp-player-settings "
                "complete",
                "bluez-daemon: audio owner source step="
                "avrcp-player-settings-set complete",
                "bluez-daemon: audio owner source step=avdtp-suspend "
                "complete",
                "bluez-daemon: audio owner source step=avdtp-close complete",
                "bluez-daemon: dbus InterfacesRemoved "
                "path=/org/bluez/hci0/dev_02/fd0 "
                "interface=org.bluez.MediaTransport1",
                "bluez-daemon: audio-a2dp-owner complete role=source "
                "peer=2",
                "re:l2cap-socket-bind=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-send=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-recv=([6-9]|[1-9][0-9]+)",
            )),
            RoleCheck("bt2", (
                "bluez-daemon: source=third/bluez/src/main.c+"
                "profiles/audio/media.c style=bluetoothd-mainloop "
                "mode=audio-a2dp-owner role=sink peer=1",
                "bluez-daemon: dbus name-owner org.bluez acquired "
                "bus=system owner=bluezdaemon",
                "bluez-daemon: dbus object-manager GetManagedObjects "
                "root=/org/bluez",
                "bluez-daemon: dbus InterfacesAdded "
                "path=/org/bluez/hci0/dev_01/player0 "
                "interface=org.bluez.MediaPlayer1 owner=bluetoothd",
                "bluez-daemon: dbus InterfacesAdded "
                "path=/org/bluez/hci0/dev_01/sep1 "
                "interface=org.bluez.MediaEndpoint1 codec=sbc role=sink",
                "bluez-daemon: dbus InterfacesAdded "
                "path=/org/bluez/hci0/dev_01/fd0 "
                "interface=org.bluez.MediaTransport1 state=idle",
                "bluez-daemon: plugin audio/avrcp mainloop registered "
                "avdtp=owned avctp=owned media=owned",
                "bluez-daemon: controller-driven l2cap policy "
                "setup=accept-owned error=respond-or-close "
                "teardown=owner-close",
                "bluez-daemon: codec policy a2dp=sbc "
                "channel-mode=joint-stereo frequency=44100 "
                "allocation=loudness avrcp=tg+ct+browsing",
                "bluez-daemon: audio owner sink step=avdtp-discover "
                "complete",
                "bluez-daemon: audio owner sink step=avdtp-setconfig "
                "complete",
                "bluez-daemon: audio owner sink step=avdtp-open complete",
                "bluez-daemon: audio owner sink step=avdtp-start complete",
                "bluez-daemon: audio owner sink media complete "
                "payload=A2DP:SBC:daemon-frame",
                "bluez-daemon: audio owner sink step=avrcp-play complete",
                "bluez-daemon: audio owner sink step=avrcp-browse complete",
                "bluez-daemon: audio owner sink step=avrcp-notify complete",
                "bluez-daemon: audio owner sink step=avrcp-volume complete",
                "bluez-daemon: audio owner sink step=avrcp-metadata complete",
                "bluez-daemon: audio owner sink step=avrcp-player-settings "
                "complete",
                "bluez-daemon: audio owner sink step="
                "avrcp-player-settings-set complete",
                "bluez-daemon: audio owner sink step=avdtp-suspend complete",
                "bluez-daemon: audio owner sink step=avdtp-close complete",
                "bluez-daemon: dbus InterfacesRemoved "
                "path=/org/bluez/hci0/dev_01/fd0 "
                "interface=org.bluez.MediaTransport1",
                "bluez-daemon: audio-a2dp-owner complete role=sink peer=1",
                "re:l2cap-socket-bind=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-send=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-recv=([8-9]|[1-9][0-9]+)",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-a2dp-reconnect-full",
        (
            RoleCheck("bt1", (
                "bluez-daemon: source=third/bluez/src/main.c+"
                "profiles/audio/media.c style=bluetoothd-mainloop "
                "mode=audio-a2dp-owner role=source peer=2",
                "bluez-daemon: dbus name-owner org.bluez acquired "
                "bus=system owner=bluezdaemon",
                "bluez-daemon: dbus object-manager GetManagedObjects "
                "root=/org/bluez",
                "interface=org.bluez.MediaPlayer1 owner=bluetoothd",
                "interface=org.bluez.MediaEndpoint1 codec=sbc role=source",
                "interface=org.bluez.MediaTransport1 state=idle",
                "bluez-daemon: plugin audio/avrcp mainloop registered "
                "avdtp=owned avctp=owned media=owned",
                "bluez-daemon: audio-a2dp-reconnect role=source peer=2 "
                "rounds=2 persistent-mainloop=1",
                "bluez-daemon: audio owner source round=1 start",
                "bluez-daemon: audio owner source round=1 complete",
                "bluez-daemon: audio owner source round=2 start",
                "bluez-daemon: audio owner source round=2 complete",
                "bluez-daemon: audio owner source step=avdtp-discover "
                "complete",
                "bluez-daemon: audio owner source step=avdtp-start complete",
                "bluez-daemon: audio owner source media complete "
                "payload=A2DP:SBC:daemon-frame",
                "bluez-daemon: audio owner source step=avrcp-play complete",
                "bluez-daemon: audio owner source step=avrcp-browse "
                "complete",
                "bluez-daemon: audio owner source step=avrcp-volume "
                "complete",
                "bluez-daemon: audio owner source step=avdtp-suspend "
                "complete",
                "bluez-daemon: audio owner source step=avdtp-close complete",
                "bluez-daemon: dbus InterfacesRemoved "
                "path=/org/bluez/hci0/dev_02/fd0 "
                "interface=org.bluez.MediaTransport1",
                "bluez-daemon: audio-a2dp-reconnect complete role=source "
                "peer=2 rounds=2",
                "re:l2cap-socket-bind=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-connect=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-send=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-recv=([1-9][2-9]|[2-9][0-9]+)",
            )),
            RoleCheck("bt2", (
                "bluez-daemon: source=third/bluez/src/main.c+"
                "profiles/audio/media.c style=bluetoothd-mainloop "
                "mode=audio-a2dp-owner role=sink peer=1",
                "bluez-daemon: dbus name-owner org.bluez acquired "
                "bus=system owner=bluezdaemon",
                "bluez-daemon: dbus object-manager GetManagedObjects "
                "root=/org/bluez",
                "interface=org.bluez.MediaPlayer1 owner=bluetoothd",
                "interface=org.bluez.MediaEndpoint1 codec=sbc role=sink",
                "interface=org.bluez.MediaTransport1 state=idle",
                "bluez-daemon: plugin audio/avrcp mainloop registered "
                "avdtp=owned avctp=owned media=owned",
                "bluez-daemon: audio-a2dp-reconnect role=sink peer=1 "
                "rounds=2 persistent-mainloop=1",
                "bluez-daemon: audio owner sink round=1 start",
                "bluez-daemon: audio owner sink round=1 complete",
                "bluez-daemon: audio owner sink round=2 start",
                "bluez-daemon: audio owner sink round=2 complete",
                "bluez-daemon: audio owner sink step=avdtp-discover "
                "complete",
                "bluez-daemon: audio owner sink step=avdtp-start complete",
                "bluez-daemon: audio owner sink media complete "
                "payload=A2DP:SBC:daemon-frame",
                "bluez-daemon: audio owner sink step=avrcp-play complete",
                "bluez-daemon: audio owner sink step=avrcp-browse complete",
                "bluez-daemon: audio owner sink step=avrcp-volume complete",
                "bluez-daemon: audio owner sink step=avdtp-suspend complete",
                "bluez-daemon: audio owner sink step=avdtp-close complete",
                "bluez-daemon: dbus InterfacesRemoved "
                "path=/org/bluez/hci0/dev_01/fd0 "
                "interface=org.bluez.MediaTransport1",
                "bluez-daemon: audio-a2dp-reconnect complete role=sink "
                "peer=1 rounds=2",
                "re:l2cap-socket-bind=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-connect=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-send=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-recv=([1-9][6-9]|[2-9][0-9]+)",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-transport-bidir",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=a2dp-source-acquire-write-release",
                "upstream-l2cap-write: payload-len=24 send-ret=24",
                "bluez-audio: media transport write len=24 "
                "payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: media transport release complete role=source",
                "bluez-audio: a2dp auto-rsp signal=0x07 msg-type=0x00 "
                "pkt-type=0x00 rsp-len=2 err=0x00 state=OPEN->STREAMING",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=a2dp-sink-acquire-read-release",
                "upstream-l2cap-recv-handle: psm=0x0019 cid=0x0041 handle=0x0052 recv-ret=24",
                "bluez-audio: media transport read complete "
                "payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: media transport release complete role=sink",
                "re:l2cap-socket-bind=([2-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-listen=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
                "re:l2cap-socket-native-recv=[1-9]",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=a2dp-sink-acquire-read-release",
                "upstream-l2cap-recv-handle: psm=0x0019 cid=0x0041 handle=0x0052 recv-ret=24",
                "bluez-audio: media transport read complete "
                "payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: media transport release complete role=sink",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=a2dp-source-acquire-write-release",
                "upstream-l2cap-write: payload-len=24 send-ret=24",
                "bluez-audio: media transport write len=24 "
                "payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: media transport release complete role=source",
                "re:l2cap-socket-bind=([2-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-listen=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
                "re:l2cap-socket-native-recv=[1-9]",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-transport-bidir-teardown",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=a2dp-source-acquire-write-release",
                "upstream-l2cap-write: payload-len=24 send-ret=24",
                "bluez-audio: media transport write len=24 "
                "payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=suspend peer=2 "
                "handle=0x0052 len=3",
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=close-stream peer=2 "
                "handle=0x0052 len=3",
                "bluez-audio: a2dp auto-rsp signal=0x07 msg-type=0x00 "
                "pkt-type=0x00 rsp-len=2 err=0x00 state=OPEN->STREAMING",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=a2dp-sink-acquire-read-release",
                "upstream-l2cap-recv-handle: psm=0x0019 cid=0x0041 handle=0x0052 recv-ret=24",
                "bluez-audio: media transport read complete "
                "payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: a2dp auto-rsp signal=0x09 msg-type=0x00 "
                "pkt-type=0x00 rsp-len=2 err=0x00 state=STREAMING->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x08 msg-type=0x00 "
                "pkt-type=0x00 rsp-len=2 err=0x00 state=OPEN->IDLE",
                "re:l2cap-socket-bind=([2-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-listen=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
                "re:l2cap-socket-native-recv=[1-9]",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
            RoleCheck("bt2", (
                "bluez-audio: a2dp auto-rsp signal=0x07 msg-type=0x00 "
                "pkt-type=0x00 rsp-len=2 err=0x00 state=OPEN->STREAMING",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=a2dp-sink-acquire-read-release",
                "upstream-l2cap-recv-handle: psm=0x0019 cid=0x0041 handle=0x0052 recv-ret=24",
                "bluez-audio: media transport read complete "
                "payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: a2dp auto-rsp signal=0x09 msg-type=0x00 "
                "pkt-type=0x00 rsp-len=2 err=0x00 state=STREAMING->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x08 msg-type=0x00 "
                "pkt-type=0x00 rsp-len=2 err=0x00 state=OPEN->IDLE",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=a2dp-source-acquire-write-release",
                "upstream-l2cap-write: payload-len=24 send-ret=24",
                "bluez-audio: media transport write len=24 "
                "payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=suspend peer=1 "
                "handle=0x0052 len=3",
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=close-stream peer=1 "
                "handle=0x0052 len=3",
                "re:l2cap-socket-bind=([2-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-listen=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
                "re:l2cap-socket-native-recv=[1-9]",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-le-audio",
        (
            RoleCheck("ble1", (
                "upstream-iso-connect: addr-type=1 connect-ret=0 "
                "state=1 sim-fastpath=1",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-broadcast-source command=start big=0 "
                "bis=1 handle=0x0101",
                "re:upstream-iso-send: addr-type=0 handle=0x0101 "
                r"payload-len=28[^\n]*send-ret=28[^\n]*"
                r"fallback-ret=-95[^\n]*sim-fastpath=0[^\n]*"
                r"upstream-iso-attach=1",
                "bluez-audio: le broadcast source queued iso payload "
                "len=28",
                "iso-socket-bind=1",
                "iso-socket-connect=1",
                "iso-socket-send=1",
                "upstream-iso-close: released",
            )),
            RoleCheck("ble2", (
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-broadcast-sink command=sync big=0 "
                "bis=1 handle=0x0101",
                "upstream-iso-connect: addr-type=1 connect-ret=0 "
                "state=1 sim-fastpath=1",
                "bluez-audio: le broadcast sink synced",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-broadcast-sink command=start",
                "bluez-audio: le broadcast sink polled=1",
                "upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 4c 43 33 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "bluez-audio: le broadcast sink iso payload received",
                "iso-socket-bind=1",
                "iso-socket-connect=1",
                "re:iso-socket-recv=[1-9]",
                "upstream-iso-close: released",
                "bluez-audio: le broadcast sink stopped",
            )),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-profile",
        (
            RoleCheck("ble1", (
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-bap-control role=source "
                "command=source-announce big=0 bis=1 handle=0x0101",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "base codec=lc3 subgroup=1 bis-count=1",
                "bluez-audio: le bap source state=idle->base-announced",
                "upstream-iso-connect: addr-type=1 connect-ret=0 "
                "state=1 sim-fastpath=1",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-bap-control role=source "
                "command=source-start big=0 bis=1 handle=0x0101",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "qos interval-us=10000",
                "bluez-audio: le bap source state=base-announced->streaming",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-broadcast-source command=start big=0 "
                "bis=1 handle=0x0101",
                "re:upstream-iso-send: addr-type=0 handle=0x0101 "
                r"payload-len=28[^\n]*send-ret=28[^\n]*"
                r"fallback-ret=-95[^\n]*sim-fastpath=0[^\n]*"
                r"upstream-iso-attach=1",
                "bluez-audio: le broadcast source queued iso payload "
                "len=28",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-bap-control role=source "
                "command=source-stop big=0 bis=1 handle=0x0101",
                "bluez-audio: le bap source state=streaming->idle",
                "iso-socket-bind=1",
                "iso-socket-connect=1",
                "iso-socket-send=1",
                "upstream-iso-close: released",
            )),
            RoleCheck("ble2", (
                "bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-bap-control role=sink "
                "command=sink-discover big=0 bis=1 handle=0x0101",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "sink discovered pac=lc3 context=media",
                "bluez-audio: le bap sink state=idle->pacs-discovered",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-bap-control role=sink "
                "command=sink-config big=0 bis=1 handle=0x0101",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "ase state=idle->codec-configured->qos-configured",
                "bluez-audio: le bap sink state=pacs-discovered->configured",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-broadcast-sink command=sync big=0 "
                "bis=1 handle=0x0101",
                "upstream-iso-connect: addr-type=1 connect-ret=0 "
                "state=1 sim-fastpath=1",
                "bluez-audio: le broadcast sink synced",
                "bluez-audio: source=third/bluez/profiles/audio/bass.c "
                "style profile=le-bap-control role=sink "
                "command=sink-sync big=0 bis=1 handle=0x0101",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "pa-sync=1 base-received=1 bis-synced=1",
                "bluez-audio: le bap sink state=configured->streaming",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-broadcast-sink command=start",
                "upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 4c 43 33 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "bluez-audio: le broadcast sink iso payload received",
                "iso-socket-bind=1",
                "iso-socket-connect=1",
                "re:iso-socket-recv=[1-9]",
                "upstream-iso-close: released",
                "bluez-audio: le broadcast sink stopped",
            )),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-broadcast-restart",
        (
            RoleCheck("ble1", (
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-bap-control role=source "
                "command=source-announce big=0 bis=1 handle=0x0101",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "base codec=lc3 subgroup=1 bis-count=1",
                "count>=2:bluez-audio: le bap source state=idle->base-announced",
                "count>=2:upstream-iso-connect: addr-type=1 connect-ret=0 "
                "state=1 sim-fastpath=1",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-bap-control role=source "
                "command=source-start big=0 bis=1 handle=0x0101",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "qos interval-us=10000",
                "count>=2:bluez-audio: le bap source state=base-announced->streaming",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-broadcast-source command=start big=0 "
                "bis=1 handle=0x0101",
                "count>=2:bluez-audio: le broadcast source queued iso payload "
                "len=28",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-bap-control role=source "
                "command=source-stop big=0 bis=1 handle=0x0101",
                "count>=2:bluez-audio: le bap source state=streaming->idle",
                "count>=2:upstream-iso-close: released",
                "iso-socket-bind=2",
                "iso-socket-connect=2",
                "iso-socket-send=2",
            )),
            RoleCheck("ble2", (
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-bap-control role=sink "
                "command=sink-discover big=0 bis=1 handle=0x0101",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "sink discovered pac=lc3 context=media",
                "count>=2:bluez-audio: le bap sink state=idle->pacs-discovered",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-bap-control role=sink "
                "command=sink-config big=0 bis=1 handle=0x0101",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "ase state=idle->codec-configured->qos-configured",
                "count>=2:bluez-audio: le bap sink state=pacs-discovered->configured",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-broadcast-sink command=sync big=0 "
                "bis=1 handle=0x0101",
                "count>=2:upstream-iso-connect: addr-type=1 connect-ret=0 "
                "state=1 sim-fastpath=1",
                "count>=2:bluez-audio: le broadcast sink synced",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bass.c "
                "style profile=le-bap-control role=sink "
                "command=sink-sync big=0 bis=1 handle=0x0101",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "pa-sync=1 base-received=1 bis-synced=1",
                "count>=2:bluez-audio: le bap sink state=configured->streaming",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-broadcast-sink command=start",
                "count>=2:upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 4c 43 33 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "count>=2:bluez-audio: le broadcast sink iso payload received",
                "count>=2:upstream-iso-close: released",
                "count>=2:bluez-audio: le broadcast sink stopped",
                "iso-socket-bind=2",
                "iso-socket-connect=2",
                "re:iso-socket-recv=([2-9]|[1-9][0-9]+)",
            )),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-unicast-profile",
        (
            RoleCheck("ble1", (
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-config cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "unicast cig=0 cis=1 ase=1 codec=lc3 context=media",
                "bluez-audio: le unicast source state=idle->configured",
                "upstream-iso-connect: addr-type=1 connect-ret=0 "
                "state=1 sim-fastpath=1",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-enable cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "cis-qos interval-us=10000",
                "bluez-audio: le unicast source state=configured->streaming",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-source command=start cig=0 "
                "cis=1 handle=0x0201",
                "re:upstream-iso-send: addr-type=1 handle=0x0201 "
                r"payload-len=28[^\n]*send-ret=28[^\n]*"
                r"fallback-ret=-95[^\n]*sim-fastpath=0[^\n]*"
                r"upstream-iso-attach=1",
                "bluez-audio: le unicast source queued cis payload len=28",
                "conn-hash acl=0 sco=0 le=0 le-peripheral=0 cis=1 bis=0",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-release cig=0 cis=1 handle=0x0201",
                "bluez-audio: le unicast source state=streaming->idle",
                "iso-socket-bind=1",
                "iso-socket-connect=1",
                "iso-socket-send=1",
                "upstream-iso-close: released",
            )),
            RoleCheck("ble2", (
                "bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-discover cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "sink discovered pac=lc3 context=media unicast=1",
                "bluez-audio: le unicast sink state=idle->pacs-discovered",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-config cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "ase state=idle->codec-configured->qos-configured",
                "bluez-audio: le unicast sink state=pacs-discovered->configured",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-sink command=sync cig=0 "
                "cis=1 handle=0x0201",
                "upstream-iso-connect: addr-type=1 connect-ret=0 "
                "state=1 sim-fastpath=1",
                "bluez-audio: le unicast sink synced",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-enable cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "cis-established=1 ase=1 receiver-ready=1",
                "bluez-audio: le unicast sink state=configured->streaming",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-sink command=start",
                "upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 43 49 53 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "bluez-audio: le unicast sink cis payload received",
                "conn-hash acl=0 sco=0 le=0 le-peripheral=0 cis=1 bis=0",
                "iso-socket-bind=1",
                "iso-socket-connect=1",
                "re:iso-socket-recv=[1-9]",
                "re:iso-socket-native-recv=[1-9]",
                "upstream-iso-close: released",
                "conn-hash acl=0 sco=0 le=0 le-peripheral=0 cis=0 bis=0",
                "bluez-audio: le unicast sink stopped",
            )),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-transport",
        (
            RoleCheck("ble1", (
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-config cig=0 cis=1 handle=0x0201",
                "bluez-audio: le unicast source state=idle->configured",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-enable cig=0 cis=1 handle=0x0201",
                "bluez-audio: le unicast source state=configured->streaming",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=unicast-source-acquire-write-release cig=0 cis=1 "
                "handle=0x0201",
                "upstream-iso-bind: addr-type=1 handle=0x0201",
                "upstream-iso-connect: addr-type=1 connect-ret=0 "
                "state=1 sim-fastpath=1",
                "bluez-audio: media transport acquire fd=iso",
                "upstream-iso-write: payload-len=28 send-ret=28",
                "bluez-audio: media transport write len=28 "
                "payload=LE-AUDIO:CIS:synthetic-frame",
                "upstream-iso-close: released",
                "bluez-audio: media transport release complete role=source",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-release cig=0 cis=1 handle=0x0201",
                "bluez-audio: le unicast source state=streaming->idle",
                "iso-socket-bind=1",
                "iso-socket-connect=1",
                "iso-socket-send=1",
            )),
            RoleCheck("ble2", (
                "bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-discover cig=0 cis=1 handle=0x0201",
                "bluez-audio: le unicast sink state=idle->pacs-discovered",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-config cig=0 cis=1 handle=0x0201",
                "bluez-audio: le unicast sink state=pacs-discovered->configured",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-enable cig=0 cis=1 handle=0x0201",
                "bluez-audio: le unicast sink state=configured->streaming",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=unicast-sink-acquire-read-release cig=0 cis=1 "
                "handle=0x0201",
                "upstream-iso-bind: addr-type=1 handle=0x0201",
                "upstream-iso-connect: addr-type=1 connect-ret=0 "
                "state=1 sim-fastpath=1",
                "bluez-audio: media transport acquire fd=iso",
                "upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 43 49 53 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "bluez-audio: media transport read complete "
                "payload=LE-AUDIO:CIS:synthetic-frame",
                "upstream-iso-close: released",
                "bluez-audio: media transport release complete role=sink",
                "iso-socket-bind=1",
                "iso-socket-connect=1",
                "re:iso-socket-recv=[1-9]",
                "re:iso-socket-native-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-transport-reconnect",
        (
            RoleCheck("ble1", (
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-config cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: le unicast source state=idle->configured",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-enable cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: le unicast source state=configured->streaming",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=unicast-source-acquire-write-release cig=0 cis=1 "
                "handle=0x0201",
                "count>=2:upstream-iso-bind: addr-type=1 handle=0x0201",
                "count>=2:upstream-iso-connect: addr-type=1 connect-ret=0 "
                "state=1 sim-fastpath=1",
                "count>=2:bluez-audio: media transport acquire fd=iso",
                "count>=2:upstream-iso-write: payload-len=28 send-ret=28",
                "count>=2:bluez-audio: media transport write len=28 "
                "payload=LE-AUDIO:CIS:synthetic-frame",
                "count>=2:upstream-iso-close: released",
                "count>=2:bluez-audio: media transport release complete role=source",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-release cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: le unicast source state=streaming->idle",
                "re:iso-socket-bind=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-connect=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-send=([2-9]|[1-9][0-9]+)",
            )),
            RoleCheck("ble2", (
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-discover cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: le unicast sink state=idle->pacs-discovered",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-config cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: le unicast sink state=pacs-discovered->configured",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-enable cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: le unicast sink state=configured->streaming",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=unicast-sink-acquire-read-release cig=0 cis=1 "
                "handle=0x0201",
                "count>=2:upstream-iso-bind: addr-type=1 handle=0x0201",
                "count>=2:upstream-iso-connect: addr-type=1 connect-ret=0 "
                "state=1 sim-fastpath=1",
                "count>=2:bluez-audio: media transport acquire fd=iso",
                "count>=2:upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 43 49 53 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "count>=2:bluez-audio: media transport read complete "
                "payload=LE-AUDIO:CIS:synthetic-frame",
                "count>=2:upstream-iso-close: released",
                "count>=2:bluez-audio: media transport release complete role=sink",
                "re:iso-socket-bind=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-connect=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-recv=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-native-recv=([2-9]|[1-9][0-9]+)",
            )),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-transport-bidir",
        (
            RoleCheck("ble1", (
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-config cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-enable cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=unicast-source-acquire-write-release cig=0 cis=1 "
                "handle=0x0201",
                "upstream-iso-write: payload-len=28 send-ret=28",
                "bluez-audio: media transport release complete role=source",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-release cig=0 cis=1 handle=0x0201",
                "bluez-audio: le unicast source state=streaming->idle",
                "bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-discover cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-config cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-enable cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=unicast-sink-acquire-read-release cig=0 cis=2 "
                "handle=0x0202",
                "upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 43 49 53 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "bluez-audio: media transport read complete "
                "payload=LE-AUDIO:CIS:synthetic-frame",
                "bluez-audio: media transport release complete role=sink",
                "re:iso-socket-bind=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-connect=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-send=[1-9]",
                "re:iso-socket-recv=[1-9]",
                "re:iso-socket-native-recv=[1-9]",
            )),
            RoleCheck("ble2", (
                "bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-discover cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-config cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-enable cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=unicast-sink-acquire-read-release cig=0 cis=1 "
                "handle=0x0201",
                "upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 43 49 53 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "bluez-audio: media transport read complete "
                "payload=LE-AUDIO:CIS:synthetic-frame",
                "bluez-audio: media transport release complete role=sink",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-config cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-enable cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=unicast-source-acquire-write-release cig=0 cis=2 "
                "handle=0x0202",
                "upstream-iso-write: payload-len=28 send-ret=28",
                "bluez-audio: media transport release complete role=source",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-release cig=0 cis=2 handle=0x0202",
                "bluez-audio: le unicast source state=streaming->idle",
                "re:iso-socket-bind=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-connect=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-send=[1-9]",
                "re:iso-socket-recv=[1-9]",
                "re:iso-socket-native-recv=[1-9]",
            )),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-transport-bidir-disable",
        (
            RoleCheck("ble1", (
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-config cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-enable cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=unicast-source-acquire-write-release cig=0 cis=1 "
                "handle=0x0201",
                "upstream-iso-write: payload-len=28 send-ret=28",
                "bluez-audio: media transport release complete role=source",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-disable cig=0 cis=1 handle=0x0201",
                "bluez-audio: le unicast source state=streaming->configured",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-release cig=0 cis=1 handle=0x0201",
                "bluez-audio: le unicast source state=streaming->idle",
                "bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-discover cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-config cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-enable cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=unicast-sink-acquire-read-release cig=0 cis=2 "
                "handle=0x0202",
                "upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 43 49 53 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "bluez-audio: media transport release complete role=sink",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-disable cig=0 cis=2 handle=0x0202",
                "bluez-audio: le unicast sink state=streaming->configured",
                "re:iso-socket-bind=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-connect=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-send=([1-9][0-9]*)",
                "re:iso-socket-recv=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-native-recv=([1-9][0-9]*)",
            )),
            RoleCheck("ble2", (
                "bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-discover cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-config cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-enable cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=unicast-sink-acquire-read-release cig=0 cis=1 "
                "handle=0x0201",
                "upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 43 49 53 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "bluez-audio: media transport release complete role=sink",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-disable cig=0 cis=1 handle=0x0201",
                "bluez-audio: le unicast sink state=streaming->configured",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-config cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-enable cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=unicast-source-acquire-write-release cig=0 cis=2 "
                "handle=0x0202",
                "upstream-iso-write: payload-len=28 send-ret=28",
                "bluez-audio: media transport release complete role=source",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-disable cig=0 cis=2 handle=0x0202",
                "bluez-audio: le unicast source state=streaming->configured",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-release cig=0 cis=2 handle=0x0202",
                "bluez-audio: le unicast source state=streaming->idle",
                "re:iso-socket-bind=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-connect=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-send=([1-9][0-9]*)",
                "re:iso-socket-recv=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-native-recv=([1-9][0-9]*)",
            )),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-transport-bidir-qos-update",
        (
            RoleCheck("ble1", (
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-config cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-enable cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=unicast-source-acquire-write-release cig=0 cis=1 "
                "handle=0x0201",
                "count>=2:upstream-iso-write: payload-len=28 send-ret=28",
                "count>=2:bluez-audio: media transport release complete role=source",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-disable cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-qos-update cig=0 cis=1 handle=0x0201",
                "bluez-audio: le unicast source state=configured->configured qos-updated",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-release cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-discover cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-config cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-enable cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=unicast-sink-acquire-read-release cig=0 cis=2 "
                "handle=0x0202",
                "count>=2:upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 43 49 53 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "count>=2:bluez-audio: media transport release complete role=sink",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-disable cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-qos-update cig=0 cis=2 handle=0x0202",
                "bluez-audio: le unicast sink state=configured->configured qos-updated",
                "re:iso-socket-bind=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-connect=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-send=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-recv=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-native-recv=([2-9]|[1-9][0-9]+)",
            )),
            RoleCheck("ble2", (
                "bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-discover cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-config cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-enable cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=unicast-sink-acquire-read-release cig=0 cis=1 "
                "handle=0x0201",
                "count>=2:upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 43 49 53 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "count>=2:bluez-audio: media transport release complete role=sink",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-disable cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-qos-update cig=0 cis=1 handle=0x0201",
                "bluez-audio: le unicast sink state=configured->configured qos-updated",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-config cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-enable cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=unicast-source-acquire-write-release cig=0 cis=2 "
                "handle=0x0202",
                "count>=2:upstream-iso-write: payload-len=28 send-ret=28",
                "count>=2:bluez-audio: media transport release complete role=source",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-disable cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-qos-update cig=0 cis=2 handle=0x0202",
                "bluez-audio: le unicast source state=configured->configured qos-updated",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-release cig=0 cis=2 handle=0x0202",
                "re:iso-socket-bind=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-connect=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-send=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-recv=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-native-recv=([2-9]|[1-9][0-9]+)",
            )),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-transport-bidir-qos-reject",
        (
            RoleCheck("ble1", (
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-config cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-enable cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=unicast-source-acquire-write-release cig=0 cis=1 "
                "handle=0x0201",
                "count>=2:upstream-iso-write: payload-len=28 send-ret=28",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-disable cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-qos-reject cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "ase cp response op=config-qos status=0x0d reason=invalid-qos",
                "bluez-audio: le unicast source state=configured->configured qos-rejected",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-qos-update cig=0 cis=1 handle=0x0201",
                "bluez-audio: le unicast source state=configured->configured qos-updated",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-release cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-discover cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-config cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-enable cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=unicast-sink-acquire-read-release cig=0 cis=2 "
                "handle=0x0202",
                "count>=2:upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 43 49 53 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-disable cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-qos-reject cig=0 cis=2 handle=0x0202",
                "bluez-audio: le unicast sink state=configured->configured qos-rejected",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-qos-update cig=0 cis=2 handle=0x0202",
                "bluez-audio: le unicast sink state=configured->configured qos-updated",
                "re:iso-socket-bind=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-connect=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-send=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-recv=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-native-recv=([2-9]|[1-9][0-9]+)",
            )),
            RoleCheck("ble2", (
                "bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-discover cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-config cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-enable cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=unicast-sink-acquire-read-release cig=0 cis=1 "
                "handle=0x0201",
                "count>=2:upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 43 49 53 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-disable cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-qos-reject cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "ase cp response op=config-qos status=0x0d reason=invalid-qos",
                "bluez-audio: le unicast sink state=configured->configured qos-rejected",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-qos-update cig=0 cis=1 handle=0x0201",
                "bluez-audio: le unicast sink state=configured->configured qos-updated",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-config cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-enable cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=unicast-source-acquire-write-release cig=0 cis=2 "
                "handle=0x0202",
                "count>=2:upstream-iso-write: payload-len=28 send-ret=28",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-disable cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-qos-reject cig=0 cis=2 handle=0x0202",
                "bluez-audio: le unicast source state=configured->configured qos-rejected",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-qos-update cig=0 cis=2 handle=0x0202",
                "bluez-audio: le unicast source state=configured->configured qos-updated",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-release cig=0 cis=2 handle=0x0202",
                "re:iso-socket-bind=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-connect=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-send=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-recv=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-native-recv=([2-9]|[1-9][0-9]+)",
            )),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-transport-bidir-qos-cancel",
        (
            RoleCheck("ble1", (
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-config cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-enable cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=unicast-source-acquire-write-release cig=0 cis=1 "
                "handle=0x0201",
                "count>=2:upstream-iso-write: payload-len=28 send-ret=28",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-disable cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-qos-cancel cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "ase cp response op=config-qos status=0x0e reason=procedure-cancelled",
                "bluez-audio: le unicast source state=configured->configured qos-cancelled",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-qos-update cig=0 cis=1 handle=0x0201",
                "bluez-audio: le unicast source state=configured->configured qos-updated",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-release cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-discover cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-config cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-enable cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=unicast-sink-acquire-read-release cig=0 cis=2 "
                "handle=0x0202",
                "count>=2:upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 43 49 53 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-disable cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-qos-cancel cig=0 cis=2 handle=0x0202",
                "bluez-audio: le unicast sink state=configured->configured qos-cancelled",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-qos-update cig=0 cis=2 handle=0x0202",
                "bluez-audio: le unicast sink state=configured->configured qos-updated",
                "re:iso-socket-bind=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-connect=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-send=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-recv=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-native-recv=([2-9]|[1-9][0-9]+)",
            )),
            RoleCheck("ble2", (
                "bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-discover cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-config cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-enable cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=unicast-sink-acquire-read-release cig=0 cis=1 "
                "handle=0x0201",
                "count>=2:upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 43 49 53 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-disable cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-qos-cancel cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "ase cp response op=config-qos status=0x0e reason=procedure-cancelled",
                "bluez-audio: le unicast sink state=configured->configured qos-cancelled",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-qos-update cig=0 cis=1 handle=0x0201",
                "bluez-audio: le unicast sink state=configured->configured qos-updated",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-config cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-enable cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=unicast-source-acquire-write-release cig=0 cis=2 "
                "handle=0x0202",
                "count>=2:upstream-iso-write: payload-len=28 send-ret=28",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-disable cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-qos-cancel cig=0 cis=2 handle=0x0202",
                "bluez-audio: le unicast source state=configured->configured qos-cancelled",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-qos-update cig=0 cis=2 handle=0x0202",
                "bluez-audio: le unicast source state=configured->configured qos-updated",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-release cig=0 cis=2 handle=0x0202",
                "re:iso-socket-bind=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-connect=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-send=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-recv=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-native-recv=([2-9]|[1-9][0-9]+)",
            )),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-transport-bidir-release-reconfig",
        (
            RoleCheck("ble1", (
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-config cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-enable cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=unicast-source-acquire-write-release cig=0 cis=1 "
                "handle=0x0201",
                "count>=2:upstream-iso-write: payload-len=28 send-ret=28",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-disable cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-release cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "ase state=configured->idle release-complete",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-discover cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-config cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-enable cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=unicast-sink-acquire-read-release cig=0 cis=2 "
                "handle=0x0202",
                "count>=2:upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 43 49 53 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-disable cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-release cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: le unicast sink state=configured->idle",
                "re:iso-socket-bind=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-connect=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-send=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-recv=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-native-recv=([2-9]|[1-9][0-9]+)",
            )),
            RoleCheck("ble2", (
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-discover cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-config cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-enable cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=unicast-sink-acquire-read-release cig=0 cis=1 "
                "handle=0x0201",
                "count>=2:upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 43 49 53 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-disable cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-release cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: le unicast sink state=configured->idle",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-config cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-enable cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=unicast-source-acquire-write-release cig=0 cis=2 "
                "handle=0x0202",
                "count>=2:upstream-iso-write: payload-len=28 send-ret=28",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-disable cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-release cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "ase state=configured->idle release-complete",
                "re:iso-socket-bind=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-connect=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-send=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-recv=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-native-recv=([2-9]|[1-9][0-9]+)",
            )),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-transport-bidir-reconnect",
        (
            RoleCheck("ble1", (
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-config cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-enable cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=unicast-source-acquire-write-release cig=0 cis=1 "
                "handle=0x0201",
                "count>=2:upstream-iso-write: payload-len=28 send-ret=28",
                "count>=2:bluez-audio: media transport write len=28 "
                "payload=LE-AUDIO:CIS:synthetic-frame",
                "count>=2:bluez-audio: media transport release complete role=source",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-release cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: le unicast source state=streaming->idle",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-discover cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-config cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-enable cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=unicast-sink-acquire-read-release cig=0 cis=2 "
                "handle=0x0202",
                "count>=2:upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 43 49 53 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "count>=2:bluez-audio: media transport read complete "
                "payload=LE-AUDIO:CIS:synthetic-frame",
                "count>=2:bluez-audio: media transport release complete role=sink",
                "re:iso-socket-bind=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-connect=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-send=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-recv=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-native-recv=([2-9]|[1-9][0-9]+)",
            )),
            RoleCheck("ble2", (
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-discover cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-config cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-enable cig=0 cis=1 handle=0x0201",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=unicast-sink-acquire-read-release cig=0 cis=1 "
                "handle=0x0201",
                "count>=2:upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 43 49 53 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "count>=2:bluez-audio: media transport read complete "
                "payload=LE-AUDIO:CIS:synthetic-frame",
                "count>=2:bluez-audio: media transport release complete role=sink",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-config cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-enable cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=unicast-source-acquire-write-release cig=0 cis=2 "
                "handle=0x0202",
                "count>=2:upstream-iso-write: payload-len=28 send-ret=28",
                "count>=2:bluez-audio: media transport write len=28 "
                "payload=LE-AUDIO:CIS:synthetic-frame",
                "count>=2:bluez-audio: media transport release complete role=source",
                "count>=2:bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-release cig=0 cis=2 handle=0x0202",
                "count>=2:bluez-audio: le unicast source state=streaming->idle",
                "re:iso-socket-bind=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-connect=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-send=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-recv=([4-9]|[1-9][0-9]+)",
                "re:iso-socket-native-recv=([2-9]|[1-9][0-9]+)",
            )),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-full-lifecycle",
        (
            RoleCheck("ble1", (
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-bap-control role=source "
                "command=source-announce big=0 bis=1 handle=0x0101",
                "bluez-audio: le bap source state=idle->base-announced",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-bap-control role=source "
                "command=source-start big=0 bis=1 handle=0x0101",
                "bluez-audio: le bap source state=base-announced->streaming",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-broadcast-source command=start big=0 "
                "bis=1 handle=0x0101",
                "bluez-audio: le broadcast source queued iso payload "
                "len=28",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-bap-control role=source "
                "command=source-stop big=0 bis=1 handle=0x0101",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-config cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-enable cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=unicast-source-acquire-write-release cig=0 cis=1 "
                "handle=0x0201",
                "upstream-iso-write: payload-len=28 send-ret=28",
                "bluez-audio: media transport write len=28 "
                "payload=LE-AUDIO:CIS:synthetic-frame",
                "bluez-audio: media transport release complete role=source",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-release cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-discover cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-config cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-enable cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=unicast-sink-acquire-read-release cig=0 cis=2 "
                "handle=0x0202",
                "upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 43 49 53 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "bluez-audio: media transport read complete "
                "payload=LE-AUDIO:CIS:synthetic-frame",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-release cig=0 cis=2 handle=0x0202",
                "re:iso-socket-send=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-recv=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-native-recv=([1-9]|[1-9][0-9]+)",
            )),
            RoleCheck("ble2", (
                "bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-bap-control role=sink "
                "command=sink-discover big=0 bis=1 handle=0x0101",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-bap-control role=sink "
                "command=sink-config big=0 bis=1 handle=0x0101",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-broadcast-sink command=sync big=0 "
                "bis=1 handle=0x0101",
                "bluez-audio: le broadcast sink synced",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-broadcast-sink command=start",
                "bluez-audio: le broadcast sink iso payload received",
                "bluez-audio: le broadcast sink stopped",
                "bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-discover cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-config cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-enable cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=sink "
                "command=unicast-sink-acquire-read-release cig=0 cis=1 "
                "handle=0x0201",
                "upstream-iso-recv: recv-ret=28 flags=0x0 "
                "payload=4c 45 2d 41 55 44 49 4f 3a 43 49 53 3a 73 "
                "79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "bluez-audio: media transport read complete "
                "payload=LE-AUDIO:CIS:synthetic-frame",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=sink "
                "command=sink-release cig=0 cis=1 handle=0x0201",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-config cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-unicast-control role=source "
                "command=source-enable cig=0 cis=2 handle=0x0202",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=media-transport role=source "
                "command=unicast-source-acquire-write-release cig=0 cis=2 "
                "handle=0x0202",
                "upstream-iso-write: payload-len=28 send-ret=28",
                "bluez-audio: media transport write len=28 "
                "payload=LE-AUDIO:CIS:synthetic-frame",
                "bluez-audio: media transport release complete role=source",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "style profile=le-unicast-control role=source "
                "command=source-release cig=0 cis=2 handle=0x0202",
                "re:iso-socket-send=[1-9]",
                "re:iso-socket-recv=([2-9]|[1-9][0-9]+)",
                "re:iso-socket-native-recv=([1-9]|[1-9][0-9]+)",
            )),
        ),
    ),
)


FORBIDDEN_PATTERNS: tuple[re.Pattern[str], ...] = (
    re.compile(r"\bPANIC\b", re.IGNORECASE),
    re.compile(r"\bASSERT", re.IGNORECASE),
    re.compile(r"segmentation fault", re.IGNORECASE),
    re.compile(r"btctl: .* failed:", re.IGNORECASE),
    re.compile(r"btaudio: .* failed:", re.IGNORECASE),
)


def read_text(path: Path) -> str:
    try:
        return path.read_text(errors="ignore")
    except FileNotFoundError:
        return ""


def case_by_name(name: str) -> CaseCheck | None:
    for case in CASES:
        if case.name == name:
            return case
    return None


def validate_role(log_dir: Path, case: CaseCheck,
                  role: RoleCheck) -> dict[str, object]:
    path = log_dir / f"{case.name}.{role.role}.log"
    text = read_text(path)
    missing = []
    for needle in role.required:
        if needle.startswith("re:"):
            if re.search(needle[3:], text) is None:
                missing.append(needle)
        elif needle.startswith("count>="):
            spec = needle[len("count>="):]
            count_text, _, counted = spec.partition(":")
            try:
                required_count = int(count_text, 10)
            except ValueError:
                missing.append(needle)
                continue

            if not counted or text.count(counted) < required_count:
                missing.append(needle)
        elif needle not in text:
            missing.append(needle)
    forbidden = [pattern.pattern for pattern in FORBIDDEN_PATTERNS
                 if pattern.search(text)]

    return {
        "role": role.role,
        "log": str(path),
        "exists": path.exists(),
        "missing": missing,
        "forbidden": forbidden,
        "passed": path.exists() and not missing and not forbidden,
    }


def validate_case(log_dir: Path, case: CaseCheck) -> dict[str, object]:
    roles = [validate_role(log_dir, case, role) for role in case.roles]
    return {
        "case": case.name,
        "roles": roles,
        "passed": all(role["passed"] for role in roles),
    }


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Validate BT/BLE hwsim usecase logs")
    parser.add_argument(
        "--log-dir",
        default=str(Path.cwd() / "build" / "bt-hwsim-usecases"),
        help="Directory containing <case>.<role>.log files")
    parser.add_argument(
        "--case",
        action="append",
        choices=[case.name for case in CASES],
        help="Case to validate. May be passed more than once.")
    parser.add_argument(
        "--json",
        action="store_true",
        help="Print JSON instead of text summary")
    parser.add_argument(
        "--list",
        action="store_true",
        help="List known cases and exit")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.list:
        for case in CASES:
            print(case.name)
        return 0

    log_dir = Path(args.log_dir).resolve()
    selected = args.case if args.case else [case.name for case in CASES]
    results = [validate_case(log_dir, case_by_name(name))
               for name in selected if case_by_name(name) is not None]
    passed = all(result["passed"] for result in results)

    if args.json:
        print(json.dumps({
            "log_dir": str(log_dir),
            "passed": passed,
            "results": results,
        }, indent=2, sort_keys=True))
    else:
        for result in results:
            status = "PASS" if result["passed"] else "FAIL"
            print(f"{status} {result['case']}")
            for role in result["roles"]:
                role_status = "PASS" if role["passed"] else "FAIL"
                print(f"  {role_status} {role['role']} {role['log']}")
                if not role["exists"]:
                    print("    missing log file")
                for needle in role["missing"]:
                    print(f"    missing: {needle}")
                for pattern in role["forbidden"]:
                    print(f"    forbidden: {pattern}")

    return 0 if passed else 1


if __name__ == "__main__":
    sys.exit(main())
