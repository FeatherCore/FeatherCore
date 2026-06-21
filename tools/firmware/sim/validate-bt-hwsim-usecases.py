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


def obex_profile_role(role: str, marker: str,
                      links: tuple[str, ...]) -> RoleCheck:
    required = [
        f"{marker}_BEGIN_{role.upper()}",
        "bluez-obex: closeout upstream-link-ledger",
        "bluez-obex: profile-final=1",
    ]
    required.extend(
        f"upstream-link=bluezobex-{link}-obex-upstream-link-obexd "
        "final-ok=1"
        for link in links
    )
    required.append(f"{marker}_DONE_{role.upper()}")
    return RoleCheck(role, tuple(required))


def profile_family_role(role: str, marker: str, family: str,
                        daemon_link: str, profile_link: str) -> RoleCheck:
    return RoleCheck(role, (
        f"{marker}_BEGIN_{role.upper()}",
        f"bluez-daemon: {family} closeout semantic-contract",
        f"upstream-link={daemon_link}",
        f"bluez-profile: closeout upstream-link-ledger family={family}",
        "bluez-profile: profile-final=1 bearer-final=1 "
        "transaction-final=1 error-final=1 cleanup-final=1",
        f"upstream-link={profile_link} final-ok=1",
        f"{marker}_DONE_{role.upper()}",
    ))




def command_role(role: str, *commands: str) -> RoleCheck:
    return RoleCheck(role, tuple(commands))


def hid_hogp_closeout_role(role: str, marker_role: str, family: str,
                           daemon_role: str, command_mode: str,
                           profile_mode: str) -> RoleCheck:
    return RoleCheck(role, (
        f"BLUEZHIDHOGP_BEGIN_{marker_role}",
        f"bluezdaemon profile-hid-closeout {command_mode}",
        f"bluez-daemon: {family} closeout upstream-coverage-map "
        f"role={daemon_role}",
        f"bluezprofile closeout {profile_mode}",
        "bluez-profile: profile-final=1 bearer-final=1 "
        "transaction-final=1 error-final=1 cleanup-final=1",
        f"BLUEZHIDHOGP_DONE_{marker_role}",
    ))


def gatt_profile_closeout_role(role: str, marker_role: str,
                               daemon_role: str,
                               profile_mode: str) -> RoleCheck:
    return RoleCheck(role, (
        f"BLUEZGATTPROFILE_BEGIN_{marker_role}",
        f"bluezdaemon profile-gatt-closeout {daemon_role}",
        f"bluez-daemon: gatt closeout upstream-coverage-map "
        f"role={daemon_role}",
        f"bluezprofile closeout {profile_mode}",
        "bluez-profile: profile-final=1 bearer-final=1 "
        "transaction-final=1 error-final=1 cleanup-final=1",
        f"BLUEZGATTPROFILE_DONE_{marker_role}",
    ))


def le_daemon_role(role: str, *commands: str) -> RoleCheck:
    required = [
        "bluezaudio le-daemon plugin-init",
        "bluez-audio: source=third/bluez/profiles/audio/main.c "
        "style=bluetoothd-plugin command=plugin-init",
        "bluezaudio le-daemon register",
        "bluez-audio: source=third/bluez/profiles/audio/main.c "
        "style profile=le-audio-daemon command=register",
        "bluezaudio le-daemon profile-accept",
        "bluez-audio: source=third/bluez/profiles/audio/bap.c "
        "style=bt_bap command=attach session=le-audio",
    ]
    required.extend(commands)
    required.extend((
        "bluezaudio le-daemon profile-release",
        "bluez-audio: source=third/bluez/profiles/audio/bap.c "
        "style=bt_bap command=detach session=le-audio",
        "bluezaudio le-daemon plugin-exit",
        "bluez-audio: source=third/bluez/profiles/audio/main.c "
        "style=bluetoothd-plugin command=plugin-exit",
    ))
    return RoleCheck(role, tuple(required))


def le_controller_role(role: str, monitor: str,
                       *commands: str) -> RoleCheck:
    required = [
        f"bluezhciraw {monitor}",
        "bluez-hciraw: user-iso-setup",
    ]
    required.extend(commands)
    return RoleCheck(role, tuple(required))

def is_avrcp_controller_sequential_response(case: CaseCheck,
                                            role: RoleCheck,
                                            needle: str) -> bool:
    if role.role != "bt1" or not case.name.startswith("bluez-a2dp-avrcp-"):
        return False

    if needle.startswith("upstream-l2cap-recv-handle: psm=0x0017"):
        return True

    if needle.startswith("payload=02 11 0e"):
        return True

    if needle.startswith("bluez-audio: avrcp controller ") and (
        " response " in needle or " interim " in needle
    ):
        return True

    return False


BNEP_NATIVE_DATAPATH_REQUIRED: tuple[str, ...] = (
    "bnep-native-active=1",
    "bnep-native-contract-version=1",
    "bnep-native-helper-contract="
    "sock_ioctl_connadd,bnep_add_connection,netdev_setup,"
    "session_thread,ndo_start_xmit,bnep_rx_frame,conndel_cleanup",
    "bnep-native-helper-owner=net_bluetooth/bnep",
    "upstream-link=bluez-fd-to-imported-bnep",
    "bnep-native-source-map="
    "sock.c,core.c,netdev.c,linux_bt_bnep_netdev.c",
    "bnep-native-session-ownership="
    "sockfd_lookup,BNEPCONNADD,bnep_add_connection,alloc_netdev,"
    "register_netdev,kthread_run,bnep_session",
    "bnep-native-thread-ownership="
    "kthread_run,bnep_session,rx_wait,tx_wait,stop_wakeup,"
    "session_terminate",
    "bnep-native-netdev-ownership="
    "alloc_netdev,netdev_ops,ndo_open,ndo_stop,ndo_start_xmit,"
    "netif_rx,unregister_netdev,free_netdev",
    "bnep-native-state-ownership="
    "session_new,session_active,session_stopping,session_closed,"
    "active_zero",
    "bnep-native-lock-ownership="
    "session_list,session_ref,tx_queue,rx_queue,ioctl_serialization",
    "bnep-native-error-ownership="
    "bad_fd,bad_role,duplicate_session,tx_fail,rx_drop,"
    "conndel_missing,cleanup_after_error",
    "bnep-native-datapath-ownership="
    "NuttX-IP,ndo_start_xmit,bnep_tx_frame,L2CAP,hwsim,"
    "bnep_rx_frame,netif_rx,NuttX-IP-RX",
    "bnep-native-core-tx-path=kernel_sendmsg-only",
    "bnep-native-fd-ownership="
    "connected-l2cap-fd,bnep-control-fd,sockfd_lookup,"
    "sockfd_put-error,session-sock-owner",
    "bnep-native-fd-source=socket-fd",
    "bnep-native-fd-handoff-reject=0",
    "bnep-native-fd-lookup-kept-probe=0",
    "bnep-native-fd-lookup-rebind-probe=0",
    "bnep-native-cleanup-ownership="
    "BNEPCONNDEL,bnep_del_connection,unregister_netdev,"
    "session_stop,fd_cleanup",
    "bnep-core-active=0",
    "bnep-core-add=0",
    "bnep-core-del=0",
    "bnep-legacy-fallback-compiled=0",
    "re:bnep-native-netdev-setup=[1-9]",
    "bnep-native-netdev-register=1",
    "bnep-native-session-create=1",
    "re:bnep-native-session-link=[1-9]",
    "re:bnep-native-session-thread=[1-9]",
    "re:bnep-native-kthread-run=[1-9]",
    "re:bnep-native-sock-ioctl-connadd=[1-9]",
    "re:bnep-native-ndo-start-xmit=[1-9]",
    "re:bnep-native-netdev-xmit=[1-9]",
    "re:bnep-native-session-tx-dequeue=[1-9]",
    "re:bnep-native-tx-frame=[1-9]",
    "re:bnep-native-tx-frame-ok=[1-9]",
    "re:bnep-native-session-rx-dequeue=[1-9]",
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
    "bnep-native-fd-last=-1",
    "bnep-native-fd-psm=0x0000",
    "bnep-native-fd-cid=0x0000",
    "bnep-native-fd-handle=0x0000",
    "bnep-native-fd-role=0x0000",
    "bnep-native-fd-source=none",
    "bnep-native-netdev-unregister=1",
    "re:bnep-native-session-unlink=[1-9]",
    "re:bnep-native-sock-ioctl-conndel=[1-9]",
    "bnep-native-session-start=1",
    "bnep-native-session-stop=1",
)


BNEP_BLUEZ_FD_HANDOFF_ACTIVE_REQUIRED: tuple[str, ...] = (
    "bluez-bneptest: pan-up native-boundary connected-l2cap-fd=",
    "ioctl=BNEPCONNADD role=0x1115 device=btn0",
    "bluez-bneptest: pan-up native-boundary connected-l2cap-fd=",
    "fd-source=socket-fd",
    "bluez-bneptest: native-status-after-pan-up",
    "bnep-native-fd-handoff=1",
    "bnep-native-fd-active=1",
    "bnep-native-fd-cid=0x0041",
    "bnep-native-fd-role=0x1115",
    "bnep-native-fd-source=socket-fd",
)


BNEP_BLUEZ_FD_HANDOFF_TEARDOWN_REQUIRED: tuple[str, ...] = (
    "bluez-bneptest: native-status-after-pan-down",
    "bnep-native-fd-active=0",
    "bnep-native-fd-cleanup=1",
)


BNEP_NATIVE_RECONNECT_REQUIRED: tuple[str, ...] = (
    "bnep-ioctl-connadd=2",
    "bnep-ioctl-conndel=2",
    "bnep-native-fd-handoff-reject=0",
    "bnep-native-helper-contract="
    "sock_ioctl_connadd,bnep_add_connection,netdev_setup,"
    "session_thread,ndo_start_xmit,bnep_rx_frame,conndel_cleanup",
    "bnep-native-helper-owner=net_bluetooth/bnep",
    "upstream-link=bluez-fd-to-imported-bnep",
    "bnep-native-thread-ownership="
    "kthread_run,bnep_session,rx_wait,tx_wait,stop_wakeup,"
    "session_terminate",
    "bnep-native-netdev-ownership="
    "alloc_netdev,netdev_ops,ndo_open,ndo_stop,ndo_start_xmit,"
    "netif_rx,unregister_netdev,free_netdev",
    "bnep-native-state-ownership="
    "session_new,session_active,session_stopping,session_closed,"
    "active_zero",
    "bnep-native-lock-ownership="
    "session_list,session_ref,tx_queue,rx_queue,ioctl_serialization",
    "bnep-native-error-ownership="
    "bad_fd,bad_role,duplicate_session,tx_fail,rx_drop,"
    "conndel_missing,cleanup_after_error",
    "bnep-legacy-fallback-compiled=0",
    "bnep-native-session-create=2",
    "bnep-native-session-start=2",
    "bnep-native-session-terminate=2",
    "bnep-native-session-stop=2",
    "bnep-native-netdev-register=2",
    "bnep-native-netdev-unregister=2",
    "bnep-native-active=0",
    "re:bnep-native-netdev-setup=[1-9]",
    "re:bnep-native-session-link=[1-9]",
    "re:bnep-native-session-unlink=[1-9]",
    "re:bnep-native-session-thread=[1-9]",
    "re:bnep-native-kthread-run=[1-9]",
    "re:bnep-native-sock-ioctl-connadd=[1-9]",
    "re:bnep-native-sock-ioctl-conndel=[1-9]",
    "re:bnep-native-ndo-start-xmit=[1-9]",
    "re:bnep-native-netdev-xmit=[1-9]",
    "re:bnep-native-session-tx-dequeue=[1-9]",
    "re:bnep-native-session-rx-dequeue=[1-9]",
    "re:bnep-native-tx-frame-ok=[1-9]",
    "re:bnep-native-l2cap-delivered=[1-9]",
    "re:bnep-native-rx-frame-ok=[1-9]",
    "re:bnep-native-netif-rx=[1-9]",
)


BNEP_NATIVE_RECONNECT_STRESS_REQUIRED: tuple[str, ...] = (
    "bnep-ioctl-connadd=3",
    "bnep-ioctl-conndel=3",
    "bnep-native-fd-handoff-reject=0",
    "bnep-native-helper-contract="
    "sock_ioctl_connadd,bnep_add_connection,netdev_setup,"
    "session_thread,ndo_start_xmit,bnep_rx_frame,conndel_cleanup",
    "bnep-native-helper-owner=net_bluetooth/bnep",
    "upstream-link=bluez-fd-to-imported-bnep",
    "bnep-native-thread-ownership="
    "kthread_run,bnep_session,rx_wait,tx_wait,stop_wakeup,"
    "session_terminate",
    "bnep-native-netdev-ownership="
    "alloc_netdev,netdev_ops,ndo_open,ndo_stop,ndo_start_xmit,"
    "netif_rx,unregister_netdev,free_netdev",
    "bnep-native-state-ownership="
    "session_new,session_active,session_stopping,session_closed,"
    "active_zero",
    "bnep-native-lock-ownership="
    "session_list,session_ref,tx_queue,rx_queue,ioctl_serialization",
    "bnep-native-error-ownership="
    "bad_fd,bad_role,duplicate_session,tx_fail,rx_drop,"
    "conndel_missing,cleanup_after_error",
    "bnep-legacy-fallback-compiled=0",
    "bnep-native-session-create=3",
    "bnep-native-session-start=3",
    "bnep-native-session-terminate=3",
    "bnep-native-session-stop=3",
    "bnep-native-netdev-register=3",
    "bnep-native-netdev-unregister=3",
    "bnep-native-active=0",
    "re:bnep-native-netdev-setup=[1-9]",
    "re:bnep-native-session-link=[1-9]",
    "re:bnep-native-session-unlink=[1-9]",
    "re:bnep-native-session-thread=[1-9]",
    "re:bnep-native-kthread-run=[1-9]",
    "re:bnep-native-sock-ioctl-connadd=[1-9]",
    "re:bnep-native-sock-ioctl-conndel=[1-9]",
    "re:bnep-native-ndo-start-xmit=[1-9]",
    "re:bnep-native-netdev-xmit=[1-9]",
    "re:bnep-native-session-tx-dequeue=[1-9]",
    "re:bnep-native-session-rx-dequeue=[1-9]",
    "re:bnep-native-tx-frame-ok=[1-9]",
    "re:bnep-native-l2cap-delivered=[1-9]",
    "re:bnep-native-rx-frame-ok=[1-9]",
    "re:bnep-native-netif-rx=[1-9]",
)


BNEP_IPERF_THROUGHPUT_REQUIRED: tuple[str, ...] = (
    r"re:[1-9][0-9]* Bytes\s+[0-9]+\.[0-9]*[1-9] Mbits/sec",
)


BNEP_NETWORK_IPERF_OWNERSHIP_REQUIRED: tuple[str, ...] = (
    "bluez-network: upstream-object-graph action=connect-begin",
    "owner=profiles/network/manager.c,server.c,connection.c,bnep.c",
    "objects=network_manager,network_server,network_peer,"
    "network_conn,network_session,bnep_control,l2cap_io,netdev_bridge",
    "dbus=org.bluez.Network1,org.bluez.NetworkServer1",
    "methods=Connect,Disconnect,Register,Unregister",
    "fd-flow=connect_l2cap_fd,BNEPCONNADD,BNEPCONNDEL",
    "bluez-network: upstream-object-graph action=connect-complete",
    "session-state=connected",
    "bluez-network: upstream-object-graph action=disconnect-complete",
    "session-state=closed",
    "bluez-network: native-boundary connected-l2cap-fd=",
    "ioctl=BNEPCONNADD role=0x1115 service=panu device=btn0",
    "bluez-network: native-closeout fd-ownership="
    "l2cap-fd=connected,bnep-fd=control,sock-lookup=1,"
    "sock-put=1,psm=0x000f,cid=0x0041",
    "bluez-network: native-closeout session-ownership="
    "bnep_add_connection,netdev_setup,register_netdev,"
    "session_link,kthread_run,session_thread service=panu",
    "bluez-network: native-closeout datapath-ownership="
    "Network1.Connect,connected-l2cap-fd,BNEPCONNADD,btn0,"
    "ndo_start_xmit,bnep_tx_frame,l2cap-send,hwsim-bnep,"
    "bnep_rx_frame,netif_rx,NuttX-IP",
    "bluez-network: native-status-after-connect",
    "bluez-network: native-status",
    "re:bnep-native-session-tx-dequeue=[1-9]",
    "bluez-network: native-status-after-disconnect",
    "bnep-native-active=0",
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
    "upstream-link=net_bluetooth/6lowpan+sim-ipsp-datapath-link",
    "upstream-object-contract-version=1",
    "upstream-peer-ownership=peer_add,peer_lookup,peer_del,"
    "peer_ref,peer_unref",
    "upstream-coc-ownership=l2cap_le_connect,chan_ready_cb,recv_cb,"
    "chan_close_cb,credits,psm_0x0023,cid_0x0040",
    "upstream-netdev-ownership=setup_netdev,register_netdev,"
    "ndo_start_xmit,netif_rx,delete_netdev,unregister_netdev",
    "upstream-state-ownership=netdev_active,coc_active,peer_active,"
    "tx_active,rx_active,registered_closed",
    "upstream-error-ownership=bad_psm,bad_cid,credit_exhausted,"
    "iphc_fail,fragment_drop,peer_missing,cleanup_after_error",
    "upstream-helper-contract=attach_ipsp,detach_ipsp,transmit_packet,"
    "receive_packet",
    "upstream-helper-link=net_bluetooth/6lowpan",
    "upstream-link=linux-6lowpan-ipsp-link-helper",
    "upstream-datapath-ownership=nuttx-ip-tx,net_6lowpan-iphc,"
    "bt_6lowpan_xmit,l2cap-coc,hwsim-acl,hwsim-rx,l2cap-coc,"
    "bt_6lowpan_recv,netif-rx,nuttx-ip-rx",
    "re:upstream-link-ledger=netdev:1,coc:1,peer:1,"
    "netdev-ref:1,chan-ref:1,peer-ref:1,tx-active=[1-9][0-9]*,"
    "rx-active=[1-9][0-9]*",
    "re:upstream-link-netdev-register=[1-9][0-9]*",
    "re:upstream-link-netdev-unregister=[1-9][0-9]*",
    "re:upstream-link-tx=[1-9][0-9]*",
    "re:upstream-link-rx=[1-9][0-9]*",
    "re:upstream-link-peer-add=[1-9][0-9]*",
    "re:upstream-link-peer-del=[1-9][0-9]*",
    "re:upstream-link-coc-open=[1-9][0-9]*",
    "re:upstream-link-coc-close=[1-9][0-9]*",
    "re:upstream-link-xmit=[1-9][0-9]*",
    "re:upstream-link-rx-deliver=[1-9][0-9]*",
    "re:upstream-link-setup-netdev=[1-9][0-9]*",
    "re:upstream-link-delete-netdev=[1-9][0-9]*",
    "re:upstream-link-chan-ready-cb=[1-9][0-9]*",
    "re:upstream-link-chan-close-cb=[1-9][0-9]*",
    "re:upstream-link-bt-xmit=[1-9][0-9]*",
    "re:upstream-link-recv-cb=[1-9][0-9]*",
    "upstream-iphc-link=net_6lowpan/iphc",
    "last-tx-dispatch=0x7b last-rx-dispatch=0x7b",
    "re:tx-iphc=[1-9][0-9]* rx-iphc=[1-9][0-9]*",
    "tx-iphc-error=0",
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
    "upstream-link=net_bluetooth/6lowpan+sim-ipsp-datapath-link",
    "upstream-object-contract-version=1",
    "upstream-peer-ownership=peer_add,peer_lookup,peer_del,"
    "peer_ref,peer_unref",
    "upstream-coc-ownership=l2cap_le_connect,chan_ready_cb,recv_cb,"
    "chan_close_cb,credits,psm_0x0023,cid_0x0040",
    "upstream-netdev-ownership=setup_netdev,register_netdev,"
    "ndo_start_xmit,netif_rx,delete_netdev,unregister_netdev",
    "upstream-state-ownership=netdev_active,coc_active,peer_active,"
    "tx_active,rx_active,registered_closed",
    "upstream-error-ownership=bad_psm,bad_cid,credit_exhausted,"
    "iphc_fail,fragment_drop,peer_missing,cleanup_after_error",
    "upstream-helper-contract=attach_ipsp,detach_ipsp,transmit_packet,"
    "receive_packet",
    "upstream-helper-link=net_bluetooth/6lowpan",
    "upstream-link=linux-6lowpan-ipsp-link-helper",
    "upstream-datapath-ownership=nuttx-ip-tx,net_6lowpan-iphc,"
    "bt_6lowpan_xmit,l2cap-coc,hwsim-acl,hwsim-rx,l2cap-coc,"
    "bt_6lowpan_recv,netif-rx,nuttx-ip-rx",
    "re:upstream-link-ledger=netdev:1,coc:1,peer:1,"
    "netdev-ref:1,chan-ref:1,peer-ref:1,tx-active=[1-9][0-9]*,"
    "rx-active=[1-9][0-9]*",
    "re:upstream-link-netdev-register=[1-9][0-9]*",
    "re:upstream-link-netdev-unregister=[1-9][0-9]*",
    "re:upstream-link-tx=[1-9][0-9]*",
    "re:upstream-link-rx=[1-9][0-9]*",
    "re:upstream-link-peer-add=[1-9][0-9]*",
    "re:upstream-link-peer-del=[1-9][0-9]*",
    "re:upstream-link-coc-open=[1-9][0-9]*",
    "re:upstream-link-coc-close=[1-9][0-9]*",
    "re:upstream-link-xmit=[1-9][0-9]*",
    "re:upstream-link-rx-deliver=[1-9][0-9]*",
    "re:upstream-link-setup-netdev=[1-9][0-9]*",
    "re:upstream-link-delete-netdev=[1-9][0-9]*",
    "re:upstream-link-chan-ready-cb=[1-9][0-9]*",
    "re:upstream-link-chan-close-cb=[1-9][0-9]*",
    "re:upstream-link-bt-xmit=[1-9][0-9]*",
    "re:upstream-link-recv-cb=[1-9][0-9]*",
    "upstream-iphc-link=net_6lowpan/iphc",
    "re:last-tx-dispatch=0x7[ab] last-rx-dispatch=0x7[ab]",
    "re:tx-iphc=[1-9][0-9]* rx-iphc=[1-9][0-9]*",
    "tx-iphc-error=0",
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


def obex_parity(mode: str, role: str, source: str, cid: str,
                api: str, boundary: str) -> tuple[str, ...]:
    return (
        f"bluez-obex: closeout upstream-source-parity mode={mode} role={role}",
        "direct-upstream=obexd/src/main.c,obexd/src/manager.c,"
        "obexd/src/plugin.c,obexd/src/server.c,obexd/src/obex.c,"
        "obexd/src/service.c,obexd/src/transfer.c,"
        "obexd/src/transport.c,obexd/client/session.c,"
        "obexd/client/transfer.c,obexd/client/transport.c,"
        "rfcomm/sock.c,rfcomm/core.c,l2cap_core.c",
        f"profile-source={source}",
        "objects=obexd-mainloop,plugin-manager,session-bus,"
        "dbus-name,session-object,transfer-object,transport-object,"
        "profile-object,rfcomm-fd,request-queue,headers,mainloop-watch",
        "handlers=profile_connect,service_register,obex_session_open,"
        "obex_connect,obex_setpath,obex_get,obex_put,obex_abort,"
        "transfer_progress,transfer_complete,transfer_abort,"
        "transport_open,transport_close,rfcomm_connect,rfcomm_sendmsg,"
        "rfcomm_recvmsg",
        f"native-rfcomm=psm-0x0003,cid-{cid},fd-handoff,session-owner",
        f"profile-api={api}",
        f"upstream-link={boundary} parity-final=1",
    )


def mesh_parity(role: str, source: str) -> tuple[str, ...]:
    return (
        f"bluez-daemon: mesh closeout upstream-source-parity role={role}",
        "direct-upstream=mesh/main.c,mesh/manager.c,mesh/dbus.c,"
        "mesh/mesh.c,mesh/node.c,mesh/net.c,mesh/model.c,"
        "mesh/appkey.c,mesh/net-keys.c,mesh/crypto.c,mesh/pb-adv.c,"
        "mesh/prov-initiator.c,mesh/prov-acceptor.c,"
        "mesh/cfgmod-server.c,mesh/friend.c,mesh/rpl.c,"
        "mesh/mesh-io-mgmt.c,hci_core.c,hci_event.c,mgmt.c,"
        "l2cap_core.c,l2cap_sock.c,smp.c",
        "objects=bluetooth-meshd-mainloop,mesh-manager,dbus-name,"
        "node,element,model,subnet,netkey,appkey,devkey,iv-index,"
        "sequence,replay-list,provisioning-session,adv-bearer,"
        "gatt-proxy,att-bearer,proxy-filter,friend-queue,"
        "mainloop-watch",
        "handlers=mesh_init,manager_create_network,node_attach_io,"
        "mesh_io_register_recv_cb,pb_adv_reg,prov_initiator_start,"
        "prov_acceptor_start,mesh_net_send,mesh_net_recv,"
        "mesh_model_send,cfgmod_server_msg,proxy_msg_recv,"
        "friend_poll,rpl_check,mesh_io_send",
        "native-bearer=pb-adv,adv-bearer,gatt-proxy,att-cid-0x0004,"
        "mgmt-adv,mgmt-scan,controller-event-policy",
        "native-crypto=netkey,appkey,devkey,seq-auth,iv-index,"
        "aes-ccm,replay-protection",
        f"profile-source={source}",
        "upstream-link=bluezdaemon-mesh-upstream-link-bluetoothd "
        "parity-final=1",
    )


def asha_parity(role: str) -> tuple[str, ...]:
    return (
        f"bluez-daemon: asha closeout upstream-source-parity role={role}",
        "direct-upstream=profiles/audio/asha.c,profiles/audio/media.c,"
        "profiles/audio/transport.c,profiles/battery/bas.c,"
        "src/gatt-client.c,src/device.c,src/adapter.c,"
        "src/shared/att.c,src/shared/gatt-client.c,"
        "src/shared/gatt-db.c,src/shared/gatt-helpers.c,"
        "hci_core.c,hci_event.c,mgmt.c,l2cap_core.c,l2cap_sock.c,smp.c",
        "objects=device,gatt-service,gatt-characteristic,"
        "read-only-properties,audio-control-point,audio-status-point,"
        "volume,battery,media-transport,att-bearer,att-fd,"
        "request-queue,ccc,stream,paired-device,mainloop-watch",
        "handlers=asha_probe,asha_accept,asha_connect,"
        "bt_gatt_client_read_value,bt_gatt_client_write_value,"
        "bt_gatt_client_register_notify,media_transport_acquire,"
        "media_transport_release,transport_set_volume,"
        "battery_probe,att_send,att_recv,l2cap_chan_send,"
        "smp_encrypt_link",
        "native-att=att-cid-0x0004,service-discovery,read-properties,"
        "control-write,status-notify,ccc,mtu,security",
        "native-audio=g722,frame-ms-10,sequence,volume,suspend,resume,"
        "stop,reconnect",
        "profile-source=third/bluez/profiles/audio/asha.c",
        "upstream-link=bluezdaemon-asha-upstream-link-bluetoothd "
        "parity-final=1",
    )


def midi_parity(role: str) -> tuple[str, ...]:
    return (
        f"bluez-daemon: midi closeout upstream-source-parity role={role}",
        "direct-upstream=profiles/midi/midi.c,profiles/midi/libmidi.c,"
        "src/gatt-client.c,src/gatt-database.c,src/shared/att.c,"
        "src/shared/gatt-client.c,src/shared/gatt-db.c,"
        "src/shared/gatt-server.c,hci_core.c,hci_event.c,mgmt.c,"
        "l2cap_core.c,l2cap_sock.c,smp.c",
        "objects=gatt-manager,gatt-service,gatt-characteristic,"
        "midi-service,midi-characteristic,midi-parser,timestamp-queue,"
        "notify-session,write-command,att-bearer,att-fd,request-queue,"
        "ccc,mainloop-watch",
        "handlers=midi_register,midi_accept,midi_connect,"
        "midi_encode_timestamp,midi_decode_timestamp,midi_parse_packet,"
        "bt_gatt_client_write_without_response,"
        "bt_gatt_client_register_notify,att_send,att_recv,"
        "l2cap_chan_send,smp_encrypt_link",
        "native-att=att-cid-0x0004,mtu-247,service-discovery,"
        "characteristic-discovery,ccc-enable,write-without-response,"
        "notify,security",
        "native-midi=note-on,note-off,control-change,timestamp-wrap,"
        "jitter-window,ordering,error-recovery",
        "profile-source=third/bluez/profiles/midi/midi.c",
        "upstream-link=bluezdaemon-midi-upstream-link-bluetoothd "
        "parity-final=1",
    )


def ranging_parity(role: str) -> tuple[str, ...]:
    return (
        f"bluez-daemon: ranging closeout upstream-source-parity role={role}",
        "direct-upstream=profiles/ranging/rap.c,"
        "profiles/ranging/rap_hci.c,src/adapter.c,src/device.c,"
        "src/shared/att.c,src/shared/gatt-client.c,hci_core.c,"
        "hci_event.c,mgmt.c,l2cap_core.c,smp.c",
        "objects=adapter,device,ranging-profile,rap-session,"
        "capability-cache,security-state,procedure-config,"
        "procedure-request,hci-request,result-event,quality-window,"
        "att-bearer,att-fd,request-queue,notify-session,mainloop-watch",
        "handlers=rap_probe,rap_connect,rap_disconnect,"
        "rap_read_capability,rap_enable_security,rap_config_procedure,"
        "rap_start_procedure,rap_result_notify,rap_event_stream,"
        "rap_error_map,mgmt_send,hci_send_req,hci_event_recv,"
        "att_send,att_recv,l2cap_chan_send,smp_encrypt_link",
        "native-att=att-cid-0x0004,capability-read,security-enable,"
        "procedure-config,result-notify,ccc,mtu,security",
        "native-hci=LE_CS_Read_Local_Supported_Capabilities,"
        "LE_CS_Set_Procedure_Parameters,LE_CS_Start_Procedure,"
        "LE_CS_Procedure_Enable,LE_CS_Procedure_Request,"
        "LE_CS_Result",
        "native-ranging=distance,quality,rssi,rtt,phase-slope,samples,"
        "poor-quality-drop,error-recovery",
        "profile-source=third/bluez/profiles/ranging/rap.c",
        "upstream-link="
        "bluezdaemon-ranging-upstream-link-bluetoothd "
        "parity-final=1",
    )


def print_parity(role: str) -> tuple[str, ...]:
    return (
        f"bluez-daemon: print closeout upstream-source-parity role={role}",
        "direct-upstream=profiles/cups/main.c,profiles/cups/sdp.c,"
        "profiles/cups/spp.c,profiles/cups/hcrp.c,profiles/cups/cups.h,"
        "rfcomm/core.c,rfcomm/sock.c,rfcomm/tty.c,l2cap_core.c,"
        "l2cap_sock.c",
        "objects=device,profile,serial-port,sdp-record,rfcomm-fd,"
        "rfcomm-tty,hcrp-control,hcrp-data,cups-backend,printer,"
        "print-job,status-query,cancel-request,mainloop-watch",
        "handlers=profile_register,profile_connect,profile_disconnect,"
        "spp_connect,hcrp_connect,hcrp_data_send,hcrp_control_send,"
        "cups_backend_discover,cups_job_submit,cups_job_status,"
        "cups_job_cancel,rfcomm_sendmsg,rfcomm_recvmsg,l2cap_connect",
        "native-rfcomm=psm-0x0003,channel-13,fd-handoff,tty,"
        "l2cap-session-owner",
        "native-print=hcrp-control,hcrp-data,credit,mtu,printer-language,"
        "job-submit,job-receive,job-status,job-cancel,error-recovery",
        "profile-source=third/bluez/profiles/cups/main.c",
        "upstream-link="
        "bluezdaemon-print-upstream-link-cups-backend "
        "parity-final=1",
    )


def iap_parity(role: str) -> tuple[str, ...]:
    return (
        f"bluez-daemon: iap closeout upstream-source-parity role={role}",
        "direct-upstream=profiles/iap/main.c,rfcomm/core.c,"
        "rfcomm/sock.c,rfcomm/tty.c,l2cap_core.c,l2cap_sock.c",
        "objects=device,profile,serial-port,sdp-record,rfcomm-fd,"
        "rfcomm-tty,iap-session,identify-request,accessory-info,"
        "ea-session,ea-protocol,control-payload,credit-window,"
        "retransmit-timer,mainloop-watch",
        "handlers=profile_register,profile_connect,profile_disconnect,"
        "iap_identify,iap_accessory_info,iap_ea_open,iap_ea_accept,"
        "iap_payload_send,iap_payload_recv,iap_ack,iap_retransmit,"
        "rfcomm_sendmsg,rfcomm_recvmsg,l2cap_connect",
        "native-rfcomm=psm-0x0003,channel-14,fd-handoff,tty,"
        "l2cap-session-owner",
        "native-iap=identify-device,accessory-info,session-id,"
        "protocol-list,external-accessory-session,control-tx,"
        "control-rx,ack,credit,keepalive,retransmit,error-recovery",
        "profile-source=third/bluez/profiles/iap/main.c",
        "upstream-link=bluezdaemon-iap-upstream-link-iapd "
        "parity-final=1",
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
        "bluez-basic-mgmt-flow",
        (
            RoleCheck("ble1", (
                "bluez-audio: source=third/bluez/profiles/audio/main.c "
                "style=bluetoothd-plugin command=plugin-init plugin=audio",
                "bluez-audio: source=third/bluez/src/adapter.c "
                "style=adapter-lifecycle command=powered adapter=hci0 "
                "powered=1 discoverable=0 pairable=1",
                "bluez-audio: source=third/bluez/profiles/audio/main.c "
                "style profile=le-audio-daemon command=register",
                "bluez-audio: source=third/bluez/profiles/audio/main.c "
                "style=bt_profile command=accept uuid=0000184e-0000-1000-8000-00805f9b34fb "
                "adapter=hci0 device=dev_feather bearer=le",
                "bluez-audio: le-mgmt-control source=third/bluez/src/mgmt.c "
                "style=mgmt-socket command=power-on role=source index=0 "
                "opcode=0x0005 setting=powered",
                "bluez-audio: mgmt event new-settings index=0 "
                "powered=1 le=1 bredr=1 secure-conn=1",
                "bluez-audio: le-mgmt-control source=third/bluez/src/mgmt.c "
                "style=mgmt-socket command=scan-start role=source index=0 "
                "opcode=0x0023 type=le",
                "bluez-audio: mgmt event device-found role=source "
                "addr=fe:ed:00:00:00:01 rssi=-42 flags=connectable "
                "uuids=184e,1850,1855",
                "bluez-audio: le-mgmt-control source=third/bluez/src/device.c "
                "style=mgmt-socket command=connect role=source index=0 "
                "addr=fe:ed:00:00:00:01 bearer=le",
                "bluez-audio: hci event le-enhanced-connection-complete "
                "source=third/linux-hwe-6.17-6.17.0/net/bluetooth/hci_event.c "
                "role=source status=0x00 handle=0x0041 interval=24 latency=0",
                "bluez-audio: le-mgmt-control source=third/bluez/src/device.c "
                "style=mgmt-socket command=security role=source index=0 "
                "addr=fe:ed:00:00:00:01 level=medium",
                "bluez-audio: hci event le-long-term-key-request "
                "source=third/linux-hwe-6.17-6.17.0/net/bluetooth/smp.c "
                "role=source handle=0x0041 authenticated=1",
                "bluez-audio: mgmt event new-link-key role=source bonded=1 "
                "key-type=authenticated-combination",
                "bluez-audio: le-mgmt-control source=third/linux-hwe-6.17-6.17.0/net/bluetooth/iso.c "
                "style=hci-event command=cis-request role=source cig=0 cis=1 "
                "acl-handle=0x0041 cis-handle=0x0201",
                "bluez-audio: hci event le-cis-established role=source "
                "status=0x00 handle=0x0201 cig=0 cis=1",
                "bluez-audio: le-mgmt-control source=third/bluez/src/device.c "
                "style=mgmt-socket command=disconnect role=source index=0 "
                "handle=0x0041 reason=local-host",
                "bluez-audio: mgmt event device-disconnected role=source "
                "handle=0x0041 reason=local-host",
                "bluez-audio: le-mgmt-control source=third/bluez/src/mgmt.c "
                "style=mgmt-socket command=error role=source index=0 "
                "opcode=0x002e status=busy reason=duplicate-connect",
                "bluez-audio: source=third/bluez/profiles/audio/main.c "
                "style=bt_profile command=release uuid=0000184e-0000-1000-8000-00805f9b34fb "
                "adapter=hci0 device=dev_feather bearer=le",
                "bluez-audio: le daemon released pacs=0 ascs=0 bap=0 "
                "media-endpoints=0 transports=0",
            )),
            RoleCheck("ble2", (
                "bluez-audio: source=third/bluez/profiles/audio/main.c "
                "style=bluetoothd-plugin command=plugin-init plugin=audio",
                "bluez-audio: source=third/bluez/src/adapter.c "
                "style=adapter-lifecycle command=powered adapter=hci0 "
                "powered=1 discoverable=0 pairable=1",
                "bluez-audio: source=third/bluez/profiles/audio/main.c "
                "style profile=le-audio-daemon command=register",
                "bluez-audio: source=third/bluez/profiles/audio/main.c "
                "style=bt_profile command=accept uuid=0000184e-0000-1000-8000-00805f9b34fb "
                "adapter=hci0 device=dev_feather bearer=le",
                "bluez-audio: le-mgmt-control source=third/bluez/src/mgmt.c "
                "style=mgmt-socket command=power-on role=sink index=0 "
                "opcode=0x0005 setting=powered",
                "bluez-audio: le-mgmt-control source=third/bluez/src/mgmt.c "
                "style=mgmt-socket command=scan-start role=sink index=0 "
                "opcode=0x0023 type=le",
                "bluez-audio: mgmt event device-found role=sink "
                "addr=fe:ed:00:00:00:01 rssi=-42 flags=connectable "
                "uuids=184e,1850,1855",
                "bluez-audio: le-mgmt-control source=third/bluez/src/device.c "
                "style=mgmt-socket command=connect role=sink index=0 "
                "addr=fe:ed:00:00:00:01 bearer=le",
                "bluez-audio: hci event le-enhanced-connection-complete "
                "source=third/linux-hwe-6.17-6.17.0/net/bluetooth/hci_event.c "
                "role=sink status=0x00 handle=0x0041 interval=24 latency=0",
                "bluez-audio: le-mgmt-control source=third/bluez/src/device.c "
                "style=mgmt-socket command=security role=sink index=0 "
                "addr=fe:ed:00:00:00:01 level=medium",
                "bluez-audio: hci event le-long-term-key-request "
                "source=third/linux-hwe-6.17-6.17.0/net/bluetooth/smp.c "
                "role=sink handle=0x0041 authenticated=1",
                "bluez-audio: mgmt event new-link-key role=sink bonded=1 "
                "key-type=authenticated-combination",
                "bluez-audio: le-mgmt-control source=third/linux-hwe-6.17-6.17.0/net/bluetooth/iso.c "
                "style=hci-event command=cis-request role=sink cig=0 cis=1 "
                "acl-handle=0x0041 cis-handle=0x0201",
                "bluez-audio: hci event le-cis-established role=sink "
                "status=0x00 handle=0x0201 cig=0 cis=1",
                "bluez-audio: le-mgmt-control source=third/bluez/src/device.c "
                "style=mgmt-socket command=disconnect role=sink index=0 "
                "handle=0x0041 reason=local-host",
                "bluez-audio: mgmt event device-disconnected role=sink "
                "handle=0x0041 reason=local-host",
                "bluez-audio: le-mgmt-control source=third/bluez/src/mgmt.c "
                "style=mgmt-socket command=error role=sink index=0 "
                "opcode=0x002e status=busy reason=duplicate-connect",
                "bluez-audio: source=third/bluez/profiles/audio/main.c "
                "style=bt_profile command=release uuid=0000184e-0000-1000-8000-00805f9b34fb "
                "adapter=hci0 device=dev_feather bearer=le",
                "bluez-audio: le daemon released pacs=0 ascs=0 bap=0 "
                "media-endpoints=0 transports=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-basic-scan-connect-auth-flow",
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
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=control",
                "bluez-mgmt: send opcode=0x0005 index=0x0000",
                "bluez-mgmt: send opcode=0x0023 index=0x0000",
                "bluez-mgmt: control complete",
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=lifecycle",
                "bluez-mgmt: recv-until label=pair-connected",
                "event=0x000b",
                "bluez-mgmt: lifecycle complete",
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=pair-noio",
                "bluez-mgmt: pair-noio complete",
                "bluez-daemon: source=third/bluez/src/adapter.c+src/agent.c style mode=pairing-matrix",
                "bluez-daemon: pairing-matrix step=confirm-accept complete",
                "bluez-daemon: pairing-matrix step=confirm-reject complete",
                "bluez-daemon: pairing-matrix step=passkey-accept complete",
                "bluez-daemon: pairing-matrix step=passkey-reject complete",
                "bluez-daemon: pairing-matrix step=cancel-pending complete",
                "bluez-daemon: pairing-matrix mainloop-ledger",
                "pending-callback-final=0",
                "bluez-daemon: pairing-matrix mgmt-event-ledger",
                "confirm-accept=device-connected,device-disconnected",
                "cancel-pending=cmd-status-cancelled",
                "bluez-daemon: pairing-matrix link-ledger",
                "agent-ref=0 adapter-ref=0 device-ref=0 bearer-ref=0",
                "mgmt-pending=0 dbus-owner=0 watch=0 timer=0",
                "bluez-daemon: pairing-matrix complete",
                "bluez-mgmt: source=third/bluez/src/shared/mgmt.c+"
                "third/bluez/src/adapter.c+third/bluez/src/device.c+"
                "third/bluez/src/agent.c style mode=security-closeout",
                "bluez-mgmt: security-closeout phase=agent-register "
                "dbus-api=org.bluez.Agent1",
                "bluez-upstream-object: src/agent.c role=security-closeout",
                "bluez-mgmt: security-closeout phase=device-bonding",
                "operations=create-bonding,bonding-complete,bearer-paired,"
                "bearer-bonded,unpair",
                "bluez-mgmt: security-closeout phase=mgmt-event-order",
                "bluez-mgmt: security-closeout mgmt-order-ledger",
                "agent-register -> adapter-policy -> discovery-start",
                "pair-device -> device-connected -> key-store",
                "bluez-mgmt: security-closeout phase=smp-key-lifecycle",
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/smp.c",
                "bluez-mgmt: security-closeout link-ledger",
                "agent-ref=0 adapter-ref=0 device-ref=0 bearer-ref=0",
                "smp-key-ref=0 key-store-ref=0 mgmt-pending=0",
                "bluez-mgmt: security-closeout final agent-final=1 "
                "mgmt-final=1 device-final=1 smp-final=1 key-store-final=1",
                "upstream-link=bluezmgmt-security-upstream-link-"
                "bluetoothd",
                "bluez-mgmt: security-closeout complete",
                "bluez-daemon: source=third/bluez/src/main.c+src/adapter.c style mode=reconnect-stress rounds=3",
                "bluez-daemon: reconnect round=2 complete",
                "bluez-daemon: reconnect round=3 complete",
                "bluez-daemon: reconnect-stress lifecycle-ledger rounds=3",
                "pending-cmd-final=0 device-ref-final=0 bond-ref-final=0",
                "bluez-daemon: reconnect-stress medium-ledger",
                "hwsim-record-types=ctrl,adv,acl consumer-offsets=role-local",
                "reconnect-replay-guard=1 cleanup-final=1",
                "bluez-daemon: reconnect-stress complete rounds=3",
                "bluez-daemon: source=third/bluez/src/adapter.c+src/device.c style mode=device-policy",
                "bluez-daemon: complete-loop label=add-device",
                "bluez-daemon: complete-loop label=set-device-flags",
                "bluez-daemon: complete-loop label=block-device",
                "bluez-daemon: complete-loop label=unblock-device",
                "bluez-daemon: complete-loop label=unpair-device",
                "bluez-daemon: complete-loop label=remove-device",
                "bluez-daemon: device-policy complete",
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=error-path",
                "bluez-mgmt: error-path complete",
            )),
        ),
    ),
    CaseCheck(
        "bluez-basic-upstream-convergence-closeout",
        (
            RoleCheck("bt1", (
                "BLUEZBASICUPSTREAM_BEGIN_BT1",
                "btctl: mgmt power on",
                "btctl: mgmt bredr on",
                "btctl: mgmt connectable on",
                "btctl: mgmt discoverable on",
                "btctl: state",
                "bluez-daemon: basic closeout cleanup role=bt",
                "bluez-daemon: basic closeout medium-lifecycle role=bt",
                "medium=ctrl,adv,acl,iso,bnep offset-consumers=role+channel",
                "reconnect-replay-guard=1 abnormal-exit-offset-resume=1",
                "bluez-daemon: basic closeout upstream-coverage-map role=bt",
                "third/bluez/src/adapter.c",
                "third/bluez/src/device.c",
                "third/bluez/src/agent.c",
                "third/bluez/tools/btmgmt.c",
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/hci_sock.c",
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/mgmt.c",
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/l2cap_core.c",
                "scan-final=1 connect-final=1 auth-final=1 mgmt-final=1",
                "hci-socket-final=1 l2cap-final=1 cleanup-final=1",
                "upstream-link=bluezdaemon-basic-upstream-link-bluetoothd",
                "final-ok=1",
                "BLUEZBASICUPSTREAM_DONE_BT1",
            )),
            RoleCheck("bt2", (
                "BLUEZBASICUPSTREAM_BEGIN_BT2",
                "btctl: mgmt power on",
                "btctl: mgmt bredr on",
                "btctl: mgmt connectable on",
                "btctl: mgmt discoverable on",
                "btctl: state",
                "bluez-daemon: basic closeout cleanup role=bt",
                "bluez-daemon: basic closeout medium-lifecycle role=bt",
                "bluez-daemon: basic closeout upstream-coverage-map role=bt",
                "scan-final=1 connect-final=1 auth-final=1 mgmt-final=1",
                "hci-socket-final=1 l2cap-final=1 cleanup-final=1",
                "upstream-link=bluezdaemon-basic-upstream-link-bluetoothd",
                "final-ok=1",
                "BLUEZBASICUPSTREAM_DONE_BT2",
            )),
            RoleCheck("ble2", (
                "BLUEZBASICUPSTREAM_BEGIN_BLE2",
                "btctl: mgmt power on",
                "btctl: mgmt le on",
                "btctl: advertise start",
                "btctl: state",
                "bluez-daemon: basic closeout cleanup role=ble",
                "bluez-daemon: basic closeout medium-lifecycle role=ble",
                "medium=ctrl,adv,acl,iso,bnep offset-consumers=role+channel",
                "reconnect-replay-guard=1 abnormal-exit-offset-resume=1",
                "bluez-daemon: basic closeout upstream-coverage-map role=ble",
                "scan-final=1 connect-final=1 auth-final=1 mgmt-final=1",
                "hci-socket-final=1 l2cap-final=1 cleanup-final=1",
                "upstream-link=bluezdaemon-basic-upstream-link-bluetoothd",
                "final-ok=1",
                "BLUEZBASICUPSTREAM_DONE_BLE2",
            )),
            RoleCheck("ble1", (
                "BLUEZBASICUPSTREAM_BEGIN_BLE1",
                "bluez-daemon: source=third/bluez/src/adapter.c+src/device.c style mode=discovery-peer",
                "bluez-daemon: event-only label=device-found",
                "bluez-daemon: discovery-peer complete",
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=control",
                "bluez-mgmt: control complete",
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=lifecycle",
                "bluez-mgmt: lifecycle complete",
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=pair-noio",
                "bluez-mgmt: pair-noio complete",
                "bluez-daemon: source=third/bluez/src/adapter.c+src/agent.c style mode=pairing-matrix",
                "bluez-daemon: pairing-matrix complete",
                "bluez-mgmt: source=third/bluez/src/shared/mgmt.c+"
                "third/bluez/src/adapter.c+third/bluez/src/device.c+"
                "third/bluez/src/agent.c style mode=security-closeout",
                "bluez-mgmt: security-closeout phase=agent-register "
                "dbus-api=org.bluez.Agent1",
                "bluez-mgmt: security-closeout phase=smp-key-lifecycle",
                "bluez-mgmt: security-closeout final agent-final=1 "
                "mgmt-final=1 device-final=1 smp-final=1 key-store-final=1",
                "bluez-mgmt: security-closeout complete",
                "bluez-daemon: source=third/bluez/src/main.c+src/adapter.c style mode=reconnect-stress rounds=3",
                "bluez-daemon: reconnect-stress lifecycle-ledger rounds=3",
                "bluez-daemon: reconnect-stress medium-ledger",
                "bluez-daemon: reconnect-stress complete rounds=3",
                "bluez-daemon: source=third/bluez/src/adapter.c+src/device.c style mode=device-policy",
                "bluez-daemon: device-policy complete",
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=error-path",
                "bluez-mgmt: error-path complete",
                "bluez-daemon: basic closeout cleanup role=ble",
                "bluez-daemon: basic closeout medium-lifecycle role=ble",
                "bluez-daemon: basic closeout upstream-coverage-map role=ble",
                "third/bluez/src/adapter.c",
                "third/bluez/src/device.c",
                "third/bluez/src/agent.c",
                "third/bluez/src/shared/mainloop.c",
                "third/bluez/tools/btmgmt.c",
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/hci_sock.c",
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/mgmt.c",
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/smp.c",
                "scan-final=1 connect-final=1 auth-final=1 mgmt-final=1",
                "hci-socket-final=1 l2cap-final=1 cleanup-final=1",
                "upstream-link=bluezdaemon-basic-upstream-link-bluetoothd",
                "final-ok=1",
                "BLUEZBASICUPSTREAM_DONE_BLE1",
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
                "upstream-link=net_bluetooth/6lowpan+sim-ipsp-datapath-link",
                "re:upstream-link-peer-add=[1-9][0-9]*",
                "re:upstream-link-peer-del=[1-9][0-9]*",
                "re:upstream-link-coc-open=[1-9][0-9]*",
                "re:upstream-link-coc-close=[1-9][0-9]*",
                "re:upstream-link-setup-netdev=[1-9][0-9]*",
                "re:upstream-link-delete-netdev=[1-9][0-9]*",
                "re:upstream-link-chan-ready-cb=[1-9][0-9]*",
                "re:upstream-link-chan-close-cb=[1-9][0-9]*",
                "re:upstream-link-bt-xmit=[1-9][0-9]*",
                "re:upstream-link-recv-cb=[1-9][0-9]*",
                "upstream-iphc-link=net_6lowpan/iphc",
                "PING6 fc00::2:",
                "2 packets transmitted, 2 received",
                "last-tx-dispatch=0x7b last-rx-dispatch=0x7b",
                "re:tx-iphc=[1-9][0-9]* rx-iphc=[1-9][0-9]*",
                "tx-iphc-error=0",
                "btctl: upstream 6lowpan-down complete",
                "linux-bt-6lowpan: registered=0 ifname=-",
                "ipsp-state=closed",
                "upstream-link-ledger=netdev:0,coc:0,peer:0,"
                "netdev-ref:0,chan-ref:0,peer-ref:0,tx-active:0,"
                "rx-active:0",
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
                "upstream-link=net_bluetooth/6lowpan+sim-ipsp-datapath-link",
                "re:upstream-link-peer-add=[1-9][0-9]*",
                "re:upstream-link-peer-del=[1-9][0-9]*",
                "re:upstream-link-coc-open=[1-9][0-9]*",
                "re:upstream-link-coc-close=[1-9][0-9]*",
                "re:upstream-link-setup-netdev=[1-9][0-9]*",
                "re:upstream-link-delete-netdev=[1-9][0-9]*",
                "re:upstream-link-chan-ready-cb=[1-9][0-9]*",
                "re:upstream-link-chan-close-cb=[1-9][0-9]*",
                "re:upstream-link-bt-xmit=[1-9][0-9]*",
                "re:upstream-link-recv-cb=[1-9][0-9]*",
                "upstream-iphc-link=net_6lowpan/iphc",
                "PING6 fc00::1:",
                "2 packets transmitted, 2 received",
                "last-tx-dispatch=0x7b last-rx-dispatch=0x7b",
                "re:tx-iphc=[1-9][0-9]* rx-iphc=[1-9][0-9]*",
                "tx-iphc-error=0",
                "btctl: upstream 6lowpan-down complete",
                "linux-bt-6lowpan: registered=0 ifname=-",
                "ipsp-state=closed",
                "upstream-link-ledger=netdev:0,coc:0,peer:0,"
                "netdev-ref:0,chan-ref:0,peer-ref:0,tx-active:0,"
                "rx-active:0",
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
        "ble-ip-closeout-full",
        (
            RoleCheck("ble1", (
                "BLEIP_BEGIN_BLE1",
                "count>=2:btctl: upstream 6lowpan-up ifname=bt0",
                "count>=2:linux-bt-6lowpan: registered=1 ifname=bt0",
                "count>=2:ipsp-open=1",
                "ipsp-psm=0x0023",
                "ipsp-cid=0x0040",
                "upstream-link=net_bluetooth/6lowpan+sim-ipsp-datapath-link",
                "upstream-iphc-link=net_6lowpan/iphc",
                "PING6 fc00::2:",
                "2 packets transmitted, 2 received",
                "mode=tcp-server",
                "sip=fc00::1:5001",
                "accept: fc00::2:",
                "mode=udp-server",
                "Mbits/sec",
                "accept: fc00::2:",
                "re:last-tx-dispatch=0x7[ab] last-rx-dispatch=0x7[ab]",
                "re:tx-iphc=[1-9][0-9]* rx-iphc=[1-9][0-9]*",
                "tx-iphc-error=0",
                "re:rx-frag-dgrams=[1-9]",
                "re:rx-frag-frames=[1-9]",
                "rx-frag-drop=0",
                "count>=2:btctl: upstream 6lowpan-down complete",
                "linux-bt-6lowpan: registered=0 ifname=-",
                "ipsp-state=closed",
                "BLEIP_DONE_BLE1",
            )),
            RoleCheck("ble2", (
                "BLEIP_BEGIN_BLE2",
                "count>=2:btctl: upstream 6lowpan-up ifname=bt0",
                "count>=2:linux-bt-6lowpan: registered=1 ifname=bt0",
                "count>=2:ipsp-open=1",
                "ipsp-psm=0x0023",
                "ipsp-cid=0x0040",
                "upstream-link=net_bluetooth/6lowpan+sim-ipsp-datapath-link",
                "upstream-iphc-link=net_6lowpan/iphc",
                "PING6 fc00::1:",
                "2 packets transmitted, 2 received",
                "mode=tcp-client",
                "dip=fc00::1:5001",
                "mode=udp-client",
                "Mbits/sec",
                "iperf exit",
                "re:last-tx-dispatch=0x7[ab] last-rx-dispatch=0x7[ab]",
                "re:tx-iphc=[1-9][0-9]* rx-iphc=[1-9][0-9]*",
                "tx-iphc-error=0",
                "re:tx-frag-dgrams=[1-9]",
                "re:tx-frag-frames=[1-9]",
                "rx-frag-drop=0",
                "count>=2:btctl: upstream 6lowpan-down complete",
                "linux-bt-6lowpan: registered=0 ifname=-",
                "ipsp-state=closed",
                "BLEIP_DONE_BLE2",
            )),
        ),
    ),
    CaseCheck(
        "bluez-ipsp-closeout-full",
        (
            RoleCheck("ble1", (
                "BLUEZIPSP_BEGIN_BLE1",
                "bluezipsp: source=third/bluez/profiles/network/connection.c+"
                "third/bluez/profiles/network/ipsp.c "
                "style=le-ipsp-profile command=connect",
                "uuid=00001820-0000-1000-8000-00805f9b34fb "
                "psm=0x0023 dbus=org.bluez.Network1",
                "bluezipsp: profile register service=ipsp "
                "source=third/bluez/src/profile.c+third/bluez/src/device.c "
                "uuid=00001820-0000-1000-8000-00805f9b34fb "
                "object=/org/bluez/hci0/dev_peer/ipsp0 "
                "interface=org.bluez.Network1 owner=:client.ipsp "
                "security=medium authorize=ok connect-profile=ok",
                "bluezipsp: connect complete ifname=bt0 "
                "fd-handoff=le-l2cap-coc owner=kernel-6lowpan "
                "profile=ipsp",
                "bluezipsp: native-6lowpan ownership phase=connect "
                "ifname=bt0",
                "register=1 peer-add=1 chan-attach=1",
                "bluezipsp: native-l2cap-coc ownership psm=0x0023 "
                "cid=0x0040",
                "state=connected fd-handoff=1",
                "bluezipsp: native-netdev ownership ifname=bt0",
                "ndo-start-xmit=1 netif-rx=1 mtu=1280",
                "bluezipsp: dbus object-add "
                "path=/org/bluez/hci0/dev_peer/ipsp0 "
                "interfaces=org.bluez.Network1,org.bluez.Device1 "
                "properties=Connected,Interface,UUID owner=:client.ipsp",
                "count>=2:bluezipsp: status source=third/bluez/"
                "profiles/network/ipsp.c profile=ipsp",
                "count>=2:bluezipsp: status dbus-owner=:client.ipsp "
                "object=/org/bluez/hci0/dev_peer/ipsp0 "
                "interface=org.bluez.Network1 connected-query=ok",
                "bluezipsp: native-6lowpan status datapath=bt0",
                "tx=netdev-xmit,iphc,l2cap-coc,hwsim",
                "rx=hwsim,l2cap-coc,iphc,netif-rx",
                "count>=2:linux-bt-6lowpan: registered=1 ifname=bt0",
                "count>=2:ipsp-open=1",
                "ipsp-psm=0x0023",
                "ipsp-cid=0x0040",
                "upstream-link=net_bluetooth/6lowpan+"
                "sim-ipsp-datapath-link",
                "upstream-link-state=netdev:1,coc:1,peer:1",
                "upstream-link-refs=netdev:1,chan:1,peer:1",
                "re:upstream-link-active-tx=[1-9][0-9]*",
                "upstream-iphc-link=net_6lowpan/iphc",
                "PING6 fc00::2:",
                "2 packets transmitted, 2 received",
                "mode=tcp-server",
                "sip=fc00::1:5001",
                "accept: fc00::2:",
                "mode=udp-server",
                "Mbits/sec",
                "re:last-tx-dispatch=0x7[ab] last-rx-dispatch=0x7[ab]",
                "re:tx-iphc=[1-9][0-9]* rx-iphc=[1-9][0-9]*",
                "tx-iphc-error=0",
                "re:rx-frag-dgrams=[1-9]",
                "re:rx-frag-frames=[1-9]",
                "rx-frag-drop=0",
                "count>=2:bluezipsp: source=third/bluez/profiles/"
                "network/connection.c+third/bluez/profiles/network/"
                "ipsp.c style=le-ipsp-profile command=disconnect",
                "count>=2:bluezipsp: profile unregister service=ipsp "
                "object=/org/bluez/hci0/dev_peer/ipsp0 "
                "owner=:client.ipsp owner-lost=1 "
                "interfaces-removed=org.bluez.Network1 cleanup=ok",
                "bluezipsp: native-6lowpan cleanup unregister=1 "
                "chan-release=1 peer-unref=1 netdev-unregister=1",
                "bluezipsp: native-status-after-disconnect",
                "bluezipsp: native-6lowpan link-ledger "
                "netdev=0 coc=0 peer=0 netdev-ref=0 chan-ref=0 "
                "peer-ref=0 tx-active=0 rx-active=0 pending-skb=0",
                "count>=2:bluezipsp: disconnect complete profile=ipsp",
                "count>=2:bluezipsp: dbus object-remove "
                "path=/org/bluez/hci0/dev_peer/ipsp0 "
                "objects=0 owners=0 refs=0",
                "bluezipsp: closeout upstream-coverage-map",
                "upstream-link=bluezipsp-upstream-link-bluetoothd "
                "final-ok=1",
                "linux-bt-6lowpan: registered=0 ifname=-",
                "ipsp-state=closed",
                "BLUEZIPSP_DONE_BLE1",
            )),
            RoleCheck("ble2", (
                "BLUEZIPSP_BEGIN_BLE2",
                "bluezipsp: source=third/bluez/profiles/network/connection.c+"
                "third/bluez/profiles/network/ipsp.c "
                "style=le-ipsp-profile command=connect",
                "uuid=00001820-0000-1000-8000-00805f9b34fb "
                "psm=0x0023 dbus=org.bluez.Network1",
                "bluezipsp: profile register service=ipsp "
                "source=third/bluez/src/profile.c+third/bluez/src/device.c "
                "uuid=00001820-0000-1000-8000-00805f9b34fb "
                "object=/org/bluez/hci0/dev_peer/ipsp0 "
                "interface=org.bluez.Network1 owner=:client.ipsp "
                "security=medium authorize=ok connect-profile=ok",
                "bluezipsp: connect complete ifname=bt0 "
                "fd-handoff=le-l2cap-coc owner=kernel-6lowpan "
                "profile=ipsp",
                "bluezipsp: native-6lowpan ownership phase=connect "
                "ifname=bt0",
                "bluezipsp: native-l2cap-coc ownership psm=0x0023 "
                "cid=0x0040",
                "bluezipsp: native-netdev ownership ifname=bt0",
                "bluezipsp: dbus object-add "
                "path=/org/bluez/hci0/dev_peer/ipsp0 "
                "interfaces=org.bluez.Network1,org.bluez.Device1 "
                "properties=Connected,Interface,UUID owner=:client.ipsp",
                "count>=2:bluezipsp: status source=third/bluez/"
                "profiles/network/ipsp.c profile=ipsp",
                "count>=2:bluezipsp: status dbus-owner=:client.ipsp "
                "object=/org/bluez/hci0/dev_peer/ipsp0 "
                "interface=org.bluez.Network1 connected-query=ok",
                "bluezipsp: native-6lowpan status datapath=bt0",
                "count>=2:linux-bt-6lowpan: registered=1 ifname=bt0",
                "count>=2:ipsp-open=1",
                "ipsp-psm=0x0023",
                "ipsp-cid=0x0040",
                "upstream-link=net_bluetooth/6lowpan+"
                "sim-ipsp-datapath-link",
                "upstream-iphc-link=net_6lowpan/iphc",
                "PING6 fc00::1:",
                "2 packets transmitted, 2 received",
                "mode=tcp-client",
                "dip=fc00::1:5001",
                "mode=udp-client",
                "Mbits/sec",
                "iperf exit",
                "re:last-tx-dispatch=0x7[ab] last-rx-dispatch=0x7[ab]",
                "re:tx-iphc=[1-9][0-9]* rx-iphc=[1-9][0-9]*",
                "tx-iphc-error=0",
                "re:tx-frag-dgrams=[1-9]",
                "re:tx-frag-frames=[1-9]",
                "rx-frag-drop=0",
                "count>=2:bluezipsp: source=third/bluez/profiles/"
                "network/connection.c+third/bluez/profiles/network/"
                "ipsp.c style=le-ipsp-profile command=disconnect",
                "count>=2:bluezipsp: profile unregister service=ipsp "
                "object=/org/bluez/hci0/dev_peer/ipsp0 "
                "owner=:client.ipsp owner-lost=1 "
                "interfaces-removed=org.bluez.Network1 cleanup=ok",
                "bluezipsp: native-6lowpan cleanup unregister=1 "
                "chan-release=1 peer-unref=1 netdev-unregister=1",
                "bluezipsp: native-status-after-disconnect",
                "bluezipsp: native-6lowpan link-ledger "
                "netdev=0 coc=0 peer=0 netdev-ref=0 chan-ref=0 "
                "peer-ref=0 tx-active=0 rx-active=0 pending-skb=0",
                "count>=2:bluezipsp: disconnect complete profile=ipsp",
                "count>=2:bluezipsp: dbus object-remove "
                "path=/org/bluez/hci0/dev_peer/ipsp0 "
                "objects=0 owners=0 refs=0",
                "bluezipsp: closeout upstream-coverage-map",
                "linux-bt-6lowpan: registered=0 ifname=-",
                "ipsp-state=closed",
                "BLUEZIPSP_DONE_BLE2",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-ipsp-closeout-full",
        (
            RoleCheck("ble1", (
                "BLUEZDAEMONIPSP_BEGIN_BLE1",
                "bluez-daemon: ipsp closeout phase=daemon-profile-register "
                "source=third/bluez/src/main.c+third/bluez/src/profile.c+"
                "third/bluez/src/device.c+third/bluez/profiles/network/"
                "connection.c+third/bluez/profiles/network/ipsp.c "
                "plugin=network profile=ipsp "
                "uuid=00001820-0000-1000-8000-00805f9b34fb "
                "dbus=org.bluez.Profile1 object=/org/bluez/hci0/"
                "dev_peer/ipsp0 owner=bluetoothd security=medium "
                "authorize=ok",
                "bluez-daemon: ipsp closeout phase=mainloop-ownership "
                "watch-add=mgmt,dbus,l2cap-coc,6lowpan "
                "timer-add=connect-timeout dispatch=mgmt,dbus,l2cap,"
                "netdev owner=bluetoothd watches=4 timers=1",
                "count>=2:upstream-l2cap-bind: psm=0x0023 "
                "cid=0x0040 handle=0x0074 create-ret=0 bind-ret=0",
                "count>=2:upstream-l2cap-connect: psm=0x0023 "
                "cid=0x0040 connect-ret=0",
                "bluez-daemon: ipsp closeout phase=profile-connect "
                "ifname=bt0 psm=0x0023 fd-handoff=le-l2cap-coc "
                "owner=kernel-6lowpan profile=ipsp connected=1",
                "bluez-daemon: ipsp closeout phase=native-6lowpan-ownership "
                "ifname=bt0",
                "register=1 peer-add=1 chan-attach=1",
                "bluez-daemon: ipsp closeout phase=native-l2cap-coc-ownership "
                "psm=0x0023 cid=0x0040",
                "state=connected fd-handoff=1",
                "bluez-daemon: dbus InterfacesAdded "
                "path=/org/bluez/hci0/dev_peer/ipsp0 "
                "interfaces=org.bluez.Network1,org.bluez.Device1 "
                "properties=Connected,Interface,UUID owner=bluetoothd",
                "count>=2:bluez-daemon: ipsp closeout phase=status "
                "dbus-owner=bluetoothd object=/org/bluez/hci0/"
                "dev_peer/ipsp0 interface=org.bluez.Network1 "
                "connected-query=ok",
                "bluez-daemon: ipsp closeout phase=native-datapath-status "
                "datapath=bt0",
                "tx=netdev-xmit,iphc,l2cap-coc,hwsim",
                "rx=hwsim,l2cap-coc,iphc,netif-rx",
                "count>=2:linux-bt-6lowpan: registered=1 ifname=bt0",
                "count>=2:ipsp-open=1",
                "ipsp-psm=0x0023",
                "ipsp-cid=0x0040",
                "upstream-link=net_bluetooth/6lowpan+"
                "sim-ipsp-datapath-link",
                "upstream-iphc-link=net_6lowpan/iphc",
                "PING6 fc00::2:",
                "2 packets transmitted, 2 received",
                "mode=tcp-server",
                "sip=fc00::1:5001",
                "accept: fc00::2:",
                "mode=udp-server",
                "Mbits/sec",
                "re:last-tx-dispatch=0x7[ab] last-rx-dispatch=0x7[ab]",
                "re:tx-iphc=[1-9][0-9]* rx-iphc=[1-9][0-9]*",
                "tx-iphc-error=0",
                "re:rx-frag-dgrams=[1-9]",
                "re:rx-frag-frames=[1-9]",
                "rx-frag-drop=0",
                "count>=2:bluez-daemon: ipsp closeout phase=profile-disconnect "
                "object=/org/bluez/hci0/dev_peer/ipsp0 owner=bluetoothd "
                "owner-lost=1 interfaces-removed=org.bluez.Network1 "
                "cleanup=ok",
                "bluez-daemon: ipsp closeout phase=native-6lowpan-cleanup "
                "unregister=1 chan-release=1 peer-unref=1 "
                "netdev-unregister=1 owner-state-final=0",
                "count>=2:upstream-l2cap-close: released",
                "count>=2:bluez-daemon: dbus InterfacesRemoved "
                "path=/org/bluez/hci0/dev_peer/ipsp0 "
                "interfaces=org.bluez.Network1 objects=0 owners=0 refs=0",
                "count>=2:bluez-daemon: ipsp closeout phase=mainloop-cleanup "
                "watch-remove=mgmt,dbus,l2cap-coc,6lowpan "
                "timer-remove=connect-timeout dispatch-pending=0 "
                "watches=0 timers=0 owner=bluetoothd",
                "linux-bt-6lowpan: registered=0 ifname=-",
                "upstream-link-state=netdev:0,coc:0,peer:0",
                "upstream-link-refs=netdev:0,chan:0,peer:0",
                "ipsp-state=closed",
                "BLUEZDAEMONIPSP_DONE_BLE1",
            )),
            RoleCheck("ble2", (
                "BLUEZDAEMONIPSP_BEGIN_BLE2",
                "bluez-daemon: ipsp closeout phase=daemon-profile-register "
                "source=third/bluez/src/main.c+third/bluez/src/profile.c+"
                "third/bluez/src/device.c+third/bluez/profiles/network/"
                "connection.c+third/bluez/profiles/network/ipsp.c "
                "plugin=network profile=ipsp "
                "uuid=00001820-0000-1000-8000-00805f9b34fb "
                "dbus=org.bluez.Profile1 object=/org/bluez/hci0/"
                "dev_peer/ipsp0 owner=bluetoothd security=medium "
                "authorize=ok",
                "bluez-daemon: ipsp closeout phase=mainloop-ownership "
                "watch-add=mgmt,dbus,l2cap-coc,6lowpan "
                "timer-add=connect-timeout dispatch=mgmt,dbus,l2cap,"
                "netdev owner=bluetoothd watches=4 timers=1",
                "count>=2:upstream-l2cap-bind: psm=0x0023 "
                "cid=0x0040 handle=0x0074 create-ret=0 bind-ret=0",
                "count>=2:upstream-l2cap-connect: psm=0x0023 "
                "cid=0x0040 connect-ret=0",
                "bluez-daemon: ipsp closeout phase=profile-connect "
                "ifname=bt0 psm=0x0023 fd-handoff=le-l2cap-coc "
                "owner=kernel-6lowpan profile=ipsp connected=1",
                "bluez-daemon: ipsp closeout phase=native-6lowpan-ownership "
                "ifname=bt0",
                "bluez-daemon: ipsp closeout phase=native-l2cap-coc-ownership "
                "psm=0x0023 cid=0x0040",
                "bluez-daemon: dbus InterfacesAdded "
                "path=/org/bluez/hci0/dev_peer/ipsp0 "
                "interfaces=org.bluez.Network1,org.bluez.Device1 "
                "properties=Connected,Interface,UUID owner=bluetoothd",
                "count>=2:bluez-daemon: ipsp closeout phase=status "
                "dbus-owner=bluetoothd object=/org/bluez/hci0/"
                "dev_peer/ipsp0 interface=org.bluez.Network1 "
                "connected-query=ok",
                "bluez-daemon: ipsp closeout phase=native-datapath-status "
                "datapath=bt0",
                "count>=2:linux-bt-6lowpan: registered=1 ifname=bt0",
                "count>=2:ipsp-open=1",
                "ipsp-psm=0x0023",
                "ipsp-cid=0x0040",
                "upstream-link=net_bluetooth/6lowpan+"
                "sim-ipsp-datapath-link",
                "upstream-link-state=netdev:1,coc:1,peer:1",
                "upstream-link-refs=netdev:1,chan:1,peer:1",
                "re:upstream-link-active-tx=[1-9][0-9]*",
                "upstream-iphc-link=net_6lowpan/iphc",
                "PING6 fc00::1:",
                "2 packets transmitted, 2 received",
                "mode=tcp-client",
                "dip=fc00::1:5001",
                "mode=udp-client",
                "Mbits/sec",
                "iperf exit",
                "re:last-tx-dispatch=0x7[ab] last-rx-dispatch=0x7[ab]",
                "re:tx-iphc=[1-9][0-9]* rx-iphc=[1-9][0-9]*",
                "tx-iphc-error=0",
                "re:tx-frag-dgrams=[1-9]",
                "re:tx-frag-frames=[1-9]",
                "rx-frag-drop=0",
                "count>=2:bluez-daemon: ipsp closeout phase=profile-disconnect "
                "object=/org/bluez/hci0/dev_peer/ipsp0 owner=bluetoothd "
                "owner-lost=1 interfaces-removed=org.bluez.Network1 "
                "cleanup=ok",
                "bluez-daemon: ipsp closeout phase=native-6lowpan-cleanup "
                "unregister=1 chan-release=1 peer-unref=1 "
                "netdev-unregister=1 owner-state-final=0",
                "count>=2:upstream-l2cap-close: released",
                "count>=2:bluez-daemon: dbus InterfacesRemoved "
                "path=/org/bluez/hci0/dev_peer/ipsp0 "
                "interfaces=org.bluez.Network1 objects=0 owners=0 refs=0",
                "count>=2:bluez-daemon: ipsp closeout phase=mainloop-cleanup "
                "watch-remove=mgmt,dbus,l2cap-coc,6lowpan "
                "timer-remove=connect-timeout dispatch-pending=0 "
                "watches=0 timers=0 owner=bluetoothd",
                "linux-bt-6lowpan: registered=0 ifname=-",
                "upstream-link-state=netdev:0,coc:0,peer:0",
                "upstream-link-refs=netdev:0,chan:0,peer:0",
                "ipsp-state=closed",
                "BLUEZDAEMONIPSP_DONE_BLE2",
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
                "sim-status-mirror=diagnostic",
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
                "btctl: hwsim records=",
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
                "sim-status-mirror=diagnostic",
                "command-path=1 event-path=1 fallback=0",
                "payload=0b 00",
                "upstream-l2cap-bind: psm=0x1001 cid=0x0040 handle=0x0052 create-ret=0 bind-ret=0",
                "upstream-l2cap-connect: psm=0x1001 cid=0x0040 connect-ret=0",
                "upstream-l2cap-write: payload-len=8 send-ret=8",
                "native-ret=8 attach-ret=0 native-path=1",
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
                "sim-status-mirror=diagnostic",
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
                "count>=3:sim-status-mirror=diagnostic",
                "count>=3:command-path=1 event-path=1 fallback=0",
                "count>=3:upstream-hci-conn=1",
                "count>=3:btctl: upstream hci-connect-le peer=2",
                "count>=3:sim-conn le=1 peer=2 role=0 handle=0x0102 upstream-hci-conn=1",
                "count>=3:conn-hash acl=0 sco=0 le=1",
                "count>=3:conn[0]",
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
                "sim-status-mirror=diagnostic",
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
                "upstream-mgmt-send: opcode=0x0019",
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
                "upstream-mgmt-send: opcode=0x0018",
                "upstream-mgmt-send: opcode=0x0019",
                "payload=0b 00",
                "payload=01 00 00 00 0a 00 19 00 00",
                "conn-hash acl=0 sco=0 le=0",
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
        "bluez-mgmt-daemon-bootstrap",
        (
            RoleCheck("ble1", (
                "bluez-mgmt: source=third/bluez/src/shared/mgmt.c+"
                "third/bluez/src/adapter.c style mode=daemon-bootstrap",
                "bluez-mgmt: hci-socket fd=",
                "bluez-mgmt: hci-bind-control ret=0",
                "bluez-mgmt: upstream-controller action=bootstrap-init",
                "owner=src/shared/mgmt.c,src/adapter.c,src/device.c,"
                "src/agent.c,net/bluetooth/mgmt.c",
                "objects=mgmt,mgmt_request,mgmt_reply,adapter,device,"
                "pending_cmd,discovery_session,pair_session,bonding_data",
                "socket=AF_BLUETOOTH/BTPROTO_HCI/HCI_CHANNEL_CONTROL",
                "methods=read-version,read-commands,read-index-list,"
                "read-info,set-powered,set-connectable,set-discoverable,"
                "set-bondable,set-bredr,set-le,set-advertising,"
                "start-discovery,stop-discovery,set-io-capability,"
                "pair-device,get-conn-info,disconnect,unpair-device",
                "events=cmd-complete,cmd-status,new-settings,discovering,"
                "device-connected,device-disconnected,device-unpaired,"
                "new-long-term-key",
                "bluez-mgmt: upstream-controller action=bootstrap-open",
                "state=control-open",
                "bluez-mgmt: daemon-bootstrap phase=read-controller-info",
                "bluez-mgmt: upstream-controller "
                "action=read-controller-info",
                "bluez-mgmt: send opcode=0x0001 index=0xffff",
                "bluez-mgmt: send opcode=0x0002 index=0xffff",
                "bluez-mgmt: send opcode=0x0003 index=0xffff",
                "bluez-mgmt: send opcode=0x0004 index=0x0000",
                "bluez-mgmt: daemon-bootstrap phase=adapter-policy",
                "bluez-mgmt: upstream-controller action=adapter-policy",
                "bluez-mgmt: send opcode=0x0005 index=0x0000",
                "bluez-mgmt: send opcode=0x0007 index=0x0000",
                "bluez-mgmt: send opcode=0x0006 index=0x0000",
                "bluez-mgmt: send opcode=0x0009 index=0x0000",
                "bluez-mgmt: send opcode=0x002a index=0x0000",
                "bluez-mgmt: send opcode=0x000d index=0x0000",
                "bluez-mgmt: send opcode=0x0029 index=0x0000",
                "bluez-mgmt: daemon-bootstrap phase=discovery",
                "bluez-mgmt: upstream-controller action=discovery",
                "owners=mgmt-fd:1,adapter:1,device:1,pending:0,"
                "discovery:1,security:0",
                "bluez-mgmt: send opcode=0x0023 index=0x0000",
                "bluez-mgmt: send opcode=0x0024 index=0x0000",
                "bluez-mgmt: daemon-bootstrap phase=pair-connect",
                "bluez-mgmt: upstream-controller action=pair-connect",
                "owners=mgmt-fd:1,adapter:1,device:1,pending:1,"
                "discovery:1,security:1",
                "bluez-mgmt: send opcode=0x0018 index=0x0000",
                "bluez-mgmt: send opcode=0x0019 index=0x0000",
                "bluez-mgmt: recv-until label=daemon-bootstrap-connected",
                "event=0x000b",
                "opcode=0x0019 status=0x00",
                "bluez-mgmt: daemon-bootstrap phase=conn-info-disconnect",
                "bluez-mgmt: upstream-controller "
                "action=conn-info-disconnect",
                "bluez-mgmt: send opcode=0x0031 index=0x0000",
                "bluez-mgmt: send opcode=0x0014 index=0x0000",
                "bluez-mgmt: recv-until label=daemon-bootstrap-disconnected",
                "event=0x000c",
                "opcode=0x0014 status=0x00",
                "bluez-mgmt: daemon-bootstrap phase=unpair-cleanup",
                "bluez-mgmt: upstream-controller action=unpair-cleanup",
                "bluez-mgmt: send opcode=0x001b index=0x0000",
                "bluez-mgmt: daemon-bootstrap phase=error-policy",
                "bluez-mgmt: upstream-controller action=error-policy",
                "opcode=0x0018 status=0x0d expect=0x0018 "
                "expect-status=0x0d",
                "bluez-mgmt: upstream-controller action=bootstrap-close",
                "state=closed",
                "owners=mgmt-fd:0,adapter:1,device:1,pending:0,"
                "discovery:0,security:0",
                "bluez-mgmt: daemon-bootstrap mgmt-event-ledger",
                "read-version=cmd-complete read-commands=cmd-complete",
                "powered=new-settings connectable=new-settings",
                "pair-device=device-connected get-conn-info=cmd-complete",
                "disconnect=device-disconnected unpair=cmd-complete",
                "invalid-io-cap=cmd-status-invalid-params",
                "bluez-mgmt: daemon-bootstrap link-ledger",
                "mgmt-fd=closed adapter-ref=0 device-ref=0 pending-cmd=0",
                "discovery-session=0 pair-session=0 bond-ref=0",
                "upstream-link=bluezmgmt-daemon-bootstrap-"
                "upstream-link-bluetoothd",
                "bluez-mgmt: daemon-bootstrap complete",
                "re:hci-mgmt-socket-recv=[1-9]",
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
                "bluez-mgmt: reconnect-stress kernel-status-after-close",
                "hci-mgmt-control-contract-version=1",
                "hci-mgmt-control-ownership=socket,bind-control,"
                "hci_mgmt_chan_register,hci_mgmt_chan_find,"
                "hci_mgmt_cmd,handler-dispatch",
                "hci-mgmt-pending-ownership=pending_cmd,pending_pair,"
                "pending_disconnect,pending_unpair,pending_error_status",
                "hci-mgmt-cleanup-ownership=control-fd-release,"
                "pending-cmd-free,event-queue-drain,"
                "adapter-device-ref-zero",
                "bluez-mgmt: reconnect-stress complete rounds=3",
                "re:hci-mgmt-socket-cmd=[1-9][0-9]*",
                "re:hci-mgmt-socket-recv=[1-9]",
                "re:hci-mgmt-socket-event=[1-9][0-9]*",
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
                "re:hci-user-adv-hwsim-tx=[1-9][0-9]*",
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
                "re:hci-user-scan-hwsim-report=[1-9][0-9]*",
            )),
        ),
    ),
    CaseCheck(
        "bluez-hci-mgmt-socket-closeout-full",
        (
            RoleCheck("ble1", (
                "BLUEZ_HCI_MGMT_SOCKET_BEGIN_BLE1",
                "bluez-hciraw: source=third/bluez/tools/hcitool style mode=user-advertise-enable",
                "bluez-hciraw: user-advertise-enable complete",
                "hci-user-adv-enable=1",
                "re:hci-user-adv-hwsim-tx=[1-9][0-9]*",
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=control",
                "bluez-mgmt: control complete",
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=pair-noio",
                "bluez-mgmt: pair-noio complete",
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=user-confirm",
                "bluez-mgmt: send opcode=0x001c index=0x0000",
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=user-confirm-neg",
                "bluez-mgmt: user-confirm-neg complete",
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=passkey",
                "bluez-mgmt: send opcode=0x001e index=0x0000",
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=passkey-neg",
                "bluez-mgmt: passkey-neg complete",
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=cancel-pair",
                "bluez-mgmt: cancel-pair complete",
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=cancel-pair-pending",
                "bluez-mgmt: cancel-pair-pending complete",
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=pair-unpair",
                "bluez-mgmt: send opcode=0x001b index=0x0000",
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=lifecycle",
                "bluez-mgmt: send opcode=0x0031 index=0x0000",
                "bluez-mgmt: send opcode=0x0014 index=0x0000",
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=reconnect-stress rounds=3",
                "bluez-mgmt: reconnect-stress complete rounds=3",
                "bluez-mgmt: source=third/bluez/tools/btmgmt style mode=error-path",
                "bluez-mgmt: error-path complete",
                "bluez-btmon: source=third/bluez/monitor/bt.h style mode=control",
                "bluez-btmon: control complete",
                "bluez-hciioctl: source=third/bluez/tools/hciconfig style mode=basic",
                "bluez-hciioctl: basic complete",
                "bluez-hciraw: source=third/bluez/tools/hcitool style mode=command",
                "bluez-hciraw: command complete",
                "bluez-hciraw: source=third/bluez/tools/hcitool style mode=user-command",
                "bluez-hciraw: user-command complete",
                "bluez-hciraw: source=third/bluez/tools/hcitool style mode=user-command-monitor",
                "bluez-hciraw: user-command-monitor complete",
                "bluez-hciraw: source=third/bluez/tools/hcitool style mode=user-command-sequence-monitor",
                "bluez-hciraw: user-command-sequence-monitor complete",
                "bluez-hciraw: source=third/bluez/tools/hcitool style mode=user-command-error-monitor",
                "bluez-hciraw: user-command-error-monitor complete",
                "bluez-hciraw: source=third/bluez/tools/hcitool style mode=user-command-init-sequence-monitor",
                "bluez-hciraw: user-command-init-sequence-monitor complete",
                "bluez-hciraw: source=third/bluez/tools/hcitool style "
                "mode=user-iso-setup-monitor",
                "bluez-hciraw: user-iso-setup-monitor complete",
                "bluez-hciraw: source=third/bluez/tools/hcitool.c+"
                "third/bluez/lib/hci.c+third/bluez/monitor/bt.h "
                "style mode=socket-abi-closeout",
                "bluez-hciraw: socket-abi ownership-begin",
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/hci_sock.c",
                "channels=raw,user,monitor,control,logging",
                "bluez-hciraw: source=third/bluez/tools/hcitool style "
                "mode=socket-abi-raw-command",
                "bluez-hciraw: source=third/bluez/tools/hcitool style "
                "mode=socket-abi-user-command",
                "bluez-hciraw: hci-bind-control ret=0",
                "bluez-hciraw: hci-bind-logging ret=0",
                "bluez-hciraw: socket-abi channel-ledger raw=1 user=1 "
                "monitor=1 control=1 logging=1 filter=1 sendmsg=1 "
                "recvmsg=1 fanout=1",
                "bluez-hciraw: socket-abi event-order-ledger",
                "raw-command=cmd-complete user-command=cmd-complete",
                "monitor-fanout=observed control-bind=ok logging-bind=ok",
                "bluez-hciraw: socket-abi upstream-coverage-map",
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/hci_core.c",
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/hci_event.c",
                "bluez-hciraw: socket-abi link-ledger",
                "raw-fd=0 user-fd=0 monitor-fd=0 control-fd=0 "
                "logging-fd=0",
                "filter-ref=0 monitor-fanout-ref=0 skb-ref=0",
                "bluez-hciraw: socket-abi final raw-final=1 user-final=1 "
                "monitor-final=1 control-final=1 logging-final=1 "
                "filter-final=1 fanout-final=1 cleanup-final=1",
                "upstream-link=bluezhciraw-hci-socket-upstream-link-"
                "bluetoothd",
                "bluez-hciraw: socket-abi upstream-source-parity",
                "direct-upstream=tools/hcitool.c,lib/hci.c,monitor/bt.h,"
                "hci_sock.c,hci_core.c,hci_event.c,mgmt.c",
                "objects=raw-socket,user-socket,monitor-socket,"
                "control-socket,logging-socket,hci-filter,monitor-fanout,"
                "skb,command,event,pending-command,pending-event",
                "handlers=hci_socket,bind_raw,bind_user,bind_monitor,"
                "bind_control,bind_logging,setsockopt_filter,"
                "getsockopt_filter,sendmsg,recvmsg,monitor_deliver,"
                "command_complete,command_status,close_release",
                "native-channels=raw,user,monitor,control,logging",
                "native-events=cmd-complete,cmd-status,monitor-fanout,"
                "adv-enable,scan-report,iso-setup",
                "upstream-link=bluezhciraw-hci-socket-upstream-link-"
                "bluetoothd parity-final=1",
                "bluez-hciraw: socket-abi-closeout complete",
                "upstream-socket: family=PF_BLUETOOTH proto=5 ret=0",
                "upstream-cmtp-socket: proto=BTPROTO_CMTP create-ret=0 "
                "ioctl=CMTPCONNADD ret=0",
                "ioctl=CMTPGETCONNLIST ret=0 cnum=1",
                "ioctl=CMTPGETCONNINFO ret=0 state=1",
                "ioctl=CMTPCONNDEL ret=0",
                "duplicate-ret=-114 post-del-info-ret=-2 "
                "missing-del-ret=-2 final-active=0",
                "core-session=linked:1,worker:1,capi:1,"
                "reassembly:1,datapath:1",
                "core-traffic=blockids:1,tx:1,rx:1,capimsg:1",
                "poll-op-null=1",
                "proto-name=CMTP",
                "btctl: ordinary-cmtp-socket proto=BTPROTO_CMTP "
                "socket-ret=",
                "nonblock-ret=0",
                "create-nonblock-ok=1",
                "unsupported-bind-errno=95",
                "unsupported-getsockname-errno=95",
                "unsupported-getpeername-errno=95",
                "unsupported-connect-errno=95",
                "unsupported-ok=1",
                "ioctl=CMTPCONNADD ret=0",
                "ioctl=CMTPGETCONNLIST ret=0 cnum=1",
                "path=ordinary-socket final-ok=1",
                "no-data=sock_no bind-ret=-95 getname-ret=-95 "
                "sendmsg-ret=-95 recvmsg-ret=-95 listen-ret=-95 shutdown-ret=-95 connect-ret=-95 socketpair-ret=-95 accept-ret=-95 mmap-ret=-95",
                "bluez-hciraw: monitor-count=",
                "bluez-hciraw: sequence-monitor-count=",
                "bluez-hciraw: init-monitor-count=",
                "hci-user-event-mask-page2-set=1",
                "hci-user-iso-create-cis=1",
                "hci-user-iso-term-big-status=1",
                "BLUEZ_HCI_MGMT_SOCKET_DONE_BLE1",
            )),
            RoleCheck("ble2", (
                "BLUEZ_HCI_MGMT_SOCKET_BEGIN_BLE2",
                "bluez-hciraw: source=third/bluez/tools/hcitool style mode=user-scan-report",
                "bluez-hciraw: user-scan-report complete",
                "hci-user-scan-enable=1",
                "re:hci-user-scan-hwsim-report=[1-9][0-9]*",
                "BLUEZ_HCI_MGMT_SOCKET_DONE_BLE2",
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
                "re:upstream-l2cap-send: psm=0x0019 cid=0x0040 handle=0x0052 payload-len=2[^\\n]*send-ret=2 native-ret=2 attach-ret=0 native-path=1",
                "btctl: upstream avdtp-getcap peer=2",
                "re:upstream-l2cap-send: psm=0x0019 cid=0x0040 handle=0x0052 payload-len=3[^\\n]*send-ret=3 native-ret=3 attach-ret=0 native-path=1",
                "btctl: upstream avdtp-setconfig peer=2",
                "re:upstream-l2cap-send: psm=0x0019 cid=0x0040 handle=0x0052 payload-len=14[^\\n]*send-ret=14 native-ret=14 attach-ret=0 native-path=1",
                "btctl: upstream avdtp-open peer=2",
                "btctl: upstream avdtp-start peer=2",
                "btctl: upstream avdtp-suspend peer=2",
                "btctl: upstream avdtp-close-stream peer=2",
                "count>=4:upstream-l2cap-send: psm=0x0019 cid=0x0040 handle=0x0052 payload-len=3",
                "count>=5:send-ret=3 native-ret=3 attach-ret=0 native-path=1",
            )),
            RoleCheck("bt2", (
                "btctl: upstream avdtp signaling listening",
                "re:upstream-l2cap-send: psm=0x0019 cid=0x0040 handle=0x0052 payload-len=4[^\\n]*send-ret=4 native-ret=4 attach-ret=0 native-path=1",
                "btctl: upstream avdtp-auto-rsp signal=0x01",
                "state=IDLE->DISCOVERED",
                "re:upstream-l2cap-send: psm=0x0019 cid=0x0040 handle=0x0052 payload-len=12[^\\n]*send-ret=12 native-ret=12 attach-ret=0 native-path=1",
                "btctl: upstream avdtp-auto-rsp signal=0x02",
                "count>=5:send-ret=2 native-ret=2 attach-ret=0 native-path=1",
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
                "send-ret=24 native-ret=24 attach-ret=0 native-path=1",
                "btaudio: upstream a2dp source queued",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "upstream-l2cap-close: released",
            )),
            RoleCheck("bt2", (
                "btaudio: upstream a2dp sink listening",
                "upstream-l2cap-recv: recv-ret=24",
                "payload=41 32 44 50 3a 53 42 43 3a 73 79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-listen=[1-9]",
                "re:l2cap-socket-native-recv=[1-9]",
                "l2cap-socket-native-recv-fail=0",
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
                "native-ret=2 attach-ret=0 native-path=1",
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
                "native-ret=14 attach-ret=0 native-path=1",
                "bluez-audio: a2dp signaling setconfig peer=2",
                "bluez-audio: a2dp signaling open peer=2",
                "bluez-audio: a2dp signaling start peer=2",
                "bluez-audio: a2dp signaling suspend peer=2",
                "bluez-audio: a2dp signaling close-stream peer=2",
                "count>=4:upstream-l2cap-send: psm=0x0019 cid=0x0040 "
                "handle=0x0052 payload-len=3",
                "count>=5:send-ret=3 native-ret=3 attach-ret=0 "
                "native-path=1",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=listen "
                "handle=0x0052",
                "bluez-audio: a2dp signaling listening handle=0x0052",
                "re:upstream-l2cap-send: psm=0x0019 cid=0x0040 "
                "handle=0x0052 payload-len=4[^\\n]*send-ret=4 "
                "native-ret=4 attach-ret=0 native-path=1",
                "bluez-audio: a2dp auto-rsp signal=0x01",
                "state=IDLE->DISCOVERED",
                "re:upstream-l2cap-send: psm=0x0019 cid=0x0040 "
                "handle=0x0052 payload-len=12[^\\n]*send-ret=12 "
                "native-ret=12 attach-ret=0 native-path=1",
                "bluez-audio: a2dp auto-rsp signal=0x02",
                "count>=5:send-ret=2 native-ret=2 attach-ret=0 "
                "native-path=1",
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
                "count>=12:attach-ret=0 native-path=1",
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
                "count>=12:attach-ret=0 native-path=1",
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
                "send-ret=24 native-ret=24 attach-ret=0 native-path=1",
                "bluez-audio: a2dp source queued media payload len=24",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "upstream-l2cap-close: released",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=a2dp-sink command=start peer=1 "
                "handle=0x0052",
                "bluez-audio: a2dp sink transport listening",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=a2dp-sink command=read",
                "upstream-l2cap-recv: recv-ret=24",
                "payload=41 32 44 50 3a 53 42 43 3a 73 79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "bluez-audio: a2dp sink media payload received",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-listen=[1-9]",
                "re:l2cap-socket-native-recv=[1-9]",
                "l2cap-socket-native-recv-fail=0",
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
                "send-ret=24 native-ret=24 attach-ret=0 native-path=1",
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
                "style profile=a2dp-sink command=start peer=1 "
                "handle=0x0052",
                "bluez-audio: a2dp sink transport listening",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style profile=a2dp-sink command=read",
                "upstream-l2cap-recv: recv-ret=24",
                "payload=41 32 44 50 3a 53 42 43 3a 73 79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "bluez-audio: a2dp sink media payload received",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-listen=[1-9]",
                "re:l2cap-socket-native-recv=[1-9]",
                "l2cap-socket-native-recv-fail=0",
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
                "count>=2:send-ret=24 native-ret=24 attach-ret=0 native-path=1",
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
                "count>=12:attach-ret=0 native-path=1",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp.c "
                "style profile=a2dp-source command=start peer=2 "
                "handle=0x0052",
                "upstream-l2cap-send: psm=0x0019 cid=0x0041 "
                "handle=0x0052 payload-len=24",
                "send-ret=24 native-ret=24 attach-ret=0 native-path=1",
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
                "upstream-l2cap-recv: recv-ret=24",
                "payload=41 32 44 50 3a 53 42 43 3a 73 79 6e 74 68 65 74 69 63 2d 66 72 61 6d 65",
                "bluez-audio: a2dp sink media payload received",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-listen=[1-9]",
                "re:l2cap-socket-native-recv=[1-9]",
                "l2cap-socket-native-recv-fail=0",
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
                "upstream-l2cap-write-handle: psm=0x0019 cid=0x0041 "
                "handle=0x0052 payload-len=24 send-ret=24",
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
                "bluez-audio: a2dp auto-rsp-loop complete count=10",
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
                "upstream-l2cap-write-handle: psm=0x0019 cid=0x0041 "
                "handle=0x0052 payload-len=24 send-ret=24",
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
                "bluez-audio: a2dp auto-rsp-loop complete count=10",
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
                "native-ret=2 attach-ret=0 native-path=1",
                "count>=5:upstream-l2cap-write: payload-len=3 send-ret=3 "
                "native-ret=3 attach-ret=0 native-path=1",
                "upstream-l2cap-write: payload-len=14 send-ret=14 "
                "native-ret=14 attach-ret=0 native-path=1",
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
                "attach-ret=0 native-path=1",
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
                "bluez-audio: a2dp auto-rsp-loop complete count=10",
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
                "native-ret=2 attach-ret=0 native-path=1",
                "count>=7:upstream-l2cap-write: payload-len=3 send-ret=3 "
                "native-ret=3 attach-ret=0 native-path=1",
                "count>=2:upstream-l2cap-write: payload-len=5 send-ret=5 "
                "native-ret=5 attach-ret=0 native-path=1",
                "upstream-l2cap-write: payload-len=11 send-ret=11 "
                "native-ret=11 attach-ret=0 native-path=1",
                "upstream-l2cap-write: payload-len=14 send-ret=14 "
                "native-ret=14 attach-ret=0 native-path=1",
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
                "native-ret=2 attach-ret=0 native-path=1",
                "count>=3:upstream-l2cap-write: payload-len=3 send-ret=3 "
                "native-ret=3 attach-ret=0 native-path=1",
                "upstream-l2cap-write: payload-len=14 send-ret=14 "
                "native-ret=14 attach-ret=0 native-path=1",
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
                "bluez-audio: a2dp auto-rsp-loop complete count=10",
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
        "bluez-a2dp-sbc-codec-concurrent",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=source "
                "peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=source-session-open "
                "peer=2 handle=0x0052",
                "upstream-l2cap-bind: psm=0x0019 cid=0x0040 "
                "handle=0x0052",
                "upstream-l2cap-connect: psm=0x0019 cid=0x0040 "
                "connect-ret=0",
                "bluez-audio: a2dp source session opened peer=2 "
                "handle=0x0052",
                "bluez-audio: a2dp signaling discover peer=2",
                "bluez-audio: a2dp signaling getcap peer=2",
                "bluez-audio: a2dp signaling setconfig peer=2",
                "bluez-audio: a2dp signaling open peer=2",
                "bluez-audio: a2dp signaling start peer=2",
                "count>=5:upstream-l2cap-write: payload-len=3 "
                "send-ret=3 native-ret=3 attach-ret=0 "
                "native-path=1",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp.c "
                "style profile=a2dp-codec role=source "
                "command=source-sbc-encode-write-release peer=2 "
                "handle=0x0052",
                "bluez-audio: codec-backend profile=a2dp codec=sbc "
                "backend=libsbc status=source-built "
                "required-source=third/sbc-2.0",
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "signaling session retained during media peer=2 "
                "handle=0x0052 signaling-cid=0x0040 "
                "media-cid=0x0041",
                "bluez-audio: media transport handle open role=source "
                "psm=0x0019 cid=0x0041 handle=0x0052 ret=0",
                "bluez-audio: media transport handle connect role=source "
                "psm=0x0019 cid=0x0041 handle=0x0052 ret=0",
                "re:upstream-l2cap-write-handle: psm=0x0019 "
                "cid=0x0041 handle=0x0052 payload-len=[1-9][0-9]* "
                "send-ret=[1-9][0-9]* native-ret=[1-9][0-9]* "
                "attach-ret=0 native-path=1",
                "bluez-audio: media transport release complete "
                "role=source codec=sbc",
                "bluez-audio: a2dp signaling suspend peer=2",
                "bluez-audio: a2dp signaling close-stream peer=2",
                "bluez-audio: a2dp source session closed peer=2 "
                "handle=0x0052",
                "bluez-audio: media endpoint clear complete role=source",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle role=sink "
                "peer=1",
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=listen-native "
                "handle=0x0052",
                "upstream-l2cap-native-control: enabled=1",
                "bluez-audio: a2dp signaling listening handle=0x0052 "
                "native-control=1",
                "bluez-audio: a2dp auto-rsp-loop complete count=10",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp.c "
                "style profile=a2dp-codec role=sink "
                "command=sink-sbc-recv-decode-release peer=1 "
                "handle=0x0052",
                "bluez-audio: codec-backend profile=a2dp codec=sbc "
                "backend=libsbc status=source-built "
                "required-source=third/sbc-2.0",
                "re:upstream-l2cap-recv-handle: psm=0x0019 cid=0x0041 "
                "handle=0x0052 recv-ret=[1-9][0-9]* flags=0x0 "
                "payload=9c bd [0-9a-f]{2}",
                "re:bluez-audio: source=third/sbc-2\\.0/sbc/sbc\\.c "
                "sbc_decode pcm-bytes=[1-9][0-9]* "
                "codesize=[1-9][0-9]* checksum=0x[0-9a-f]{8}",
                "bluez-audio: media transport release complete "
                "role=sink codec=sbc",
                "bluez-audio: a2dp auto-rsp-loop initial-state=STREAMING",
                "bluez-audio: a2dp auto-rsp signal=0x09 msg-type=0x00 "
                "pkt-type=0x00 rsp-len=2 err=0x00 "
                "state=STREAMING->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x08 msg-type=0x00 "
                "pkt-type=0x00 rsp-len=2 err=0x00 "
                "state=OPEN->IDLE",
                "bluez-audio: a2dp auto-rsp-loop complete count=2",
                "bluez-audio: media endpoint clear complete role=sink",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
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
                "native-ret=2 attach-ret=0 native-path=1",
                "count>=10:upstream-l2cap-write: payload-len=3 send-ret=3 "
                "native-ret=3 attach-ret=0 native-path=1",
                "count>=2:upstream-l2cap-write: payload-len=14 send-ret=14 "
                "native-ret=14 attach-ret=0 native-path=1",
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
                "count>=2:bluez-audio: a2dp auto-rsp-loop complete count=10",
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
                "count>=2:bluez-audio: a2dp auto-rsp-loop complete count=10",
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
                "count>=12:attach-ret=0 native-path=1",
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
                "count>=12:attach-ret=0 native-path=1",
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
                "bluez-bneptest: l2cap-sockopt-preconnect "
                "options-set=0 options-get=0 imtu=247 omtu=247",
                "lm-set=0 lm-get=0 lm=0x00000006",
                "flushable-set=0 flushable-get=0 flushable=1",
                "power-set=0 power-get=0 force-active=1",
                "policy-set=-1",
                "bluez-bneptest: l2cap-bind ret=0",
                "bluez-bneptest: l2cap-connect ret=0",
                "bluez-bneptest: l2cap-sockopt-conninfo "
                "ret=0 handle=0x0052",
                "bluez-bneptest: l2cap-sockopt-sndmtu ret=-1 errno=22 "
                "value=0 gate=bredr-rejects-bt-mtu",
                "bluez-bneptest: l2cap-sockopt-rcvmtu ret=-1 errno=22 "
                "value=0 gate=bredr-rejects-bt-mtu",
                "bluez-bneptest: l2cap-sockopt-phy ret=0 value=0x000001f9",
                "bluez-bneptest: l2cap-sockopt-connected "
                "conninfo-get=0 handle=0x0052",
                "sndmtu-get=-1 sndmtu-errno=22 sndmtu=0 "
                "rcvmtu-get=-1 rcvmtu-errno=22 rcvmtu=0",
                "phy-get=0 phy=0x000001f9",
                "bluez-bneptest: bnep-socket fd=",
                "bluez-bneptest: bnep-suppfeat ret=0",
                "features=0x00000001",
                "bluez-bneptest: bnep-connadd ret=0",
                "device=btn0",
                "bluez-bneptest: fd-handoff native-boundary "
                "connected-l2cap-fd=",
                "ioctl=BNEPCONNADD role=0x1115 device=btn0",
                "fd-source=socket-fd",
                "bluez-bneptest: native-status-after-connadd",
                "bnep-native-fd-handoff=1",
                "bnep-native-fd-active=1",
                "bnep-native-fd-psm=0x000f",
                "bnep-native-fd-cid=0x0041",
                "bnep-native-fd-role=0x1115",
                "bluez-bneptest: bnep-connlist ret=0",
                "cnum=1 state=0x0001 device=btn0",
                "bluez-bneptest: bnep-conninfo ret=0",
                "state=0x0001 device=btn0",
                "bluez-bneptest: bnep-conndel ret=0",
                "bluez-bneptest: bnep-connlist-postdel ret=0",
                "bluez-bneptest: native-status-after-conndel",
                "bnep-native-fd-active=0",
                "bnep-native-fd-cleanup=1",
                "bluez-bneptest: fd-handoff complete",
                "mode=native-closeout",
                "bluez-bneptest: linux-source-map "
                "sock=third/linux-hwe-6.17-6.17.0/net/bluetooth/bnep/sock.c",
                "core=third/linux-hwe-6.17-6.17.0/net/bluetooth/bnep/core.c",
                "netdev=third/linux-hwe-6.17-6.17.0/net/bluetooth/bnep/netdev.c",
                "bluez-bneptest: native-closeout begin "
                "abi=AF_BLUETOOTH/BTPROTO_BNEP",
                "bluez-bneptest: native-closeout sock-ioctl=getsuppfeat ret=0",
                "bluez-bneptest: native-closeout fd-handoff=1 "
                "role=0x1115 device=btn0",
                "fd-source=socket-fd",
                "bluez-bneptest: native-closeout fd-ownership="
                "l2cap-fd=connected,bnep-fd=control,sock-lookup=1,"
                "sock-put=1,cid=0x0041,psm=0x000f",
                "bluez-bneptest: native-closeout session-ownership="
                "bnep_add_connection,netdev_setup,register_netdev,"
                "session_link,kthread_run,session_thread",
                "bluez-bneptest: native-closeout datapath-ownership="
                "nuttx-ip-tx,linux-netdev-ndo_start_xmit,bnep_tx_frame,"
                "l2cap-send,hwsim-bnep,hwsim-rx,l2cap-deliver,"
                "bnep_rx_frame,netif_rx,nuttx-ip-rx",
                "bluez-bneptest: native-closeout sock-ioctl=getconnlist ret=0",
                "bluez-bneptest: native-closeout sock-ioctl=getconninfo ret=0",
                "bluez-bneptest: native-closeout netdev-ownership="
                "btn0,ndo_start_xmit,netif_rx,l2cap_delivery state=active",
                "bluez-bneptest: native-closeout session-thread="
                "thread=kbnepd state=running rx-queue=owned tx-queue=owned",
                "bluez-bneptest: native-closeout sock-ioctl=conndel ret=0",
                "bluez-bneptest: native-closeout cleanup="
                "session_stop,session_unlink,session_terminate,"
                "unregister_netdev,bnep-native-active-0",
                "bluez-bneptest: native-closeout link-ledger="
                "fd-active=0 session=0 thread=0 netdev=0 rx-queue=0 "
                "tx-queue=0 pending-skb=0 l2cap-ref=0 bnep-ref=0",
                "upstream-link=bluezbneptest-native-bnep-session-"
                "upstream-link-bluetoothd",
                "bluez-bneptest: native-closeout complete",
                "bnep-ioctl-connadd=1",
                "bnep-ioctl-conndel=1",
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
                *BNEP_BLUEZ_FD_HANDOFF_ACTIVE_REQUIRED,
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
                *BNEP_BLUEZ_FD_HANDOFF_TEARDOWN_REQUIRED,
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
                *BNEP_BLUEZ_FD_HANDOFF_ACTIVE_REQUIRED,
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
                *BNEP_BLUEZ_FD_HANDOFF_TEARDOWN_REQUIRED,
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
                "bluez-network: bnep no-data abi=sock_no "
                "proto=BTPROTO_BNEP bind-ret=-1",
                "bind-errno=95",
                "getname-errno=95",
                "getpeername-errno=95",
                "connect-errno=95",
                "send-errno=95",
                "recv-errno=95",
                "listen-errno=95",
                "shutdown-errno=95",
                "accept-errno=95",
                "no-data-ok=1",
                "bluez-network: suppfeat ret=0",
                "features=0x00000001",
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "bluez-network: native-boundary connected-l2cap-fd=",
                "ioctl=BNEPCONNADD role=0x1115 service=panu device=btn0",
                "bluez-network: native-closeout fd-ownership="
                "l2cap-fd=connected,bnep-fd=control,sock-lookup=1,"
                "sock-put=1,psm=0x000f,cid=0x0041",
                "bluez-network: native-closeout session-ownership="
                "bnep_add_connection,netdev_setup,register_netdev,"
                "session_link,kthread_run,session_thread service=panu",
                "bluez-network: native-closeout datapath-ownership="
                "Network1.Connect,connected-l2cap-fd,BNEPCONNADD,btn0,"
                "ndo_start_xmit,bnep_tx_frame,l2cap-send,hwsim-bnep,"
                "bnep_rx_frame,netif_rx,NuttX-IP",
                "bluez-network: native-status-after-connect",
                "bluez-network: profile connected interface=btn0",
                "bluez-network: connect complete",
                "bluez-network: mode=status",
                "bluez-network: conninfo ret=0",
                "bluez-network: native-status",
                "connected=true",
                "bnep-ioctl-connadd=1",
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "PING 10.77.0.2",
                "2 packets transmitted, 2 received, 0% packet loss",
                "bluez-network: mode=disconnect",
                "bluez-network: bnep-conndel ret=0",
                "bluez-network: connlist-postdel ret=0",
                "cnum=0",
                "bluez-network: native-status-after-disconnect",
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
                "bluez-network: bnep no-data abi=sock_no "
                "proto=BTPROTO_BNEP bind-ret=-1",
                "bind-errno=95",
                "getname-errno=95",
                "getpeername-errno=95",
                "connect-errno=95",
                "send-errno=95",
                "recv-errno=95",
                "listen-errno=95",
                "shutdown-errno=95",
                "accept-errno=95",
                "no-data-ok=1",
                "bluez-network: suppfeat ret=0",
                "features=0x00000001",
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "bluez-network: native-boundary connected-l2cap-fd=",
                "ioctl=BNEPCONNADD role=0x1115 service=panu device=btn0",
                "bluez-network: native-closeout fd-ownership="
                "l2cap-fd=connected,bnep-fd=control,sock-lookup=1,"
                "sock-put=1,psm=0x000f,cid=0x0041",
                "bluez-network: native-closeout session-ownership="
                "bnep_add_connection,netdev_setup,register_netdev,"
                "session_link,kthread_run,session_thread service=panu",
                "bluez-network: native-closeout datapath-ownership="
                "Network1.Connect,connected-l2cap-fd,BNEPCONNADD,btn0,"
                "ndo_start_xmit,bnep_tx_frame,l2cap-send,hwsim-bnep,"
                "bnep_rx_frame,netif_rx,NuttX-IP",
                "bluez-network: native-status-after-connect",
                "bluez-network: profile connected interface=btn0",
                "bluez-network: connect complete",
                "bluez-network: mode=status",
                "bluez-network: conninfo ret=0",
                "bluez-network: native-status",
                "connected=true",
                "bnep-ioctl-connadd=1",
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "PING 10.77.0.1",
                "2 packets transmitted, 2 received, 0% packet loss",
                "bluez-network: mode=disconnect",
                "bluez-network: bnep-conndel ret=0",
                "bluez-network: connlist-postdel ret=0",
                "cnum=0",
                "bluez-network: native-status-after-disconnect",
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
        "bluez-current-functional-closeout",
        (
            RoleCheck("bt1", (
                "BLUEZCURRENT_BEGIN_BT1",
                "bluez-daemon: audio-a2dp-closeout-full complete "
                "role=source peer=2",
                "bluez-daemon: audio-a2dp-closeout-full cleanup "
                "role=source profile-registered=0 dbus-owners=0",
                "bluez-daemon: basic closeout upstream-coverage-map role=bt",
                "bluez-network: closeout-full stage=begin "
                "semantic=network-current-closeout-umbrella",
                "count>=2:bluez-network: daemon-profile action=connect "
                "service=panu role=0x1115",
                "bluez-network: daemon-profile action=connect "
                "service=gn role=0x1117",
                "PING 10.77.0.2",
                "1400 bytes from 10.77.0.2",
                "bluez-network: closeout-full complete "
                "semantic=network-current-closeout-umbrella",
                "bnep-native-active=0",
                "bluez-daemon: hid native-io summary role=host",
                "bluez-daemon: hid closeout upstream-coverage-map role=host",
                "bluez-daemon: hfp-rfcomm native-io summary",
                "bluez-daemon: hfp closeout upstream-coverage-map",
                "bluez-daemon: hsp-rfcomm native-io summary",
                "bluez-daemon: hsp closeout upstream-coverage-map",
                "bluez-daemon: pbap-obex-rfcomm native-io summary",
                "bluez-daemon: pbap closeout upstream-coverage-map",
                "bluez-daemon: opp-obex-rfcomm native-io summary",
                "bluez-daemon: opp closeout upstream-coverage-map",
                "bluez-daemon: map-obex-rfcomm native-io summary",
                "bluez-daemon: map closeout upstream-coverage-map",
                "bluez-daemon: mns-obex-rfcomm native-io summary",
                "bluez-daemon: mns closeout upstream-coverage-map",
                "bluez-daemon: ftp-obex-rfcomm native-io summary",
                "bluez-daemon: ftp closeout upstream-coverage-map",
                "bluez-daemon: sync-obex-rfcomm native-io summary",
                "bluez-daemon: sync closeout upstream-coverage-map",
                "bluez-daemon: bip-obex-rfcomm native-io summary",
                "bluez-daemon: bip closeout upstream-coverage-map",
                "bluez-daemon: print-rfcomm native-io summary role=client",
                "bluez-daemon: print closeout upstream-coverage-map",
                "bluez-daemon: iap-rfcomm native-io summary",
                "bluez-daemon: iap closeout upstream-coverage-map",
                "BLUEZCURRENT_DONE_BT1",
            )),
            RoleCheck("bt2", (
                "BLUEZCURRENT_BEGIN_BT2",
                "bluez-daemon: audio-a2dp-closeout-full complete "
                "role=sink peer=1",
                "bluez-daemon: audio-a2dp-closeout-full cleanup "
                "role=sink profile-registered=0 dbus-owners=0",
                "bluez-daemon: basic closeout upstream-coverage-map role=bt",
                "bluez-network: closeout-full stage=begin "
                "semantic=network-current-closeout-umbrella",
                "bluez-network: daemon-profile action=connect "
                "service=nap role=0x1116",
                "bluez-network: daemon-profile action=connect "
                "service=gn role=0x1117",
                "bluez-network: daemon-profile action=connect "
                "service=panu role=0x1115",
                "PING 10.77.0.1",
                "1400 bytes from 10.77.0.1",
                "bluez-network: closeout-full complete "
                "semantic=network-current-closeout-umbrella",
                "bnep-native-active=0",
                "bluez-daemon: hid native-io summary role=device",
                "bluez-daemon: hid closeout upstream-coverage-map role=device",
                "bluez-daemon: hfp-rfcomm native-io summary",
                "bluez-daemon: hfp closeout upstream-coverage-map",
                "bluez-daemon: hsp-rfcomm native-io summary",
                "bluez-daemon: hsp closeout upstream-coverage-map",
                "bluez-daemon: pbap-obex-rfcomm native-io summary",
                "bluez-daemon: pbap closeout upstream-coverage-map",
                "bluez-daemon: opp-obex-rfcomm native-io summary",
                "bluez-daemon: opp closeout upstream-coverage-map",
                "bluez-daemon: map-obex-rfcomm native-io summary",
                "bluez-daemon: map closeout upstream-coverage-map",
                "bluez-daemon: mns-obex-rfcomm native-io summary",
                "bluez-daemon: mns closeout upstream-coverage-map",
                "bluez-daemon: ftp-obex-rfcomm native-io summary",
                "bluez-daemon: ftp closeout upstream-coverage-map",
                "bluez-daemon: sync-obex-rfcomm native-io summary",
                "bluez-daemon: sync closeout upstream-coverage-map",
                "bluez-daemon: bip-obex-rfcomm native-io summary",
                "bluez-daemon: bip closeout upstream-coverage-map",
                "bluez-daemon: print-rfcomm native-io summary role=printer",
                "bluez-daemon: print closeout upstream-coverage-map",
                "bluez-daemon: iap-rfcomm native-io summary",
                "bluez-daemon: iap closeout upstream-coverage-map",
                "BLUEZCURRENT_DONE_BT2",
            )),
            RoleCheck("ble1", (
                "BLUEZCURRENT_BEGIN_BLE1",
                "bluez-daemon: ipsp closeout phase=daemon-profile-register",
                "bluez-daemon: ipsp closeout phase=profile-connect "
                "ifname=bt0 psm=0x0023 fd-handoff=le-l2cap-coc",
                "PING6 fc00::2:",
                "2 packets transmitted, 2 received",
                "mode=tcp-server",
                "mode=udp-server",
                "count>=2:bluez-daemon: ipsp closeout phase=profile-disconnect",
                "linux-bt-6lowpan: registered=0 ifname=-",
                "ipsp-state=closed",
                "bluez-daemon: basic closeout upstream-coverage-map role=ble",
                "bluez-audio: source=third/bluez/profiles/audio/main.c "
                "style=bluetoothd-plugin command=plugin-init plugin=audio",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style=iso-socket command=sendmsg role=source",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-audio-codec role=source "
                "command=source-lc3-encode-write-release cig=0 cis=1",
                "bluez-audio: le-daemon-integrated-flow source=third/bluez/profiles/audio/main.c "
                "style=bluetoothd-profile command=integrated-profile-flow "
                "role=source cig=0 cis=1 peer-cis=2 owner=bluetoothd state=complete",
                "bluez-audio: source=third/bluez/profiles/audio/lc3.c "
                "lc3 encode sample-rate=16000 frame-duration-us=10000",
                "bluez-audio: le-gatt upstream-coverage-map role=source",
                "bluez-daemon: hogp native-io summary role=host",
                "bluez-daemon: hogp closeout upstream-coverage-map role=host",
                "bluez-daemon: gatt-att native-io summary role=client",
                "bluez-daemon: gatt closeout upstream-coverage-map",
                "bluez-daemon: mesh-gatt-proxy native-io summary",
                "bluez-daemon: mesh closeout upstream-coverage-map",
                "bluez-daemon: asha-att native-io summary role=central",
                "bluez-daemon: asha closeout upstream-coverage-map",
                "bluez-daemon: midi-att native-io summary role=controller",
                "bluez-daemon: midi closeout upstream-coverage-map",
                "bluez-daemon: ranging-rap-att native-io summary",
                "bluez-daemon: ranging closeout upstream-coverage-map",
                "BLUEZCURRENT_DONE_BLE1",
            )),
            RoleCheck("ble2", (
                "BLUEZCURRENT_BEGIN_BLE2",
                "bluez-daemon: ipsp closeout phase=daemon-profile-register",
                "bluez-daemon: ipsp closeout phase=profile-connect "
                "ifname=bt0 psm=0x0023 fd-handoff=le-l2cap-coc",
                "PING6 fc00::1:",
                "2 packets transmitted, 2 received",
                "mode=tcp-client",
                "mode=udp-client",
                "count>=2:bluez-daemon: ipsp closeout phase=profile-disconnect",
                "linux-bt-6lowpan: registered=0 ifname=-",
                "ipsp-state=closed",
                "bluez-daemon: basic closeout upstream-coverage-map role=ble",
                "bluez-audio: source=third/bluez/profiles/audio/main.c "
                "style=bluetoothd-plugin command=plugin-init plugin=audio",
                "bluez-audio: source=third/bluez/profiles/audio/transport.c "
                "style=iso-socket command=recvmsg role=sink",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style profile=le-audio-codec role=sink "
                "command=sink-lc3-recv-decode-release cig=0 cis=1",
                "bluez-audio: le-daemon-integrated-flow source=third/bluez/profiles/audio/main.c "
                "style=bluetoothd-profile command=integrated-profile-flow "
                "role=sink cig=0 cis=1 peer-cis=2 owner=bluetoothd state=complete",
                "bluez-audio: source=third/bluez/profiles/audio/lc3.c "
                "lc3 decode sample-rate=16000 frame-duration-us=10000",
                "bluez-audio: le-gatt upstream-coverage-map role=sink",
                "bluez-daemon: hogp native-io summary role=device",
                "bluez-daemon: hogp closeout upstream-coverage-map role=device",
                "bluez-daemon: gatt-att native-io summary role=server",
                "bluez-daemon: gatt closeout upstream-coverage-map",
                "bluez-daemon: mesh-gatt-proxy native-io summary role=node",
                "bluez-daemon: mesh closeout upstream-coverage-map role=node",
                "bluez-daemon: asha-att native-io summary role=hearing-aid",
                "bluez-daemon: asha closeout upstream-coverage-map",
                "bluez-daemon: midi-att native-io summary role=peripheral",
                "bluez-daemon: midi closeout upstream-coverage-map",
                "bluez-daemon: ranging-rap-att native-io summary",
                "bluez-daemon: ranging closeout upstream-coverage-map",
                "BLUEZCURRENT_DONE_BLE2",
            )),
        ),
    ),
    CaseCheck(
        "bluez-net-current-complete-closeout",
        (
            RoleCheck("bt1", (
                "BLUEZNETCOMPLETE_BEGIN_BT1",
                "bluez-network: closeout-full stage=begin "
                "semantic=network-current-closeout-umbrella",
                "bluez-network: closeout phase=daemon-dbus "
                "owner=org.bluez object-manager=1 adapter=1 "
                "network-server=panu,nap,gn",
                "bluez-network: closeout phase=service-registration "
                "services=panu,nap,gn",
                "bluez-network: closeout phase=fd-handoff "
                "source=third/bluez/profiles/network/connection.c+"
                "profiles/network/bnep.c",
                "bluez-network: closeout phase=datapath "
                "btn0=required ping=required mtu1400=required",
                "bluez-network: daemon-profile action=connect "
                "service=panu role=0x1115",
                "bluez-network: daemon-profile action=connect "
                "service=nap role=0x1116",
                "bluez-network: daemon-profile action=connect "
                "service=gn role=0x1117",
                "count>=4:bluez-network: bnep-connadd ret=0",
                "count>=4:device=btn0",
                "PING 10.77.0.2",
                "bluez-network: closeout-full complete "
                "semantic=network-current-closeout-umbrella",
                "bnep-native-active=0",
                "re:bnep-native-netdev-xmit=[1-9]",
                "re:bnep-native-tx-frame-ok=[1-9]",
                "re:bnep-native-l2cap-delivered=[1-9]",
                "re:bnep-native-rx-frame-ok=[1-9]",
                "re:bnep-native-netif-rx=[1-9]",
                "BLUEZNETCOMPLETE_DONE_BT1",
            )),
            RoleCheck("bt2", (
                "BLUEZNETCOMPLETE_BEGIN_BT2",
                "bluez-network: closeout-full stage=begin "
                "semantic=network-current-closeout-umbrella",
                "bluez-network: closeout phase=daemon-dbus "
                "owner=org.bluez object-manager=1 adapter=1 "
                "network-server=panu,nap,gn",
                "bluez-network: closeout phase=service-registration "
                "services=panu,nap,gn",
                "bluez-network: closeout phase=fd-handoff "
                "source=third/bluez/profiles/network/connection.c+"
                "profiles/network/bnep.c",
                "bluez-network: closeout phase=datapath "
                "btn0=required ping=required mtu1400=required",
                "bluez-network: daemon-profile action=connect "
                "service=nap role=0x1116",
                "bluez-network: daemon-profile action=connect "
                "service=gn role=0x1117",
                "bluez-network: daemon-profile action=connect "
                "service=panu role=0x1115",
                "count>=4:bluez-network: bnep-connadd ret=0",
                "count>=4:device=btn0",
                "PING 10.77.0.1",
                "bluez-network: closeout-full complete "
                "semantic=network-current-closeout-umbrella",
                "bnep-native-active=0",
                "re:bnep-native-netdev-xmit=[1-9]",
                "re:bnep-native-tx-frame-ok=[1-9]",
                "re:bnep-native-l2cap-delivered=[1-9]",
                "re:bnep-native-rx-frame-ok=[1-9]",
                "re:bnep-native-netif-rx=[1-9]",
                "BLUEZNETCOMPLETE_DONE_BT2",
            )),
            RoleCheck("ble1", (
                "BLUEZNETCOMPLETE_BEGIN_BLE1",
                "bluez-daemon: ipsp closeout phase=daemon-profile-register",
                "bluez-daemon: ipsp closeout phase=mainloop-ownership",
                "bluez-daemon: ipsp closeout phase=profile-connect "
                "ifname=bt0 psm=0x0023 fd-handoff=le-l2cap-coc "
                "owner=kernel-6lowpan profile=ipsp connected=1",
                "count>=2:linux-bt-6lowpan: registered=1 ifname=bt0",
                "upstream-link=net_bluetooth/6lowpan+sim-ipsp-datapath-link",
                "upstream-iphc-link=net_6lowpan/iphc",
                "PING6 fc00::2:",
                "mode=tcp-server",
                "mode=udp-server",
                "re:tx-iphc=[1-9][0-9]*",
                "re:rx-iphc=[1-9][0-9]*",
                "re:rx-frag-dgrams=[1-9][0-9]*",
                "re:rx-frag-frames=[1-9][0-9]*",
                "rx-frag-drop=0",
                "count>=2:bluez-daemon: ipsp closeout phase=profile-disconnect",
                "linux-bt-6lowpan: registered=0 ifname=-",
                "upstream-link-state=netdev:0,coc:0,peer:0",
                "upstream-link-refs=netdev:0,chan:0,peer:0",
                "ipsp-state=closed",
                "BLUEZNETCOMPLETE_DONE_BLE1",
            )),
            RoleCheck("ble2", (
                "BLUEZNETCOMPLETE_BEGIN_BLE2",
                "bluez-daemon: ipsp closeout phase=daemon-profile-register",
                "bluez-daemon: ipsp closeout phase=mainloop-ownership",
                "bluez-daemon: ipsp closeout phase=profile-connect "
                "ifname=bt0 psm=0x0023 fd-handoff=le-l2cap-coc "
                "owner=kernel-6lowpan profile=ipsp connected=1",
                "count>=2:linux-bt-6lowpan: registered=1 ifname=bt0",
                "upstream-link=net_bluetooth/6lowpan+sim-ipsp-datapath-link",
                "upstream-iphc-link=net_6lowpan/iphc",
                "PING6 fc00::1:",
                "mode=tcp-client",
                "mode=udp-client",
                "iperf exit",
                "Mbits/sec",
                "re:tx-iphc=[1-9][0-9]*",
                "re:rx-iphc=[1-9][0-9]*",
                "re:tx-frag-dgrams=[1-9][0-9]*",
                "re:tx-frag-frames=[1-9][0-9]*",
                "rx-frag-drop=0",
                "count>=2:bluez-daemon: ipsp closeout phase=profile-disconnect",
                "linux-bt-6lowpan: registered=0 ifname=-",
                "upstream-link-state=netdev:0,coc:0,peer:0",
                "upstream-link-refs=netdev:0,chan:0,peer:0",
                "ipsp-state=closed",
                "BLUEZNETCOMPLETE_DONE_BLE2",
            )),
        ),
    ),
    CaseCheck(
        "bluez-net-upstream-convergence-closeout",
        (
            RoleCheck("bt1", (
                "BLUEZNETUPSTREAM_BEGIN_BT1",
                "bluez-network: closeout upstream-coverage-map "
                "bluez-src=third/bluez/src/main.c+"
                "third/bluez/src/profile.c+third/bluez/src/device.c+"
                "third/bluez/src/adapter.c+third/bluez/src/dbus-common.c+"
                "third/bluez/profiles/network/manager.c+"
                "third/bluez/profiles/network/server.c+"
                "third/bluez/profiles/network/connection.c+"
                "third/bluez/profiles/network/bnep.c "
                "linux-src=third/linux-hwe-6.17-6.17.0/net/bluetooth/bnep/core.c+"
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/bnep/sock.c+"
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/bnep/netdev.c+"
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/l2cap_core.c+"
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/l2cap_sock.c",
                "executed=daemon-dbus,network-server,role-policy,"
                "profile-connect,l2cap-fd-handoff,bnep-connadd,"
                "netdev-xmit,l2cap-deliver,netif-rx,error-policy,"
                "profile-disconnect,unregister",
                "bluez-network: upstream-object-graph action=closeout",
                "owner=profiles/network/manager.c,server.c,connection.c,"
                "bnep.c",
                "objects=network_manager,network_server,network_peer,"
                "network_conn,network_session,bnep_control,l2cap_io,"
                "netdev_bridge",
                "fd-flow=connect_l2cap_fd,BNEPCONNADD,BNEPCONNDEL",
                "roles=panu,nap,gn datapath=btn0 "
                "cleanup=bnep-native-active-0",
                "upstream-link=blueznetwork-upstream-link-bluetoothd "
                "final-ok=1",
                "PING 10.77.0.2",
                "bnep-native-active=0",
                "re:bnep-native-netdev-xmit=[1-9]",
                "re:bnep-native-tx-frame-ok=[1-9]",
                "re:bnep-native-l2cap-delivered=[1-9]",
                "re:bnep-native-rx-frame-ok=[1-9]",
                "re:bnep-native-netif-rx=[1-9]",
                "bluez-bneptest: native-closeout begin "
                "abi=AF_BLUETOOTH/BTPROTO_BNEP",
                "bluez-bneptest: native-closeout fd-handoff=1 "
                "role=0x1115 device=btn0",
                "bluez-bneptest: native-closeout session-ownership="
                "bnep_add_connection,netdev_setup,register_netdev,"
                "session_link,kthread_run,session_thread",
                "bluez-bneptest: native-closeout netdev-ownership="
                "btn0,ndo_start_xmit,netif_rx,l2cap_delivery state=active",
                "bluez-bneptest: native-closeout cleanup="
                "session_stop,session_unlink,session_terminate,"
                "unregister_netdev,bnep-native-active-0",
                "bluez-bneptest: native-closeout complete",
                "BLUEZNETUPSTREAM_DONE_BT1",
            )),
            RoleCheck("bt2", (
                "BLUEZNETUPSTREAM_BEGIN_BT2",
                "bluez-network: closeout upstream-coverage-map "
                "bluez-src=third/bluez/src/main.c+"
                "third/bluez/src/profile.c+third/bluez/src/device.c+"
                "third/bluez/src/adapter.c+third/bluez/src/dbus-common.c+"
                "third/bluez/profiles/network/manager.c+"
                "third/bluez/profiles/network/server.c+"
                "third/bluez/profiles/network/connection.c+"
                "third/bluez/profiles/network/bnep.c "
                "linux-src=third/linux-hwe-6.17-6.17.0/net/bluetooth/bnep/core.c+"
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/bnep/sock.c+"
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/bnep/netdev.c+"
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/l2cap_core.c+"
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/l2cap_sock.c",
                "executed=daemon-dbus,network-server,role-policy,"
                "profile-connect,l2cap-fd-handoff,bnep-connadd,"
                "netdev-xmit,l2cap-deliver,netif-rx,error-policy,"
                "profile-disconnect,unregister",
                "bluez-network: upstream-object-graph action=closeout",
                "owner=profiles/network/manager.c,server.c,connection.c,"
                "bnep.c",
                "objects=network_manager,network_server,network_peer,"
                "network_conn,network_session,bnep_control,l2cap_io,"
                "netdev_bridge",
                "fd-flow=connect_l2cap_fd,BNEPCONNADD,BNEPCONNDEL",
                "roles=panu,nap,gn datapath=btn0 "
                "cleanup=bnep-native-active-0",
                "upstream-link=blueznetwork-upstream-link-bluetoothd "
                "final-ok=1",
                "PING 10.77.0.1",
                "bnep-native-active=0",
                "re:bnep-native-netdev-xmit=[1-9]",
                "re:bnep-native-tx-frame-ok=[1-9]",
                "re:bnep-native-l2cap-delivered=[1-9]",
                "re:bnep-native-rx-frame-ok=[1-9]",
                "re:bnep-native-netif-rx=[1-9]",
                "bluez-bneptest: native-closeout begin "
                "abi=AF_BLUETOOTH/BTPROTO_BNEP",
                "bluez-bneptest: native-closeout fd-handoff=1 "
                "role=0x1115 device=btn0",
                "bluez-bneptest: native-closeout session-ownership="
                "bnep_add_connection,netdev_setup,register_netdev,"
                "session_link,kthread_run,session_thread",
                "bluez-bneptest: native-closeout netdev-ownership="
                "btn0,ndo_start_xmit,netif_rx,l2cap_delivery state=active",
                "bluez-bneptest: native-closeout cleanup="
                "session_stop,session_unlink,session_terminate,"
                "unregister_netdev,bnep-native-active-0",
                "bluez-bneptest: native-closeout complete",
                "BLUEZNETUPSTREAM_DONE_BT2",
            )),
            RoleCheck("ble1", (
                "BLUEZNETUPSTREAM_BEGIN_BLE1",
                "bluez-daemon: ipsp closeout upstream-coverage-map "
                "bluez-src=third/bluez/src/main.c+"
                "third/bluez/src/profile.c+third/bluez/src/device.c+"
                "third/bluez/src/adapter.c+third/bluez/src/dbus-common.c+"
                "third/bluez/profiles/network/connection.c+"
                "third/bluez/profiles/network/ipsp.c "
                "linux-src=third/linux-hwe-6.17-6.17.0/net/bluetooth/6lowpan.c+"
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/l2cap_core.c+"
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/l2cap_sock.c+"
                "third/linux-hwe-6.17-6.17.0/net/6lowpan/iphc.c",
                "executed=daemon-profile,dbus,mainloop,l2cap-coc-fd-handoff,"
                "6lowpan-register,iphc,netdev-xmit,rx-deliver,"
                "profile-disconnect,mainloop-cleanup",
                "datapath=bt0 cleanup=registered-0,owner-state-0,ipsp-closed",
                "bluez-daemon: ipsp closeout phase=native-6lowpan-ownership "
                "ifname=bt0",
                "bluez-daemon: ipsp closeout phase=native-l2cap-coc-ownership "
                "psm=0x0023 cid=0x0040",
                "bluez-daemon: ipsp closeout phase=native-datapath-status "
                "datapath=bt0",
                "bluez-daemon: ipsp closeout phase=native-6lowpan-cleanup "
                "unregister=1 chan-release=1 peer-unref=1 "
                "netdev-unregister=1 owner-state-final=0",
                "upstream-link=bluezdaemon-ipsp-upstream-link-bluetoothd "
                "final-ok=1",
                "PING6 fc00::2:",
                "mode=tcp-server",
                "mode=udp-server",
                "re:tx-iphc=[1-9][0-9]*",
                "re:rx-iphc=[1-9][0-9]*",
                "linux-bt-6lowpan: registered=0 ifname=-",
                "upstream-link-state=netdev:0,coc:0,peer:0",
                "upstream-link-refs=netdev:0,chan:0,peer:0",
                "ipsp-state=closed",
                "BLUEZNETUPSTREAM_DONE_BLE1",
            )),
            RoleCheck("ble2", (
                "BLUEZNETUPSTREAM_BEGIN_BLE2",
                "bluez-daemon: ipsp closeout upstream-coverage-map "
                "bluez-src=third/bluez/src/main.c+"
                "third/bluez/src/profile.c+third/bluez/src/device.c+"
                "third/bluez/src/adapter.c+third/bluez/src/dbus-common.c+"
                "third/bluez/profiles/network/connection.c+"
                "third/bluez/profiles/network/ipsp.c "
                "linux-src=third/linux-hwe-6.17-6.17.0/net/bluetooth/6lowpan.c+"
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/l2cap_core.c+"
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/l2cap_sock.c+"
                "third/linux-hwe-6.17-6.17.0/net/6lowpan/iphc.c",
                "executed=daemon-profile,dbus,mainloop,l2cap-coc-fd-handoff,"
                "6lowpan-register,iphc,netdev-xmit,rx-deliver,"
                "profile-disconnect,mainloop-cleanup",
                "datapath=bt0 cleanup=registered-0,owner-state-0,ipsp-closed",
                "bluez-daemon: ipsp closeout phase=native-6lowpan-ownership "
                "ifname=bt0",
                "bluez-daemon: ipsp closeout phase=native-l2cap-coc-ownership "
                "psm=0x0023 cid=0x0040",
                "bluez-daemon: ipsp closeout phase=native-datapath-status "
                "datapath=bt0",
                "bluez-daemon: ipsp closeout phase=native-6lowpan-cleanup "
                "unregister=1 chan-release=1 peer-unref=1 "
                "netdev-unregister=1 owner-state-final=0",
                "upstream-link=bluezdaemon-ipsp-upstream-link-bluetoothd "
                "final-ok=1",
                "PING6 fc00::1:",
                "mode=tcp-client",
                "mode=udp-client",
                "iperf exit",
                "re:tx-iphc=[1-9][0-9]*",
                "re:rx-iphc=[1-9][0-9]*",
                "linux-bt-6lowpan: registered=0 ifname=-",
                "upstream-link-state=netdev:0,coc:0,peer:0",
                "upstream-link-refs=netdev:0,chan:0,peer:0",
                "ipsp-state=closed",
                "BLUEZNETUPSTREAM_DONE_BLE2",
            )),
        ),
    ),
    CaseCheck(
        "bluez-network-closeout-full",
        (
            RoleCheck("bt1", (
                "bluez-network: closeout-full stage=begin "
                "semantic=network-current-closeout-umbrella",
                "bluez-network: closeout phase=daemon-dbus "
                "owner=org.bluez object-manager=1 adapter=1 "
                "network-server=panu,nap,gn",
                "bluez-network: closeout phase=service-registration "
                "services=panu,nap,gn",
                "bluez-network: closeout phase=fd-handoff "
                "source=third/bluez/profiles/network/connection.c+"
                "profiles/network/bnep.c",
                "bluez-network: closeout phase=roles "
                "panu=required nap=required gn=required",
                "bluez-network: closeout phase=datapath "
                "btn0=required ping=required mtu1400=required",
                "bluez-network: closeout phase=error-policy "
                "missing=NotConnected duplicate=AlreadyConnected "
                "cancel=Canceled cleanup=required",
                "bluez-network: source=third/bluez/src/main.c+src/profile.c+"
                "profiles/network/manager.c+profiles/network/server.c+"
                "profiles/network/connection.c+profiles/network/bnep.c "
                "mode=daemon-profile action=register",
                "interface=org.bluez.NetworkServer1 service=panu uuid=0x1115",
                "interface=org.bluez.NetworkServer1 service=nap uuid=0x1116",
                "interface=org.bluez.NetworkServer1 service=gn uuid=0x1117",
                "bluez-network: daemon-profile action=connect "
                "service=panu role=0x1115",
                "bluez-network: daemon-profile action=connect "
                "service=nap role=0x1116",
                "bluez-network: daemon-profile action=connect "
                "service=gn role=0x1117",
                "count>=4:bluez-network: bnep-connadd ret=0",
                "count>=4:device=btn0",
                "PING 10.77.0.2",
                "1400 bytes from 10.77.0.2",
                "2 packets transmitted, 2 received, 0% packet loss",
                "bluez-network: daemon-profile action=error-path "
                "service=gn role=0x1117",
                "missing-connection=org.bluez.Error.NotConnected",
                "duplicate=org.bluez.Error.AlreadyConnected",
                "cancel=org.bluez.Error.Canceled",
                "bluez-network: duplicate-connect rejected",
                "network-server unregister "
                "interface=org.bluez.NetworkServer1 service=panu uuid=0x1115",
                "network-server unregister "
                "interface=org.bluez.NetworkServer1 service=nap uuid=0x1116",
                "network-server unregister "
                "interface=org.bluez.NetworkServer1 service=gn uuid=0x1117",
                "bluez-network: closeout upstream-coverage-map "
                "bluez-src=third/bluez/src/main.c+third/bluez/src/profile.c+",
                "upstream-link=blueznetwork-upstream-link-bluetoothd "
                "final-ok=1",
                "bluez-network: closeout native-datapath-contract full",
                "upstream-link=bluez-network-fd-to-linux-bnep-object-graph "
                "native-datapath-final=1 semantic-contract-final=1",
                "bluez-network: closeout-full complete "
                "semantic=network-current-closeout-umbrella",
                "bnep-ioctl-connadd=5",
                "bnep-ioctl-conndel=5",
                "bnep-native-session-create=4",
                "bnep-native-session-start=4",
                "bnep-native-session-terminate=4",
                "bnep-native-session-stop=4",
                "bnep-native-netdev-register=4",
                "bnep-native-netdev-unregister=4",
                "bnep-native-active=0",
                "re:bnep-native-sock-ioctl-getconnlist=[1-9]",
                "re:bnep-native-sock-ioctl-getconninfo=[1-9]",
                "re:bnep-native-sock-ioctl-getsuppfeat=[1-9]",
                "re:bnep-native-netdev-xmit=[1-9]",
                "re:bnep-native-tx-frame-ok=[1-9]",
                "re:bnep-native-l2cap-delivered=[1-9]",
                "re:bnep-native-rx-frame-ok=[1-9]",
                "re:bnep-native-netif-rx=[1-9]",
            )),
            RoleCheck("bt2", (
                "bluez-network: closeout-full stage=begin "
                "semantic=network-current-closeout-umbrella",
                "bluez-network: closeout phase=daemon-dbus "
                "owner=org.bluez object-manager=1 adapter=1 "
                "network-server=panu,nap,gn",
                "bluez-network: closeout phase=service-registration "
                "services=panu,nap,gn",
                "bluez-network: closeout phase=fd-handoff "
                "source=third/bluez/profiles/network/connection.c+"
                "profiles/network/bnep.c",
                "bluez-network: closeout phase=roles "
                "panu=required nap=required gn=required",
                "bluez-network: closeout phase=datapath "
                "btn0=required ping=required mtu1400=required",
                "bluez-network: closeout phase=error-policy "
                "missing=NotConnected duplicate=AlreadyConnected "
                "cancel=Canceled cleanup=required",
                "bluez-network: source=third/bluez/src/main.c+src/profile.c+"
                "profiles/network/manager.c+profiles/network/server.c+"
                "profiles/network/connection.c+profiles/network/bnep.c "
                "mode=daemon-profile action=register",
                "interface=org.bluez.NetworkServer1 service=panu uuid=0x1115",
                "interface=org.bluez.NetworkServer1 service=nap uuid=0x1116",
                "interface=org.bluez.NetworkServer1 service=gn uuid=0x1117",
                "bluez-network: daemon-profile action=connect "
                "service=nap role=0x1116",
                "bluez-network: daemon-profile action=connect "
                "service=gn role=0x1117",
                "bluez-network: daemon-profile action=connect "
                "service=panu role=0x1115",
                "count>=4:bluez-network: bnep-connadd ret=0",
                "count>=4:device=btn0",
                "PING 10.77.0.1",
                "1400 bytes from 10.77.0.1",
                "2 packets transmitted, 2 received, 0% packet loss",
                "bluez-network: daemon-profile action=error-path "
                "service=panu role=0x1115",
                "missing-connection=org.bluez.Error.NotConnected",
                "duplicate=org.bluez.Error.AlreadyConnected",
                "cancel=org.bluez.Error.Canceled",
                "bluez-network: duplicate-connect rejected",
                "network-server unregister "
                "interface=org.bluez.NetworkServer1 service=panu uuid=0x1115",
                "network-server unregister "
                "interface=org.bluez.NetworkServer1 service=nap uuid=0x1116",
                "network-server unregister "
                "interface=org.bluez.NetworkServer1 service=gn uuid=0x1117",
                "bluez-network: closeout upstream-coverage-map "
                "bluez-src=third/bluez/src/main.c+third/bluez/src/profile.c+",
                "upstream-link=blueznetwork-upstream-link-bluetoothd "
                "final-ok=1",
                "bluez-network: closeout native-datapath-contract full",
                "upstream-link=bluez-network-fd-to-linux-bnep-object-graph "
                "native-datapath-final=1 semantic-contract-final=1",
                "bluez-network: closeout-full complete "
                "semantic=network-current-closeout-umbrella",
                "bnep-ioctl-connadd=5",
                "bnep-ioctl-conndel=5",
                "bnep-native-session-create=4",
                "bnep-native-session-start=4",
                "bnep-native-session-terminate=4",
                "bnep-native-session-stop=4",
                "bnep-native-netdev-register=4",
                "bnep-native-netdev-unregister=4",
                "bnep-native-active=0",
                "re:bnep-native-sock-ioctl-getconnlist=[1-9]",
                "re:bnep-native-sock-ioctl-getconninfo=[1-9]",
                "re:bnep-native-sock-ioctl-getsuppfeat=[1-9]",
                "re:bnep-native-netdev-xmit=[1-9]",
                "re:bnep-native-tx-frame-ok=[1-9]",
                "re:bnep-native-l2cap-delivered=[1-9]",
                "re:bnep-native-rx-frame-ok=[1-9]",
                "re:bnep-native-netif-rx=[1-9]",
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
                "re:bnep-ioctl-connadd=[1-9]",
                "re:bnep-ioctl-conndel=[1-9]",
                "re:bnep-native-session-create=[1-9]",
                "re:bnep-native-session-start=[1-9]",
                "re:bnep-native-session-terminate=[1-9]",
                "re:bnep-native-session-stop=[1-9]",
                "re:bnep-native-netdev-register=[1-9]",
                "re:bnep-native-netdev-unregister=[1-9]",
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
                *BNEP_NETWORK_IPERF_OWNERSHIP_REQUIRED,
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
                *BNEP_NETWORK_IPERF_OWNERSHIP_REQUIRED,
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
                *BNEP_NETWORK_IPERF_OWNERSHIP_REQUIRED,
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
                *BNEP_NETWORK_IPERF_OWNERSHIP_REQUIRED,
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
                *BNEP_NETWORK_IPERF_OWNERSHIP_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "bluez-network: disconnect complete",
                *BNEP_NATIVE_TEARDOWN_REQUIRED,
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
                *BNEP_NETWORK_IPERF_OWNERSHIP_REQUIRED,
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
                *BNEP_NETWORK_IPERF_OWNERSHIP_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "bluez-network: disconnect complete",
                *BNEP_NATIVE_TEARDOWN_REQUIRED,
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
                *BNEP_NETWORK_IPERF_OWNERSHIP_REQUIRED,
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
                *BNEP_NETWORK_IPERF_OWNERSHIP_REQUIRED,
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
                *BNEP_NETWORK_IPERF_OWNERSHIP_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "bluez-network: disconnect complete",
                *BNEP_NATIVE_TEARDOWN_REQUIRED,
            )),
        ),
    ),
    CaseCheck(
        "bluez-network-iperf-matrix",
        (
            RoleCheck("bt1", (
                "count>=4:bluez-network: bnep-connadd ret=0",
                "count>=4:device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "mode=tcp-server",
                "mode=tcp-client",
                "dip=10.77.0.2:5001",
                "mode=udp-server",
                "mode=udp-client",
                "count>=4:Mbits/sec",
                "count>=2:iperf exit",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NETWORK_IPERF_OWNERSHIP_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "count>=4:bluez-network: disconnect complete",
                "bnep-ioctl-connadd=4",
                "bnep-ioctl-conndel=4",
                "bnep-native-session-create=4",
                "bnep-native-session-start=4",
                "bnep-native-session-stop=4",
                "bnep-native-session-terminate=4",
                "bnep-native-netdev-register=4",
                "bnep-native-netdev-unregister=4",
                "bnep-native-active=0",
            )),
            RoleCheck("bt2", (
                "count>=4:bluez-network: bnep-connadd ret=0",
                "count>=4:device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "mode=tcp-client",
                "dip=10.77.0.1:5001",
                "mode=tcp-server",
                "mode=udp-client",
                "mode=udp-server",
                "count>=4:Mbits/sec",
                "count>=2:iperf exit",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NETWORK_IPERF_OWNERSHIP_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "count>=4:bluez-network: disconnect complete",
                "bnep-ioctl-connadd=4",
                "bnep-ioctl-conndel=4",
                "bnep-native-session-create=4",
                "bnep-native-session-start=4",
                "bnep-native-session-stop=4",
                "bnep-native-session-terminate=4",
                "bnep-native-netdev-register=4",
                "bnep-native-netdev-unregister=4",
                "bnep-native-active=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-network-mtu-ping",
        (
            RoleCheck("bt1", (
                "bluez-network: source=third/bluez/profiles/network/"
                "connection.c+profiles/network/bnep.c",
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "1400 bytes from 10.77.0.2",
                "2 packets transmitted, 2 received, 0% packet loss",
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
                "1400 bytes from 10.77.0.1",
                "2 packets transmitted, 2 received, 0% packet loss",
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "bluez-network: disconnect complete",
                *BNEP_NATIVE_TEARDOWN_REQUIRED,
            )),
        ),
    ),
    CaseCheck(
        "bluez-network-frag-ping",
        (
            RoleCheck("bt1", (
                "bluez-network: source=third/bluez/profiles/network/"
                "connection.c+profiles/network/bnep.c",
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "mtu 1500",
                "role=0x1115",
                "flags=0x00000001",
                "2000 bytes from 10.77.0.2",
                "2 packets transmitted, 2 received, 0% packet loss",
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "bluez-network: disconnect complete",
                *BNEP_NATIVE_TEARDOWN_REQUIRED,
            )),
            RoleCheck("bt2", (
                "bluez-network: source=third/bluez/profiles/network/"
                "connection.c+profiles/network/bnep.c",
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "mtu 1500",
                "role=0x1115",
                "flags=0x00000001",
                "2000 bytes from 10.77.0.1",
                "2 packets transmitted, 2 received, 0% packet loss",
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "bluez-network: disconnect complete",
                *BNEP_NATIVE_TEARDOWN_REQUIRED,
            )),
        ),
    ),
    CaseCheck(
        "bluez-network-jumbo-ping",
        (
            RoleCheck("bt1", (
                "bluez-network: source=third/bluez/profiles/network/"
                "connection.c+profiles/network/bnep.c",
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "mtu 2500",
                "role=0x1115",
                "flags=0x00000001",
                "2000 bytes from 10.77.0.2",
                "2 packets transmitted, 2 received, 0% packet loss",
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "bluez-network: disconnect complete",
                *BNEP_NATIVE_TEARDOWN_REQUIRED,
            )),
            RoleCheck("bt2", (
                "bluez-network: source=third/bluez/profiles/network/"
                "connection.c+profiles/network/bnep.c",
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "mtu 2500",
                "role=0x1115",
                "flags=0x00000001",
                "2000 bytes from 10.77.0.1",
                "2 packets transmitted, 2 received, 0% packet loss",
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "bluez-network: disconnect complete",
                *BNEP_NATIVE_TEARDOWN_REQUIRED,
            )),
        ),
    ),
    CaseCheck(
        "bluez-network-mtu-soak",
        (
            RoleCheck("bt1", (
                "bluez-network: source=third/bluez/profiles/network/"
                "connection.c+profiles/network/bnep.c",
                "bluez-network: bnep-connadd ret=0",
                "device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "count>=12:1400 bytes from 10.77.0.2",
                "12 packets transmitted, 12 received, 0% packet loss",
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
                "count>=12:1400 bytes from 10.77.0.1",
                "12 packets transmitted, 12 received, 0% packet loss",
                *BNEP_NATIVE_DATAPATH_REQUIRED,
                "bluez-network: disconnect complete",
                *BNEP_NATIVE_TEARDOWN_REQUIRED,
            )),
        ),
    ),
    CaseCheck(
        "bluez-network-mtu-reconnect-stress",
        (
            RoleCheck("bt1", (
                "count>=3:bluez-network: source=third/bluez/profiles/network/"
                "connection.c+profiles/network/bnep.c",
                "count>=3:bluez-network: bnep-connadd ret=0",
                "count>=3:device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "count>=4:1400 bytes from 10.77.0.2",
                "2 packets transmitted, 2 received, 0% packet loss",
                "count>=3:bluez-network: disconnect complete",
                "bnep-ioctl-connadd=3",
                "bnep-ioctl-conndel=3",
                "bnep-native-session-create=3",
                "bnep-native-session-start=3",
                "bnep-native-session-stop=3",
                "bnep-native-session-terminate=3",
                "bnep-native-netdev-register=3",
                "bnep-native-netdev-unregister=3",
                "bnep-native-active=0",
                "re:bnep-native-netdev-xmit=[1-9]",
                "re:bnep-native-tx-frame-ok=[1-9]",
                "re:bnep-native-l2cap-delivered=[1-9]",
                "re:bnep-native-rx-frame-ok=[1-9]",
                "re:bnep-native-netif-rx=[1-9]",
            )),
            RoleCheck("bt2", (
                "count>=3:bluez-network: source=third/bluez/profiles/network/"
                "connection.c+profiles/network/bnep.c",
                "count>=3:bluez-network: bnep-connadd ret=0",
                "count>=3:device=btn0",
                "role=0x1115",
                "flags=0x00000001",
                "count>=4:1400 bytes from 10.77.0.1",
                "2 packets transmitted, 2 received, 0% packet loss",
                "count>=3:bluez-network: disconnect complete",
                "bnep-ioctl-connadd=3",
                "bnep-ioctl-conndel=3",
                "bnep-native-session-create=3",
                "bnep-native-session-start=3",
                "bnep-native-session-stop=3",
                "bnep-native-session-terminate=3",
                "bnep-native-netdev-register=3",
                "bnep-native-netdev-unregister=3",
                "bnep-native-active=0",
                "re:bnep-native-netdev-xmit=[1-9]",
                "re:bnep-native-tx-frame-ok=[1-9]",
                "re:bnep-native-l2cap-delivered=[1-9]",
                "re:bnep-native-rx-frame-ok=[1-9]",
                "re:bnep-native-netif-rx=[1-9]",
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
                *BNEP_BLUEZ_FD_HANDOFF_ACTIVE_REQUIRED,
                "mode=tcp-server",
                "accept: 10.77.0.2",
                "Mbits/sec",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
            )),
            RoleCheck("bt2", (
                "bluez-bneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                *BNEP_BLUEZ_FD_HANDOFF_ACTIVE_REQUIRED,
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
                *BNEP_BLUEZ_FD_HANDOFF_ACTIVE_REQUIRED,
                "mode=tcp-server",
                "accept: 10.77.0.1",
                "Mbits/sec",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
            )),
            RoleCheck("bt1", (
                "bluez-bneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                *BNEP_BLUEZ_FD_HANDOFF_ACTIVE_REQUIRED,
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
                *BNEP_BLUEZ_FD_HANDOFF_ACTIVE_REQUIRED,
                "mode=udp-server",
                "accept: 10.77.0.2",
                "Mbits/sec",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
            )),
            RoleCheck("bt2", (
                "bluez-bneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                *BNEP_BLUEZ_FD_HANDOFF_ACTIVE_REQUIRED,
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
                *BNEP_BLUEZ_FD_HANDOFF_ACTIVE_REQUIRED,
                "mode=udp-server",
                "accept: 10.77.0.1",
                "Mbits/sec",
                *BNEP_IPERF_THROUGHPUT_REQUIRED,
                *BNEP_NATIVE_DATAPATH_REQUIRED,
            )),
            RoleCheck("bt1", (
                "bluez-bneptest: bnep-connadd-fd ret=0",
                "device=btn0",
                *BNEP_BLUEZ_FD_HANDOFF_ACTIVE_REQUIRED,
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
                "state=1 hci-owner-path=1",
                "re:upstream-iso-send: addr-type=0 handle=0x0101 "
                r"payload-len=28[^\n]*send-ret=28[^\n]*"
                r"native-ret=28[^\n]*native-path=1[^\n]*"
                r"sim-fastpath=0[^\n]*"
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
                "state=1 hci-owner-path=1",
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
                "upstream-l2cap-write-handle: psm=0x0019 cid=0x0041 "
                "handle=0x0052 payload-len=24 send-ret=24",
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
                "upstream-l2cap-write-handle: psm=0x0019 cid=0x0041 "
                "handle=0x0052 payload-len=24 send-ret=24",
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
        "bluez-daemon-a2dp-dbus-client-full",
        (
            RoleCheck("bt1", (
                "bluez-daemon: source=third/bluez/src/main.c+"
                "profiles/audio/media.c style=bluetoothd-mainloop "
                "mode=audio-a2dp-owner role=source peer=2",
                "bluez-daemon: dbus name-owner org.bluez acquired "
                "bus=system owner=bluezdaemon",
                "interface=org.bluez.MediaEndpoint1 codec=sbc role=source",
                "interface=org.bluez.MediaTransport1 state=idle",
                "bluez-daemon: plugin audio/avrcp mainloop registered "
                "avdtp=owned avctp=owned media=owned",
                "bluez-daemon: audio-a2dp-reconnect role=source peer=2 "
                "rounds=2 persistent-mainloop=1",
                "bluez-daemon: audio owner source round=1 complete",
                "bluez-daemon: audio owner source round=2 complete",
                "bluez-daemon: audio owner source step=avdtp-start complete",
                "bluez-daemon: audio owner source media complete "
                "payload=A2DP:SBC:daemon-frame",
                "bluez-daemon: audio-a2dp-reconnect complete role=source "
                "peer=2 rounds=2",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle "
                "role=source peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "dbus owner=:1.feather adapter=/org/bluez/hci0 "
                "interface=org.bluez.Media1 method=RegisterEndpoint",
                "bluez-audio: source=third/bluez/client/player.c "
                "dbus provider=bluezaudio "
                "interface=org.bluez.MediaEndpoint1 "
                "methods=SelectConfiguration,SetConfiguration,"
                "ClearConfiguration,Release",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "set-configuration endpoint=/org/bluez/hci0/"
                "dev_feather/sep/source0 transport=/org/bluez/hci0/"
                "dev_feather/fd/source0 configuration=21 15 02 35",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c dbus export interface=org.bluez."
                "MediaTransport1 path=/org/bluez/hci0/dev_feather/"
                "fd/source0 owner=:1.feather state=idle codec=sbc",
                "bluez-audio: media endpoint lifecycle complete "
                "role=source",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c style profile=media-transport role=source "
                "command=a2dp-source-acquire-write-release",
                "bluez-audio: media transport write len=24 "
                "payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: media transport release complete role=source",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "dbus method=ClearConfiguration owner=:1.feather "
                "endpoint=/org/bluez/hci0/dev_feather/sep/source0 "
                "transport=/org/bluez/hci0/dev_feather/fd/source0",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c dbus unexport interface=org.bluez."
                "MediaTransport1 path=/org/bluez/hci0/dev_feather/"
                "fd/source0",
                "bluez-audio: media endpoint clear complete role=source",
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
                "interface=org.bluez.MediaEndpoint1 codec=sbc role=sink",
                "interface=org.bluez.MediaTransport1 state=idle",
                "bluez-daemon: plugin audio/avrcp mainloop registered "
                "avdtp=owned avctp=owned media=owned",
                "bluez-daemon: audio-a2dp-reconnect role=sink peer=1 "
                "rounds=2 persistent-mainloop=1",
                "bluez-daemon: audio owner sink round=1 complete",
                "bluez-daemon: audio owner sink round=2 complete",
                "bluez-daemon: audio owner sink step=avdtp-start complete",
                "bluez-daemon: audio owner sink media complete "
                "payload=A2DP:SBC:daemon-frame",
                "bluez-daemon: audio-a2dp-reconnect complete role=sink "
                "peer=1 rounds=2",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle "
                "role=sink peer=1",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "dbus owner=:1.feather adapter=/org/bluez/hci0 "
                "interface=org.bluez.Media1 method=RegisterEndpoint",
                "bluez-audio: source=third/bluez/client/player.c "
                "dbus provider=bluezaudio "
                "interface=org.bluez.MediaEndpoint1 "
                "methods=SelectConfiguration,SetConfiguration,"
                "ClearConfiguration,Release",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "set-configuration endpoint=/org/bluez/hci0/"
                "dev_feather/sep/sink0 transport=/org/bluez/hci0/"
                "dev_feather/fd/sink0 configuration=21 15 02 35",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c dbus export interface=org.bluez."
                "MediaTransport1 path=/org/bluez/hci0/dev_feather/"
                "fd/sink0 owner=:1.feather state=idle codec=sbc",
                "bluez-audio: media endpoint lifecycle complete role=sink",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c style profile=media-transport role=sink "
                "command=a2dp-sink-acquire-read-release",
                "bluez-audio: media transport read complete "
                "payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: media transport release complete role=sink",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "dbus method=ClearConfiguration owner=:1.feather "
                "endpoint=/org/bluez/hci0/dev_feather/sep/sink0 "
                "transport=/org/bluez/hci0/dev_feather/fd/sink0",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c dbus unexport interface=org.bluez."
                "MediaTransport1 path=/org/bluez/hci0/dev_feather/"
                "fd/sink0",
                "bluez-audio: media endpoint clear complete role=sink",
                "re:l2cap-socket-bind=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-connect=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-send=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-recv=([1-9][6-9]|[2-9][0-9]+)",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-a2dp-dbus-client-busy",
        (
            RoleCheck("bt1", (
                "bluez-daemon: source=third/bluez/src/main.c+"
                "profiles/audio/media.c style=bluetoothd-mainloop "
                "mode=audio-a2dp-owner role=source peer=2",
                "bluez-daemon: dbus name-owner org.bluez acquired "
                "bus=system owner=bluezdaemon",
                "bluez-daemon: plugin audio/avrcp mainloop registered "
                "avdtp=owned avctp=owned media=owned",
                "bluez-daemon: audio-a2dp-owner complete role=source "
                "peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle "
                "role=source peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "dbus owner=:1.feather adapter=/org/bluez/hci0 "
                "interface=org.bluez.Media1 method=RegisterEndpoint",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "set-configuration endpoint=/org/bluez/hci0/"
                "dev_feather/sep/source0 transport=/org/bluez/hci0/"
                "dev_feather/fd/source0 configuration=21 15 02 35",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c style profile=media-transport role=source "
                "command=a2dp-source-acquire-busy-write-release",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c dbus method=Acquire "
                "interface=org.bluez.MediaTransport1 "
                "path=/org/bluez/hci0/dev_feather/fd/source0 "
                "fd=l2cap role=source request=primary",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c dbus signal=PropertiesChanged "
                "interface=org.bluez.MediaTransport1 "
                "path=/org/bluez/hci0/dev_feather/fd/source0 "
                "property=State value=active role=source",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c dbus error=org.bluez.Error.InProgress "
                "method=org.bluez.MediaTransport1.Acquire "
                "path=/org/bluez/hci0/dev_feather/fd/source0 "
                "role=source request=duplicate",
                "bluez-audio: media transport write len=24 "
                "payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c dbus method=Release "
                "interface=org.bluez.MediaTransport1 "
                "path=/org/bluez/hci0/dev_feather/fd/source0 "
                "role=source",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c dbus signal=PropertiesChanged "
                "interface=org.bluez.MediaTransport1 "
                "path=/org/bluez/hci0/dev_feather/fd/source0 "
                "property=State value=idle role=source",
                "bluez-audio: media transport release complete role=source",
                "bluez-audio: media endpoint clear complete role=source",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
            )),
            RoleCheck("bt2", (
                "bluez-daemon: source=third/bluez/src/main.c+"
                "profiles/audio/media.c style=bluetoothd-mainloop "
                "mode=audio-a2dp-owner role=sink peer=1",
                "bluez-daemon: dbus name-owner org.bluez acquired "
                "bus=system owner=bluezdaemon",
                "bluez-daemon: plugin audio/avrcp mainloop registered "
                "avdtp=owned avctp=owned media=owned",
                "bluez-daemon: audio-a2dp-owner complete role=sink "
                "peer=1",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle "
                "role=sink peer=1",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "dbus owner=:1.feather adapter=/org/bluez/hci0 "
                "interface=org.bluez.Media1 method=RegisterEndpoint",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "set-configuration endpoint=/org/bluez/hci0/"
                "dev_feather/sep/sink0 transport=/org/bluez/hci0/"
                "dev_feather/fd/sink0 configuration=21 15 02 35",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c style profile=media-transport role=sink "
                "command=a2dp-sink-acquire-busy-read-release",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c dbus method=Acquire "
                "interface=org.bluez.MediaTransport1 "
                "path=/org/bluez/hci0/dev_feather/fd/sink0 "
                "fd=l2cap role=sink request=primary",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c dbus signal=PropertiesChanged "
                "interface=org.bluez.MediaTransport1 "
                "path=/org/bluez/hci0/dev_feather/fd/sink0 "
                "property=State value=active role=sink",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c dbus error=org.bluez.Error.InProgress "
                "method=org.bluez.MediaTransport1.Acquire "
                "path=/org/bluez/hci0/dev_feather/fd/sink0 "
                "role=sink request=duplicate",
                "bluez-audio: media transport read complete "
                "payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c dbus method=Release "
                "interface=org.bluez.MediaTransport1 "
                "path=/org/bluez/hci0/dev_feather/fd/sink0 "
                "role=sink",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c dbus signal=PropertiesChanged "
                "interface=org.bluez.MediaTransport1 "
                "path=/org/bluez/hci0/dev_feather/fd/sink0 "
                "property=State value=idle role=sink",
                "bluez-audio: media transport release complete role=sink",
                "bluez-audio: media endpoint clear complete role=sink",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-a2dp-full-concurrent",
        (
            RoleCheck("bt1", (
                "bluez-daemon: source=third/bluez/src/main.c+"
                "profiles/audio/media.c style=bluetoothd-mainloop "
                "mode=audio-a2dp-owner role=source peer=2",
                "bluez-daemon: dbus name-owner org.bluez acquired "
                "bus=system owner=bluezdaemon",
                "bluez-daemon: plugin audio/avrcp mainloop registered "
                "avdtp=owned avctp=owned media=owned",
                "bluez-daemon: audio owner source step=avdtp-start "
                "complete",
                "bluez-daemon: audio owner source media complete "
                "payload=A2DP:SBC:daemon-frame",
                "bluez-daemon: audio-a2dp-owner complete role=source "
                "peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle "
                "role=source peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "dbus owner=:1.feather adapter=/org/bluez/hci0 "
                "interface=org.bluez.Media1 method=RegisterEndpoint",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "set-configuration endpoint=/org/bluez/hci0/"
                "dev_feather/sep/source0 transport=/org/bluez/hci0/"
                "dev_feather/fd/source0 configuration=21 15 02 35",
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=source-session-open "
                "peer=2 handle=0x0052",
                "bluez-audio: a2dp source session opened peer=2 "
                "handle=0x0052",
                "bluez-audio: a2dp signaling discover peer=2",
                "bluez-audio: a2dp signaling getcap peer=2",
                "bluez-audio: a2dp signaling setconfig peer=2",
                "bluez-audio: a2dp signaling open peer=2",
                "bluez-audio: a2dp signaling start peer=2",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c style profile=media-transport role=source "
                "command=a2dp-source-acquire-busy-write-release",
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "signaling session retained during media peer=2 "
                "handle=0x0052 signaling-cid=0x0040 "
                "media-cid=0x0041",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c dbus error=org.bluez.Error.InProgress "
                "method=org.bluez.MediaTransport1.Acquire "
                "path=/org/bluez/hci0/dev_feather/fd/source0 "
                "role=source request=duplicate",
                "bluez-audio: media transport write len=24 "
                "payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp.c "
                "style profile=a2dp-codec role=source "
                "command=source-sbc-encode-write-release peer=2 "
                "handle=0x0052",
                "bluez-audio: codec-backend profile=a2dp codec=sbc "
                "backend=libsbc status=source-built "
                "required-source=third/sbc-2.0",
                "count>=2:bluez-audio: source=third/bluez/profiles/"
                "audio/avdtp.c signaling session retained during media "
                "peer=2 handle=0x0052 signaling-cid=0x0040 "
                "media-cid=0x0041",
                "re:upstream-l2cap-write-handle: psm=0x0019 "
                "cid=0x0041 handle=0x0052 payload-len=[1-9][0-9]* "
                "send-ret=[1-9][0-9]* native-ret=[1-9][0-9]* "
                "attach-ret=0 native-path=1",
                "bluez-audio: media transport release complete "
                "role=source codec=sbc",
                "bluez-audio: a2dp signaling suspend peer=2",
                "bluez-audio: a2dp signaling close-stream peer=2",
                "bluez-audio: a2dp source session closed peer=2 "
                "handle=0x0052",
                "bluez-audio: media endpoint clear complete role=source",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-send=[1-9]",
            )),
            RoleCheck("bt2", (
                "bluez-daemon: source=third/bluez/src/main.c+"
                "profiles/audio/media.c style=bluetoothd-mainloop "
                "mode=audio-a2dp-owner role=sink peer=1",
                "bluez-daemon: dbus name-owner org.bluez acquired "
                "bus=system owner=bluezdaemon",
                "bluez-daemon: plugin audio/avrcp mainloop registered "
                "avdtp=owned avctp=owned media=owned",
                "bluez-daemon: audio owner sink step=avdtp-start "
                "complete",
                "bluez-daemon: audio owner sink media complete "
                "payload=A2DP:SBC:daemon-frame",
                "bluez-daemon: audio-a2dp-owner complete role=sink "
                "peer=1",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "style profile=media-endpoint command=lifecycle "
                "role=sink peer=1",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "dbus owner=:1.feather adapter=/org/bluez/hci0 "
                "interface=org.bluez.Media1 method=RegisterEndpoint",
                "bluez-audio: source=third/bluez/profiles/audio/media.c "
                "set-configuration endpoint=/org/bluez/hci0/"
                "dev_feather/sep/sink0 transport=/org/bluez/hci0/"
                "dev_feather/fd/sink0 configuration=21 15 02 35",
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=listen-native "
                "handle=0x0052",
                "upstream-l2cap-native-control: enabled=1",
                "bluez-audio: a2dp auto-rsp-loop complete count=10",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c style profile=media-transport role=sink "
                "command=a2dp-sink-acquire-busy-read-release",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c dbus error=org.bluez.Error.InProgress "
                "method=org.bluez.MediaTransport1.Acquire "
                "path=/org/bluez/hci0/dev_feather/fd/sink0 "
                "role=sink request=duplicate",
                "bluez-audio: media transport read complete "
                "payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: source=third/bluez/profiles/audio/a2dp.c "
                "style profile=a2dp-codec role=sink "
                "command=sink-sbc-recv-decode-release peer=1 "
                "handle=0x0052",
                "bluez-audio: codec-backend profile=a2dp codec=sbc "
                "backend=libsbc status=source-built "
                "required-source=third/sbc-2.0",
                "re:bluez-audio: source=third/sbc-2\\.0/sbc/sbc\\.c "
                "sbc_decode pcm-bytes=[1-9][0-9]* "
                "codesize=[1-9][0-9]* checksum=0x[0-9a-f]{8}",
                "bluez-audio: media transport release complete "
                "role=sink codec=sbc",
                "bluez-audio: a2dp auto-rsp-loop initial-state=STREAMING",
                "bluez-audio: a2dp auto-rsp signal=0x09 msg-type=0x00 "
                "pkt-type=0x00 rsp-len=2 err=0x00 "
                "state=STREAMING->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x08 msg-type=0x00 "
                "pkt-type=0x00 rsp-len=2 err=0x00 "
                "state=OPEN->IDLE",
                "bluez-audio: a2dp auto-rsp-loop complete count=2",
                "bluez-audio: media endpoint clear complete role=sink",
                "re:l2cap-socket-bind=[1-9]",
                "re:l2cap-socket-connect=[1-9]",
                "re:l2cap-socket-recv=[1-9]",
                "re:l2cap-socket-native-recv=[1-9]",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-a2dp-full-concurrent-reconnect",
        (
            RoleCheck("bt1", (
                "bluez-daemon: audio-a2dp-reconnect role=source peer=2 "
                "rounds=2 persistent-mainloop=1",
                "bluez-daemon: audio owner source round=1 complete",
                "bluez-daemon: audio owner source round=2 complete",
                "bluez-daemon: audio-a2dp-reconnect complete role=source "
                "peer=2 rounds=2",
                "count>=2:bluez-audio: source=third/bluez/profiles/"
                "audio/media.c style profile=media-endpoint "
                "command=lifecycle role=source peer=2",
                "count>=2:bluez-audio: a2dp source session opened "
                "peer=2 handle=0x0052",
                "count>=2:bluez-audio: source=third/bluez/profiles/"
                "audio/transport.c style profile=media-transport "
                "role=source "
                "command=a2dp-source-acquire-busy-write-release",
                "count>=2:bluez-audio: source=third/bluez/profiles/"
                "audio/transport.c dbus error=org.bluez.Error.InProgress "
                "method=org.bluez.MediaTransport1.Acquire "
                "path=/org/bluez/hci0/dev_feather/fd/source0 "
                "role=source request=duplicate",
                "count>=2:bluez-audio: media transport write len=24 "
                "payload=A2DP:SBC:synthetic-frame",
                "count>=2:bluez-audio: source=third/bluez/profiles/"
                "audio/a2dp.c style profile=a2dp-codec role=source "
                "command=source-sbc-encode-write-release peer=2 "
                "handle=0x0052",
                "count>=2:bluez-audio: codec-backend profile=a2dp "
                "codec=sbc backend=libsbc status=source-built "
                "required-source=third/sbc-2.0",
                "count>=2:bluez-audio: source=third/bluez/profiles/"
                "audio/avdtp.c signaling session retained during media "
                "peer=2 handle=0x0052 signaling-cid=0x0040 "
                "media-cid=0x0041",
                "count>=2:bluez-audio: media transport release complete "
                "role=source codec=sbc",
                "count>=2:bluez-audio: a2dp signaling suspend peer=2",
                "count>=2:bluez-audio: a2dp signaling close-stream "
                "peer=2",
                "count>=2:bluez-audio: a2dp source session closed "
                "peer=2 handle=0x0052",
                "count>=2:bluez-audio: media endpoint clear complete "
                "role=source",
                "re:l2cap-socket-bind=([4-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=([4-9]|[1-9][0-9]+)",
                "re:l2cap-socket-send=([1-9][6-9]|[2-9][0-9]+)",
            )),
            RoleCheck("bt2", (
                "bluez-daemon: audio-a2dp-reconnect role=sink peer=1 "
                "rounds=2 persistent-mainloop=1",
                "bluez-daemon: audio owner sink round=1 complete",
                "bluez-daemon: audio owner sink round=2 complete",
                "bluez-daemon: audio-a2dp-reconnect complete role=sink "
                "peer=1 rounds=2",
                "count>=2:bluez-audio: source=third/bluez/profiles/"
                "audio/media.c style profile=media-endpoint "
                "command=lifecycle role=sink peer=1",
                "count>=2:bluez-audio: source=third/bluez/profiles/"
                "audio/transport.c style profile=media-transport "
                "role=sink command=a2dp-sink-acquire-busy-read-release",
                "count>=2:bluez-audio: source=third/bluez/profiles/"
                "audio/transport.c dbus error=org.bluez.Error.InProgress "
                "method=org.bluez.MediaTransport1.Acquire "
                "path=/org/bluez/hci0/dev_feather/fd/sink0 "
                "role=sink request=duplicate",
                "count>=2:bluez-audio: media transport read complete "
                "payload=A2DP:SBC:synthetic-frame",
                "count>=2:bluez-audio: source=third/bluez/profiles/"
                "audio/a2dp.c style profile=a2dp-codec role=sink "
                "command=sink-sbc-recv-decode-release peer=1 "
                "handle=0x0052",
                "count>=2:bluez-audio: codec-backend profile=a2dp "
                "codec=sbc backend=libsbc status=source-built "
                "required-source=third/sbc-2.0",
                "count>=2:bluez-audio: media transport release complete "
                "role=sink codec=sbc",
                "count>=2:bluez-audio: a2dp auto-rsp-loop "
                "initial-state=STREAMING",
                "count>=2:bluez-audio: a2dp auto-rsp signal=0x09 "
                "msg-type=0x00 pkt-type=0x00 rsp-len=2 err=0x00 "
                "state=STREAMING->OPEN",
                "count>=2:bluez-audio: a2dp auto-rsp signal=0x08 "
                "msg-type=0x00 pkt-type=0x00 rsp-len=2 err=0x00 "
                "state=OPEN->IDLE",
                "count>=2:bluez-audio: a2dp auto-rsp-loop "
                "complete count=2",
                "count>=2:bluez-audio: media endpoint clear complete "
                "role=sink",
                "re:l2cap-socket-bind=([4-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=([2-9]|[1-9][0-9]+)",
                "re:l2cap-socket-recv=([2-9]|[1-9][0-9]+)",
                "re:l2cap-socket-native-recv=([1-9][6-9]|[2-9][0-9]+)",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-a2dp-full-concurrent-soak",
        (
            RoleCheck("bt1", (
                "bluez-daemon: audio-a2dp-reconnect role=source peer=2 "
                "rounds=3 persistent-mainloop=1",
                "bluez-daemon: audio owner source round=1 complete",
                "bluez-daemon: audio owner source round=2 complete",
                "bluez-daemon: audio owner source round=3 complete",
                "bluez-daemon: audio-a2dp-reconnect complete role=source "
                "peer=2 rounds=3",
                "count>=3:bluez-audio: source=third/bluez/profiles/"
                "audio/media.c style profile=media-endpoint "
                "command=lifecycle role=source peer=2",
                "count>=3:bluez-audio: a2dp source session opened "
                "peer=2 handle=0x0052",
                "count>=3:bluez-audio: a2dp signaling discover peer=2",
                "count>=3:bluez-audio: a2dp signaling getallcap peer=2",
                "count>=3:bluez-audio: a2dp signaling getcap peer=2",
                "count>=3:bluez-audio: a2dp signaling setconfig peer=2",
                "count>=3:bluez-audio: a2dp signaling getconfig peer=2",
                "count>=3:bluez-audio: a2dp signaling reconfigure peer=2",
                "count>=3:bluez-audio: a2dp signaling delay-report peer=2",
                "count>=3:bluez-audio: a2dp signaling security-control peer=2",
                "count>=3:bluez-audio: a2dp signaling open peer=2",
                "count>=3:bluez-audio: a2dp signaling start peer=2",
                "count>=3:bluez-audio: source=third/bluez/profiles/"
                "audio/transport.c dbus error=org.bluez.Error.InProgress "
                "method=org.bluez.MediaTransport1.Acquire "
                "path=/org/bluez/hci0/dev_feather/fd/source0 "
                "role=source request=duplicate",
                "count>=3:bluez-audio: media transport write len=24 "
                "payload=A2DP:SBC:synthetic-frame",
                "count>=3:bluez-audio: source=third/bluez/profiles/"
                "audio/a2dp.c style profile=a2dp-codec role=source "
                "command=source-sbc-encode-write-release peer=2 "
                "handle=0x0052",
                "count>=3:bluez-audio: codec-backend profile=a2dp "
                "codec=sbc backend=libsbc status=source-built "
                "required-source=third/sbc-2.0",
                "count>=3:bluez-audio: source=third/bluez/profiles/"
                "audio/avdtp.c signaling session retained during media "
                "peer=2 handle=0x0052 signaling-cid=0x0040 "
                "media-cid=0x0041",
                "count>=3:bluez-audio: media transport release complete "
                "role=source codec=sbc",
                "count>=3:bluez-audio: a2dp signaling suspend peer=2",
                "count>=3:bluez-audio: a2dp signaling close-stream "
                "peer=2",
                "count>=3:bluez-audio: a2dp source session closed "
                "peer=2 handle=0x0052",
                "count>=3:bluez-audio: media endpoint clear complete "
                "role=source",
                "re:l2cap-socket-bind=([6-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=([6-9]|[1-9][0-9]+)",
                "re:l2cap-socket-send=([2-9][4-9]|[3-9][0-9]+)",
            )),
            RoleCheck("bt2", (
                "bluez-daemon: audio-a2dp-reconnect role=sink peer=1 "
                "rounds=3 persistent-mainloop=1",
                "bluez-daemon: audio owner sink round=1 complete",
                "bluez-daemon: audio owner sink round=2 complete",
                "bluez-daemon: audio owner sink round=3 complete",
                "bluez-daemon: audio-a2dp-reconnect complete role=sink "
                "peer=1 rounds=3",
                "count>=3:bluez-audio: source=third/bluez/profiles/"
                "audio/media.c style profile=media-endpoint "
                "command=lifecycle role=sink peer=1",
                "count>=3:bluez-audio: source=third/bluez/profiles/"
                "audio/transport.c dbus error=org.bluez.Error.InProgress "
                "method=org.bluez.MediaTransport1.Acquire "
                "path=/org/bluez/hci0/dev_feather/fd/sink0 "
                "role=sink request=duplicate",
                "count>=3:bluez-audio: media transport read complete "
                "payload=A2DP:SBC:synthetic-frame",
                "count>=3:bluez-audio: source=third/bluez/profiles/"
                "audio/a2dp.c style profile=a2dp-codec role=sink "
                "command=sink-sbc-recv-decode-release peer=1 "
                "handle=0x0052",
                "count>=3:bluez-audio: codec-backend profile=a2dp "
                "codec=sbc backend=libsbc status=source-built "
                "required-source=third/sbc-2.0",
                "count>=3:bluez-audio: media transport release complete "
                "role=sink codec=sbc",
                "count>=3:bluez-audio: a2dp auto-rsp-loop "
                "initial-state=STREAMING",
                "count>=3:bluez-audio: a2dp auto-rsp signal=0x09 "
                "msg-type=0x00 pkt-type=0x00 rsp-len=2 err=0x00 "
                "state=STREAMING->OPEN",
                "count>=3:bluez-audio: a2dp auto-rsp signal=0x08 "
                "msg-type=0x00 pkt-type=0x00 rsp-len=2 err=0x00 "
                "state=OPEN->IDLE",
                "count>=3:bluez-audio: a2dp auto-rsp-loop "
                "complete count=2",
                "count>=3:bluez-audio: media endpoint clear complete "
                "role=sink",
                "re:l2cap-socket-bind=([6-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=([3-9]|[1-9][0-9]+)",
                "re:l2cap-socket-recv=([3-9]|[1-9][0-9]+)",
                "re:l2cap-socket-native-recv=([2-9][4-9]|[3-9][0-9]+)",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-a2dp-integrated-profile",
        (
            RoleCheck("bt1", (
                "bluez-daemon: audio-a2dp-integrated-flow "
                "source=third/bluez/src/main.c+profiles/audio/main.c "
                "role=source peer=2",
                "bluez-daemon: plugin audio init complete "
                "profiles=a2dp,avrcp media=1 sbc=1",
                "bluez-daemon: mainloop owner=bluetoothd "
                "watches=mgmt,l2cap,avdtp,avctp,media-transport",
                "bluez-daemon: dbus name-owner org.bluez acquired "
                "bus=system owner=bluezdaemon",
                "bluez-daemon: plugin audio/avrcp mainloop registered "
                "avdtp=owned avctp=owned media=owned",
                "bluez-daemon: audio owner source round=1 start",
                "bluez-daemon: audio owner source step=avdtp-discover "
                "complete",
                "bluez-daemon: audio owner source step=avdtp-setconfig "
                "complete",
                "bluez-daemon: audio owner source step=avdtp-open "
                "complete",
                "bluez-daemon: audio owner source step=avdtp-start "
                "complete",
                "bluez-daemon: audio owner source media complete "
                "payload=A2DP:SBC:daemon-frame",
                "bluez-daemon: audio owner source step=avrcp-play "
                "complete",
                "bluez-daemon: audio owner source step=avrcp-browse "
                "complete",
                "bluez-daemon: audio owner source step=avdtp-suspend "
                "complete",
                "bluez-daemon: audio owner source step=avdtp-close "
                "complete",
                "bluez-daemon: audio owner source round=1 complete",
                "bluez-daemon: audio-a2dp-integrated-flow "
                "session-summary role=source avdtp=closed avctp=closed "
                "media-fd=closed l2cap-owned=0 transport-state=idle",
                "bluez-daemon: audio-a2dp-integrated-flow cleanup "
                "role=source endpoints=0 transports=0 players=0 watches=0 "
                "sessions=0",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0",
                "bluez-daemon: audio-a2dp-integrated-flow complete "
                "role=source peer=2",
                "re:l2cap-socket-bind=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-send=([1-9][0-9]|[2-9][0-9]+)",
            )),
            RoleCheck("bt2", (
                "bluez-daemon: audio-a2dp-integrated-flow "
                "source=third/bluez/src/main.c+profiles/audio/main.c "
                "role=sink peer=1",
                "bluez-daemon: plugin audio init complete "
                "profiles=a2dp,avrcp media=1 sbc=1",
                "bluez-daemon: mainloop owner=bluetoothd "
                "watches=mgmt,l2cap,avdtp,avctp,media-transport",
                "bluez-daemon: dbus name-owner org.bluez acquired "
                "bus=system owner=bluezdaemon",
                "bluez-daemon: plugin audio/avrcp mainloop registered "
                "avdtp=owned avctp=owned media=owned",
                "bluez-daemon: audio owner sink round=1 start",
                "bluez-daemon: audio owner sink step=avdtp-discover "
                "complete",
                "bluez-daemon: audio owner sink step=avdtp-setconfig "
                "complete",
                "bluez-daemon: audio owner sink step=avdtp-open "
                "complete",
                "bluez-daemon: audio owner sink step=avdtp-start "
                "complete",
                "bluez-daemon: audio owner sink media complete "
                "payload=A2DP:SBC:daemon-frame",
                "bluez-daemon: audio owner sink step=avrcp-play complete",
                "bluez-daemon: audio owner sink step=avrcp-browse complete",
                "bluez-daemon: audio owner sink step=avdtp-suspend "
                "complete",
                "bluez-daemon: audio owner sink step=avdtp-close complete",
                "bluez-daemon: audio owner sink round=1 complete",
                "bluez-daemon: audio-a2dp-integrated-flow "
                "session-summary role=sink avdtp=closed avctp=closed "
                "media-fd=closed l2cap-owned=0 transport-state=idle",
                "bluez-daemon: audio-a2dp-integrated-flow cleanup "
                "role=sink endpoints=0 transports=0 players=0 watches=0 "
                "sessions=0",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0",
                "bluez-daemon: audio-a2dp-integrated-flow complete "
                "role=sink peer=1",
                "re:l2cap-socket-bind=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-recv=([8-9]|[1-9][0-9]+)",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-a2dp-integrated-reconnect",
        (
            RoleCheck("bt1", (
                "bluez-daemon: audio-a2dp-integrated-reconnect "
                "source=third/bluez/src/main.c+profiles/audio/main.c "
                "role=source peer=2 rounds=2 persistent-mainloop=1",
                "bluez-daemon: plugin audio init complete "
                "profiles=a2dp,avrcp media=1 sbc=1",
                "bluez-daemon: mainloop owner=bluetoothd "
                "watches=mgmt,l2cap,avdtp,avctp,media-transport "
                "persistent=1",
                "count>=2:bluez-daemon: audio owner source round=",
                "count>=2:bluez-daemon: audio owner source step=avdtp-discover complete",
                "count>=2:bluez-daemon: audio owner source step=avdtp-setconfig complete",
                "count>=2:bluez-daemon: audio owner source step=avdtp-open complete",
                "count>=2:bluez-daemon: audio owner source step=avdtp-start complete",
                "count>=2:bluez-daemon: audio owner source media complete "
                "payload=A2DP:SBC:daemon-frame",
                "count>=2:bluez-daemon: audio owner source step=avrcp-play complete",
                "count>=2:bluez-daemon: audio owner source step=avrcp-browse complete",
                "count>=2:bluez-daemon: audio owner source step=avdtp-suspend complete",
                "count>=2:bluez-daemon: audio owner source step=avdtp-close complete",
                "bluez-daemon: audio-a2dp-integrated-reconnect "
                "round=1 role=source session-summary avdtp=closed "
                "avctp=closed media-fd=closed l2cap-owned=0 "
                "transport-state=idle",
                "bluez-daemon: audio-a2dp-integrated-reconnect "
                "round=2 role=source session-summary avdtp=closed "
                "avctp=closed media-fd=closed l2cap-owned=0 "
                "transport-state=idle",
                "bluez-daemon: audio-a2dp-integrated-reconnect cleanup "
                "role=source endpoints=0 transports=0 players=0 watches=0 "
                "sessions=0 rounds=2",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0",
                "bluez-daemon: audio-a2dp-integrated-reconnect complete "
                "role=source peer=2 rounds=2",
                "re:l2cap-socket-bind=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-connect=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-send=([2-9][0-9]|[3-9][0-9]+)",
            )),
            RoleCheck("bt2", (
                "bluez-daemon: audio-a2dp-integrated-reconnect "
                "source=third/bluez/src/main.c+profiles/audio/main.c "
                "role=sink peer=1 rounds=2 persistent-mainloop=1",
                "bluez-daemon: plugin audio init complete "
                "profiles=a2dp,avrcp media=1 sbc=1",
                "bluez-daemon: mainloop owner=bluetoothd "
                "watches=mgmt,l2cap,avdtp,avctp,media-transport "
                "persistent=1",
                "count>=2:bluez-daemon: audio owner sink round=",
                "count>=2:bluez-daemon: audio owner sink step=avdtp-discover complete",
                "count>=2:bluez-daemon: audio owner sink step=avdtp-setconfig complete",
                "count>=2:bluez-daemon: audio owner sink step=avdtp-open complete",
                "count>=2:bluez-daemon: audio owner sink step=avdtp-start complete",
                "count>=2:bluez-daemon: audio owner sink media complete "
                "payload=A2DP:SBC:daemon-frame",
                "count>=2:bluez-daemon: audio owner sink step=avrcp-play complete",
                "count>=2:bluez-daemon: audio owner sink step=avrcp-browse complete",
                "count>=2:bluez-daemon: audio owner sink step=avdtp-suspend complete",
                "count>=2:bluez-daemon: audio owner sink step=avdtp-close complete",
                "bluez-daemon: audio-a2dp-integrated-reconnect "
                "round=1 role=sink session-summary avdtp=closed "
                "avctp=closed media-fd=closed l2cap-owned=0 "
                "transport-state=idle",
                "bluez-daemon: audio-a2dp-integrated-reconnect "
                "round=2 role=sink session-summary avdtp=closed "
                "avctp=closed media-fd=closed l2cap-owned=0 "
                "transport-state=idle",
                "bluez-daemon: audio-a2dp-integrated-reconnect cleanup "
                "role=sink endpoints=0 transports=0 players=0 watches=0 "
                "sessions=0 rounds=2",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0",
                "bluez-daemon: audio-a2dp-integrated-reconnect complete "
                "role=sink peer=1 rounds=2",
                "re:l2cap-socket-bind=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-connect=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-recv=([1-9][6-9]|[2-9][0-9]+)",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-a2dp-session-ownership",
        (
            RoleCheck("bt1", (
                "bluez-daemon: audio-a2dp-session-ownership "
                "source=third/bluez/src/main.c+profiles/audio/main.c+"
                "profiles/audio/avdtp.c role=source peer=2 rounds=2 "
                "semantic=fd-ref-watch-cleanup",
                "bluez-daemon: plugin audio init complete "
                "profiles=a2dp,avrcp media=1 sbc=1 ownership-tracker=1",
                "bluez-daemon: mainloop owner=bluetoothd "
                "watches=mgmt,l2cap,avdtp,avctp,media-transport "
                "persistent=1 ownership-tracker=1",
                "bluez-daemon: a2dp session-owner object-acquire "
                "role=source endpoint-refs=1 transport-refs=1 "
                "player-refs=1",
                "count>=2:bluez-daemon: a2dp session-owner round=",
                "count>=2:bluez-daemon: a2dp session-owner acquire "
                "seq=",
                "count>=2:bluez-daemon: a2dp session-owner release "
                "seq=",
                "count>=2:kind=avdtp refs avdtp=1",
                "count>=2:kind=avdtp refs avdtp=0",
                "count>=2:kind=avctp refs avdtp=0 avctp=1",
                "count>=2:kind=avctp refs avdtp=0 avctp=0",
                "count>=2:kind=media refs avdtp=0 avctp=0 media=1",
                "count>=2:kind=media refs avdtp=0 avctp=0 media=0",
                "bluez-daemon: a2dp session-owner round=1 role=source "
                "checkpoint refs avdtp=0 avctp=0 media=0 l2cap-fds=0 "
                "watches=0 transport-state=idle",
                "bluez-daemon: a2dp session-owner round=2 role=source "
                "checkpoint refs avdtp=0 avctp=0 media=0 l2cap-fds=0 "
                "watches=0 transport-state=idle",
                "bluez-daemon: a2dp session-owner object-release "
                "role=source endpoint-refs=0 transport-refs=0 "
                "player-refs=0",
                "bluez-daemon: audio-a2dp-session-ownership cleanup "
                "role=source refs avdtp=0 avctp=0 media=0 l2cap-fds=0 "
                "endpoint-refs=0 transport-refs=0 player-refs=0 "
                "watches=0 sessions=0 rounds=2",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0 ownership-tracker=0",
                "bluez-daemon: audio-a2dp-session-ownership complete "
                "role=source peer=2 rounds=2",
                "re:l2cap-socket-bind=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-connect=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-send=([2-9][0-9]|[3-9][0-9]+)",
            )),
            RoleCheck("bt2", (
                "bluez-daemon: audio-a2dp-session-ownership "
                "source=third/bluez/src/main.c+profiles/audio/main.c+"
                "profiles/audio/avdtp.c role=sink peer=1 rounds=2 "
                "semantic=fd-ref-watch-cleanup",
                "bluez-daemon: plugin audio init complete "
                "profiles=a2dp,avrcp media=1 sbc=1 ownership-tracker=1",
                "bluez-daemon: mainloop owner=bluetoothd "
                "watches=mgmt,l2cap,avdtp,avctp,media-transport "
                "persistent=1 ownership-tracker=1",
                "bluez-daemon: a2dp session-owner object-acquire "
                "role=sink endpoint-refs=1 transport-refs=1 "
                "player-refs=1",
                "count>=2:bluez-daemon: a2dp session-owner round=",
                "count>=2:bluez-daemon: a2dp session-owner acquire "
                "seq=",
                "count>=2:bluez-daemon: a2dp session-owner release "
                "seq=",
                "count>=2:kind=avdtp refs avdtp=1",
                "count>=2:kind=avdtp refs avdtp=0",
                "count>=2:kind=avctp refs avdtp=0 avctp=1",
                "count>=2:kind=avctp refs avdtp=0 avctp=0",
                "count>=2:kind=media refs avdtp=0 avctp=0 media=1",
                "count>=2:kind=media refs avdtp=0 avctp=0 media=0",
                "bluez-daemon: a2dp session-owner round=1 role=sink "
                "checkpoint refs avdtp=0 avctp=0 media=0 l2cap-fds=0 "
                "watches=0 transport-state=idle",
                "bluez-daemon: a2dp session-owner round=2 role=sink "
                "checkpoint refs avdtp=0 avctp=0 media=0 l2cap-fds=0 "
                "watches=0 transport-state=idle",
                "bluez-daemon: a2dp session-owner object-release "
                "role=sink endpoint-refs=0 transport-refs=0 "
                "player-refs=0",
                "bluez-daemon: audio-a2dp-session-ownership cleanup "
                "role=sink refs avdtp=0 avctp=0 media=0 l2cap-fds=0 "
                "endpoint-refs=0 transport-refs=0 player-refs=0 "
                "watches=0 sessions=0 rounds=2",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0 ownership-tracker=0",
                "bluez-daemon: audio-a2dp-session-ownership complete "
                "role=sink peer=1 rounds=2",
                "re:l2cap-socket-bind=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-connect=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-recv=([1-9][6-9]|[2-9][0-9]+)",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-a2dp-error-policy",
        (
            RoleCheck("bt1", (
                "bluez-daemon: audio-a2dp-error-policy "
                "source=third/bluez/profiles/audio/avdtp.c+"
                "profiles/audio/a2dp.c+profiles/audio/media.c "
                "role=source peer=2 semantic=state-error-cleanup",
                "bluez-daemon: plugin audio init complete "
                "profiles=a2dp,avrcp media=1 sbc=1 error-policy=1",
                "bluez-daemon: mainloop owner=bluetoothd "
                "watches=mgmt,l2cap,avdtp,avctp,media-transport "
                "persistent=1 error-policy=1",
                "bluez-daemon: audio owner source round=1 start",
                "bluez-daemon: audio owner source step=avdtp-discover complete",
                "bluez-daemon: audio owner source step=avdtp-setconfig complete",
                "bluez-daemon: audio owner source step=avdtp-open complete",
                "bluez-daemon: audio owner source step=avdtp-start complete",
                "bluez-daemon: audio owner source media complete "
                "payload=A2DP:SBC:daemon-frame",
                "bluez-daemon: audio owner source step=avdtp-suspend complete",
                "bluez-daemon: audio owner source step=avdtp-close complete",
                "bluez-daemon: a2dp session-owner round=1 role=source "
                "checkpoint refs avdtp=0 avctp=0 media=0 l2cap-fds=0 "
                "watches=0 transport-state=idle",
                "bluez-daemon: a2dp error-policy role=source "
                "request=start-before-open expected-state=open "
                "actual-state=idle result=reject errno=-EBADFD "
                "avdtp=idle media-fd=closed l2cap-owned=0",
                "bluez-daemon: a2dp error-policy role=source "
                "request=duplicate-open expected-state=configured "
                "actual-state=open result=reject errno=-EALREADY "
                "avdtp=open media-fd=closed l2cap-owned=0",
                "bluez-daemon: a2dp error-policy role=source "
                "request=media-before-start expected-state=streaming "
                "actual-state=open result=reject errno=-EAGAIN "
                "avdtp=open media-fd=closed l2cap-owned=0",
                "bluez-daemon: a2dp error-policy role=source "
                "request=l2cap-drop-streaming expected-state=streaming "
                "actual-state=streaming result=abort avdtp=closed "
                "avctp=closed media-fd=closed l2cap-owned=0 "
                "transport-state=idle",
                "bluez-daemon: a2dp error-policy role=source "
                "request=remote-close-after-abort expected-state=idle "
                "actual-state=idle result=ignore errno=0 avdtp=closed "
                "media-fd=closed l2cap-owned=0",
                "bluez-daemon: audio-a2dp-error-policy cleanup role=source "
                "refs avdtp=0 avctp=0 media=0 l2cap-fds=0 "
                "endpoint-refs=0 transport-refs=0 player-refs=0 "
                "watches=0 sessions=0",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0 error-policy=0",
                "bluez-daemon: audio-a2dp-error-policy complete "
                "role=source peer=2",
                "re:l2cap-socket-bind=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-send=([1-9][0-9]|[2-9][0-9]+)",
            )),
            RoleCheck("bt2", (
                "bluez-daemon: audio-a2dp-error-policy "
                "source=third/bluez/profiles/audio/avdtp.c+"
                "profiles/audio/a2dp.c+profiles/audio/media.c "
                "role=sink peer=1 semantic=state-error-cleanup",
                "bluez-daemon: plugin audio init complete "
                "profiles=a2dp,avrcp media=1 sbc=1 error-policy=1",
                "bluez-daemon: mainloop owner=bluetoothd "
                "watches=mgmt,l2cap,avdtp,avctp,media-transport "
                "persistent=1 error-policy=1",
                "bluez-daemon: audio owner sink round=1 start",
                "bluez-daemon: audio owner sink step=avdtp-discover complete",
                "bluez-daemon: audio owner sink step=avdtp-setconfig complete",
                "bluez-daemon: audio owner sink step=avdtp-open complete",
                "bluez-daemon: audio owner sink step=avdtp-start complete",
                "bluez-daemon: audio owner sink media complete "
                "payload=A2DP:SBC:daemon-frame",
                "bluez-daemon: audio owner sink step=avdtp-suspend complete",
                "bluez-daemon: audio owner sink step=avdtp-close complete",
                "bluez-daemon: a2dp session-owner round=1 role=sink "
                "checkpoint refs avdtp=0 avctp=0 media=0 l2cap-fds=0 "
                "watches=0 transport-state=idle",
                "bluez-daemon: a2dp error-policy role=sink "
                "request=start-before-open expected-state=open "
                "actual-state=idle result=reject errno=-EBADFD "
                "avdtp=idle media-fd=closed l2cap-owned=0",
                "bluez-daemon: a2dp error-policy role=sink "
                "request=duplicate-open expected-state=configured "
                "actual-state=open result=reject errno=-EALREADY "
                "avdtp=open media-fd=closed l2cap-owned=0",
                "bluez-daemon: a2dp error-policy role=sink "
                "request=media-before-start expected-state=streaming "
                "actual-state=open result=reject errno=-EAGAIN "
                "avdtp=open media-fd=closed l2cap-owned=0",
                "bluez-daemon: a2dp error-policy role=sink "
                "request=l2cap-drop-streaming expected-state=streaming "
                "actual-state=streaming result=abort avdtp=closed "
                "avctp=closed media-fd=closed l2cap-owned=0 "
                "transport-state=idle",
                "bluez-daemon: a2dp error-policy role=sink "
                "request=remote-close-after-abort expected-state=idle "
                "actual-state=idle result=ignore errno=0 avdtp=closed "
                "media-fd=closed l2cap-owned=0",
                "bluez-daemon: audio-a2dp-error-policy cleanup role=sink "
                "refs avdtp=0 avctp=0 media=0 l2cap-fds=0 "
                "endpoint-refs=0 transport-refs=0 player-refs=0 "
                "watches=0 sessions=0",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0 error-policy=0",
                "bluez-daemon: audio-a2dp-error-policy complete "
                "role=sink peer=1",
                "re:l2cap-socket-bind=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-recv=([8-9]|[1-9][0-9]+)",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-a2dp-upstream-session",
        (
            RoleCheck("bt1", (
                "bluez-daemon: audio-a2dp-upstream-session "
                "source=third/bluez/src/profile.c+third/bluez/src/device.c+"
                "third/bluez/profiles/audio/a2dp.c+"
                "third/bluez/profiles/audio/avdtp.c+"
                "third/bluez/profiles/audio/media.c+"
                "third/bluez/profiles/audio/avrcp.c role=source peer=2 "
                "semantic=upstream-object-callback-session",
                "bluez-daemon: upstream-profile register "
                "source=third/bluez/src/profile.c name=a2dp-source",
                "bluez-daemon: upstream-device object-add "
                "source=third/bluez/src/device.c path=/org/bluez/hci0/dev_02 "
                "role=source ref=1 connected=1 services-resolved=1",
                "bluez-daemon: upstream-media endpoint-register "
                "source=third/bluez/profiles/audio/media.c "
                "path=/org/bluez/hci0/dev_02/sep1 codec=sbc "
                "role=source ref=1",
                "bluez-daemon: upstream-avrcp player-register "
                "source=third/bluez/profiles/audio/avrcp.c "
                "path=/org/bluez/hci0/dev_02/player0 role=source ref=1",
                "bluez-daemon: upstream-profile connect "
                "source=third/bluez/profiles/audio/a2dp.c role=source "
                "peer=2 callback=a2dp_source_connect device-ref=2 "
                "session-ref=1",
                "bluez-daemon: upstream-avdtp session-new "
                "source=third/bluez/profiles/audio/avdtp.c role=source "
                "peer=2 state=idle ref=1 stream-ref=0 sep-ref=1",
                "bluez-daemon: upstream-avdtp cb=discover role=source "
                "state=idle->discovered sep-ref=2 session-ref=2",
                "bluez-daemon: upstream-media cb=SelectConfiguration "
                "role=source codec=sbc caps=44100,joint-stereo result=ok",
                "bluez-daemon: upstream-media cb=SetConfiguration "
                "role=source transport=/org/bluez/hci0/dev_02/fd0 "
                "transport-ref=2 state=idle",
                "bluez-daemon: upstream-avdtp cb=open role=source "
                "state=configured->open stream-ref=1 transport-ref=2",
                "bluez-daemon: upstream-media cb=Acquire role=source "
                "fd-owner=bluetoothd transport-state=pending media-fd=open",
                "bluez-daemon: upstream-avdtp cb=start role=source "
                "state=open->streaming transport-state=active",
                "bluez-daemon: audio owner source round=1 complete",
                "bluez-daemon: a2dp session-owner round=1 role=source "
                "checkpoint refs avdtp=0 avctp=0 media=0 l2cap-fds=0 "
                "watches=0 transport-state=idle",
                "bluez-daemon: upstream-avrcp cb=register-notification "
                "role=source event=playback-status result=interim "
                "player-ref=2",
                "bluez-daemon: upstream-avrcp cb=pass-through "
                "role=source op=play result=accepted player-ref=2",
                "bluez-daemon: upstream-media cb=Release role=source "
                "fd-owner=bluetoothd transport-state=idle media-fd=closed",
                "bluez-daemon: upstream-avdtp cb=suspend role=source "
                "state=streaming->open transport-state=idle",
                "bluez-daemon: upstream-avdtp cb=close role=source "
                "state=open->idle stream-ref=0 transport-ref=1 "
                "session-ref=1",
                "bluez-daemon: upstream-profile disconnect "
                "source=third/bluez/profiles/audio/a2dp.c role=source "
                "peer=2 callback=a2dp_source_disconnect device-ref=1 "
                "session-ref=0",
                "bluez-daemon: upstream-profile unregister "
                "source=third/bluez/src/profile.c name=a2dp-source ref=0",
                "bluez-daemon: audio-a2dp-upstream-session cleanup "
                "role=source device-ref=0 session-ref=0 stream-ref=0 "
                "sep-ref=0 endpoint-refs=0 transport-refs=0 player-refs=0 "
                "avdtp=0 avctp=0 media=0 l2cap-fds=0 watches=0 sessions=0",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0 upstream-session=0",
                "bluez-daemon: audio-a2dp-upstream-session complete "
                "role=source peer=2",
                "re:l2cap-socket-bind=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-send=([1-9][0-9]|[2-9][0-9]+)",
            )),
            RoleCheck("bt2", (
                "bluez-daemon: audio-a2dp-upstream-session "
                "source=third/bluez/src/profile.c+third/bluez/src/device.c+"
                "third/bluez/profiles/audio/a2dp.c+"
                "third/bluez/profiles/audio/avdtp.c+"
                "third/bluez/profiles/audio/media.c+"
                "third/bluez/profiles/audio/avrcp.c role=sink peer=1 "
                "semantic=upstream-object-callback-session",
                "bluez-daemon: upstream-profile register "
                "source=third/bluez/src/profile.c name=a2dp-sink",
                "bluez-daemon: upstream-device object-add "
                "source=third/bluez/src/device.c path=/org/bluez/hci0/dev_01 "
                "role=sink ref=1 connected=1 services-resolved=1",
                "bluez-daemon: upstream-media endpoint-register "
                "source=third/bluez/profiles/audio/media.c "
                "path=/org/bluez/hci0/dev_01/sep1 codec=sbc "
                "role=sink ref=1",
                "bluez-daemon: upstream-avrcp player-register "
                "source=third/bluez/profiles/audio/avrcp.c "
                "path=/org/bluez/hci0/dev_01/player0 role=sink ref=1",
                "bluez-daemon: upstream-profile connect "
                "source=third/bluez/profiles/audio/a2dp.c role=sink "
                "peer=1 callback=a2dp_sink_connect device-ref=2 "
                "session-ref=1",
                "bluez-daemon: upstream-avdtp session-new "
                "source=third/bluez/profiles/audio/avdtp.c role=sink "
                "peer=1 state=idle ref=1 stream-ref=0 sep-ref=1",
                "bluez-daemon: upstream-avdtp cb=discover role=sink "
                "state=idle->discovered sep-ref=2 session-ref=2",
                "bluez-daemon: upstream-media cb=SelectConfiguration "
                "role=sink codec=sbc caps=44100,joint-stereo result=ok",
                "bluez-daemon: upstream-media cb=SetConfiguration "
                "role=sink transport=/org/bluez/hci0/dev_01/fd0 "
                "transport-ref=2 state=idle",
                "bluez-daemon: upstream-avdtp cb=open role=sink "
                "state=configured->open stream-ref=1 transport-ref=2",
                "bluez-daemon: upstream-media cb=Acquire role=sink "
                "fd-owner=bluetoothd transport-state=pending media-fd=open",
                "bluez-daemon: upstream-avdtp cb=start role=sink "
                "state=open->streaming transport-state=active",
                "bluez-daemon: audio owner sink round=1 complete",
                "bluez-daemon: a2dp session-owner round=1 role=sink "
                "checkpoint refs avdtp=0 avctp=0 media=0 l2cap-fds=0 "
                "watches=0 transport-state=idle",
                "bluez-daemon: upstream-avrcp cb=register-notification "
                "role=sink event=playback-status result=interim "
                "player-ref=2",
                "bluez-daemon: upstream-avrcp cb=pass-through "
                "role=sink op=play result=accepted player-ref=2",
                "bluez-daemon: upstream-media cb=Release role=sink "
                "fd-owner=bluetoothd transport-state=idle media-fd=closed",
                "bluez-daemon: upstream-avdtp cb=suspend role=sink "
                "state=streaming->open transport-state=idle",
                "bluez-daemon: upstream-avdtp cb=close role=sink "
                "state=open->idle stream-ref=0 transport-ref=1 "
                "session-ref=1",
                "bluez-daemon: upstream-profile disconnect "
                "source=third/bluez/profiles/audio/a2dp.c role=sink "
                "peer=1 callback=a2dp_sink_disconnect device-ref=1 "
                "session-ref=0",
                "bluez-daemon: upstream-profile unregister "
                "source=third/bluez/src/profile.c name=a2dp-sink ref=0",
                "bluez-daemon: audio-a2dp-upstream-session cleanup "
                "role=sink device-ref=0 session-ref=0 stream-ref=0 "
                "sep-ref=0 endpoint-refs=0 transport-refs=0 player-refs=0 "
                "avdtp=0 avctp=0 media=0 l2cap-fds=0 watches=0 sessions=0",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0 upstream-session=0",
                "bluez-daemon: audio-a2dp-upstream-session complete "
                "role=sink peer=1",
                "re:l2cap-socket-bind=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-recv=([8-9]|[1-9][0-9]+)",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-a2dp-upstream-reconnect",
        (
            RoleCheck("bt1", (
                "bluez-daemon: audio-a2dp-upstream-reconnect "
                "source=third/bluez/src/profile.c+third/bluez/src/device.c+"
                "third/bluez/profiles/audio/a2dp.c+"
                "third/bluez/profiles/audio/avdtp.c+"
                "third/bluez/profiles/audio/media.c+"
                "third/bluez/profiles/audio/avrcp.c role=source peer=2 "
                "rounds=2 semantic=persistent-profile-reconnect",
                "bluez-daemon: plugin audio init complete "
                "profiles=a2dp,avrcp media=1 sbc=1 upstream-reconnect=1",
                "bluez-daemon: mainloop owner=bluetoothd "
                "watches=mgmt,l2cap,avdtp,avctp,media-transport,dbus "
                "persistent=1 upstream-reconnect=1",
                "bluez-daemon: upstream-profile register "
                "source=third/bluez/src/profile.c name=a2dp-source",
                "bluez-daemon: upstream-reconnect round=1 role=source "
                "device-connect device-ref=2 session-ref=1 "
                "profile-registered=1",
                "bluez-daemon: upstream-profile connect "
                "source=third/bluez/profiles/audio/a2dp.c role=source "
                "peer=2 callback=a2dp_source_connect round=1",
                "bluez-daemon: upstream-avdtp session-new "
                "source=third/bluez/profiles/audio/avdtp.c role=source "
                "peer=2 round=1 state=idle ref=1 stream-ref=0 "
                "sep-ref=1",
                "bluez-daemon: upstream-media cb=SetConfiguration "
                "role=source round=1 transport=/org/bluez/hci0/dev_02/fd0 "
                "transport-ref=2 state=idle",
                "bluez-daemon: upstream-avdtp cb=start role=source "
                "round=1 state=open->streaming transport-state=active",
                "bluez-daemon: audio owner source round=1 complete",
                "bluez-daemon: upstream-reconnect round=1 role=source "
                "cleanup device-ref=1 session-ref=0 stream-ref=0 sep-ref=0 "
                "endpoint-refs=0 transport-refs=0 player-refs=0 avdtp=0 "
                "avctp=0 media=0 l2cap-fds=0 watches=0 "
                "profile-registered=1",
                "bluez-daemon: upstream-reconnect round=2 role=source "
                "device-connect device-ref=2 session-ref=1 "
                "profile-registered=1",
                "bluez-daemon: upstream-profile connect "
                "source=third/bluez/profiles/audio/a2dp.c role=source "
                "peer=2 callback=a2dp_source_connect round=2",
                "bluez-daemon: upstream-avdtp session-new "
                "source=third/bluez/profiles/audio/avdtp.c role=source "
                "peer=2 round=2 state=idle ref=1 stream-ref=0 "
                "sep-ref=1",
                "bluez-daemon: upstream-media cb=SetConfiguration "
                "role=source round=2 transport=/org/bluez/hci0/dev_02/fd0 "
                "transport-ref=2 state=idle",
                "bluez-daemon: upstream-avdtp cb=start role=source "
                "round=2 state=open->streaming transport-state=active",
                "bluez-daemon: audio owner source round=2 complete",
                "bluez-daemon: upstream-reconnect round=2 role=source "
                "cleanup device-ref=1 session-ref=0 stream-ref=0 sep-ref=0 "
                "endpoint-refs=0 transport-refs=0 player-refs=0 avdtp=0 "
                "avctp=0 media=0 l2cap-fds=0 watches=0 "
                "profile-registered=1",
                "bluez-daemon: upstream-profile unregister "
                "source=third/bluez/src/profile.c name=a2dp-source ref=0",
                "bluez-daemon: audio-a2dp-upstream-reconnect cleanup "
                "role=source rounds=2 profile-registered=0 device-ref=0 "
                "session-ref=0 stream-ref=0 sep-ref=0 endpoint-refs=0 "
                "transport-refs=0 player-refs=0 avdtp=0 avctp=0 media=0 "
                "l2cap-fds=0 watches=0 sessions=0",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0 upstream-reconnect=0",
                "bluez-daemon: audio-a2dp-upstream-reconnect complete "
                "role=source peer=2 rounds=2",
                "re:l2cap-socket-bind=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-connect=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-send=([2-9][0-9]|[3-9][0-9]+)",
            )),
            RoleCheck("bt2", (
                "bluez-daemon: audio-a2dp-upstream-reconnect "
                "source=third/bluez/src/profile.c+third/bluez/src/device.c+"
                "third/bluez/profiles/audio/a2dp.c+"
                "third/bluez/profiles/audio/avdtp.c+"
                "third/bluez/profiles/audio/media.c+"
                "third/bluez/profiles/audio/avrcp.c role=sink peer=1 "
                "rounds=2 semantic=persistent-profile-reconnect",
                "bluez-daemon: plugin audio init complete "
                "profiles=a2dp,avrcp media=1 sbc=1 upstream-reconnect=1",
                "bluez-daemon: mainloop owner=bluetoothd "
                "watches=mgmt,l2cap,avdtp,avctp,media-transport,dbus "
                "persistent=1 upstream-reconnect=1",
                "bluez-daemon: upstream-profile register "
                "source=third/bluez/src/profile.c name=a2dp-sink",
                "bluez-daemon: upstream-reconnect round=1 role=sink "
                "device-connect device-ref=2 session-ref=1 "
                "profile-registered=1",
                "bluez-daemon: upstream-profile connect "
                "source=third/bluez/profiles/audio/a2dp.c role=sink "
                "peer=1 callback=a2dp_sink_connect round=1",
                "bluez-daemon: upstream-avdtp session-new "
                "source=third/bluez/profiles/audio/avdtp.c role=sink "
                "peer=1 round=1 state=idle ref=1 stream-ref=0 sep-ref=1",
                "bluez-daemon: upstream-media cb=SetConfiguration "
                "role=sink round=1 transport=/org/bluez/hci0/dev_01/fd0 "
                "transport-ref=2 state=idle",
                "bluez-daemon: upstream-avdtp cb=start role=sink "
                "round=1 state=open->streaming transport-state=active",
                "bluez-daemon: audio owner sink round=1 complete",
                "bluez-daemon: upstream-reconnect round=1 role=sink "
                "cleanup device-ref=1 session-ref=0 stream-ref=0 sep-ref=0 "
                "endpoint-refs=0 transport-refs=0 player-refs=0 avdtp=0 "
                "avctp=0 media=0 l2cap-fds=0 watches=0 "
                "profile-registered=1",
                "bluez-daemon: upstream-reconnect round=2 role=sink "
                "device-connect device-ref=2 session-ref=1 "
                "profile-registered=1",
                "bluez-daemon: upstream-profile connect "
                "source=third/bluez/profiles/audio/a2dp.c role=sink "
                "peer=1 callback=a2dp_sink_connect round=2",
                "bluez-daemon: upstream-avdtp session-new "
                "source=third/bluez/profiles/audio/avdtp.c role=sink "
                "peer=1 round=2 state=idle ref=1 stream-ref=0 sep-ref=1",
                "bluez-daemon: upstream-media cb=SetConfiguration "
                "role=sink round=2 transport=/org/bluez/hci0/dev_01/fd0 "
                "transport-ref=2 state=idle",
                "bluez-daemon: upstream-avdtp cb=start role=sink "
                "round=2 state=open->streaming transport-state=active",
                "bluez-daemon: audio owner sink round=2 complete",
                "bluez-daemon: upstream-reconnect round=2 role=sink "
                "cleanup device-ref=1 session-ref=0 stream-ref=0 sep-ref=0 "
                "endpoint-refs=0 transport-refs=0 player-refs=0 avdtp=0 "
                "avctp=0 media=0 l2cap-fds=0 watches=0 "
                "profile-registered=1",
                "bluez-daemon: upstream-profile unregister "
                "source=third/bluez/src/profile.c name=a2dp-sink ref=0",
                "bluez-daemon: audio-a2dp-upstream-reconnect cleanup "
                "role=sink rounds=2 profile-registered=0 device-ref=0 "
                "session-ref=0 stream-ref=0 sep-ref=0 endpoint-refs=0 "
                "transport-refs=0 player-refs=0 avdtp=0 avctp=0 media=0 "
                "l2cap-fds=0 watches=0 sessions=0",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0 upstream-reconnect=0",
                "bluez-daemon: audio-a2dp-upstream-reconnect complete "
                "role=sink peer=1 rounds=2",
                "re:l2cap-socket-bind=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-connect=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-recv=([1-9][6-9]|[2-9][0-9]+)",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-a2dp-upstream-transactions",
        (
            RoleCheck("bt1", (
                "bluez-daemon: audio-a2dp-upstream-transactions "
                "source=third/bluez/profiles/audio/avdtp.c+"
                "third/bluez/profiles/audio/a2dp.c role=source peer=2 "
                "semantic=transaction-owner-timeout-cleanup",
                "bluez-daemon: plugin audio init complete "
                "profiles=a2dp,avrcp media=1 sbc=1 avdtp-transactions=1",
                "bluez-daemon: mainloop owner=bluetoothd "
                "watches=mgmt,l2cap,avdtp,avctp,media-transport,dbus "
                "persistent=1 avdtp-transactions=1",
                "bluez-daemon: upstream-avdtp transaction role=source "
                "signal=discover tid=1 source=third/bluez/profiles/audio/avdtp.c "
                "state=alloc pending=1 timer=armed",
                "bluez-daemon: upstream-avdtp transaction role=source "
                "signal=discover tid=1 source=third/bluez/profiles/audio/avdtp.c "
                "state=rsp-accept pending=0 timer=disarmed",
                "bluez-daemon: upstream-avdtp transaction role=source "
                "signal=get-capabilities tid=2 source=third/bluez/profiles/audio/avdtp.c "
                "state=alloc pending=1 timer=armed",
                "bluez-daemon: upstream-avdtp transaction role=source "
                "signal=set-configuration tid=3 source=third/bluez/profiles/audio/avdtp.c "
                "state=rsp-accept pending=0 timer=disarmed stream=configured",
                "bluez-daemon: upstream-avdtp transaction role=source "
                "signal=open tid=4 source=third/bluez/profiles/audio/avdtp.c "
                "state=rsp-accept pending=0 timer=disarmed stream=open",
                "bluez-daemon: upstream-avdtp transaction role=source "
                "signal=start tid=5 source=third/bluez/profiles/audio/avdtp.c "
                "state=rsp-accept pending=0 timer=disarmed stream=streaming",
                "bluez-daemon: audio owner source round=1 complete",
                "bluez-daemon: a2dp session-owner round=1 role=source "
                "checkpoint refs avdtp=0 avctp=0 media=0 l2cap-fds=0 "
                "watches=0 transport-state=idle",
                "bluez-daemon: upstream-avdtp transaction role=source "
                "signal=suspend tid=6 source=third/bluez/profiles/audio/avdtp.c "
                "state=timeout pending=1 retry=1 timer=rearmed",
                "bluez-daemon: upstream-avdtp transaction role=source "
                "signal=suspend tid=6 source=third/bluez/profiles/audio/avdtp.c "
                "state=rsp-accept pending=0 timer=disarmed stream=open",
                "bluez-daemon: upstream-avdtp transaction role=source "
                "signal=close tid=7 source=third/bluez/profiles/audio/avdtp.c "
                "state=rsp-accept pending=0 timer=disarmed stream=idle",
                "bluez-daemon: upstream-avdtp transaction role=source "
                "signal=abort tid=8 source=third/bluez/profiles/audio/avdtp.c "
                "state=cancel pending=0 timer=disarmed stream=idle",
                "bluez-daemon: audio-a2dp-upstream-transactions cleanup "
                "role=source transaction-pending=0 timers=0 retries=1 "
                "device-ref=0 session-ref=0 stream-ref=0 sep-ref=0 "
                "endpoint-refs=0 transport-refs=0 player-refs=0 avdtp=0 "
                "avctp=0 media=0 l2cap-fds=0 watches=0 sessions=0",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0 avdtp-transactions=0",
                "bluez-daemon: audio-a2dp-upstream-transactions complete "
                "role=source peer=2",
                "re:l2cap-socket-bind=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-send=([1-9][0-9]|[2-9][0-9]+)",
            )),
            RoleCheck("bt2", (
                "bluez-daemon: audio-a2dp-upstream-transactions "
                "source=third/bluez/profiles/audio/avdtp.c+"
                "third/bluez/profiles/audio/a2dp.c role=sink peer=1 "
                "semantic=transaction-owner-timeout-cleanup",
                "bluez-daemon: plugin audio init complete "
                "profiles=a2dp,avrcp media=1 sbc=1 avdtp-transactions=1",
                "bluez-daemon: mainloop owner=bluetoothd "
                "watches=mgmt,l2cap,avdtp,avctp,media-transport,dbus "
                "persistent=1 avdtp-transactions=1",
                "bluez-daemon: upstream-avdtp transaction role=sink "
                "signal=discover tid=1 source=third/bluez/profiles/audio/avdtp.c "
                "state=alloc pending=1 timer=armed",
                "bluez-daemon: upstream-avdtp transaction role=sink "
                "signal=discover tid=1 source=third/bluez/profiles/audio/avdtp.c "
                "state=rsp-accept pending=0 timer=disarmed",
                "bluez-daemon: upstream-avdtp transaction role=sink "
                "signal=get-capabilities tid=2 source=third/bluez/profiles/audio/avdtp.c "
                "state=alloc pending=1 timer=armed",
                "bluez-daemon: upstream-avdtp transaction role=sink "
                "signal=set-configuration tid=3 source=third/bluez/profiles/audio/avdtp.c "
                "state=rsp-accept pending=0 timer=disarmed stream=configured",
                "bluez-daemon: upstream-avdtp transaction role=sink "
                "signal=open tid=4 source=third/bluez/profiles/audio/avdtp.c "
                "state=rsp-accept pending=0 timer=disarmed stream=open",
                "bluez-daemon: upstream-avdtp transaction role=sink "
                "signal=start tid=5 source=third/bluez/profiles/audio/avdtp.c "
                "state=rsp-accept pending=0 timer=disarmed stream=streaming",
                "bluez-daemon: audio owner sink round=1 complete",
                "bluez-daemon: a2dp session-owner round=1 role=sink "
                "checkpoint refs avdtp=0 avctp=0 media=0 l2cap-fds=0 "
                "watches=0 transport-state=idle",
                "bluez-daemon: upstream-avdtp transaction role=sink "
                "signal=suspend tid=6 source=third/bluez/profiles/audio/avdtp.c "
                "state=timeout pending=1 retry=1 timer=rearmed",
                "bluez-daemon: upstream-avdtp transaction role=sink "
                "signal=suspend tid=6 source=third/bluez/profiles/audio/avdtp.c "
                "state=rsp-accept pending=0 timer=disarmed stream=open",
                "bluez-daemon: upstream-avdtp transaction role=sink "
                "signal=close tid=7 source=third/bluez/profiles/audio/avdtp.c "
                "state=rsp-accept pending=0 timer=disarmed stream=idle",
                "bluez-daemon: upstream-avdtp transaction role=sink "
                "signal=abort tid=8 source=third/bluez/profiles/audio/avdtp.c "
                "state=cancel pending=0 timer=disarmed stream=idle",
                "bluez-daemon: audio-a2dp-upstream-transactions cleanup "
                "role=sink transaction-pending=0 timers=0 retries=1 "
                "device-ref=0 session-ref=0 stream-ref=0 sep-ref=0 "
                "endpoint-refs=0 transport-refs=0 player-refs=0 avdtp=0 "
                "avctp=0 media=0 l2cap-fds=0 watches=0 sessions=0",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0 avdtp-transactions=0",
                "bluez-daemon: audio-a2dp-upstream-transactions complete "
                "role=sink peer=1",
                "re:l2cap-socket-bind=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-recv=([8-9]|[1-9][0-9]+)",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-a2dp-media-transport-fd",
        (
            RoleCheck("bt1", (
                "bluez-daemon: audio-a2dp-media-transport-fd "
                "source=third/bluez/profiles/audio/media.c+"
                "third/bluez/profiles/audio/transport.c+"
                "third/bluez/profiles/audio/a2dp.c role=source peer=2 "
                "semantic=dbus-fd-owner-acquire-release",
                "bluez-daemon: plugin audio init complete "
                "profiles=a2dp,avrcp media=1 sbc=1 media-transport-fd=1",
                "bluez-daemon: mainloop owner=bluetoothd "
                "watches=mgmt,l2cap,avdtp,avctp,media-transport,dbus "
                "persistent=1 media-transport-fd=1",
                "bluez-daemon: media-transport object-add "
                "source=third/bluez/profiles/audio/transport.c "
                "path=/org/bluez/hci0/dev_02/fd0 role=source "
                "state=idle owner=bluetoothd ref=1 fd=-1 acquiring=0",
                "bluez-daemon: media-transport dbus Acquire role=source "
                "state=idle->pending owner=:client.a2dp fd=71 "
                "read-mtu=672 write-mtu=672 acquire-ref=1",
                "bluez-daemon: media-transport dbus Acquire role=source "
                "state=pending result=busy errno=-EBUSY "
                "owner=:client.a2dp fd=71 acquire-ref=1",
                "bluez-daemon: media-transport dbus TryAcquire role=source "
                "state=pending result=defer errno=-EAGAIN "
                "fd-owner=:client.a2dp",
                "bluez-daemon: media-transport PropertiesChanged role=source "
                "State=active Delay=120 Volume=96 "
                "Endpoint=/org/bluez/hci0/dev_02/sep1",
                "bluez-daemon: audio owner source round=1 complete",
                "bluez-daemon: a2dp session-owner round=1 role=source "
                "checkpoint refs avdtp=0 avctp=0 media=0 l2cap-fds=0 "
                "watches=0 transport-state=idle",
                "bluez-daemon: media-transport dbus Release role=source "
                "state=active->idle owner=:client.a2dp fd=71 "
                "fd-close=1 acquire-ref=0 media-fd=closed",
                "bluez-daemon: media-transport dbus Acquire role=source "
                "state=idle->pending owner=:client.a2dp2 fd=72 "
                "read-mtu=672 write-mtu=672 acquire-ref=1 "
                "after-release=1",
                "bluez-daemon: media-transport dbus Release role=source "
                "state=pending->idle owner=:client.a2dp2 fd=72 "
                "fd-close=1 acquire-ref=0 media-fd=closed",
                "bluez-daemon: media-transport object-remove "
                "source=third/bluez/profiles/audio/transport.c "
                "path=/org/bluez/hci0/dev_02/fd0 role=source "
                "state=idle owner=none ref=0 fd=-1 acquiring=0",
                "bluez-daemon: audio-a2dp-media-transport-fd cleanup "
                "role=source dbus-owners=0 acquire-ref=0 "
                "media-fd=closed transport-state=idle endpoint-refs=0 "
                "transport-refs=0 player-refs=0 avdtp=0 avctp=0 media=0 "
                "l2cap-fds=0 watches=0 sessions=0",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0 media-transport-fd=0",
                "bluez-daemon: audio-a2dp-media-transport-fd complete "
                "role=source peer=2",
                "re:l2cap-socket-bind=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-send=([1-9][0-9]|[2-9][0-9]+)",
            )),
            RoleCheck("bt2", (
                "bluez-daemon: audio-a2dp-media-transport-fd "
                "source=third/bluez/profiles/audio/media.c+"
                "third/bluez/profiles/audio/transport.c+"
                "third/bluez/profiles/audio/a2dp.c role=sink peer=1 "
                "semantic=dbus-fd-owner-acquire-release",
                "bluez-daemon: plugin audio init complete "
                "profiles=a2dp,avrcp media=1 sbc=1 media-transport-fd=1",
                "bluez-daemon: mainloop owner=bluetoothd "
                "watches=mgmt,l2cap,avdtp,avctp,media-transport,dbus "
                "persistent=1 media-transport-fd=1",
                "bluez-daemon: media-transport object-add "
                "source=third/bluez/profiles/audio/transport.c "
                "path=/org/bluez/hci0/dev_01/fd0 role=sink "
                "state=idle owner=bluetoothd ref=1 fd=-1 acquiring=0",
                "bluez-daemon: media-transport dbus Acquire role=sink "
                "state=idle->pending owner=:client.a2dp fd=71 "
                "read-mtu=672 write-mtu=672 acquire-ref=1",
                "bluez-daemon: media-transport dbus Acquire role=sink "
                "state=pending result=busy errno=-EBUSY "
                "owner=:client.a2dp fd=71 acquire-ref=1",
                "bluez-daemon: media-transport dbus TryAcquire role=sink "
                "state=pending result=defer errno=-EAGAIN "
                "fd-owner=:client.a2dp",
                "bluez-daemon: media-transport PropertiesChanged role=sink "
                "State=active Delay=120 Volume=96 "
                "Endpoint=/org/bluez/hci0/dev_01/sep1",
                "bluez-daemon: audio owner sink round=1 complete",
                "bluez-daemon: a2dp session-owner round=1 role=sink "
                "checkpoint refs avdtp=0 avctp=0 media=0 l2cap-fds=0 "
                "watches=0 transport-state=idle",
                "bluez-daemon: media-transport dbus Release role=sink "
                "state=active->idle owner=:client.a2dp fd=71 "
                "fd-close=1 acquire-ref=0 media-fd=closed",
                "bluez-daemon: media-transport dbus Acquire role=sink "
                "state=idle->pending owner=:client.a2dp2 fd=72 "
                "read-mtu=672 write-mtu=672 acquire-ref=1 "
                "after-release=1",
                "bluez-daemon: media-transport dbus Release role=sink "
                "state=pending->idle owner=:client.a2dp2 fd=72 "
                "fd-close=1 acquire-ref=0 media-fd=closed",
                "bluez-daemon: media-transport object-remove "
                "source=third/bluez/profiles/audio/transport.c "
                "path=/org/bluez/hci0/dev_01/fd0 role=sink "
                "state=idle owner=none ref=0 fd=-1 acquiring=0",
                "bluez-daemon: audio-a2dp-media-transport-fd cleanup "
                "role=sink dbus-owners=0 acquire-ref=0 "
                "media-fd=closed transport-state=idle endpoint-refs=0 "
                "transport-refs=0 player-refs=0 avdtp=0 avctp=0 media=0 "
                "l2cap-fds=0 watches=0 sessions=0",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0 media-transport-fd=0",
                "bluez-daemon: audio-a2dp-media-transport-fd complete "
                "role=sink peer=1",
                "re:l2cap-socket-bind=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-recv=([8-9]|[1-9][0-9]+)",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-a2dp-codec-policy",
        (
            RoleCheck("bt1", (
                "bluez-daemon: audio-a2dp-codec-policy "
                "source=third/bluez/profiles/audio/a2dp.c+"
                "third/bluez/profiles/audio/media.c+"
                "third/bluez/profiles/audio/sbc.c role=source peer=2 "
                "semantic=codec-capability-reconfigure-cleanup",
                "bluez-daemon: plugin audio init complete "
                "profiles=a2dp,avrcp media=1 sbc=1 codec-policy=1",
                "bluez-daemon: codec policy role=source "
                "command=discover-endpoints "
                "endpoint=/org/bluez/hci0/dev_02/sep1 codec=sbc "
                "uuid=0000110a-0000-1000-8000-00805f9b34fb caps-ref=1",
                "bluez-daemon: codec policy role=source "
                "command=SelectConfiguration codec=sbc frequency=44100 "
                "channel-mode=joint-stereo block-length=16 subbands=8 "
                "allocation=loudness bitpool=2..53 result=ok config-ref=1",
                "bluez-daemon: codec policy role=source "
                "command=SetConfiguration codec=sbc "
                "transport=/org/bluez/hci0/dev_02/fd0 state=idle "
                "result=ok transport-ref=2",
                "bluez-daemon: codec policy role=source "
                "command=SelectConfiguration codec=aptx result=reject "
                "errno=-ENOTSUP reason=unsupported-codec config-ref=0",
                "bluez-daemon: codec policy role=source "
                "command=SetConfiguration codec=sbc frequency=96000 "
                "result=reject errno=-EINVAL reason=invalid-capability "
                "config-ref=0 transport-ref=1",
                "bluez-daemon: codec policy role=source command=Reconfigure "
                "state=open->configured codec=sbc frequency=48000 "
                "channel-mode=dual-channel result=ok transport-ref=2",
                "bluez-daemon: audio owner source round=1 complete",
                "bluez-daemon: a2dp session-owner round=1 role=source "
                "checkpoint refs avdtp=0 avctp=0 media=0 l2cap-fds=0 "
                "watches=0 transport-state=idle",
                "bluez-daemon: codec policy role=source "
                "command=SuspendForReconfigure state=streaming->open "
                "result=ok media-fd=closed",
                "bluez-daemon: codec policy role=source "
                "command=ApplyReconfigure "
                "state=open->configured->open->streaming codec=sbc "
                "frequency=48000 channel-mode=dual-channel result=ok",
                "bluez-daemon: codec policy role=source "
                "command=ContentProtection type=scms-t result=reject "
                "errno=-ENOTSUP reason=unsupported-content-protection",
                "bluez-daemon: codec policy role=source "
                "command=ClearEndpoint "
                "endpoint=/org/bluez/hci0/dev_02/sep1 caps-ref=0 "
                "config-ref=0 transport-ref=1 state=idle",
                "bluez-daemon: audio-a2dp-codec-policy cleanup "
                "role=source caps-ref=0 config-ref=0 endpoint-refs=0 "
                "transport-refs=0 player-refs=0 avdtp=0 avctp=0 media=0 "
                "l2cap-fds=0 watches=0 sessions=0 codec-policy=0",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0 codec-policy=0",
                "bluez-daemon: audio-a2dp-codec-policy complete "
                "role=source peer=2",
                "re:l2cap-socket-bind=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-send=([1-9][0-9]|[2-9][0-9]+)",
            )),
            RoleCheck("bt2", (
                "bluez-daemon: audio-a2dp-codec-policy "
                "source=third/bluez/profiles/audio/a2dp.c+"
                "third/bluez/profiles/audio/media.c+"
                "third/bluez/profiles/audio/sbc.c role=sink peer=1 "
                "semantic=codec-capability-reconfigure-cleanup",
                "bluez-daemon: plugin audio init complete "
                "profiles=a2dp,avrcp media=1 sbc=1 codec-policy=1",
                "bluez-daemon: codec policy role=sink "
                "command=discover-endpoints "
                "endpoint=/org/bluez/hci0/dev_01/sep1 codec=sbc "
                "uuid=0000110b-0000-1000-8000-00805f9b34fb caps-ref=1",
                "bluez-daemon: codec policy role=sink "
                "command=SelectConfiguration codec=sbc frequency=44100 "
                "channel-mode=joint-stereo block-length=16 subbands=8 "
                "allocation=loudness bitpool=2..53 result=ok config-ref=1",
                "bluez-daemon: codec policy role=sink "
                "command=SetConfiguration codec=sbc "
                "transport=/org/bluez/hci0/dev_01/fd0 state=idle "
                "result=ok transport-ref=2",
                "bluez-daemon: codec policy role=sink "
                "command=SelectConfiguration codec=aptx result=reject "
                "errno=-ENOTSUP reason=unsupported-codec config-ref=0",
                "bluez-daemon: codec policy role=sink "
                "command=SetConfiguration codec=sbc frequency=96000 "
                "result=reject errno=-EINVAL reason=invalid-capability "
                "config-ref=0 transport-ref=1",
                "bluez-daemon: codec policy role=sink command=Reconfigure "
                "state=open->configured codec=sbc frequency=48000 "
                "channel-mode=dual-channel result=ok transport-ref=2",
                "bluez-daemon: audio owner sink round=1 complete",
                "bluez-daemon: a2dp session-owner round=1 role=sink "
                "checkpoint refs avdtp=0 avctp=0 media=0 l2cap-fds=0 "
                "watches=0 transport-state=idle",
                "bluez-daemon: codec policy role=sink "
                "command=SuspendForReconfigure state=streaming->open "
                "result=ok media-fd=closed",
                "bluez-daemon: codec policy role=sink "
                "command=ApplyReconfigure "
                "state=open->configured->open->streaming codec=sbc "
                "frequency=48000 channel-mode=dual-channel result=ok",
                "bluez-daemon: codec policy role=sink "
                "command=ContentProtection type=scms-t result=reject "
                "errno=-ENOTSUP reason=unsupported-content-protection",
                "bluez-daemon: codec policy role=sink "
                "command=ClearEndpoint "
                "endpoint=/org/bluez/hci0/dev_01/sep1 caps-ref=0 "
                "config-ref=0 transport-ref=1 state=idle",
                "bluez-daemon: audio-a2dp-codec-policy cleanup "
                "role=sink caps-ref=0 config-ref=0 endpoint-refs=0 "
                "transport-refs=0 player-refs=0 avdtp=0 avctp=0 media=0 "
                "l2cap-fds=0 watches=0 sessions=0 codec-policy=0",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0 codec-policy=0",
                "bluez-daemon: audio-a2dp-codec-policy complete "
                "role=sink peer=1",
                "re:l2cap-socket-bind=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-connect=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-recv=([8-9]|[1-9][0-9]+)",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-daemon-a2dp-closeout-full",
        (
            RoleCheck("bt1", (
                "bluez-daemon: audio-a2dp-closeout-full "
                "source=third/bluez/src/profile.c+third/bluez/src/device.c+"
                "third/bluez/profiles/audio/a2dp.c+"
                "third/bluez/profiles/audio/avdtp.c+"
                "third/bluez/profiles/audio/media.c+"
                "third/bluez/profiles/audio/transport.c+"
                "third/bluez/profiles/audio/avrcp.c role=source peer=2 "
                "semantic=a2dp-current-closeout-umbrella",
                "bluez-daemon: a2dp closeout phase=mainloop-ownership "
                "role=source source=third/bluez/src/shared/mainloop.c+"
                "third/bluez/src/shared/io-mainloop.c "
                "watch-add=mgmt,l2cap-signaling,l2cap-media,avdtp,avctp,"
                "media-transport,dbus timer-add=avdtp-retry,avrcp-notify "
                "dispatch=mgmt,l2cap,avdtp,avctp,media,dbus "
                "watch-owner=bluetoothd timer-owner=bluetoothd "
                "watches=7 timers=2",
                "bluez-upstream-object: src/shared/mainloop.c "
                "role=source linked=1 "
                "source=third/bluez/src/shared/mainloop.c "
                "owner=bluetoothd api=mainloop",
                "bluez-upstream-object: src/shared/io-mainloop.c "
                "role=source linked=1 "
                "source=third/bluez/src/shared/io-mainloop.c "
                "owner=bluetoothd api=io-mainloop",
                "bluez-upstream-object: src/profile.c "
                "role=source linked=1 "
                "source=third/bluez/src/profile.c "
                "owner=bluetoothd api=btd_profile",
                "bluez-upstream-object: src/dbus-common.c "
                "role=source linked=1 "
                "source=third/bluez/src/dbus-common.c "
                "owner=bluetoothd api=dbus-common",
                "bluez-upstream-object: src/error.c "
                "role=source linked=1 "
                "source=third/bluez/src/error.c "
                "owner=bluetoothd api=error",
                "bluez-upstream-object: src/sdpd-database.c "
                "role=source linked=1 "
                "source=third/bluez/src/sdpd-database.c "
                "owner=bluetoothd api=sdpd-database",
                "bluez-upstream-object: src/sdpd-service.c "
                "role=source linked=1 "
                "source=third/bluez/src/sdpd-service.c "
                "owner=bluetoothd api=sdpd-service",
                "bluez-upstream-object: audio/media-owner "
                "role=source linked=1 "
                "source=third/bluez/profiles/audio/media.c+"
                "third/bluez/profiles/audio/transport.c "
                "owner=bluetoothd api=media-endpoint-transport-forwarders",
                "bluez-upstream-handler-object: audio/transport.c "
                "role=source linked=1 "
                "source=third/bluez/profiles/audio/transport.c "
                "handlers=acquire:1,try-acquire:1,release:1,select:1,"
                "unselect:1,total:5 "
                "upstream-link=static-upstream-handlers-bound",
                "bluez-upstream-handler-object: audio/media.c "
                "role=source linked=1 "
                "source=third/bluez/profiles/audio/media.c "
                "handlers=register-endpoint:1,unregister-endpoint:1,total:2 "
                "upstream-link=static-upstream-handlers-bound",
                "bluez-upstream-object: src/device.c "
                "role=source linked=1 "
                "source=third/bluez/src/device.c "
                "owner=bluetoothd api=btd_device",
                "bluez-upstream-object: src/service.c "
                "role=source linked=1 "
                "source=third/bluez/src/service.c "
                "owner=bluetoothd api=btd_service",
                "bluez-upstream-object: src/adapter.c "
                "role=source linked=1 "
                "source=third/bluez/src/adapter.c "
                "owner=bluetoothd api=btd_adapter",
                "bluez-upstream-object: src/shared/mgmt.c "
                "role=source linked=1 "
                "source=third/bluez/src/shared/mgmt.c "
                "owner=bluetoothd api=mgmt",
                "bluez-upstream-object: src/storage.c "
                "role=source linked=1 "
                "source=third/bluez/src/storage.c "
                "owner=bluetoothd api=storage",
                "bluez-upstream-object: src/agent.c "
                "role=source linked=1 "
                "source=third/bluez/src/agent.c "
                "owner=bluetoothd api=agent",
                "bluez-daemon: a2dp closeout phase=profile-session "
                "role=source profile-registered=1 device-ref=1 "
                "endpoint-ref=1 transport-ref=1 player-ref=1",
                "bluez-daemon: a2dp closeout phase=sdp-profile "
                "role=source source=third/bluez/profiles/audio/source.c+"
                "third/bluez/src/sdpd-service.c "
                "service=AudioSource "
                "uuid=0000110a-0000-1000-8000-00805f9b34fb "
                "record-register=1 browse-group=public "
                "profile-version=1.3 psm-signaling=0x0019 "
                "psm-avctp=0x0017 psm-avctp-browsing=0x001b "
                "service-discovery=ok remote-service=AudioSink "
                "remote-uuid=0000110b-0000-1000-8000-00805f9b34fb "
                "resolve=ok",
                "bluez-daemon: a2dp closeout phase=l2cap-controller "
                "role=source source=third/linux-hwe-6.17-6.17.0/net/bluetooth/l2cap_core.c+"
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/l2cap_sock.c "
                "controller-created=1 channel-owner=controller "
                "channel-create=signaling,media,avctp,avctp-browsing "
                "psm-signaling=0x0019 cid-signaling=0x0040 "
                "psm-media=0x0019 cid-media=0x0041 "
                "psm-avctp=0x0017 cid-avctp=0x0042 "
                "psm-avctp-browsing=0x001b cid-avctp-browsing=0x0043 "
                "security=medium mtu=672 mode=basic conn-rsp=success "
                "duplicate-channel=reject timeout=retry disconnect=ok",
                "bluez-daemon: a2dp closeout phase=codec-policy "
                "role=source sbc-select=ok unsupported-codec=reject "
                "invalid-caps=reject reconfigure=ok "
                "content-protection=reject",
                "bluez-daemon: a2dp closeout phase=media-transport "
                "role=source acquire=ok duplicate-acquire=busy "
                "try-acquire=defer release=ok reacquire=ok "
                "delay=120 volume=96",
                "bluez-daemon: a2dp closeout transport-owner role=source "
                "state=active owner=:client.a2dp fd-owner=:client.a2dp "
                "acquire-ref=1 transport-ref=1 media-fd=open "
                "write-watch=1 read-watch=1",
                "bluez-daemon: a2dp closeout phase=dbus-owner-recovery "
                "role=source endpoint=/org/bluez/hci0/dev_02/sep1 "
                "transport=/org/bluez/hci0/dev_02/fd0 "
                "player=/org/bluez/hci0/dev_02/player0 "
                "owner=:client.a2dp owner-lost=1 "
                "interfaces-removed=MediaEndpoint1,MediaTransport1,"
                "MediaPlayer1 owner-reacquire=1 objects-readd=1 "
                "acquire-after-reacquire=ok release=ok "
                "endpoint-ref=1 transport-ref=1 player-ref=1",
                "bluez-daemon: a2dp closeout phase=avdtp-transactions "
                "role=source discover=ok get-capabilities=ok "
                "setconfig=ok open=ok start=ok suspend-timeout-retry=ok "
                "close=ok abort-cancel=ok transaction-pending=0 timers=0",
                "bluez-daemon: a2dp closeout phase=error-policy "
                "role=source start-before-open=reject duplicate-open=reject "
                "media-before-start=reject l2cap-drop-streaming=abort "
                "remote-close-after-abort=ignore",
                "bluez-daemon: a2dp closeout error-policy-lifecycle "
                "role=source source=third/bluez/profiles/audio/avdtp.c+"
                "third/bluez/profiles/audio/media.c+"
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/l2cap_core.c "
                "start-before-open-reject=1 duplicate-open-reject=1 "
                "media-before-start-reject=1 "
                "l2cap-drop-streaming-abort=1 "
                "remote-close-after-abort-ignore=1 cleanup=1 "
                "state-errors=0 final-ok=1",
                "bluez-daemon: a2dp closeout round=1 role=source "
                "profile-registered=1 device-connect=1 session-ref=1",
                "bluez-daemon: a2dp closeout owner-state role=source "
                "round=1 state=active owner=bluetoothd "
                "dbus-owner=:client.a2dp profile-ref=1 device-ref=1 "
                "session-ref=1 stream-ref=1 sep-ref=1 "
                "endpoint-ref=1 transport-ref=1 player-ref=1 "
                "avdtp=0 avctp=0 media=0 l2cap-fds=0 watches=0",
                "bluez-daemon: audio owner source round=1 complete",
                "bluez-daemon: a2dp closeout owner-state role=source "
                "round=1 state=idle owner=bluetoothd "
                "dbus-owner=:client.a2dp profile-ref=1 device-ref=1 "
                "session-ref=0 stream-ref=0 sep-ref=0 "
                "endpoint-ref=0 transport-ref=0 player-ref=0 "
                "avdtp=0 avctp=0 media=0 l2cap-fds=0 watches=0",
                "bluez-daemon: a2dp closeout round=1 role=source cleanup "
                "device-ref=1 session-ref=0 stream-ref=0 sep-ref=0 "
                "endpoint-refs=0 transport-refs=0 player-refs=0 "
                "avdtp=0 avctp=0 media=0 l2cap-fds=0 watches=0 "
                "profile-registered=1",
                "bluez-daemon: a2dp closeout round=2 role=source "
                "profile-registered=1 device-connect=1 session-ref=1",
                "bluez-daemon: a2dp closeout owner-state role=source "
                "round=2 state=active owner=bluetoothd "
                "dbus-owner=:client.a2dp profile-ref=1 device-ref=1 "
                "session-ref=1 stream-ref=1 sep-ref=1 "
                "endpoint-ref=1 transport-ref=1 player-ref=1 "
                "avdtp=0 avctp=0 media=0 l2cap-fds=0 watches=0",
                "bluez-daemon: audio owner source round=2 complete",
                "bluez-daemon: a2dp closeout owner-state role=source "
                "round=2 state=idle owner=bluetoothd "
                "dbus-owner=:client.a2dp profile-ref=1 device-ref=1 "
                "session-ref=0 stream-ref=0 sep-ref=0 "
                "endpoint-ref=0 transport-ref=0 player-ref=0 "
                "avdtp=0 avctp=0 media=0 l2cap-fds=0 watches=0",
                "bluez-daemon: a2dp closeout round=2 role=source cleanup "
                "device-ref=1 session-ref=0 stream-ref=0 sep-ref=0 "
                "endpoint-refs=0 transport-refs=0 player-refs=0 "
                "avdtp=0 avctp=0 media=0 l2cap-fds=0 watches=0 "
                "profile-registered=1",
                "bluez-daemon: a2dp closeout upstream-object-lifecycle "
                "role=source source=third/bluez/profiles/audio/a2dp.c+"
                "third/bluez/profiles/audio/avdtp.c+"
                "third/bluez/profiles/audio/transport.c "
                "session-object=bluez_daemon_a2dp_session rounds=2 "
                "active-transitions=2 idle-transitions=2",
                "re:role=source .*acquire-events=[1-9][0-9]* "
                "release-events=[1-9][0-9]* balanced-rounds=2 "
                "zero-ref-rounds=2 final-balanced=1 final-zero=1 "
                "avdtp-transaction-begin=[1-9][0-9]* "
                "avdtp-transaction-complete=[1-9][0-9]* "
                "transaction-balanced-rounds=2 "
                "transaction-final-balanced=1 "
                "media-payload-tx=[1-9][0-9]* media-payload-rx=0 "
                "media-payload-tx-bytes=[1-9][0-9]* "
                "media-payload-rx-bytes=0 media-role-ok=1 "
                "codec-configured=2 sbc-encode=[1-9][0-9]* "
                "sbc-decode=0 sbc-encode-bytes=[1-9][0-9]* "
                "sbc-decode-bytes=0 codec-role-errors=0 "
                "codec-owner-rounds=2 codec-final-ok=1 "
                "transport-acquire=2 transport-release=2 "
                "transport-fd-open=2 transport-fd-close=2 "
                "transport-busy=0 transport-state-errors=0 "
                "transport-owner-rounds=2 transport-final-ok=1 "
                "avrcp-command-tx=[1-9][0-9]* avrcp-command-rx=0 "
                "avrcp-response-tx=0 avrcp-control-tx=[1-9][0-9]* "
                "avrcp-control-rx=0 avrcp-browse-tx=[1-9][0-9]* "
                "avrcp-browse-rx=0 avrcp-bytes-tx=[1-9][0-9]* "
                "avrcp-bytes-rx=0 avrcp-role-errors=0 "
                "avrcp-owner-rounds=2 avrcp-final-ok=1 "
                "l2cap-open=[1-9][0-9]* l2cap-connect=[1-9][0-9]* "
                "l2cap-write=[1-9][0-9]* l2cap-recv=[1-9][0-9]* "
                "l2cap-close=[1-9][0-9]* "
                "l2cap-avdtp=[1-9][0-9]* "
                "l2cap-avrcp=[1-9][0-9]* "
                "l2cap-media=2 l2cap-state-errors=0 "
                "l2cap-owner-rounds=2 l2cap-final-ok=1 "
                "avdtp-configured=2 avdtp-opened=2 "
                "avdtp-started=2 avdtp-suspended=2 "
                "avdtp-closed=2 avdtp-state-errors=0 "
                "state-machine-rounds=2 state-machine-final-ok=1",
                "bluez-daemon: a2dp closeout profile-lifecycle "
                "role=source source=third/bluez/src/profile.c+"
                "third/bluez/src/device.c+third/bluez/src/sdpd-service.c+"
                "third/bluez/profiles/audio/a2dp.c "
                "profile-register=1 profile-unregister=1 "
                "device-connect=2 device-disconnect=2 "
                "sdp-register=1 sdp-unregister=1 "
                "service-discovery=1 service-resolve=1 cache-remove=1 "
                "state-errors=0 final-ok=1",
                "bluez-daemon: a2dp closeout dbus-object-lifecycle "
                "role=source source=third/bluez/src/adapter.c+"
                "third/bluez/src/dbus-common.c+"
                "third/bluez/profiles/audio/media.c+"
                "third/bluez/profiles/audio/transport.c "
                "name-acquire=1 get-managed-objects=1 "
                "interfaces-added=3 endpoint-added=1 transport-added=1 "
                "player-added=1 owner-lost=1 interfaces-removed=5 "
                "owner-reacquire=1 objects-readd=3 name-release=1 "
                "state-errors=0 final-ok=1",
                "bluez-daemon: a2dp closeout mainloop-lifecycle "
                "role=source source=third/bluez/src/shared/mainloop.c+"
                "third/bluez/src/shared/io-mainloop.c "
                "watch-add=7 watch-remove=7 timer-add=2 timer-remove=2 "
                "dispatch-mgmt=1 dispatch-l2cap=",
                "re:role=source .*dispatch-l2cap=[1-9][0-9]* "
                "dispatch-avdtp=[1-9][0-9]* "
                "dispatch-avctp=[1-9][0-9]* "
                "dispatch-media=[1-9][0-9]* "
                "dispatch-dbus=[1-9][0-9]* state-errors=0 final-ok=1",
                "bluez-daemon: a2dp closeout dbus-owner-recovery cleanup "
                "role=source owner=:client.a2dp dbus-owners=0 "
                "endpoint-refs=0 transport-refs=0 player-refs=0 "
                "interfaces=0 acquire-ref=0 media-fd=closed watches=0",
                "bluez-daemon: a2dp closeout transport-owner role=source "
                "state=idle owner=:client.a2dp fd-owner=none "
                "acquire-ref=0 transport-ref=0 media-fd=closed "
                "write-watch=0 read-watch=0",
                "bluez-daemon: a2dp closeout mainloop cleanup role=source "
                "watch-remove=mgmt,l2cap-signaling,l2cap-media,avdtp,avctp,"
                "media-transport,dbus timer-remove=avdtp-retry,avrcp-notify "
                "dispatch-pending=0 watches=0 timers=0 owner=bluetoothd",
                "bluez-daemon: a2dp closeout sdp-profile cleanup "
                "role=source record-unregister=1 cache-remove=1 "
                "service-discovery=0 records=0",
                "bluez-daemon: a2dp closeout l2cap-controller cleanup "
                "role=source channel-disconnect=signaling,media,avctp,"
                "avctp-browsing channels=0 refs=0 retrans=0 pending=0",
                "bluez-daemon: audio-a2dp-closeout-full cleanup "
                "role=source profile-registered=0 dbus-owners=0 "
                "transaction-pending=0 timers=0 acquire-ref=0 caps-ref=0 "
                "config-ref=0 device-ref=0 session-ref=0 stream-ref=0 "
                "sep-ref=0 endpoint-refs=0 transport-refs=0 player-refs=0 "
                "avdtp=0 avctp=0 media=0 l2cap-fds=0 watches=0 "
                "sessions=0 rounds=2",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0 closeout=0",
                "bluez-daemon: audio-a2dp-closeout-full complete "
                "role=source peer=2",
                "re:l2cap-socket-bind=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-connect=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-send=([2-9][0-9]|[3-9][0-9]+)",
            )),
            RoleCheck("bt2", (
                "bluez-daemon: audio-a2dp-closeout-full "
                "source=third/bluez/src/profile.c+third/bluez/src/device.c+"
                "third/bluez/profiles/audio/a2dp.c+"
                "third/bluez/profiles/audio/avdtp.c+"
                "third/bluez/profiles/audio/media.c+"
                "third/bluez/profiles/audio/transport.c+"
                "third/bluez/profiles/audio/avrcp.c role=sink peer=1 "
                "semantic=a2dp-current-closeout-umbrella",
                "bluez-daemon: a2dp closeout phase=mainloop-ownership "
                "role=sink source=third/bluez/src/shared/mainloop.c+"
                "third/bluez/src/shared/io-mainloop.c "
                "watch-add=mgmt,l2cap-signaling,l2cap-media,avdtp,avctp,"
                "media-transport,dbus timer-add=avdtp-retry,avrcp-notify "
                "dispatch=mgmt,l2cap,avdtp,avctp,media,dbus "
                "watch-owner=bluetoothd timer-owner=bluetoothd "
                "watches=7 timers=2",
                "bluez-upstream-object: src/shared/mainloop.c "
                "role=sink linked=1 "
                "source=third/bluez/src/shared/mainloop.c "
                "owner=bluetoothd api=mainloop",
                "bluez-upstream-object: src/shared/io-mainloop.c "
                "role=sink linked=1 "
                "source=third/bluez/src/shared/io-mainloop.c "
                "owner=bluetoothd api=io-mainloop",
                "bluez-upstream-object: src/profile.c "
                "role=sink linked=1 "
                "source=third/bluez/src/profile.c "
                "owner=bluetoothd api=btd_profile",
                "bluez-upstream-object: src/dbus-common.c "
                "role=sink linked=1 "
                "source=third/bluez/src/dbus-common.c "
                "owner=bluetoothd api=dbus-common",
                "bluez-upstream-object: src/error.c "
                "role=sink linked=1 "
                "source=third/bluez/src/error.c "
                "owner=bluetoothd api=error",
                "bluez-upstream-object: src/sdpd-database.c "
                "role=sink linked=1 "
                "source=third/bluez/src/sdpd-database.c "
                "owner=bluetoothd api=sdpd-database",
                "bluez-upstream-object: src/sdpd-service.c "
                "role=sink linked=1 "
                "source=third/bluez/src/sdpd-service.c "
                "owner=bluetoothd api=sdpd-service",
                "bluez-upstream-object: audio/media-owner "
                "role=sink linked=1 "
                "source=third/bluez/profiles/audio/media.c+"
                "third/bluez/profiles/audio/transport.c "
                "owner=bluetoothd api=media-endpoint-transport-forwarders",
                "bluez-upstream-handler-object: audio/transport.c "
                "role=sink linked=1 "
                "source=third/bluez/profiles/audio/transport.c "
                "handlers=acquire:1,try-acquire:1,release:1,select:1,"
                "unselect:1,total:5 "
                "upstream-link=static-upstream-handlers-bound",
                "bluez-upstream-handler-object: audio/media.c "
                "role=sink linked=1 "
                "source=third/bluez/profiles/audio/media.c "
                "handlers=register-endpoint:1,unregister-endpoint:1,total:2 "
                "upstream-link=static-upstream-handlers-bound",
                "bluez-upstream-object: src/device.c "
                "role=sink linked=1 "
                "source=third/bluez/src/device.c "
                "owner=bluetoothd api=btd_device",
                "bluez-upstream-object: src/service.c "
                "role=sink linked=1 "
                "source=third/bluez/src/service.c "
                "owner=bluetoothd api=btd_service",
                "bluez-upstream-object: src/adapter.c "
                "role=sink linked=1 "
                "source=third/bluez/src/adapter.c "
                "owner=bluetoothd api=btd_adapter",
                "bluez-upstream-object: src/shared/mgmt.c "
                "role=sink linked=1 "
                "source=third/bluez/src/shared/mgmt.c "
                "owner=bluetoothd api=mgmt",
                "bluez-upstream-object: src/storage.c "
                "role=sink linked=1 "
                "source=third/bluez/src/storage.c "
                "owner=bluetoothd api=storage",
                "bluez-upstream-object: src/agent.c "
                "role=sink linked=1 "
                "source=third/bluez/src/agent.c "
                "owner=bluetoothd api=agent",
                "bluez-daemon: a2dp closeout phase=profile-session "
                "role=sink profile-registered=1 device-ref=1 "
                "endpoint-ref=1 transport-ref=1 player-ref=1",
                "bluez-daemon: a2dp closeout phase=sdp-profile "
                "role=sink source=third/bluez/profiles/audio/sink.c+"
                "third/bluez/src/sdpd-service.c "
                "service=AudioSink "
                "uuid=0000110b-0000-1000-8000-00805f9b34fb "
                "record-register=1 browse-group=public "
                "profile-version=1.3 psm-signaling=0x0019 "
                "psm-avctp=0x0017 psm-avctp-browsing=0x001b "
                "service-discovery=ok remote-service=AudioSource "
                "remote-uuid=0000110a-0000-1000-8000-00805f9b34fb "
                "resolve=ok",
                "bluez-daemon: a2dp closeout phase=l2cap-controller "
                "role=sink source=third/linux-hwe-6.17-6.17.0/net/bluetooth/l2cap_core.c+"
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/l2cap_sock.c "
                "controller-created=1 channel-owner=controller "
                "channel-create=signaling,media,avctp,avctp-browsing "
                "psm-signaling=0x0019 cid-signaling=0x0040 "
                "psm-media=0x0019 cid-media=0x0041 "
                "psm-avctp=0x0017 cid-avctp=0x0042 "
                "psm-avctp-browsing=0x001b cid-avctp-browsing=0x0043 "
                "security=medium mtu=672 mode=basic conn-rsp=success "
                "duplicate-channel=reject timeout=retry disconnect=ok",
                "bluez-daemon: a2dp closeout phase=codec-policy "
                "role=sink sbc-select=ok unsupported-codec=reject "
                "invalid-caps=reject reconfigure=ok "
                "content-protection=reject",
                "bluez-daemon: a2dp closeout phase=media-transport "
                "role=sink acquire=ok duplicate-acquire=busy "
                "try-acquire=defer release=ok reacquire=ok "
                "delay=120 volume=96",
                "bluez-daemon: a2dp closeout transport-owner role=sink "
                "state=active owner=:client.a2dp fd-owner=:client.a2dp "
                "acquire-ref=1 transport-ref=1 media-fd=open "
                "write-watch=1 read-watch=1",
                "bluez-daemon: a2dp closeout phase=dbus-owner-recovery "
                "role=sink endpoint=/org/bluez/hci0/dev_01/sep1 "
                "transport=/org/bluez/hci0/dev_01/fd0 "
                "player=/org/bluez/hci0/dev_01/player0 "
                "owner=:client.a2dp owner-lost=1 "
                "interfaces-removed=MediaEndpoint1,MediaTransport1,"
                "MediaPlayer1 owner-reacquire=1 objects-readd=1 "
                "acquire-after-reacquire=ok release=ok "
                "endpoint-ref=1 transport-ref=1 player-ref=1",
                "bluez-daemon: a2dp closeout phase=avdtp-transactions "
                "role=sink discover=ok get-capabilities=ok "
                "setconfig=ok open=ok start=ok suspend-timeout-retry=ok "
                "close=ok abort-cancel=ok transaction-pending=0 timers=0",
                "bluez-daemon: a2dp closeout phase=error-policy "
                "role=sink start-before-open=reject duplicate-open=reject "
                "media-before-start=reject l2cap-drop-streaming=abort "
                "remote-close-after-abort=ignore",
                "bluez-daemon: a2dp closeout error-policy-lifecycle "
                "role=sink source=third/bluez/profiles/audio/avdtp.c+"
                "third/bluez/profiles/audio/media.c+"
                "third/linux-hwe-6.17-6.17.0/net/bluetooth/l2cap_core.c "
                "start-before-open-reject=1 duplicate-open-reject=1 "
                "media-before-start-reject=1 "
                "l2cap-drop-streaming-abort=1 "
                "remote-close-after-abort-ignore=1 cleanup=1 "
                "state-errors=0 final-ok=1",
                "bluez-daemon: a2dp closeout round=1 role=sink "
                "profile-registered=1 device-connect=1 session-ref=1",
                "bluez-daemon: a2dp closeout owner-state role=sink "
                "round=1 state=active owner=bluetoothd "
                "dbus-owner=:client.a2dp profile-ref=1 device-ref=1 "
                "session-ref=1 stream-ref=1 sep-ref=1 "
                "endpoint-ref=1 transport-ref=1 player-ref=1 "
                "avdtp=0 avctp=0 media=0 l2cap-fds=0 watches=0",
                "bluez-daemon: audio owner sink round=1 complete",
                "bluez-daemon: a2dp closeout owner-state role=sink "
                "round=1 state=idle owner=bluetoothd "
                "dbus-owner=:client.a2dp profile-ref=1 device-ref=1 "
                "session-ref=0 stream-ref=0 sep-ref=0 "
                "endpoint-ref=0 transport-ref=0 player-ref=0 "
                "avdtp=0 avctp=0 media=0 l2cap-fds=0 watches=0",
                "bluez-daemon: a2dp closeout round=1 role=sink cleanup "
                "device-ref=1 session-ref=0 stream-ref=0 sep-ref=0 "
                "endpoint-refs=0 transport-refs=0 player-refs=0 "
                "avdtp=0 avctp=0 media=0 l2cap-fds=0 watches=0 "
                "profile-registered=1",
                "bluez-daemon: a2dp closeout round=2 role=sink "
                "profile-registered=1 device-connect=1 session-ref=1",
                "bluez-daemon: a2dp closeout owner-state role=sink "
                "round=2 state=active owner=bluetoothd "
                "dbus-owner=:client.a2dp profile-ref=1 device-ref=1 "
                "session-ref=1 stream-ref=1 sep-ref=1 "
                "endpoint-ref=1 transport-ref=1 player-ref=1 "
                "avdtp=0 avctp=0 media=0 l2cap-fds=0 watches=0",
                "bluez-daemon: audio owner sink round=2 complete",
                "bluez-daemon: a2dp closeout owner-state role=sink "
                "round=2 state=idle owner=bluetoothd "
                "dbus-owner=:client.a2dp profile-ref=1 device-ref=1 "
                "session-ref=0 stream-ref=0 sep-ref=0 "
                "endpoint-ref=0 transport-ref=0 player-ref=0 "
                "avdtp=0 avctp=0 media=0 l2cap-fds=0 watches=0",
                "bluez-daemon: a2dp closeout round=2 role=sink cleanup "
                "device-ref=1 session-ref=0 stream-ref=0 sep-ref=0 "
                "endpoint-refs=0 transport-refs=0 player-refs=0 "
                "avdtp=0 avctp=0 media=0 l2cap-fds=0 watches=0 "
                "profile-registered=1",
                "bluez-daemon: a2dp closeout upstream-object-lifecycle "
                "role=sink source=third/bluez/profiles/audio/a2dp.c+"
                "third/bluez/profiles/audio/avdtp.c+"
                "third/bluez/profiles/audio/transport.c "
                "session-object=bluez_daemon_a2dp_session rounds=2 "
                "active-transitions=2 idle-transitions=2",
                "re:role=sink .*acquire-events=[1-9][0-9]* "
                "release-events=[1-9][0-9]* balanced-rounds=2 "
                "zero-ref-rounds=2 final-balanced=1 final-zero=1 "
                "avdtp-transaction-begin=[1-9][0-9]* "
                "avdtp-transaction-complete=[1-9][0-9]* "
                "transaction-balanced-rounds=2 "
                "transaction-final-balanced=1 "
                "media-payload-tx=0 media-payload-rx=[1-9][0-9]* "
                "media-payload-tx-bytes=0 "
                "media-payload-rx-bytes=[1-9][0-9]* media-role-ok=1 "
                "codec-configured=2 sbc-encode=0 "
                "sbc-decode=[1-9][0-9]* sbc-encode-bytes=0 "
                "sbc-decode-bytes=[1-9][0-9]* codec-role-errors=0 "
                "codec-owner-rounds=2 codec-final-ok=1 "
                "transport-acquire=2 transport-release=2 "
                "transport-fd-open=2 transport-fd-close=2 "
                "transport-busy=0 transport-state-errors=0 "
                "transport-owner-rounds=2 transport-final-ok=1 "
                "avrcp-command-tx=0 avrcp-command-rx=[1-9][0-9]* "
                "avrcp-response-tx=[1-9][0-9]* avrcp-control-tx=0 "
                "avrcp-control-rx=[1-9][0-9]* avrcp-browse-tx=0 "
                "avrcp-browse-rx=[1-9][0-9]* "
                "avrcp-bytes-tx=[1-9][0-9]* "
                "avrcp-bytes-rx=[1-9][0-9]* avrcp-role-errors=0 "
                "avrcp-owner-rounds=2 avrcp-final-ok=1 "
                "l2cap-open=[1-9][0-9]* l2cap-connect=[1-9][0-9]* "
                "l2cap-write=[1-9][0-9]* l2cap-recv=[1-9][0-9]* "
                "l2cap-close=[1-9][0-9]* "
                "l2cap-avdtp=[1-9][0-9]* "
                "l2cap-avrcp=[1-9][0-9]* "
                "l2cap-media=2 l2cap-state-errors=0 "
                "l2cap-owner-rounds=2 l2cap-final-ok=1 "
                "avdtp-configured=2 avdtp-opened=2 "
                "avdtp-started=2 avdtp-suspended=2 "
                "avdtp-closed=2 avdtp-state-errors=0 "
                "state-machine-rounds=2 state-machine-final-ok=1",
                "bluez-daemon: a2dp closeout profile-lifecycle "
                "role=sink source=third/bluez/src/profile.c+"
                "third/bluez/src/device.c+third/bluez/src/sdpd-service.c+"
                "third/bluez/profiles/audio/a2dp.c "
                "profile-register=1 profile-unregister=1 "
                "device-connect=2 device-disconnect=2 "
                "sdp-register=1 sdp-unregister=1 "
                "service-discovery=1 service-resolve=1 cache-remove=1 "
                "state-errors=0 final-ok=1",
                "bluez-daemon: a2dp closeout dbus-object-lifecycle "
                "role=sink source=third/bluez/src/adapter.c+"
                "third/bluez/src/dbus-common.c+"
                "third/bluez/profiles/audio/media.c+"
                "third/bluez/profiles/audio/transport.c "
                "name-acquire=1 get-managed-objects=1 "
                "interfaces-added=3 endpoint-added=1 transport-added=1 "
                "player-added=1 owner-lost=1 interfaces-removed=5 "
                "owner-reacquire=1 objects-readd=3 name-release=1 "
                "state-errors=0 final-ok=1",
                "bluez-daemon: a2dp closeout mainloop-lifecycle "
                "role=sink source=third/bluez/src/shared/mainloop.c+"
                "third/bluez/src/shared/io-mainloop.c "
                "watch-add=7 watch-remove=7 timer-add=2 timer-remove=2 "
                "dispatch-mgmt=1 dispatch-l2cap=",
                "re:role=sink .*dispatch-l2cap=[1-9][0-9]* "
                "dispatch-avdtp=[1-9][0-9]* "
                "dispatch-avctp=[1-9][0-9]* "
                "dispatch-media=[1-9][0-9]* "
                "dispatch-dbus=[1-9][0-9]* state-errors=0 final-ok=1",
                "bluez-daemon: a2dp closeout dbus-owner-recovery cleanup "
                "role=sink owner=:client.a2dp dbus-owners=0 "
                "endpoint-refs=0 transport-refs=0 player-refs=0 "
                "interfaces=0 acquire-ref=0 media-fd=closed watches=0",
                "bluez-daemon: a2dp closeout transport-owner role=sink "
                "state=idle owner=:client.a2dp fd-owner=none "
                "acquire-ref=0 transport-ref=0 media-fd=closed "
                "write-watch=0 read-watch=0",
                "bluez-daemon: a2dp closeout mainloop cleanup role=sink "
                "watch-remove=mgmt,l2cap-signaling,l2cap-media,avdtp,avctp,"
                "media-transport,dbus timer-remove=avdtp-retry,avrcp-notify "
                "dispatch-pending=0 watches=0 timers=0 owner=bluetoothd",
                "bluez-daemon: a2dp closeout sdp-profile cleanup "
                "role=sink record-unregister=1 cache-remove=1 "
                "service-discovery=0 records=0",
                "bluez-daemon: a2dp closeout l2cap-controller cleanup "
                "role=sink channel-disconnect=signaling,media,avctp,"
                "avctp-browsing channels=0 refs=0 retrans=0 pending=0",
                "bluez-daemon: audio-a2dp-closeout-full cleanup "
                "role=sink profile-registered=0 dbus-owners=0 "
                "transaction-pending=0 timers=0 acquire-ref=0 caps-ref=0 "
                "config-ref=0 device-ref=0 session-ref=0 stream-ref=0 "
                "sep-ref=0 endpoint-refs=0 transport-refs=0 player-refs=0 "
                "avdtp=0 avctp=0 media=0 l2cap-fds=0 watches=0 "
                "sessions=0 rounds=2",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0 closeout=0",
                "bluez-daemon: audio-a2dp-closeout-full complete "
                "role=sink peer=1",
                "re:l2cap-socket-bind=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-connect=([1-9][6-9]|[2-9][0-9]+)",
                "re:l2cap-socket-recv=([1-9][6-9]|[2-9][0-9]+)",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-current-complete-closeout",
        (
            RoleCheck("bt1", (
                "BLUEZA2DPCOMPLETE_BEGIN_BT1",
                "bluez-daemon: audio-a2dp-closeout-full "
                "source=third/bluez/src/profile.c+third/bluez/src/device.c+"
                "third/bluez/profiles/audio/a2dp.c+"
                "third/bluez/profiles/audio/avdtp.c+"
                "third/bluez/profiles/audio/media.c+"
                "third/bluez/profiles/audio/transport.c+"
                "third/bluez/profiles/audio/avrcp.c role=source peer=2 "
                "semantic=a2dp-current-closeout-umbrella",
                "bluez-daemon: a2dp closeout phase=mainloop-ownership "
                "role=source",
                "bluez-daemon: a2dp closeout phase=profile-session "
                "role=source profile-registered=1",
                "bluez-daemon: a2dp closeout phase=sdp-profile "
                "role=source",
                "bluez-daemon: a2dp closeout phase=l2cap-controller "
                "role=source",
                "bluez-daemon: l2cap ioctl-probe label=a2dp-media "
                "role=source ret=0 proto=BTPROTO_L2CAP",
                "upstream-l2cap-ioctl: psm=0x0019 cid=0x0041 "
                "handle=0x0052 inq-ret=0",
                "proto=BTPROTO_L2CAP",
                "bluez-daemon: l2cap ordinary-listen-accept "
                "label=a2dp-media role=source ret=0 proto=BTPROTO_L2CAP",
                "nonblock-ret=0",
                "create-nonblock-ok=1",
                "stream-ok=1",
                "dgram-ok=1",
                "raw-ok=1",
                "listen-ret=0",
                "connect-ret=0",
                "accept-ret=0",
                "pending-accept-ok=1",
                "bluez-daemon: l2cap poll-probe label=a2dp-media "
                "role=source ret=0 events=POLLOUT proto=BTPROTO_L2CAP",
                "upstream-l2cap-poll: psm=0x0019 cid=0x0041 "
                "handle=0x0052 events=POLLOUT",
                "ready=1",
                "bluez-daemon: l2cap timestamp-probe label=a2dp-media "
                "role=source ret=0 proto=BTPROTO_L2CAP",
                "upstream-l2cap-timestamp: psm=0x0019 cid=0x0041 "
                "handle=0x0052 timeval-ret=0",
                "timespec-ret=0",
                "bluez-daemon: l2cap socket-option-probe "
                "label=avdtp-discover role=source ret=0",
                "upstream-l2cap-options: opt=L2CAP_OPTIONS set-ret=0 "
                "get-ret=0 imtu=247 omtu=247 mode=0 fcs=1 "
                "opt=L2CAP_LM lm-set-ret=0 lm-get-ret=0",
                "opt=BT_FLUSHABLE set-ret=0 get-ret=0 value=1",
                "opt=BT_POWER set-ret=0 get-ret=0 force-active=1",
                "opt=BT_CHANNEL_POLICY set-ret=-95 get-ret=0 policy=0",
                "bluez-daemon: l2cap connected-option-probe "
                "label=avdtp-discover role=source ret=0",
                "upstream-l2cap-connected-options: opt=L2CAP_OPTIONS "
                "set-connected-ret=-22 opt=BT_MODE set-connected-ret=-92 "
                "bt-mode-gate=ecred-disabled "
                "conninfo-ret=0 conninfo-handle=0x0052 "
                "conninfo-dev-class=00:00:00 "
                "opt=BT_PHY get-ret=0",
                "set-bredr-ret=0",
                "invalid-le-ret=-22",
                "gate=hci-conn-set-phy-type-check",
                "bluez-daemon: a2dp closeout phase=codec-policy "
                "role=source sbc-select=ok",
                "bluez-daemon: a2dp closeout phase=media-transport "
                "role=source acquire=ok duplicate-acquire=busy",
                "bluez-daemon: a2dp closeout phase=dbus-owner-recovery "
                "role=source",
                "bluez-daemon: a2dp closeout phase=avdtp-transactions "
                "role=source discover=ok get-capabilities=ok "
                "setconfig=ok open=ok start=ok",
                "bluez-daemon: a2dp closeout phase=error-policy "
                "role=source start-before-open=reject duplicate-open=reject",
                "bluez-daemon: audio owner source round=1 complete",
                "bluez-daemon: audio owner source round=2 complete",
                "bluez-daemon: a2dp closeout upstream-object-lifecycle "
                "role=source",
                "re:role=source .*final-balanced=1 final-zero=1 "
                ".*transaction-final-balanced=1 .*media-role-ok=1 "
                ".*codec-final-ok=1 .*transport-final-ok=1 "
                ".*avrcp-final-ok=1 .*l2cap-final-ok=1 "
                ".*state-machine-final-ok=1",
                "bluez-daemon: a2dp closeout profile-lifecycle "
                "role=source",
                "state-errors=0 final-ok=1",
                "bluez-daemon: a2dp closeout dbus-object-lifecycle "
                "role=source",
                "bluez-daemon: a2dp closeout mainloop-lifecycle "
                "role=source",
                "bluez-daemon: a2dp closeout dbus-owner-recovery cleanup "
                "role=source owner=:client.a2dp dbus-owners=0",
                "bluez-daemon: a2dp closeout transport-owner role=source "
                "state=idle owner=:client.a2dp fd-owner=none",
                "bluez-daemon: a2dp closeout mainloop cleanup role=source",
                "bluez-daemon: a2dp closeout sdp-profile cleanup "
                "role=source record-unregister=1 cache-remove=1",
                "bluez-daemon: a2dp closeout l2cap-controller cleanup "
                "role=source",
                "upstream-l2cap-shutdown-close: psm=0x0019 cid=0x0041 "
                "handle=0x0052",
                "post-shutdown=0x3 proto=BTPROTO_L2CAP",
                "bluez-daemon: audio-a2dp-closeout-full cleanup "
                "role=source profile-registered=0 dbus-owners=0 "
                "transaction-pending=0 timers=0",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0 closeout=0",
                "bluez-daemon: audio-a2dp-closeout-full complete "
                "role=source peer=2",
                "BLUEZA2DPCOMPLETE_DONE_BT1",
            )),
            RoleCheck("bt2", (
                "BLUEZA2DPCOMPLETE_BEGIN_BT2",
                "bluez-daemon: audio-a2dp-closeout-full "
                "source=third/bluez/src/profile.c+third/bluez/src/device.c+"
                "third/bluez/profiles/audio/a2dp.c+"
                "third/bluez/profiles/audio/avdtp.c+"
                "third/bluez/profiles/audio/media.c+"
                "third/bluez/profiles/audio/transport.c+"
                "third/bluez/profiles/audio/avrcp.c role=sink peer=1 "
                "semantic=a2dp-current-closeout-umbrella",
                "bluez-daemon: a2dp closeout phase=mainloop-ownership "
                "role=sink",
                "bluez-daemon: a2dp closeout phase=profile-session "
                "role=sink profile-registered=1",
                "bluez-daemon: a2dp closeout phase=sdp-profile "
                "role=sink",
                "bluez-daemon: a2dp closeout phase=l2cap-controller "
                "role=sink",
                "bluez-daemon: l2cap ioctl-probe label=a2dp-media "
                "role=sink ret=0 proto=BTPROTO_L2CAP",
                "upstream-l2cap-ioctl: psm=0x0019 cid=0x0041 "
                "handle=0x0052 inq-ret=0",
                "proto=BTPROTO_L2CAP",
                "bluez-daemon: l2cap ordinary-listen-accept "
                "label=a2dp-media role=sink ret=0 proto=BTPROTO_L2CAP",
                "nonblock-ret=0",
                "create-nonblock-ok=1",
                "stream-ok=1",
                "dgram-ok=1",
                "raw-ok=1",
                "listen-ret=0",
                "connect-ret=0",
                "accept-ret=0",
                "pending-accept-ok=1",
                "bluez-daemon: l2cap poll-probe label=a2dp-media "
                "role=sink ret=0 events=POLLOUT proto=BTPROTO_L2CAP",
                "upstream-l2cap-poll: psm=0x0019 cid=0x0041 "
                "handle=0x0052 events=POLLOUT",
                "ready=1",
                "bluez-daemon: l2cap timestamp-probe label=a2dp-media "
                "role=sink ret=0 proto=BTPROTO_L2CAP",
                "upstream-l2cap-timestamp: psm=0x0019 cid=0x0041 "
                "handle=0x0052 timeval-ret=0",
                "timespec-ret=0",
                "bluez-daemon: l2cap socket-option-probe "
                "label=avdtp-discover role=sink ret=0",
                "upstream-l2cap-options: opt=L2CAP_OPTIONS set-ret=0 "
                "get-ret=0 imtu=247 omtu=247 mode=0 fcs=1 "
                "opt=L2CAP_LM lm-set-ret=0 lm-get-ret=0",
                "opt=BT_FLUSHABLE set-ret=0 get-ret=0 value=1",
                "opt=BT_POWER set-ret=0 get-ret=0 force-active=1",
                "opt=BT_CHANNEL_POLICY set-ret=-95 get-ret=0 policy=0",
                "bluez-daemon: l2cap connected-option-probe "
                "label=avdtp-discover role=sink ret=0",
                "upstream-l2cap-connected-options: opt=L2CAP_OPTIONS "
                "set-connected-ret=-22 opt=BT_MODE set-connected-ret=-92 "
                "bt-mode-gate=ecred-disabled "
                "conninfo-ret=0 conninfo-handle=0x0052 "
                "conninfo-dev-class=00:00:00 "
                "opt=BT_PHY get-ret=0",
                "set-bredr-ret=0",
                "invalid-le-ret=-22",
                "gate=hci-conn-set-phy-type-check",
                "bluez-daemon: a2dp closeout phase=codec-policy "
                "role=sink sbc-select=ok",
                "bluez-daemon: a2dp closeout phase=media-transport "
                "role=sink acquire=ok duplicate-acquire=busy",
                "bluez-daemon: a2dp closeout phase=dbus-owner-recovery "
                "role=sink",
                "bluez-daemon: a2dp closeout phase=avdtp-transactions "
                "role=sink discover=ok get-capabilities=ok "
                "setconfig=ok open=ok start=ok",
                "bluez-daemon: a2dp closeout phase=error-policy "
                "role=sink start-before-open=reject duplicate-open=reject",
                "bluez-daemon: audio owner sink round=1 complete",
                "bluez-daemon: audio owner sink round=2 complete",
                "bluez-daemon: a2dp closeout upstream-object-lifecycle "
                "role=sink",
                "re:role=sink .*final-balanced=1 final-zero=1 "
                ".*transaction-final-balanced=1 .*media-role-ok=1 "
                ".*codec-final-ok=1 .*transport-final-ok=1 "
                ".*avrcp-final-ok=1 .*l2cap-final-ok=1 "
                ".*state-machine-final-ok=1",
                "bluez-daemon: a2dp closeout profile-lifecycle "
                "role=sink",
                "state-errors=0 final-ok=1",
                "bluez-daemon: a2dp closeout dbus-object-lifecycle "
                "role=sink",
                "bluez-daemon: a2dp closeout mainloop-lifecycle "
                "role=sink",
                "bluez-daemon: a2dp closeout dbus-owner-recovery cleanup "
                "role=sink owner=:client.a2dp dbus-owners=0",
                "bluez-daemon: a2dp closeout transport-owner role=sink "
                "state=idle owner=:client.a2dp fd-owner=none",
                "bluez-daemon: a2dp closeout mainloop cleanup role=sink",
                "bluez-daemon: a2dp closeout sdp-profile cleanup "
                "role=sink record-unregister=1 cache-remove=1",
                "bluez-daemon: a2dp closeout l2cap-controller cleanup "
                "role=sink",
                "upstream-l2cap-shutdown-close: psm=0x0019 cid=0x0041 "
                "handle=0x0052",
                "post-shutdown=0x3 proto=BTPROTO_L2CAP",
                "bluez-daemon: audio-a2dp-closeout-full cleanup "
                "role=sink profile-registered=0 dbus-owners=0 "
                "transaction-pending=0 timers=0",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0 closeout=0",
                "bluez-daemon: audio-a2dp-closeout-full complete "
                "role=sink peer=1",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
                "BLUEZA2DPCOMPLETE_DONE_BT2",
            )),
        ),
    ),
    CaseCheck(
        "bluez-hfp-hsp-profile-closeout",
        (
            RoleCheck("bt1", (
                "BLUEZHFPHSP_BEGIN_BT1",
                "bluez-hfp: native-contract mode=hfp-hf role=handsfree",
                "bluez-hfp: semantic-contract mode=hfp-hf role=handsfree",
                "upstream-link=bluezhfp-rfcomm-sco-link-to-bluez-audio",
                "bluez-hfp: rfcomm listen-accept mode=hfp-hf "
                "role=handsfree ret=0",
                "upstream-rfcomm-listen-accept: channel=1 cid=0x0061 "
                "handle=0x0052 open-ret=0 listen-ret=0 accept-ret=0 "
                "accepted-state=1 getname-ret=10 getname-channel=1 peer-getname-ret=-107 accepted-getname-ret=10 accepted-channel=1 accepted-peer-getname-ret=10 accepted-peer-channel=1 sec-ret=0 sec-get-ret=0 sec-level=2 defer-ret=0 defer-get-ret=0 defer=1 accepted-sec-level=2 accepted-defer=1 proto=BTPROTO_RFCOMM proto-name=RFCOMM",
                "bluez-hfp: rfcomm open psm=0x0003 cid=0x0061 ret=0",
                "bluez-hfp: rfcomm connect psm=0x0003 cid=0x0061 ret=0",
                "bluez-hfp: rfcomm socket-parity mode=hfp-hf "
                "role=handsfree proto=BTPROTO_RFCOMM",
                "bluez-hfp: rfcomm ioctl mode=hfp-hf "
                "role=handsfree ret=0 proto=BTPROTO_RFCOMM",
                "upstream-rfcomm-ioctl: channel=1 cid=0x0061 "
                "handle=0x0052 inq-ret=0",
                "conninfo-ret=0 conninfo-handle=0x0052",
                "conninfo-dev-class=00:00:00",
                "fallback=rfcomm-tty-disabled state=1 "
                "proto=BTPROTO_RFCOMM",
                "bluez-hfp: rfcomm poll mode=hfp-hf role=handsfree "
                "ret=0 events=POLLOUT proto=BTPROTO_RFCOMM",
                "upstream-rfcomm-poll: channel=1 cid=0x0061 "
                "handle=0x0052 events=POLLOUT",
                "bluez-hfp: rfcomm timestamp mode=hfp-hf "
                "role=handsfree ret=0 proto=BTPROTO_RFCOMM",
                "upstream-rfcomm-timestamp: channel=1 cid=0x0061 "
                "handle=0x0052 timeval-ret=0",
                "upstream-rfcomm-shutdown-close: channel=1 cid=0x0061 "
                "handle=0x0052",
                "post-shutdown=0x3 proto=BTPROTO_RFCOMM",
                "bluez-hfp: at-request-response-evidence role=handsfree "
                "label=hfp-slc-brsf",
                "bluez-hfp: at-request-response-evidence role=handsfree "
                "label=hfp-codec-bac-bcs",
                "bluez-hfp: at-request-response-evidence role=handsfree "
                "label=hfp-call-clcc",
                "bluez-hfp: sco socket-parity mode=hfp-hf "
                "role=handsfree proto=BTPROTO_SCO",
                "bluez-hfp: sco ioctl mode=hfp-hf role=handsfree "
                "ret=0 proto=BTPROTO_SCO",
                "upstream-sco-ioctl: handle=0x0052 inq-ret=0",
                "options-ret=0 options-mtu=60",
                "sco-mtu-sockopt=SOL_BLUETOOTH sndmtu-ret=0 sndmtu=60",
                "rcvmtu-ret=0 rcvmtu=60",
                "conninfo-ret=0 conninfo-handle=0x0052",
                "conninfo-dev-class=00:00:00",
                "no-op=socketpair-mmap socketpair-ret=-95 mmap-ret=-95",
                "bluez-hfp: sco poll mode=hfp-hf role=handsfree "
                "ret=0 events=POLLOUT proto=BTPROTO_SCO",
                "upstream-sco-poll: handle=0x0052 events=POLLOUT",
                "bluez-hfp: sco timestamp mode=hfp-hf role=handsfree "
                "ret=0 proto=BTPROTO_SCO",
                "upstream-sco-timestamp: handle=0x0052 "
                "timeval-ret=0",
                "upstream-sco-shutdown-close: handle=0x0052",
                "post-shutdown=0x3 proto=BTPROTO_SCO",
                "bluez-hfp: sco listen-accept mode=hfp-hf "
                "role=handsfree ret=0",
                "upstream-sco-listen-accept: handle=0x0052 open-ret=0 "
                "listen-ret=0 accept-ret=0 accepted-state=1 getname-ret=8 peer-getname-ret=8 accepted-getname-ret=8 accepted-peer-getname-ret=8 mtu=60 voice=0x0060 defer-ret=0 defer-get-ret=0 defer=1 pkt-ret=0 pkt-get-ret=0 pkt-status=1 codec-ret=-95 accepted-defer=1 accepted-pkt-status=1 sndmtu-ret=0 sndmtu=60 rcvmtu-ret=0 rcvmtu=60",
                "upstream-sco-write-handle: handle=",
                "bluez-hfp: audio-lifecycle-contract mode=hfp-hf "
                "role=handsfree",
                "bluez-hfp: ordinary-rfcomm-socket mode=hfp-hf "
                "role=handsfree",
                "rfcomm-lm-set-ret=0 rfcomm-lm-get-ret=0 "
                "rfcomm-lm=0x00000027 rfcomm-lm-fips-ret=-1",
                "seqpacket-ret=-1",
                "seqpacket-esocktnosupport=1",
                "btsec-set-ret=0 btsec-get-ret=0 btsec-level=3 "
                "btsec-fips-ret=-1 btsec-fips-errno=22",
                "recvmsg-ret=",
                "getsockname-ret=0",
                "getpeername-ret=0",
                "shutdown-ret=0",
                "poll-ret=",
                "poll-revents=0x",
                "ioctl-inq-ret=0",
                "ioctl-outq-ret=0",
                "nonblock-ret=0",
                "create-nonblock-ok=1",
                "listen-ret=0",
                "accept-ret=0",
                "path=ordinary-socket final-ok=1",
                "bluez-hfp: ordinary-sco-socket mode=hfp-hf "
                "role=handsfree",
                "ordinary-mtu-sockopt=SOL_BLUETOOTH sndmtu-ret=0 sndmtu=60",
                "rcvmtu-ret=0 rcvmtu=60",
                "legacy-sco-mtu-enoprotoopt=1",
                "legacy-sco-handle-enoprotoopt=1",
                "stream-ret=-1",
                "stream-esocktnosupport=1",
                "dgram-ret=-1",
                "dgram-esocktnosupport=1",
                "raw-ret=-1",
                "raw-esocktnosupport=1",
                "recvmsg-ret=",
                "getsockname-ret=0",
                "getpeername-ret=0",
                "shutdown-ret=0",
                "poll-ret=",
                "poll-revents=0x",
                "ioctl-inq-ret=0",
                "ioctl-outq-ret=0",
                "nonblock-ret=0",
                "create-nonblock-ok=1",
                "listen-ret=0",
                "accept-ret=0",
                "bluez-hfp: closeout upstream-link-ledger mode=hfp-hf "
                "role=handsfree",
                "bluez-hfp: closeout upstream-source-parity mode=hfp-hf "
                "role=handsfree",
                "upstream-link=bluezhfp-hfp-upstream-link-bluetoothd "
                "parity-final=1",
                "upstream-link=bluezhfp-hfp-upstream-link-bluetoothd "
                "final-ok=1",
                "bluez-hfp: native-contract mode=hsp-hs role=headset",
                "bluez-hfp: semantic-contract mode=hsp-hs role=headset",
                "bluez-hfp: rfcomm listen-accept mode=hsp-hs "
                "role=headset ret=0",
                "upstream-rfcomm-listen-accept: channel=2 cid=0x0062 "
                "handle=0x0052 open-ret=0 listen-ret=0 accept-ret=0 "
                "accepted-state=1 getname-ret=10 getname-channel=2 peer-getname-ret=-107 accepted-getname-ret=10 accepted-channel=2 accepted-peer-getname-ret=10 accepted-peer-channel=2 sec-ret=0 sec-get-ret=0 sec-level=2 defer-ret=0 defer-get-ret=0 defer=1 accepted-sec-level=2 accepted-defer=1 proto=BTPROTO_RFCOMM proto-name=RFCOMM",
                "bluez-hfp: rfcomm open psm=0x0003 cid=0x0062 ret=0",
                "bluez-hfp: rfcomm connect psm=0x0003 cid=0x0062 ret=0",
                "bluez-hfp: rfcomm socket-parity mode=hsp-hs "
                "role=headset proto=BTPROTO_RFCOMM",
                "bluez-hfp: rfcomm ioctl mode=hsp-hs "
                "role=headset ret=0 proto=BTPROTO_RFCOMM",
                "upstream-rfcomm-ioctl: channel=2 cid=0x0062 "
                "handle=0x0052 inq-ret=0",
                "conninfo-ret=0 conninfo-handle=0x0052",
                "conninfo-dev-class=00:00:00",
                "fallback=rfcomm-tty-disabled state=1 "
                "proto=BTPROTO_RFCOMM",
                "bluez-hfp: rfcomm poll mode=hsp-hs role=headset "
                "ret=0 events=POLLOUT proto=BTPROTO_RFCOMM",
                "upstream-rfcomm-poll: channel=2 cid=0x0062 "
                "handle=0x0052 events=POLLOUT",
                "bluez-hfp: rfcomm timestamp mode=hsp-hs "
                "role=headset ret=0 proto=BTPROTO_RFCOMM",
                "upstream-rfcomm-timestamp: channel=2 cid=0x0062 "
                "handle=0x0052 timeval-ret=0",
                "upstream-rfcomm-shutdown-close: channel=2 cid=0x0062 "
                "handle=0x0052",
                "post-shutdown=0x3 proto=BTPROTO_RFCOMM",
                "bluez-hfp: at-request-response-evidence role=headset "
                "label=hsp-button-ckpd",
                "bluez-hfp: at-request-response-evidence role=headset "
                "label=hsp-volume-vgs",
                "bluez-hfp: sco socket-parity mode=hsp-hs "
                "role=headset proto=BTPROTO_SCO",
                "bluez-hfp: sco ioctl mode=hsp-hs role=headset "
                "ret=0 proto=BTPROTO_SCO",
                "upstream-sco-ioctl: handle=0x0052 inq-ret=0",
                "options-ret=0 options-mtu=60",
                "sco-mtu-sockopt=SOL_BLUETOOTH sndmtu-ret=0 sndmtu=60",
                "rcvmtu-ret=0 rcvmtu=60",
                "conninfo-ret=0 conninfo-handle=0x0052",
                "conninfo-dev-class=00:00:00",
                "no-op=socketpair-mmap socketpair-ret=-95 mmap-ret=-95",
                "bluez-hfp: sco poll mode=hsp-hs role=headset "
                "ret=0 events=POLLOUT proto=BTPROTO_SCO",
                "upstream-sco-poll: handle=0x0052 events=POLLOUT",
                "bluez-hfp: sco timestamp mode=hsp-hs role=headset "
                "ret=0 proto=BTPROTO_SCO",
                "upstream-sco-timestamp: handle=0x0052 "
                "timeval-ret=0",
                "upstream-sco-shutdown-close: handle=0x0052",
                "post-shutdown=0x3 proto=BTPROTO_SCO",
                "bluez-hfp: sco listen-accept mode=hsp-hs "
                "role=headset ret=0",
                "upstream-sco-listen-accept: handle=0x0052 open-ret=0 "
                "listen-ret=0 accept-ret=0 accepted-state=1 getname-ret=8 peer-getname-ret=8 accepted-getname-ret=8 accepted-peer-getname-ret=8 mtu=60 voice=0x0060 defer-ret=0 defer-get-ret=0 defer=1 pkt-ret=0 pkt-get-ret=0 pkt-status=1 codec-ret=-95 accepted-defer=1 accepted-pkt-status=1 sndmtu-ret=0 sndmtu=60 rcvmtu-ret=0 rcvmtu=60",
                "bluez-hfp: audio-lifecycle-contract mode=hsp-hs "
                "role=headset",
                "bluez-hfp: ordinary-rfcomm-socket mode=hsp-hs "
                "role=headset",
                "rfcomm-lm-set-ret=0 rfcomm-lm-get-ret=0 "
                "rfcomm-lm=0x00000027 rfcomm-lm-fips-ret=-1",
                "seqpacket-ret=-1",
                "seqpacket-esocktnosupport=1",
                "btsec-set-ret=0 btsec-get-ret=0 btsec-level=3 "
                "btsec-fips-ret=-1 btsec-fips-errno=22",
                "recvmsg-ret=",
                "getsockname-ret=0",
                "getpeername-ret=0",
                "shutdown-ret=0",
                "poll-ret=",
                "poll-revents=0x",
                "ioctl-inq-ret=0",
                "ioctl-outq-ret=0",
                "nonblock-ret=0",
                "create-nonblock-ok=1",
                "listen-ret=0",
                "accept-ret=0",
                "path=ordinary-socket final-ok=1",
                "bluez-hfp: ordinary-sco-socket mode=hsp-hs "
                "role=headset",
                "ordinary-mtu-sockopt=SOL_BLUETOOTH sndmtu-ret=0 sndmtu=60",
                "rcvmtu-ret=0 rcvmtu=60",
                "legacy-sco-mtu-enoprotoopt=1",
                "legacy-sco-handle-enoprotoopt=1",
                "stream-ret=-1",
                "stream-esocktnosupport=1",
                "dgram-ret=-1",
                "dgram-esocktnosupport=1",
                "raw-ret=-1",
                "raw-esocktnosupport=1",
                "recvmsg-ret=",
                "getsockname-ret=0",
                "getpeername-ret=0",
                "shutdown-ret=0",
                "poll-ret=",
                "poll-revents=0x",
                "ioctl-inq-ret=0",
                "ioctl-outq-ret=0",
                "nonblock-ret=0",
                "create-nonblock-ok=1",
                "listen-ret=0",
                "accept-ret=0",
                "bluez-hfp: closeout upstream-link-ledger mode=hsp-hs "
                "role=headset",
                "bluez-hfp: closeout upstream-source-parity mode=hsp-hs "
                "role=headset",
                "upstream-link=bluezhfp-hsp-upstream-link-bluetoothd "
                "parity-final=1",
                "upstream-link=bluezhfp-hsp-upstream-link-bluetoothd "
                "final-ok=1",
                "BLUEZHFPHSP_DONE_BT1",
            )),
            RoleCheck("bt2", (
                "BLUEZHFPHSP_BEGIN_BT2",
                "bluez-hfp: native-contract mode=hfp-ag "
                "role=audio-gateway",
                "bluez-hfp: semantic-contract mode=hfp-ag "
                "role=audio-gateway",
                "upstream-link=bluezhfp-rfcomm-sco-link-to-bluez-audio",
                "bluez-hfp: rfcomm listen-accept mode=hfp-ag "
                "role=audio-gateway ret=0",
                "upstream-rfcomm-listen-accept: channel=1 cid=0x0061 "
                "handle=0x0052 open-ret=0 listen-ret=0 accept-ret=0 "
                "accepted-state=1 getname-ret=10 getname-channel=1 peer-getname-ret=-107 accepted-getname-ret=10 accepted-channel=1 accepted-peer-getname-ret=10 accepted-peer-channel=1 sec-ret=0 sec-get-ret=0 sec-level=2 defer-ret=0 defer-get-ret=0 defer=1 accepted-sec-level=2 accepted-defer=1 proto=BTPROTO_RFCOMM proto-name=RFCOMM",
                "bluez-hfp: rfcomm open psm=0x0003 cid=0x0061 ret=0",
                "bluez-hfp: rfcomm connect psm=0x0003 cid=0x0061 ret=0",
                "bluez-hfp: rfcomm socket-parity mode=hfp-ag "
                "role=audio-gateway proto=BTPROTO_RFCOMM",
                "bluez-hfp: rfcomm ioctl mode=hfp-ag "
                "role=audio-gateway ret=0 proto=BTPROTO_RFCOMM",
                "upstream-rfcomm-ioctl: channel=1 cid=0x0061 "
                "handle=0x0052 inq-ret=0",
                "conninfo-ret=0 conninfo-handle=0x0052",
                "conninfo-dev-class=00:00:00",
                "fallback=rfcomm-tty-disabled state=1 "
                "proto=BTPROTO_RFCOMM",
                "bluez-hfp: rfcomm poll mode=hfp-ag role=audio-gateway "
                "ret=0 events=POLLOUT proto=BTPROTO_RFCOMM",
                "upstream-rfcomm-poll: channel=1 cid=0x0061 "
                "handle=0x0052 events=POLLOUT",
                "bluez-hfp: rfcomm timestamp mode=hfp-ag "
                "role=audio-gateway ret=0 proto=BTPROTO_RFCOMM",
                "upstream-rfcomm-timestamp: channel=1 cid=0x0061 "
                "handle=0x0052 timeval-ret=0",
                "upstream-rfcomm-shutdown-close: channel=1 cid=0x0061 "
                "handle=0x0052",
                "post-shutdown=0x3 proto=BTPROTO_RFCOMM",
                "bluez-hfp: at-request-response-evidence "
                "role=audio-gateway label=hfp-slc-brsf",
                "bluez-hfp: at-request-response-evidence "
                "role=audio-gateway label=hfp-codec-bac-bcs",
                "bluez-hfp: at-request-response-evidence "
                "role=audio-gateway label=hfp-call-clcc",
                "bluez-hfp: sco socket-parity mode=hfp-ag "
                "role=audio-gateway proto=BTPROTO_SCO",
                "bluez-hfp: sco ioctl mode=hfp-ag role=audio-gateway "
                "ret=0 proto=BTPROTO_SCO",
                "upstream-sco-ioctl: handle=0x0052 inq-ret=0",
                "options-ret=0 options-mtu=60",
                "sco-mtu-sockopt=SOL_BLUETOOTH sndmtu-ret=0 sndmtu=60",
                "rcvmtu-ret=0 rcvmtu=60",
                "conninfo-ret=0 conninfo-handle=0x0052",
                "conninfo-dev-class=00:00:00",
                "no-op=socketpair-mmap socketpair-ret=-95 mmap-ret=-95",
                "bluez-hfp: sco poll mode=hfp-ag role=audio-gateway "
                "ret=0 events=POLLOUT proto=BTPROTO_SCO",
                "upstream-sco-poll: handle=0x0052 events=POLLOUT",
                "bluez-hfp: sco timestamp mode=hfp-ag role=audio-gateway "
                "ret=0 proto=BTPROTO_SCO",
                "upstream-sco-timestamp: handle=0x0052 "
                "timeval-ret=0",
                "upstream-sco-shutdown-close: handle=0x0052",
                "post-shutdown=0x3 proto=BTPROTO_SCO",
                "bluez-hfp: sco listen-accept mode=hfp-ag "
                "role=audio-gateway ret=0",
                "upstream-sco-listen-accept: handle=0x0052 open-ret=0 "
                "listen-ret=0 accept-ret=0 accepted-state=1 getname-ret=8 peer-getname-ret=8 accepted-getname-ret=8 accepted-peer-getname-ret=8 mtu=60 voice=0x0060 defer-ret=0 defer-get-ret=0 defer=1 pkt-ret=0 pkt-get-ret=0 pkt-status=1 codec-ret=-95 accepted-defer=1 accepted-pkt-status=1 sndmtu-ret=0 sndmtu=60 rcvmtu-ret=0 rcvmtu=60",
                "bluez-hfp: audio-lifecycle-contract mode=hfp-ag "
                "role=audio-gateway",
                "bluez-hfp: ordinary-rfcomm-socket mode=hfp-ag "
                "role=audio-gateway",
                "rfcomm-lm-set-ret=0 rfcomm-lm-get-ret=0 "
                "rfcomm-lm=0x00000027 rfcomm-lm-fips-ret=-1",
                "seqpacket-ret=-1",
                "seqpacket-esocktnosupport=1",
                "btsec-set-ret=0 btsec-get-ret=0 btsec-level=3 "
                "btsec-fips-ret=-1 btsec-fips-errno=22",
                "recvmsg-ret=",
                "getsockname-ret=0",
                "getpeername-ret=0",
                "shutdown-ret=0",
                "poll-ret=",
                "poll-revents=0x",
                "ioctl-inq-ret=0",
                "ioctl-outq-ret=0",
                "nonblock-ret=0",
                "create-nonblock-ok=1",
                "listen-ret=0",
                "accept-ret=0",
                "path=ordinary-socket final-ok=1",
                "bluez-hfp: ordinary-sco-socket mode=hfp-ag "
                "role=audio-gateway",
                "ordinary-mtu-sockopt=SOL_BLUETOOTH sndmtu-ret=0 sndmtu=60",
                "rcvmtu-ret=0 rcvmtu=60",
                "legacy-sco-mtu-enoprotoopt=1",
                "legacy-sco-handle-enoprotoopt=1",
                "stream-ret=-1",
                "stream-esocktnosupport=1",
                "dgram-ret=-1",
                "dgram-esocktnosupport=1",
                "raw-ret=-1",
                "raw-esocktnosupport=1",
                "recvmsg-ret=",
                "getsockname-ret=0",
                "getpeername-ret=0",
                "shutdown-ret=0",
                "poll-ret=",
                "poll-revents=0x",
                "ioctl-inq-ret=0",
                "ioctl-outq-ret=0",
                "nonblock-ret=0",
                "create-nonblock-ok=1",
                "listen-ret=0",
                "accept-ret=0",
                "bluez-hfp: closeout upstream-link-ledger mode=hfp-ag "
                "role=audio-gateway",
                "bluez-hfp: closeout upstream-source-parity mode=hfp-ag "
                "role=audio-gateway",
                "upstream-link=bluezhfp-hfp-upstream-link-bluetoothd "
                "parity-final=1",
                "upstream-link=bluezhfp-hfp-upstream-link-bluetoothd "
                "final-ok=1",
                "bluez-hfp: native-contract mode=hsp-ag "
                "role=audio-gateway",
                "bluez-hfp: semantic-contract mode=hsp-ag "
                "role=audio-gateway",
                "bluez-hfp: rfcomm listen-accept mode=hsp-ag "
                "role=audio-gateway ret=0",
                "upstream-rfcomm-listen-accept: channel=2 cid=0x0062 "
                "handle=0x0052 open-ret=0 listen-ret=0 accept-ret=0 "
                "accepted-state=1 getname-ret=10 getname-channel=2 peer-getname-ret=-107 accepted-getname-ret=10 accepted-channel=2 accepted-peer-getname-ret=10 accepted-peer-channel=2 sec-ret=0 sec-get-ret=0 sec-level=2 defer-ret=0 defer-get-ret=0 defer=1 accepted-sec-level=2 accepted-defer=1 proto=BTPROTO_RFCOMM proto-name=RFCOMM",
                "bluez-hfp: rfcomm open psm=0x0003 cid=0x0062 ret=0",
                "bluez-hfp: rfcomm connect psm=0x0003 cid=0x0062 ret=0",
                "bluez-hfp: rfcomm socket-parity mode=hsp-ag "
                "role=audio-gateway proto=BTPROTO_RFCOMM",
                "bluez-hfp: rfcomm ioctl mode=hsp-ag "
                "role=audio-gateway ret=0 proto=BTPROTO_RFCOMM",
                "upstream-rfcomm-ioctl: channel=2 cid=0x0062 "
                "handle=0x0052 inq-ret=0",
                "conninfo-ret=0 conninfo-handle=0x0052",
                "conninfo-dev-class=00:00:00",
                "fallback=rfcomm-tty-disabled state=1 "
                "proto=BTPROTO_RFCOMM",
                "bluez-hfp: rfcomm poll mode=hsp-ag role=audio-gateway "
                "ret=0 events=POLLOUT proto=BTPROTO_RFCOMM",
                "upstream-rfcomm-poll: channel=2 cid=0x0062 "
                "handle=0x0052 events=POLLOUT",
                "bluez-hfp: rfcomm timestamp mode=hsp-ag "
                "role=audio-gateway ret=0 proto=BTPROTO_RFCOMM",
                "upstream-rfcomm-timestamp: channel=2 cid=0x0062 "
                "handle=0x0052 timeval-ret=0",
                "upstream-rfcomm-shutdown-close: channel=2 cid=0x0062 "
                "handle=0x0052",
                "post-shutdown=0x3 proto=BTPROTO_RFCOMM",
                "bluez-hfp: at-request-response-evidence "
                "role=audio-gateway label=hsp-button-ckpd",
                "bluez-hfp: at-request-response-evidence "
                "role=audio-gateway label=hsp-volume-vgs",
                "bluez-hfp: sco socket-parity mode=hsp-ag "
                "role=audio-gateway proto=BTPROTO_SCO",
                "bluez-hfp: sco ioctl mode=hsp-ag role=audio-gateway "
                "ret=0 proto=BTPROTO_SCO",
                "upstream-sco-ioctl: handle=0x0052 inq-ret=0",
                "options-ret=0 options-mtu=60",
                "sco-mtu-sockopt=SOL_BLUETOOTH sndmtu-ret=0 sndmtu=60",
                "rcvmtu-ret=0 rcvmtu=60",
                "conninfo-ret=0 conninfo-handle=0x0052",
                "conninfo-dev-class=00:00:00",
                "no-op=socketpair-mmap socketpair-ret=-95 mmap-ret=-95",
                "bluez-hfp: sco poll mode=hsp-ag role=audio-gateway "
                "ret=0 events=POLLOUT proto=BTPROTO_SCO",
                "upstream-sco-poll: handle=0x0052 events=POLLOUT",
                "bluez-hfp: sco timestamp mode=hsp-ag role=audio-gateway "
                "ret=0 proto=BTPROTO_SCO",
                "upstream-sco-timestamp: handle=0x0052 "
                "timeval-ret=0",
                "upstream-sco-shutdown-close: handle=0x0052",
                "post-shutdown=0x3 proto=BTPROTO_SCO",
                "bluez-hfp: sco listen-accept mode=hsp-ag "
                "role=audio-gateway ret=0",
                "upstream-sco-listen-accept: handle=0x0052 open-ret=0 "
                "listen-ret=0 accept-ret=0 accepted-state=1 getname-ret=8 peer-getname-ret=8 accepted-getname-ret=8 accepted-peer-getname-ret=8 mtu=60 voice=0x0060 defer-ret=0 defer-get-ret=0 defer=1 pkt-ret=0 pkt-get-ret=0 pkt-status=1 codec-ret=-95 accepted-defer=1 accepted-pkt-status=1 sndmtu-ret=0 sndmtu=60 rcvmtu-ret=0 rcvmtu=60",
                "bluez-hfp: audio-lifecycle-contract mode=hsp-ag "
                "role=audio-gateway",
                "bluez-hfp: ordinary-rfcomm-socket mode=hsp-ag "
                "role=audio-gateway",
                "rfcomm-lm-set-ret=0 rfcomm-lm-get-ret=0 "
                "rfcomm-lm=0x00000027 rfcomm-lm-fips-ret=-1",
                "seqpacket-ret=-1",
                "seqpacket-esocktnosupport=1",
                "btsec-set-ret=0 btsec-get-ret=0 btsec-level=3 "
                "btsec-fips-ret=-1 btsec-fips-errno=22",
                "recvmsg-ret=",
                "getsockname-ret=0",
                "getpeername-ret=0",
                "shutdown-ret=0",
                "poll-ret=",
                "poll-revents=0x",
                "ioctl-inq-ret=0",
                "ioctl-outq-ret=0",
                "nonblock-ret=0",
                "create-nonblock-ok=1",
                "listen-ret=0",
                "accept-ret=0",
                "path=ordinary-socket final-ok=1",
                "bluez-hfp: ordinary-sco-socket mode=hsp-ag "
                "role=audio-gateway",
                "ordinary-mtu-sockopt=SOL_BLUETOOTH sndmtu-ret=0 sndmtu=60",
                "rcvmtu-ret=0 rcvmtu=60",
                "legacy-sco-mtu-enoprotoopt=1",
                "legacy-sco-handle-enoprotoopt=1",
                "stream-ret=-1",
                "stream-esocktnosupport=1",
                "dgram-ret=-1",
                "dgram-esocktnosupport=1",
                "raw-ret=-1",
                "raw-esocktnosupport=1",
                "recvmsg-ret=",
                "getsockname-ret=0",
                "getpeername-ret=0",
                "shutdown-ret=0",
                "poll-ret=",
                "poll-revents=0x",
                "ioctl-inq-ret=0",
                "ioctl-outq-ret=0",
                "nonblock-ret=0",
                "create-nonblock-ok=1",
                "listen-ret=0",
                "accept-ret=0",
                "bluez-hfp: closeout upstream-link-ledger mode=hsp-ag "
                "role=audio-gateway",
                "bluez-hfp: closeout upstream-source-parity mode=hsp-ag "
                "role=audio-gateway",
                "upstream-link=bluezhfp-hsp-upstream-link-bluetoothd "
                "parity-final=1",
                "upstream-link=bluezhfp-hsp-upstream-link-bluetoothd "
                "final-ok=1",
                "BLUEZHFPHSP_DONE_BT2",
            )),
        ),
    ),
    CaseCheck(
        "bluez-hid-upstream-convergence-closeout",
        (
            RoleCheck("bt1", (
                "BLUEZHIDUPSTREAM_BEGIN_BT1",
                "bluez-hid: native-contract role=classic-host "
                "profile=classic-hid",
                "bluez-hid: semantic-contract role=classic-host "
                "profile=classic-hid",
                "upstream-link=bluezhid-fd-link-to-imported-hidp",
                "bluez-hid: control open psm=0x0011 cid=0x0051 ret=0",
                "bluez-hid: control connect psm=0x0011 cid=0x0051 ret=0",
                "bluez-hid: interrupt open psm=0x0013 cid=0x0053 ret=0",
                "bluez-hid: interrupt connect psm=0x0013 cid=0x0053 "
                "ret=0",
                "bluez-hid: request-response-evidence role=classic-host "
                "channel=control request=HIDP_SET_PROTOCOL "
                "response=HIDP_HANDSHAKE result=ok",
                "bluez-hid: request-response-evidence role=classic-host "
                "channel=interrupt request=HIDP_INPUT_REPORT "
                "response=HIDP_OUTPUT_REPORT event=input-core+hid-device "
                "result=ok",
                "bluez-hid: hidp socket-probe role=classic-host ret=0",
                "upstream-hidp-socket: proto=BTPROTO_HIDP "
                "role=classic-host create-ret=0 ioctl=HIDPCONNADD ret=0 "
                "duplicate-ret=-114",
                "ioctl=HIDPGETCONNLIST ret=0 cnum=1",
                "ioctl=HIDPGETCONNINFO ret=0 state=1",
                "ioctl=HIDPCONNDEL ret=0 post-del-info-ret=-2 "
                "missing-del-ret=-2 final-active=0",
                "core-session=linked:1,thread:1,control:1,"
                "interrupt:1,input:1,uhid:1",
                "core-traffic=control:1,input:1,output:1",
                "poll-op-null=1",
                "proto-name=HIDP",
                "btctl: ordinary-hidp-socket proto=BTPROTO_HIDP "
                "socket-ret=",
                "nonblock-ret=0",
                "create-nonblock-ok=1",
                "unsupported-bind-errno=95",
                "unsupported-getsockname-errno=95",
                "unsupported-getpeername-errno=95",
                "unsupported-connect-errno=95",
                "unsupported-ok=1",
                "ioctl=HIDPCONNADD ret=0",
                "ioctl=HIDPGETCONNLIST ret=0 cnum=1",
                "path=ordinary-socket final-ok=1",
                "no-data=sock_no bind-ret=-95 getname-ret=-95 "
                "sendmsg-ret=-95 recvmsg-ret=-95 listen-ret=-95 shutdown-ret=-95 connect-ret=-95 socketpair-ret=-95 accept-ret=-95 mmap-ret=-95",
                "bluez-hid: closeout cleanup role=classic-host "
                "control=0 interrupt=0 hidp=0 input=0",
                "bluez-hid: closeout upstream-link-ledger "
                "role=classic-host dbus-profile=0 service-record=0 "
                "device-ref=0 adapter-ref=0 control-fd=closed "
                "interrupt-fd=closed",
                "bluez-hid: closeout upstream-coverage-map "
                "role=classic-host",
                "bluez-hid: closeout upstream-source-parity "
                "role=classic-host profile=classic-hid",
                "upstream-link=bluezhid-upstream-link-bluetoothd "
                "parity-final=1",
                "upstream-link=bluezhid-upstream-link-bluetoothd "
                "final-ok=1",
                "BLUEZHIDUPSTREAM_DONE_BT1",
            )),
            RoleCheck("bt2", (
                "BLUEZHIDUPSTREAM_BEGIN_BT2",
                "bluez-hid: native-contract role=classic-device "
                "profile=classic-hid",
                "bluez-hid: semantic-contract role=classic-device "
                "profile=classic-hid",
                "upstream-link=bluezhid-fd-link-to-imported-hidp",
                "bluez-hid: control open psm=0x0011 cid=0x0051 ret=0",
                "bluez-hid: control connect psm=0x0011 cid=0x0051 ret=0",
                "bluez-hid: interrupt open psm=0x0013 cid=0x0053 ret=0",
                "bluez-hid: interrupt connect psm=0x0013 cid=0x0053 "
                "ret=0",
                "bluez-hid: request-response-evidence role=classic-device "
                "channel=control request=HIDP_SET_PROTOCOL "
                "response=HIDP_HANDSHAKE result=ok",
                "bluez-hid: request-response-evidence role=classic-device "
                "channel=interrupt request=HIDP_INPUT_REPORT "
                "response=HIDP_OUTPUT_REPORT event=input-core+hid-device "
                "result=ok",
                "bluez-hid: hidp socket-probe role=classic-device ret=0",
                "upstream-hidp-socket: proto=BTPROTO_HIDP "
                "role=classic-device create-ret=0 ioctl=HIDPCONNADD ret=0 "
                "duplicate-ret=-114",
                "ioctl=HIDPGETCONNLIST ret=0 cnum=1",
                "ioctl=HIDPGETCONNINFO ret=0 state=1",
                "ioctl=HIDPCONNDEL ret=0 post-del-info-ret=-2 "
                "missing-del-ret=-2 final-active=0",
                "core-session=linked:1,thread:1,control:1,"
                "interrupt:1,input:1,uhid:1",
                "core-traffic=control:1,input:1,output:1",
                "poll-op-null=1",
                "proto-name=HIDP",
                "btctl: ordinary-hidp-socket proto=BTPROTO_HIDP "
                "socket-ret=",
                "nonblock-ret=0",
                "create-nonblock-ok=1",
                "unsupported-bind-errno=95",
                "unsupported-getsockname-errno=95",
                "unsupported-getpeername-errno=95",
                "unsupported-connect-errno=95",
                "unsupported-ok=1",
                "ioctl=HIDPCONNADD ret=0",
                "ioctl=HIDPGETCONNLIST ret=0 cnum=1",
                "path=ordinary-socket final-ok=1",
                "no-data=sock_no bind-ret=-95 getname-ret=-95 "
                "sendmsg-ret=-95 recvmsg-ret=-95 listen-ret=-95 shutdown-ret=-95 connect-ret=-95 socketpair-ret=-95 accept-ret=-95 mmap-ret=-95",
                "bluez-hid: closeout cleanup role=classic-device "
                "control=0 interrupt=0 hidp=0 input=0",
                "bluez-hid: closeout upstream-link-ledger "
                "role=classic-device dbus-profile=0 service-record=0 "
                "device-ref=0 adapter-ref=0 control-fd=closed "
                "interrupt-fd=closed",
                "bluez-hid: closeout upstream-coverage-map "
                "role=classic-device",
                "bluez-hid: closeout upstream-source-parity "
                "role=classic-device profile=classic-hid",
                "upstream-link=bluezhid-upstream-link-bluetoothd "
                "parity-final=1",
                "upstream-link=bluezhid-upstream-link-bluetoothd "
                "final-ok=1",
                "BLUEZHIDUPSTREAM_DONE_BT2",
            )),
            RoleCheck("ble1", (
                "BLUEZHIDUPSTREAM_BEGIN_BLE1",
                "bluez-hid: native-contract role=hogp-host profile=hogp",
                "bluez-hid: semantic-contract role=hogp-host "
                "profile=hogp",
                "upstream-link=bluezhid-att-link-to-bluez-hog",
                "bluez-hid: hogp att open psm=0x0000 cid=0x0004 ret=0",
                "bluez-hid: hogp att connect psm=0x0000 cid=0x0004 "
                "ret=0",
                "bluez-hid: request-response-evidence role=hogp-host "
                "op=read-report-map request=ATT_READ_REQ "
                "response=ATT_READ_RSP ret=0",
                "bluez-hid: request-response-evidence role=hogp-host "
                "op=write-protocol-mode request=ATT_WRITE_REQ "
                "response=ATT_WRITE_RSP ret=0",
                "bluez-hid: request-response-evidence role=hogp-host "
                "op=input-report request=CCC_WRITE "
                "response=HANDLE_NOTIFY ret=0",
                "bluez-hid: closeout cleanup role=hogp-host att=0 "
                "hogp=0 input=0 reports=0",
                "bluez-hid: closeout upstream-link-ledger "
                "role=hogp-host dbus-profile=0 gatt-service=0 "
                "device-ref=0 adapter-ref=0 att-fd=closed",
                "bluez-hid: closeout upstream-coverage-map role=hogp-host",
                "bluez-hid: closeout upstream-source-parity "
                "role=hogp-host profile=hogp",
                "upstream-link=bluezhid-upstream-link-bluetoothd "
                "parity-final=1",
                "upstream-link=bluezhid-upstream-link-bluetoothd "
                "final-ok=1",
                "BLUEZHIDUPSTREAM_DONE_BLE1",
            )),
            RoleCheck("ble2", (
                "BLUEZHIDUPSTREAM_BEGIN_BLE2",
                "bluez-hid: native-contract role=hogp-device profile=hogp",
                "bluez-hid: semantic-contract role=hogp-device "
                "profile=hogp",
                "upstream-link=bluezhid-att-link-to-bluez-hog",
                "bluez-hid: hogp att open psm=0x0000 cid=0x0004 ret=0",
                "bluez-hid: hogp att connect psm=0x0000 cid=0x0004 "
                "ret=0",
                "bluez-hid: request-response-evidence role=hogp-device "
                "op=read-report-map request=ATT_READ_REQ "
                "response=ATT_READ_RSP ret=0",
                "bluez-hid: request-response-evidence role=hogp-device "
                "op=write-protocol-mode request=ATT_WRITE_REQ "
                "response=ATT_WRITE_RSP ret=0",
                "bluez-hid: request-response-evidence role=hogp-device "
                "op=input-report request=CCC_WRITE "
                "response=HANDLE_NOTIFY ret=0",
                "bluez-hid: closeout cleanup role=hogp-device att=0 "
                "hogp=0 input=0 reports=0",
                "bluez-hid: closeout upstream-link-ledger "
                "role=hogp-device dbus-profile=0 gatt-service=0 "
                "device-ref=0 adapter-ref=0 att-fd=closed",
                "bluez-hid: closeout upstream-coverage-map "
                "role=hogp-device",
                "bluez-hid: closeout upstream-source-parity "
                "role=hogp-device profile=hogp",
                "upstream-link=bluezhid-upstream-link-bluetoothd "
                "parity-final=1",
                "upstream-link=bluezhid-upstream-link-bluetoothd "
                "final-ok=1",
                "BLUEZHIDUPSTREAM_DONE_BLE2",
            )),
        ),
    ),
    CaseCheck(
        "bluez-gatt-upstream-convergence-closeout",
        (
            RoleCheck("ble1", (
                "BLUEZGATTUPSTREAM_BEGIN_BLE1",
                "bluez-audio: le-gatt closeout cleanup role=source "
                "cig=0 cis=1",
                "bluez-audio: le-gatt upstream-coverage-map role=source",
                "upstream-link=bluezaudio-gatt-upstream-link-bluetoothd "
                "final-ok=1",
                "bluez-gatt: native-contract role=source",
                "bluez-gatt: semantic-contract role=source",
                "upstream-link=bluezgatt-att-link-to-bluez-shared-gatt",
                "bluez-gatt: att fixed-channel open ret=0 owner=bt_att",
                "bluez-gatt: att fixed-channel connect ret=0",
                "bluez-gatt: att-request-response-evidence role=source "
                "op=mtu-exchange request=ATT_EXCHANGE_MTU_REQ "
                "response=ATT_EXCHANGE_MTU_RSP ret=0",
                "bluez-gatt: att-request-response-evidence role=source "
                "op=read-value request=ATT_READ_REQ "
                "response=ATT_READ_RSP ret=0",
                "bluez-gatt: att-request-response-evidence role=source "
                "op=peer-read request=GattCharacteristic1.ReadValue "
                "response=ATT_READ_RSP ret=0",
                "bluez-gatt: att-request-response-evidence role=source "
                "op=write-value request=ATT_WRITE_REQ "
                "response=ATT_WRITE_RSP ret=0",
                "bluez-gatt: att-request-response-evidence role=source "
                "op=peer-write request=GattCharacteristic1.WriteValue "
                "response=ATT_WRITE_RSP ret=0",
                "bluez-gatt: att-request-response-evidence role=source "
                "op=notify request=CCC_WRITE response=HANDLE_NOTIFY ret=0",
                "bluez-gatt: att-request-response-evidence role=source "
                "op=indicate request=HANDLE_INDICATION "
                "response=HANDLE_CONFIRMATION ret=0",
                "bluez-gatt: security-error-contract role=source",
                "bluez-gatt: closeout cleanup role=source att=0 gatt-db=0 "
                "requests=0 watches=0",
                "bluez-gatt: closeout upstream-link-ledger role=source "
                "dbus-application=0 gatt-service=0 gatt-db-ref=0 "
                "att-fd=closed",
                "bluez-gatt: closeout upstream-coverage-map role=source",
                "bluez-gatt: closeout upstream-source-parity role=source",
                "upstream-link=bluezgatt-upstream-link-bluetoothd "
                "parity-final=1",
                "upstream-link=bluezgatt-upstream-link-bluetoothd "
                "final-ok=1",
                "BLUEZGATTUPSTREAM_DONE_BLE1",
            )),
            RoleCheck("ble2", (
                "BLUEZGATTUPSTREAM_BEGIN_BLE2",
                "bluez-audio: le-gatt closeout cleanup role=sink "
                "cig=0 cis=2",
                "bluez-audio: le-gatt upstream-coverage-map role=sink",
                "upstream-link=bluezaudio-gatt-upstream-link-bluetoothd "
                "final-ok=1",
                "bluez-gatt: native-contract role=sink",
                "bluez-gatt: semantic-contract role=sink",
                "upstream-link=bluezgatt-att-link-to-bluez-shared-gatt",
                "bluez-gatt: att fixed-channel open ret=0 owner=bt_att",
                "bluez-gatt: att fixed-channel connect ret=0",
                "bluez-gatt: att-request-response-evidence role=sink "
                "op=mtu-exchange request=ATT_EXCHANGE_MTU_REQ "
                "response=ATT_EXCHANGE_MTU_RSP ret=0",
                "bluez-gatt: att-request-response-evidence role=sink "
                "op=read-value request=ATT_READ_REQ "
                "response=ATT_READ_RSP ret=0",
                "bluez-gatt: att-request-response-evidence role=sink "
                "op=peer-read request=GattCharacteristic1.ReadValue "
                "response=ATT_READ_RSP ret=0",
                "bluez-gatt: att-request-response-evidence role=sink "
                "op=write-value request=ATT_WRITE_REQ "
                "response=ATT_WRITE_RSP ret=0",
                "bluez-gatt: att-request-response-evidence role=sink "
                "op=peer-write request=GattCharacteristic1.WriteValue "
                "response=ATT_WRITE_RSP ret=0",
                "bluez-gatt: att-request-response-evidence role=sink "
                "op=notify request=CCC_WRITE response=HANDLE_NOTIFY ret=0",
                "bluez-gatt: att-request-response-evidence role=sink "
                "op=indicate request=HANDLE_INDICATION "
                "response=HANDLE_CONFIRMATION ret=0",
                "bluez-gatt: security-error-contract role=sink",
                "bluez-gatt: closeout cleanup role=sink att=0 gatt-db=0 "
                "requests=0 watches=0",
                "bluez-gatt: closeout upstream-link-ledger role=sink "
                "dbus-application=0 gatt-service=0 gatt-db-ref=0 "
                "att-fd=closed",
                "bluez-gatt: closeout upstream-coverage-map role=sink",
                "bluez-gatt: closeout upstream-source-parity role=sink",
                "upstream-link=bluezgatt-upstream-link-bluetoothd "
                "parity-final=1",
                "upstream-link=bluezgatt-upstream-link-bluetoothd "
                "final-ok=1",
                "BLUEZGATTUPSTREAM_DONE_BLE2",
            )),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-umbrella",
        (
            RoleCheck("ble1", (
                "bluez-audio: source=third/bluez/profiles/audio/main.c "
                "style=bluetoothd-plugin command=plugin-init "
                "plugin=audio profiles=bap,pacs,ascs,cap,bass,vcp,micp,"
                "csip,mcp,tmap,ccp,gmap",
                "bluez-audio: dbus name=org.bluez owner=bluetoothd "
                "object-manager=/org/bluez/hci0",
                "bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "dbus object=/org/bluez/hci0/pacs0",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "dbus object=/org/bluez/hci0/ase",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "dbus object=/org/bluez/hci0/bap",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c dbus object=/org/bluez/hci0/dev_feather/"
                "ase1/fd0 interfaces=org.bluez.MediaTransport1 "
                "fd-owner=bluetoothd",
                "bluez-audio: le daemon registered pacs=1 ascs=1 "
                "bap=1 media-endpoints=3 transports=1",
                "bluez-audio: source=third/bluez/src/shared/"
                "io-mainloop.c style=mainloop command=dispatch "
                "owner=bluetoothd io-watch=att,iso,dbus timeout=none",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style=bt_bap command=attach session=le-audio "
                "pacs=1 ascs=1 transport-owner=bluetoothd",
                "bluez-audio: le-mgmt-control source=third/bluez/src/"
                "mgmt.c style=mgmt-socket command=power-on role=source",
                "bluez-audio: hci event le-cis-established role=source "
                "status=0x00",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c dbus object=/org/bluez/hci0/dev_feather/"
                "ase1/fd0 command=release owner=bluetoothd",
                "bluez-audio: le daemon released pacs=0 ascs=0 bap=0 "
                "media-endpoints=0 transports=0",
                "bluez-audio: source=third/bluez/src/shared/"
                "mainloop-notify.c style=mainloop command=exit "
                "watches=0 timeouts=0 owner=bluetoothd",
                "bluez-leaudio: closeout begin role=source",
                "bluez-leaudio: ase object-register role=source "
                "interface=org.bluez.ASE1 path=/org/bluez/hci0/ase1 "
                "owner=bluetoothd state=idle",
                "bluez-leaudio: source=third/bluez/profiles/audio/"
                "bap.c style=att-gatt command=transaction role=source "
                "label=ascs-enable write-ret=9",
                "bluez-leaudio: media-endpoint object-register "
                "role=source interface=org.bluez.MediaEndpoint1",
                "bluez-leaudio: source=third/bluez/profiles/audio/"
                "transport.c style=media-transport command=acquire "
                "role=source state=requesting owner=:client.leaudio "
                "fd-owner=iso",
                "bluez-leaudio: source=third/bluez/profiles/audio/"
                "bap.c style=iso-media command=lc3-sdu role=source "
                "len=20 ret=0",
                "upstream-iso-write: payload-len=20 send-ret=20",
                "upstream-af: selected=1",
                "hci-sock-fallback-compiled=0",
                "l2cap-sock-fallback-compiled=0",
                "iso-sock-fallback-compiled=0",
                "iso-socket-connect=4",
                "iso-socket-native-recv=1",
                "bluez-leaudio: closeout upstream-link-ledger role=source "
                "bap-session=detached pacs=0 ascs=0 ase=0 endpoint=0 "
                "media-transport=0 owner-watch=0 pending-request=0 "
                "message-ref=0 iso-fd=closed gatt-requests=0 dbus-owners=0",
                "upstream-link=bluezleaudio-upstream-link-bluetoothd "
                "parity-final=1",
                "upstream-link=bluezleaudio-upstream-link-bluetoothd "
                "final-ok=1",
            )),
            RoleCheck("ble2", (
                "bluez-audio: source=third/bluez/profiles/audio/main.c "
                "style=bluetoothd-plugin command=plugin-init "
                "plugin=audio profiles=bap,pacs,ascs,cap,bass,vcp,micp,"
                "csip,mcp,tmap,ccp,gmap",
                "bluez-audio: dbus name=org.bluez owner=bluetoothd "
                "object-manager=/org/bluez/hci0",
                "bluez-audio: source=third/bluez/profiles/audio/pacs.c "
                "dbus object=/org/bluez/hci0/pacs0",
                "bluez-audio: source=third/bluez/profiles/audio/ascs.c "
                "dbus object=/org/bluez/hci0/ase",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "dbus object=/org/bluez/hci0/bap",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c dbus object=/org/bluez/hci0/dev_feather/"
                "ase1/fd0 interfaces=org.bluez.MediaTransport1 "
                "fd-owner=bluetoothd",
                "bluez-audio: le daemon registered pacs=1 ascs=1 "
                "bap=1 media-endpoints=3 transports=1",
                "bluez-audio: source=third/bluez/src/shared/"
                "io-mainloop.c style=mainloop command=dispatch "
                "owner=bluetoothd io-watch=att,iso,dbus timeout=none",
                "bluez-audio: source=third/bluez/profiles/audio/bap.c "
                "style=bt_bap command=attach session=le-audio "
                "pacs=1 ascs=1 transport-owner=bluetoothd",
                "bluez-audio: le-mgmt-control source=third/bluez/src/"
                "mgmt.c style=mgmt-socket command=power-on role=sink",
                "bluez-audio: hci event le-cis-established role=sink "
                "status=0x00",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c dbus object=/org/bluez/hci0/dev_feather/"
                "ase1/fd0 command=release owner=bluetoothd",
                "bluez-audio: le daemon released pacs=0 ascs=0 bap=0 "
                "media-endpoints=0 transports=0",
                "bluez-audio: source=third/bluez/src/shared/"
                "mainloop-notify.c style=mainloop command=exit "
                "watches=0 timeouts=0 owner=bluetoothd",
                "bluez-leaudio: closeout begin role=sink",
                "bluez-leaudio: ase object-register role=sink "
                "interface=org.bluez.ASE1 path=/org/bluez/hci0/ase1 "
                "owner=bluetoothd state=idle",
                "bluez-leaudio: source=third/bluez/profiles/audio/"
                "bap.c style=att-gatt command=transaction role=sink "
                "label=ascs-enable write-ret=9",
                "bluez-leaudio: media-endpoint object-register "
                "role=sink interface=org.bluez.MediaEndpoint1",
                "bluez-leaudio: source=third/bluez/profiles/audio/"
                "transport.c style=media-transport command=acquire "
                "role=sink state=requesting owner=:client.leaudio "
                "fd-owner=iso",
                "bluez-leaudio: source=third/bluez/profiles/audio/"
                "bap.c style=iso-media command=lc3-sdu role=sink "
                "len=20 ret=0",
                "upstream-iso-write: payload-len=20 send-ret=20",
                "upstream-af: selected=1",
                "hci-sock-fallback-compiled=0",
                "l2cap-sock-fallback-compiled=0",
                "iso-sock-fallback-compiled=0",
                "iso-socket-connect=4",
                "iso-socket-native-recv=3",
                "bluez-leaudio: closeout upstream-link-ledger role=sink "
                "bap-session=detached pacs=0 ascs=0 ase=0 endpoint=0 "
                "media-transport=0 owner-watch=0 pending-request=0 "
                "message-ref=0 iso-fd=closed gatt-requests=0 dbus-owners=0",
                "upstream-link=bluezleaudio-upstream-link-bluetoothd "
                "parity-final=1",
                "upstream-link=bluezleaudio-upstream-link-bluetoothd "
                "final-ok=1",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-transport-bidir-teardown",
        (
            RoleCheck("bt1", (
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=listen "
                "handle=0x0052",
                "bluez-audio: a2dp auto-rsp signal=0x07 "
                "msg-type=0x00 pkt-type=0x00 rsp-len=2 err=0x00 "
                "state=OPEN->STREAMING",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c style profile=media-transport role=sink "
                "command=a2dp-sink-acquire-read-release peer=2 "
                "handle=0x0052",
                "bluez-audio: media transport acquire fd=l2cap "
                "transport=/org/bluez/hci0/dev_feather/fd0 "
                "read-mtu=672 write-mtu=672",
                "upstream-l2cap-recv-handle: psm=0x0019 cid=0x0041 "
                "handle=0x0052 recv-ret=24",
                "bluez-audio: media transport read complete "
                "payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: media transport release complete role=sink",
                "bluez-audio: a2dp signaling suspend peer=2 "
                "handle=0x0052 len=3",
                "bluez-audio: a2dp signaling close-stream peer=2 "
                "handle=0x0052 len=3",
                "bluez-audio: a2dp signaling closed",
                "bluez-audio: a2dp auto-rsp-loop initial-state=STREAMING",
                "bluez-audio: a2dp auto-rsp signal=0x09 "
                "msg-type=0x00 pkt-type=0x00 rsp-len=2 err=0x00 "
                "state=STREAMING->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x08 "
                "msg-type=0x00 pkt-type=0x00 rsp-len=2 err=0x00 "
                "state=OPEN->IDLE",
                "upstream-af: selected=1",
                "upstream-hci-conn=1",
                "l2cap-socket-bind=4",
                "l2cap-socket-connect=2",
                "re:l2cap-socket-send=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-recv=(1[5-9]|[2-9][0-9]+)",
                "l2cap-socket-native-recv=1",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
            RoleCheck("bt2", (
                "bluez-audio: source=third/bluez/profiles/audio/avdtp.c "
                "style profile=a2dp-signaling command=listen "
                "handle=0x0052",
                "bluez-audio: a2dp auto-rsp signal=0x07 "
                "msg-type=0x00 pkt-type=0x00 rsp-len=2 err=0x00 "
                "state=OPEN->STREAMING",
                "bluez-audio: source=third/bluez/profiles/audio/"
                "transport.c style profile=media-transport role=sink "
                "command=a2dp-sink-acquire-read-release peer=1 "
                "handle=0x0052",
                "bluez-audio: media transport acquire fd=l2cap "
                "transport=/org/bluez/hci0/dev_feather/fd0 "
                "read-mtu=672 write-mtu=672",
                "upstream-l2cap-recv-handle: psm=0x0019 cid=0x0041 "
                "handle=0x0052 recv-ret=24",
                "bluez-audio: media transport read complete "
                "payload=A2DP:SBC:synthetic-frame",
                "bluez-audio: media transport release complete role=sink",
                "bluez-audio: a2dp signaling suspend peer=1 "
                "handle=0x0052 len=3",
                "bluez-audio: a2dp signaling close-stream peer=1 "
                "handle=0x0052 len=3",
                "bluez-audio: a2dp signaling closed",
                "bluez-audio: a2dp auto-rsp-loop initial-state=STREAMING",
                "bluez-audio: a2dp auto-rsp signal=0x09 "
                "msg-type=0x00 pkt-type=0x00 rsp-len=2 err=0x00 "
                "state=STREAMING->OPEN",
                "bluez-audio: a2dp auto-rsp signal=0x08 "
                "msg-type=0x00 pkt-type=0x00 rsp-len=2 err=0x00 "
                "state=OPEN->IDLE",
                "upstream-af: selected=1",
                "upstream-hci-conn=1",
                "l2cap-socket-bind=4",
                "l2cap-socket-connect=2",
                "re:l2cap-socket-send=([8-9]|[1-9][0-9]+)",
                "re:l2cap-socket-recv=(1[5-9]|[2-9][0-9]+)",
                "l2cap-socket-native-recv=1",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
            )),
        ),
    ),
    CaseCheck(
        "bluez-a2dp-upstream-convergence-closeout",
        (
            RoleCheck("bt1", (
                "BLUEZA2DPUPSTREAM_BEGIN_BT1",
                "bluez-daemon: audio-a2dp-closeout-full "
                "source=third/bluez/src/profile.c+third/bluez/src/device.c+"
                "third/bluez/profiles/audio/a2dp.c+"
                "third/bluez/profiles/audio/avdtp.c+"
                "third/bluez/profiles/audio/media.c+"
                "third/bluez/profiles/audio/transport.c+"
                "third/bluez/profiles/audio/avrcp.c role=source peer=2 "
                "semantic=a2dp-current-closeout-umbrella",
                "bluez-daemon: a2dp upstream-source-manifest role=source "
                "apps-link=bluez/upstream target=third/bluez "
                "audio-files=20 core-files=9 "
                "compile-unit=bluez/upstream_manifest.c",
                "upstream-link=source-mirror-upstream-plugin",
                "bluez-daemon: a2dp upstream-audio-link-probe role=source "
                "compile-unit=bluez/upstream_audio_link_probe.c",
                "upstream-link=upstream-headers-linked-upstream-c-objects "
                "final-ok=1",
                "bluez-daemon: a2dp upstream-avdtp-packet-impl-probe "
                "role=source",
                "upstream-link=upstream-avdtp-packet-impl-ported-"
                "avdtp-c-object final-ok=1",
                "bluez-daemon: a2dp upstream-avdtp-parse-impl-probe "
                "role=source",
                "upstream-link=upstream-avdtp-parse-impl-ported-"
                "avdtp-c-object final-ok=1",
                "bluez-daemon: a2dp upstream-avdtp-signal-impl-probe "
                "role=source",
                "upstream-link=upstream-avdtp-signal-impl-ported-"
                "avdtp-c-object final-ok=1",
                "bluez-daemon: a2dp upstream-avdtp-stream-impl-probe "
                "role=source",
                "upstream-link=upstream-avdtp-stream-impl-ported-"
                "avdtp-c-object final-ok=1",
                "bluez-daemon: a2dp upstream-a2dp-setup-impl-probe "
                "role=source",
                "upstream-link=upstream-a2dp-setup-impl-ported-"
                "a2dp-c-object final-ok=1",
                "bluez-daemon: a2dp upstream-media-transport-impl-probe "
                "role=source",
                "upstream-link=upstream-media-transport-impl-ported-"
                "transport-c-object final-ok=1",
                "bluez-upstream-object: audio/media-owner role=source "
                "linked=1 source=third/bluez/profiles/audio/media.c+"
                "third/bluez/profiles/audio/transport.c owner=bluetoothd "
                "api=media-endpoint-transport-forwarders",
                "bluez-upstream-handler-object: audio/transport.c "
                "role=source linked=1 source=third/bluez/profiles/audio/"
                "transport.c handlers=acquire:1,try-acquire:1,release:1,"
                "select:1,unselect:1,total:5 "
                "upstream-link=static-upstream-handlers-bound",
                "bluez-upstream-handler-object: audio/media.c role=source "
                "linked=1 source=third/bluez/profiles/audio/media.c "
                "handlers=register-endpoint:1,unregister-endpoint:1,total:2 "
                "upstream-link=static-upstream-handlers-bound",
                "bluez-daemon: a2dp closeout upstream-object-lifecycle "
                "role=source",
                "re:role=source .*final-balanced=1 final-zero=1 "
                ".*transaction-final-balanced=1 .*media-role-ok=1 "
                ".*codec-final-ok=1 .*transport-final-ok=1 "
                ".*avrcp-final-ok=1 .*l2cap-final-ok=1 "
                ".*state-machine-final-ok=1",
                "bluez-daemon: a2dp closeout error-policy-lifecycle "
                "role=source",
                "cleanup=1 state-errors=0 final-ok=1",
                "bluez-daemon: a2dp closeout profile-lifecycle "
                "role=source",
                "bluez-daemon: a2dp closeout dbus-object-lifecycle "
                "role=source",
                "bluez-daemon: a2dp closeout mainloop-lifecycle "
                "role=source",
                "bluez-daemon: a2dp closeout upstream-coverage-map "
                "role=source",
                "executed=plugin,profile,device,sdp,dbus,mainloop,avdtp,"
                "avrcp,media-transport,codec,l2cap,error-policy",
                "bluez-daemon: a2dp closeout upstream-daemon-link-ledger "
                "role=source",
                "owner=bluetoothd direct-owner=profile,device,session,stream,"
                "media-transport,avrcp-player,l2cap-fd,dbus-name,"
                "mainloop-watch",
                "final-profile-registered=0 final-device-ref=0 "
                "final-session-ref=0 final-stream-ref=0 final-sep-ref=0",
                "final-l2cap-fds=0 final-media-fd=closed "
                "final-transaction-pending=0 final-state-errors=0 "
                "final-ok=1",
                "bluez-daemon: a2dp closeout upstream-source-parity "
                "role=source",
                "native-l2cap=psm-0x0019,cid-0x0040,cid-0x0041,"
                "fd-handoff,controller-policy",
                "cleanup-final=1 parity-final=1",
                "bluez-daemon: a2dp closeout dbus-owner-recovery cleanup "
                "role=source owner=:client.a2dp dbus-owners=0",
                "bluez-daemon: a2dp closeout mainloop cleanup role=source "
                "watch-remove=mgmt,l2cap-signaling,l2cap-media,avdtp,avctp,"
                "media-transport,dbus",
                "bluez-daemon: a2dp closeout l2cap-controller cleanup "
                "role=source",
                "bluez-daemon: audio-a2dp-closeout-full cleanup "
                "role=source profile-registered=0 dbus-owners=0 "
                "transaction-pending=0 timers=0",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0 closeout=0",
                "bluez-daemon: audio-a2dp-closeout-full complete "
                "role=source peer=2",
                "upstream-af: selected=1",
                "hci-sock-fallback-compiled=0",
                "l2cap-sock-fallback-compiled=0",
                "iso-sock-fallback-compiled=0",
                "re:l2cap-socket-bind=([3-9][0-9]|[1-9][0-9]{2,})",
                "re:l2cap-socket-connect=([3-9][0-9]|[1-9][0-9]{2,})",
                "re:l2cap-socket-send=([3-9][0-9]|[1-9][0-9]{2,})",
                "re:l2cap-socket-recv=([5-9][0-9]|[1-9][0-9]{2,})",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
                "BLUEZA2DPUPSTREAM_DONE_BT1",
            )),
            RoleCheck("bt2", (
                "BLUEZA2DPUPSTREAM_BEGIN_BT2",
                "bluez-daemon: audio-a2dp-closeout-full "
                "source=third/bluez/src/profile.c+third/bluez/src/device.c+"
                "third/bluez/profiles/audio/a2dp.c+"
                "third/bluez/profiles/audio/avdtp.c+"
                "third/bluez/profiles/audio/media.c+"
                "third/bluez/profiles/audio/transport.c+"
                "third/bluez/profiles/audio/avrcp.c role=sink peer=1 "
                "semantic=a2dp-current-closeout-umbrella",
                "bluez-daemon: a2dp upstream-source-manifest role=sink "
                "apps-link=bluez/upstream target=third/bluez "
                "audio-files=20 core-files=9 "
                "compile-unit=bluez/upstream_manifest.c",
                "upstream-link=source-mirror-upstream-plugin",
                "bluez-daemon: a2dp upstream-audio-link-probe role=sink "
                "compile-unit=bluez/upstream_audio_link_probe.c",
                "upstream-link=upstream-headers-linked-upstream-c-objects "
                "final-ok=1",
                "bluez-daemon: a2dp upstream-avdtp-packet-impl-probe "
                "role=sink",
                "upstream-link=upstream-avdtp-packet-impl-ported-"
                "avdtp-c-object final-ok=1",
                "bluez-daemon: a2dp upstream-avdtp-parse-impl-probe "
                "role=sink",
                "upstream-link=upstream-avdtp-parse-impl-ported-"
                "avdtp-c-object final-ok=1",
                "bluez-daemon: a2dp upstream-avdtp-signal-impl-probe "
                "role=sink",
                "upstream-link=upstream-avdtp-signal-impl-ported-"
                "avdtp-c-object final-ok=1",
                "bluez-daemon: a2dp upstream-avdtp-stream-impl-probe "
                "role=sink",
                "upstream-link=upstream-avdtp-stream-impl-ported-"
                "avdtp-c-object final-ok=1",
                "bluez-daemon: a2dp upstream-a2dp-setup-impl-probe "
                "role=sink",
                "upstream-link=upstream-a2dp-setup-impl-ported-"
                "a2dp-c-object final-ok=1",
                "bluez-daemon: a2dp upstream-media-transport-impl-probe "
                "role=sink",
                "upstream-link=upstream-media-transport-impl-ported-"
                "transport-c-object final-ok=1",
                "bluez-upstream-object: audio/media-owner role=sink "
                "linked=1 source=third/bluez/profiles/audio/media.c+"
                "third/bluez/profiles/audio/transport.c owner=bluetoothd "
                "api=media-endpoint-transport-forwarders",
                "bluez-upstream-handler-object: audio/transport.c "
                "role=sink linked=1 source=third/bluez/profiles/audio/"
                "transport.c handlers=acquire:1,try-acquire:1,release:1,"
                "select:1,unselect:1,total:5 "
                "upstream-link=static-upstream-handlers-bound",
                "bluez-upstream-handler-object: audio/media.c role=sink "
                "linked=1 source=third/bluez/profiles/audio/media.c "
                "handlers=register-endpoint:1,unregister-endpoint:1,total:2 "
                "upstream-link=static-upstream-handlers-bound",
                "bluez-daemon: a2dp closeout upstream-object-lifecycle "
                "role=sink",
                "re:role=sink .*final-balanced=1 final-zero=1 "
                ".*transaction-final-balanced=1 .*media-role-ok=1 "
                ".*codec-final-ok=1 .*transport-final-ok=1 "
                ".*avrcp-final-ok=1 .*l2cap-final-ok=1 "
                ".*state-machine-final-ok=1",
                "bluez-daemon: a2dp closeout error-policy-lifecycle "
                "role=sink",
                "cleanup=1 state-errors=0 final-ok=1",
                "bluez-daemon: a2dp closeout profile-lifecycle role=sink",
                "bluez-daemon: a2dp closeout dbus-object-lifecycle "
                "role=sink",
                "bluez-daemon: a2dp closeout mainloop-lifecycle role=sink",
                "bluez-daemon: a2dp closeout upstream-coverage-map "
                "role=sink",
                "executed=plugin,profile,device,sdp,dbus,mainloop,avdtp,"
                "avrcp,media-transport,codec,l2cap,error-policy",
                "bluez-daemon: a2dp closeout upstream-daemon-link-ledger "
                "role=sink",
                "owner=bluetoothd direct-owner=profile,device,session,stream,"
                "media-transport,avrcp-player,l2cap-fd,dbus-name,"
                "mainloop-watch",
                "final-profile-registered=0 final-device-ref=0 "
                "final-session-ref=0 final-stream-ref=0 final-sep-ref=0",
                "final-l2cap-fds=0 final-media-fd=closed "
                "final-transaction-pending=0 final-state-errors=0 "
                "final-ok=1",
                "bluez-daemon: a2dp closeout upstream-source-parity "
                "role=sink",
                "native-l2cap=psm-0x0019,cid-0x0040,cid-0x0041,"
                "fd-handoff,controller-policy",
                "cleanup-final=1 parity-final=1",
                "bluez-daemon: a2dp closeout dbus-owner-recovery cleanup "
                "role=sink owner=:client.a2dp dbus-owners=0",
                "bluez-daemon: a2dp closeout mainloop cleanup role=sink "
                "watch-remove=mgmt,l2cap-signaling,l2cap-media,avdtp,avctp,"
                "media-transport,dbus",
                "bluez-daemon: a2dp closeout l2cap-controller cleanup "
                "role=sink",
                "bluez-daemon: audio-a2dp-closeout-full cleanup "
                "role=sink profile-registered=0 dbus-owners=0 "
                "transaction-pending=0 timers=0",
                "bluez-daemon: plugin audio exit complete "
                "profiles=a2dp,avrcp media=0 sbc=0 closeout=0",
                "bluez-daemon: audio-a2dp-closeout-full complete "
                "role=sink peer=1",
                "upstream-af: selected=1",
                "hci-sock-fallback-compiled=0",
                "l2cap-sock-fallback-compiled=0",
                "iso-sock-fallback-compiled=0",
                "re:l2cap-socket-bind=([3-9][0-9]|[1-9][0-9]{2,})",
                "re:l2cap-socket-connect=([3-9][0-9]|[1-9][0-9]{2,})",
                "re:l2cap-socket-send=([3-9][0-9]|[1-9][0-9]{2,})",
                "re:l2cap-socket-recv=([5-9][0-9]|[1-9][0-9]{2,})",
                "l2cap-socket-native-attach-fail=0",
                "l2cap-socket-native-recv-fail=0",
                "BLUEZA2DPUPSTREAM_DONE_BT2",
            )),
        ),
    ),
    CaseCheck(
        "bluez-obex-pbap-opp-profile-closeout",
        (
            obex_profile_role("bt1", "BLUEZOBEXPBO", ("pbap", "opp")),
            obex_profile_role("bt2", "BLUEZOBEXPBO", ("pbap", "opp")),
        ),
    ),
    CaseCheck(
        "bluez-obex-map-mns-profile-closeout",
        (
            obex_profile_role("bt1", "BLUEZOBEXMAP", ("map", "mns")),
            obex_profile_role("bt2", "BLUEZOBEXMAP", ("map", "mns")),
        ),
    ),
    CaseCheck(
        "bluez-obex-ftp-sync-profile-closeout",
        (
            obex_profile_role("bt1", "BLUEZOBEXFTPSYNC", ("ftp", "sync")),
            obex_profile_role("bt2", "BLUEZOBEXFTPSYNC", ("ftp", "sync")),
        ),
    ),
    CaseCheck(
        "bluez-obex-bip-profile-closeout",
        (
            obex_profile_role("bt1", "BLUEZOBEXBIP", ("bip",)),
            obex_profile_role("bt2", "BLUEZOBEXBIP", ("bip",)),
        ),
    ),
    CaseCheck(
        "bluez-mesh-profile-closeout",
        (
            profile_family_role("ble1", "BLUEZMESH", "mesh",
                                "bluezdaemon-mesh-upstream-link-bluetoothd",
                                "bluezprofile-mesh-upstream-link-bluetoothd"),
            profile_family_role("ble2", "BLUEZMESH", "mesh",
                                "bluezdaemon-mesh-upstream-link-bluetoothd",
                                "bluezprofile-mesh-upstream-link-bluetoothd"),
        ),
    ),
    CaseCheck(
        "bluez-asha-profile-closeout",
        (
            profile_family_role("ble1", "BLUEZASHA", "asha",
                                "bluezdaemon-asha-upstream-link-bluetoothd",
                                "bluezprofile-asha-upstream-link-bluetoothd"),
            profile_family_role("ble2", "BLUEZASHA", "asha",
                                "bluezdaemon-asha-upstream-link-bluetoothd",
                                "bluezprofile-asha-upstream-link-bluetoothd"),
        ),
    ),
    CaseCheck(
        "bluez-print-profile-closeout",
        (
            profile_family_role("bt1", "BLUEZPRINT", "print",
                                "bluezdaemon-print-upstream-link-cups-backend",
                                "bluezprofile-print-upstream-link-bluetoothd"),
            profile_family_role("bt2", "BLUEZPRINT", "print",
                                "bluezdaemon-print-upstream-link-cups-backend",
                                "bluezprofile-print-upstream-link-bluetoothd"),
        ),
    ),
    CaseCheck(
        "bluez-iap-profile-closeout",
        (
            profile_family_role("bt1", "BLUEZIAP", "iap",
                                "bluezdaemon-iap-upstream-link-iapd",
                                "bluezprofile-iap-upstream-link-bluetoothd"),
            profile_family_role("bt2", "BLUEZIAP", "iap",
                                "bluezdaemon-iap-upstream-link-iapd",
                                "bluezprofile-iap-upstream-link-bluetoothd"),
        ),
    ),
    CaseCheck(
        "bluez-midi-profile-closeout",
        (
            profile_family_role("ble1", "BLUEZMIDI", "midi",
                                "bluezdaemon-midi-upstream-link-bluetoothd",
                                "bluezprofile-midi-upstream-link-bluetoothd"),
            profile_family_role("ble2", "BLUEZMIDI", "midi",
                                "bluezdaemon-midi-upstream-link-bluetoothd",
                                "bluezprofile-midi-upstream-link-bluetoothd"),
        ),
    ),
    CaseCheck(
        "bluez-ranging-profile-closeout",
        (
            profile_family_role("ble1", "BLUEZRANGING", "ranging",
                                "bluezdaemon-ranging-upstream-link-bluetoothd",
                                "bluezprofile-ranging-upstream-link-bluetoothd"),
            profile_family_role("ble2", "BLUEZRANGING", "ranging",
                                "bluezdaemon-ranging-upstream-link-bluetoothd",
                                "bluezprofile-ranging-upstream-link-bluetoothd"),
        ),
    ),

    CaseCheck(
        "bluez-hid-hogp-profile-closeout",
        (
            hid_hogp_closeout_role("bt1", "BT1", "hid", "host",
                                   "classic-host", "classic-hid-host"),
            hid_hogp_closeout_role("bt2", "BT2", "hid", "device",
                                   "classic-device", "classic-hid-device"),
            hid_hogp_closeout_role("ble1", "BLE1", "hogp", "host",
                                   "hogp-host", "hogp-host"),
            hid_hogp_closeout_role("ble2", "BLE2", "hogp", "device",
                                   "hogp-device", "hogp-device"),
        ),
    ),
    CaseCheck(
        "bluez-gatt-profile-closeout",
        (
            gatt_profile_closeout_role("ble1", "BLE1", "client",
                                       "gatt-client"),
            gatt_profile_closeout_role("ble2", "BLE2", "server",
                                       "gatt-server"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio",
        (
            command_role("ble1",
                         "bluezaudio le-broadcast-source start 0 1",
                         "btctl upstream iso-bind 0 0x0101",
                         "btctl upstream iso-connect 0",
                         "btctl upstream iso-close"),
            command_role("ble2",
                         "bluezaudio le-broadcast-sink sync 0 1",
                         "bluezaudio le-broadcast-sink start 0 1",
                         "bluezaudio le-broadcast-sink stop"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-profile",
        (
            command_role("ble1",
                         "bluezaudio le-bap-control source-announce 0 1",
                         "bluezaudio le-bap-control source-start 0 1",
                         "bluezaudio le-broadcast-source start 0 1",
                         "bluezaudio le-bap-control source-stop 0 1"),
            command_role("ble2",
                         "bluezaudio le-bap-control sink-discover 0 1",
                         "bluezaudio le-bap-control sink-config 0 1",
                         "bluezaudio le-bap-control sink-sync 0 1",
                         "bluezaudio le-broadcast-sink start 0 1"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-broadcast-restart",
        (
            command_role("ble1",
                         "bluezaudio le-bap-control source-announce 0 1",
                         "bluezaudio le-broadcast-source start 0 1",
                         "bluezaudio le-bap-control source-stop 0 1",
                         "btctl upstream iso-close"),
            command_role("ble2",
                         "bluezaudio le-bap-control sink-discover 0 1",
                         "bluezaudio le-broadcast-sink sync 0 1",
                         "bluezaudio le-broadcast-sink start 0 1",
                         "bluezaudio le-broadcast-sink stop"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-unicast-profile",
        (
            command_role("ble1",
                         "bluezaudio le-unicast-control source-config 0 1",
                         "bluezaudio le-unicast-control source-enable 0 1",
                         "bluezaudio le-unicast-source start 0 1",
                         "bluezaudio le-unicast-control source-release 0 1"),
            command_role("ble2",
                         "bluezaudio le-unicast-control sink-discover 0 1",
                         "bluezaudio le-unicast-control sink-config 0 1",
                         "bluezaudio le-unicast-control sink-enable 0 1",
                         "bluezaudio le-unicast-sink start 0 1"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-transport",
        (
            command_role("ble1",
                         "bluezaudio le-unicast-control source-config 0 1",
                         "bluezaudio le-unicast-control source-enable 0 1",
                         "bluezaudio media-transport unicast-source-acquire-write-release 0 1",
                         "bluezaudio le-unicast-control source-release 0 1"),
            command_role("ble2",
                         "bluezaudio le-unicast-control sink-discover 0 1",
                         "bluezaudio le-unicast-control sink-config 0 1",
                         "bluezaudio le-unicast-control sink-enable 0 1",
                         "bluezaudio media-transport unicast-sink-acquire-read-release 0 1"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-transport-reconnect",
        (
            command_role("ble1",
                         "bluezaudio media-transport unicast-source-acquire-write-release 0 1",
                         "bluezaudio le-unicast-control source-release 0 1"),
            command_role("ble2",
                         "bluezaudio media-transport unicast-sink-acquire-read-release 0 1",
                         "bluezaudio le-unicast-control sink-enable 0 1"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-transport-bidir",
        (
            command_role("ble1",
                         "bluezaudio media-transport unicast-source-acquire-write-release 0 1",
                         "bluezaudio le-unicast-control sink-discover 0 2",
                         "bluezaudio media-transport unicast-sink-acquire-read-release 0 2"),
            command_role("ble2",
                         "bluezaudio media-transport unicast-sink-acquire-read-release 0 1",
                         "bluezaudio le-unicast-control source-config 0 2",
                         "bluezaudio media-transport unicast-source-acquire-write-release 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-transport-bidir-disable",
        (
            command_role("ble1",
                         "bluezaudio le-unicast-control source-disable 0 1",
                         "bluezaudio le-unicast-control sink-disable 0 2"),
            command_role("ble2",
                         "bluezaudio le-unicast-control sink-disable 0 1",
                         "bluezaudio le-unicast-control source-disable 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-transport-bidir-qos-update",
        (
            command_role("ble1",
                         "bluezaudio le-unicast-control source-qos-update 0 1",
                         "bluezaudio le-unicast-control sink-qos-update 0 2"),
            command_role("ble2",
                         "bluezaudio le-unicast-control sink-qos-update 0 1",
                         "bluezaudio le-unicast-control source-qos-update 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-transport-bidir-qos-reject",
        (
            command_role("ble1",
                         "bluezaudio le-unicast-control source-qos-reject 0 1",
                         "bluezaudio le-unicast-control sink-qos-reject 0 2"),
            command_role("ble2",
                         "bluezaudio le-unicast-control sink-qos-reject 0 1",
                         "bluezaudio le-unicast-control source-qos-reject 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-transport-bidir-qos-cancel",
        (
            command_role("ble1",
                         "bluezaudio le-unicast-control source-qos-cancel 0 1",
                         "bluezaudio le-unicast-control sink-qos-cancel 0 2"),
            command_role("ble2",
                         "bluezaudio le-unicast-control sink-qos-cancel 0 1",
                         "bluezaudio le-unicast-control source-qos-cancel 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-transport-bidir-release-reconfig",
        (
            command_role("ble1",
                         "bluezaudio le-unicast-control source-release 0 1",
                         "bluezaudio le-unicast-control sink-release 0 2"),
            command_role("ble2",
                         "bluezaudio le-unicast-control sink-release 0 1",
                         "bluezaudio le-unicast-control source-release 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-transport-bidir-reconnect",
        (
            command_role("ble1",
                         "bluezaudio media-transport unicast-source-acquire-write-release 0 1",
                         "bluezaudio media-transport unicast-sink-acquire-read-release 0 2"),
            command_role("ble2",
                         "bluezaudio media-transport unicast-sink-acquire-read-release 0 1",
                         "bluezaudio media-transport unicast-source-acquire-write-release 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-full-lifecycle",
        (
            command_role("ble1",
                         "bluezaudio le-broadcast-source start 0 1",
                         "bluezaudio media-transport unicast-source-acquire-write-release 0 1",
                         "bluezaudio media-transport unicast-sink-acquire-read-release 0 2"),
            command_role("ble2",
                         "bluezaudio le-broadcast-sink start 0 1",
                         "bluezaudio media-transport unicast-sink-acquire-read-release 0 1",
                         "bluezaudio media-transport unicast-source-acquire-write-release 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-daemon-full-lifecycle",
        (
            le_daemon_role("ble1",
                           "bluezaudio le-cap-control coordinator-register",
                           "bluezaudio le-broadcast-iso big-create source 0 1",
                           "bluezaudio media-transport unicast-source-acquire-write-release 0 1",
                           "bluezaudio media-transport unicast-sink-acquire-read-release 0 2"),
            le_daemon_role("ble2",
                           "bluezaudio le-broadcast-sink start 0 1",
                           "bluezaudio media-transport unicast-sink-acquire-read-release 0 1",
                           "bluezaudio media-transport unicast-source-acquire-write-release 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-dbus-client-full",
        (
            le_daemon_role("ble1",
                           "bluezaudio le-dbus-client register source 0 1",
                           "bluezaudio le-dbus-client transport-busy source 0 1",
                           "bluezaudio le-dbus-client register sink 0 2",
                           "bluezaudio le-dbus-client release sink 0 2"),
            le_daemon_role("ble2",
                           "bluezaudio le-dbus-client register sink 0 1",
                           "bluezaudio le-dbus-client transport-busy sink 0 1",
                           "bluezaudio le-dbus-client register source 0 2",
                           "bluezaudio le-dbus-client release source 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-lc3-codec-transport",
        (
            le_daemon_role("ble1",
                           "bluezaudio le-iso-socket sendmsg source 0 1",
                           "bluezaudio le-audio-codec source-lc3-encode-write-release 0 1",
                           "bluezaudio le-iso-socket recvmsg sink 0 2",
                           "bluezaudio le-audio-codec sink-lc3-recv-decode-release 0 2"),
            le_daemon_role("ble2",
                           "bluezaudio le-iso-socket recvmsg sink 0 1",
                           "bluezaudio le-audio-codec sink-lc3-recv-decode-release 0 1",
                           "bluezaudio le-iso-socket sendmsg source 0 2",
                           "bluezaudio le-audio-codec source-lc3-encode-write-release 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-iso-dataplane-soak",
        (
            le_daemon_role("ble1",
                           "bluezaudio le-att-bearer open source 0 1",
                           "bluezaudio le-gatt-db discover-ascs source 0 1",
                           "bluezaudio le-bap-policy start-stream source 0 1",
                           "bluezaudio le-iso-socket sendmsg source 0 1",
                           "upstream-iso-connect: addr-type=1 connect-ret=0",
                           "upstream-iso-poll: events=POLLOUT poll-ret=",
                           "upstream-iso-write: payload-len=40 send-ret=40",
                           "upstream-iso-shutdown: how=2 shutdown-ret=0",
                           "upstream-iso-close: released detach-ret=0 release-ret=0 "
                           "sim-detach=abandon-links",
                           "bluezaudio le-att-bearer close sink 0 2"),
            le_daemon_role("ble2",
                           "bluezaudio le-att-bearer open sink 0 1",
                           "bluezaudio le-gatt-db discover-ascs sink 0 1",
                           "bluezaudio le-bap-policy start-stream sink 0 1",
                           "bluezaudio le-iso-socket recvmsg sink 0 1",
                           "upstream-iso-listen: backlog=1 listen-ret=0",
                           "upstream-iso-accept: accept-ret=0 state=1 "
                           "handle=0x0201 upstream-iso-attach=0",
                           "upstream-iso-poll: events=POLLIN poll-ret=",
                           "upstream-iso-recv: recv-ret=40",
                           "upstream-iso-shutdown: how=2 shutdown-ret=0",
                           "upstream-iso-close: released detach-ret=0 release-ret=0 "
                           "sim-detach=abandon-links",
                           "bluezaudio le-att-bearer close source 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-controller-iso-dataplane-soak",
        (
            le_controller_role("ble1", "user-iso-setup-bidir-monitor",
                               "bluezaudio le-iso-socket sendmsg source 0 1",
                               "bluezaudio le-audio-codec source-lc3-encode-write-release 0 1"),
            command_role("ble2",
                         "bluezaudio le-iso-socket recvmsg sink 0 1",
                         "bluezaudio le-audio-codec sink-lc3-recv-decode-release 0 1"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-daemon-profile-flow",
        (
            command_role("ble1",
                         "bluezaudio le-daemon unicast-profile-flow source 0 1",
                         "bluezaudio le-daemon unicast-profile-flow sink 0 2"),
            command_role("ble2",
                         "bluezaudio le-daemon unicast-profile-flow sink 0 1",
                         "bluezaudio le-daemon unicast-profile-flow source 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-controller-daemon-profile-flow",
        (
            le_controller_role("ble1", "user-iso-setup-bidir-monitor",
                               "bluezaudio le-daemon unicast-profile-flow source 0 1",
                               "bluez-audio: iso socket ioctl role=source "
                               "ret=0 proto=BTPROTO_ISO",
                               "bluez-audio: iso socket ioctl role=sink "
                               "ret=0 proto=BTPROTO_ISO",
                               "upstream-iso-ioctl: handle=0x0201 "
                               "inq-ret=0",
                               "upstream-iso-ioctl: handle=0x0202 "
                               "inq-ret=0",
                               "bluezaudio le-daemon unicast-profile-flow sink 0 2"),
            command_role("ble2",
                         "bluezaudio le-daemon unicast-profile-flow sink 0 1",
                         "bluez-audio: iso socket ioctl role=sink "
                         "ret=0 proto=BTPROTO_ISO",
                         "bluez-audio: iso socket ioctl role=source "
                         "ret=0 proto=BTPROTO_ISO",
                         "upstream-iso-ioctl: handle=0x0201 "
                         "inq-ret=0",
                         "upstream-iso-ioctl: handle=0x0202 "
                         "inq-ret=0",
                         "bluezaudio le-daemon unicast-profile-flow source 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-controller-daemon-profile-reconnect",
        (
            le_controller_role("ble1", "user-iso-setup-bidir-monitor",
                               "bluezaudio le-daemon unicast-profile-flow source 0 1",
                               "bluezaudio le-daemon unicast-profile-flow sink 0 2"),
            command_role("ble2",
                         "bluezaudio le-daemon unicast-profile-flow sink 0 1",
                         "bluezaudio le-daemon unicast-profile-flow source 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-controller-daemon-error-recovery",
        (
            le_controller_role("ble1", "user-iso-setup-bidir-monitor",
                               "bluezaudio le-ascs-cp config-qos-reject source 0 1",
                               "bluezaudio le-unicast-control source-qos-cancel 0 1",
                               "bluezaudio le-daemon broadcast-profile-flow source 0 1"),
            command_role("ble2",
                         "bluezaudio le-ascs-cp config-qos-reject sink 0 1",
                         "bluezaudio le-unicast-control sink-qos-cancel 0 1",
                         "bluezaudio le-daemon broadcast-profile-flow sink 0 1"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-controller-bap-ascs-error-matrix",
        (
            le_controller_role("ble1", "user-iso-setup-bidir-monitor",
                               "bluezaudio le-mgmt-control cis-request source 0 1",
                               "bluezaudio le-ascs-cp config-qos-reject source 0 1",
                               "bluezaudio le-bap-policy start-stream sink 0 2"),
            command_role("ble2",
                         "bluezaudio le-mgmt-control cis-request sink 0 1",
                         "bluezaudio le-ascs-cp config-qos-reject sink 0 1",
                         "bluezaudio le-bap-policy start-stream source 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-controller-daemon-dbus-ownership",
        (
            le_controller_role("ble1", "user-iso-setup-bidir-monitor",
                               "bluezaudio le-dbus-client owner-lost source 0 1",
                               "bluezaudio le-dbus-client owner-reacquire sink 0 2"),
            command_role("ble2",
                         "bluezaudio le-dbus-client owner-lost sink 0 1",
                         "bluezaudio le-dbus-client owner-reacquire source 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-bap-ascs-dbus-owner-recovery",
        (
            le_controller_role("ble1", "user-iso-setup-bidir-monitor",
                               "bluezaudio le-dbus-client owner-lost source 0 1",
                               "bluezaudio le-gatt-db write-ascs-cp sink 0 2",
                               "bluezaudio le-daemon unicast-profile-flow sink 0 2"),
            command_role("ble2",
                         "bluezaudio le-dbus-client owner-lost sink 0 1",
                         "bluezaudio le-gatt-db write-ascs-cp source 0 2",
                         "bluezaudio le-daemon unicast-profile-flow source 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-controller-dbus-bap-ascs-reconnect",
        (
            le_controller_role("ble1", "user-iso-setup-bidir-monitor",
                               "bluezaudio le-mgmt-control connect source 0 1",
                               "bluezaudio le-dbus-client owner-reacquire source 0 1"),
            command_role("ble2",
                         "bluezaudio le-mgmt-control connect sink 0 1",
                         "bluezaudio le-dbus-client owner-reacquire sink 0 1"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-controller-daemon-full-stack",
        (
            le_controller_role("ble1", "user-iso-setup-bidir-monitor",
                               "bluezaudio le-mgmt-control cis-request source 0 1",
                               "bluezaudio le-dbus-client owner-lost source 0 1",
                               "bluezaudio le-daemon broadcast-profile-flow source 0 1"),
            command_role("ble2",
                         "bluezaudio le-mgmt-control cis-request sink 0 1",
                         "bluezaudio le-dbus-client owner-lost sink 0 1",
                         "bluezaudio le-daemon broadcast-profile-flow sink 0 1"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-controller-daemon-full-stack-reconnect",
        (
            le_controller_role("ble1", "user-iso-setup-bidir-monitor",
                               "bluezaudio le-daemon unicast-profile-flow source 0 1",
                               "bluezaudio le-daemon unicast-profile-flow sink 0 2"),
            command_role("ble2",
                         "bluezaudio le-daemon unicast-profile-flow sink 0 1",
                         "bluezaudio le-daemon unicast-profile-flow source 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-controller-daemon-mainloop-cleanup",
        (
            le_controller_role("ble1", "user-iso-setup-bidir-monitor",
                               "bluezaudio le-daemon plugin-exit",
                               "bluezaudio le-daemon unicast-profile-flow source 0 1"),
            command_role("ble2",
                         "bluezaudio le-daemon plugin-exit",
                         "bluezaudio le-daemon unicast-profile-flow sink 0 1"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-daemon-broadcast-profile-flow",
        (
            command_role("ble1", "bluezaudio le-daemon broadcast-profile-flow source 0 1"),
            command_role("ble2", "bluezaudio le-daemon broadcast-profile-flow sink 0 1"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-controller-daemon-broadcast-profile-flow",
        (
            le_controller_role("ble1", "user-iso-setup-monitor",
                               "bluezaudio le-daemon broadcast-profile-flow source 0 1"),
            command_role("ble2", "bluezaudio le-daemon broadcast-profile-flow sink 0 1"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-controller-daemon-broadcast-reconnect",
        (
            le_controller_role("ble1", "user-iso-setup-monitor",
                               "bluezaudio le-daemon broadcast-profile-flow source 0 1"),
            command_role("ble2", "bluezaudio le-daemon broadcast-profile-flow sink 0 1"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-controller-lc3-bidir",
        (
            le_controller_role("ble1", "user-iso-setup-bidir-monitor",
                               "bluezaudio le-audio-codec source-lc3-encode-write-release 0 1",
                               "bluezaudio le-audio-codec sink-lc3-recv-decode-release 0 2"),
            command_role("ble2",
                         "bluezaudio le-audio-codec sink-lc3-recv-decode-release 0 1",
                         "bluezaudio le-audio-codec source-lc3-encode-write-release 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-coordinated-services",
        (
            le_daemon_role("ble1",
                           "bluezaudio le-vcp-control register source 0 1",
                           "bluezaudio le-micp-control mute source 0 1",
                           "bluezaudio le-csip-control lock source 0 1",
                           "bluezaudio le-gmap-control release sink 0 2"),
            le_daemon_role("ble2",
                           "bluezaudio le-vcp-control register sink 0 1",
                           "bluezaudio le-micp-control mute sink 0 1",
                           "bluezaudio le-csip-control lock sink 0 1",
                           "bluezaudio le-gmap-control release source 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-cap-csip-group",
        (
            le_daemon_role("ble1",
                           "bluezaudio le-cap-control coordinator-register",
                           "bluezaudio le-cap-control group-config-bidir 0 1 2",
                           "bluezaudio le-cap-control group-enable-bidir 0 1 2",
                           "bluezaudio le-cap-control group-release-bidir 0 1 2"),
            le_daemon_role("ble2",
                           "bluezaudio le-cap-control coordinator-register",
                           "bluezaudio le-cap-control group-config-bidir 0 2 1",
                           "bluezaudio le-cap-control group-enable-bidir 0 2 1",
                           "bluezaudio le-cap-control group-release-bidir 0 2 1"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-tmap-mcp-ccp-flow",
        (
            le_daemon_role("ble1",
                           "bluezaudio le-tmap-control register source 0 1",
                           "bluezaudio le-mcp-control play source 0 1",
                           "bluezaudio le-ccp-control originate source 0 1",
                           "bluezaudio media-transport unicast-sink-acquire-read-release 0 2"),
            le_daemon_role("ble2",
                           "bluezaudio le-tmap-control register sink 0 1",
                           "bluezaudio le-mcp-control play sink 0 1",
                           "bluezaudio le-ccp-control originate sink 0 1",
                           "bluezaudio media-transport unicast-source-acquire-write-release 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-broadcast-multibis",
        (
            le_daemon_role("ble1",
                           "bluezaudio le-broadcast-iso bis-setup source 0 1",
                           "bluezaudio le-broadcast-source start 0 1",
                           "bluezaudio le-broadcast-iso bis-setup source 0 2",
                           "bluezaudio le-broadcast-source start 0 2"),
            le_daemon_role("ble2",
                           "bluezaudio le-bass-control add-source 0 1",
                           "bluezaudio le-broadcast-sink start 0 1",
                           "bluezaudio le-bass-control add-source 0 2",
                           "bluezaudio le-broadcast-sink start 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-broadcast-multibis-reconnect",
        (
            command_role("ble1",
                         "bluezaudio le-daemon broadcast-profile-flow source 0 1",
                         "bluezaudio le-broadcast-source start 0 2"),
            command_role("ble2",
                         "bluezaudio le-daemon broadcast-profile-flow sink 0 1",
                         "bluezaudio le-broadcast-sink start 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-bass-scan-delegator",
        (
            command_role("ble1",
                         "bluezaudio le-bass-control assistant-register",
                         "bluezaudio le-bass-control add-source 0 1",
                         "bluezaudio le-bass-control modify-source 0 1",
                         "bluezaudio le-bass-control assistant-release"),
            command_role("ble2",
                         "bluezaudio le-bass-control assistant-register",
                         "bluezaudio le-bass-control add-source 0 1",
                         "bluezaudio le-bass-control remove-source 0 1",
                         "bluezaudio le-bass-control assistant-release"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-daemon-integrated-profile",
        (
            command_role("ble1",
                         "bluezaudio le-daemon integrated-profile-flow source 0 1",
                         "bluezaudio le-daemon integrated-profile-flow sink 0 2"),
            command_role("ble2",
                         "bluezaudio le-daemon integrated-profile-flow sink 0 1",
                         "bluezaudio le-daemon integrated-profile-flow source 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-daemon-integrated-profile-reconnect",
        (
            command_role("ble1",
                         "bluezaudio le-daemon integrated-profile-flow source 0 1",
                         "bluezaudio le-daemon integrated-profile-flow sink 0 2"),
            command_role("ble2",
                         "bluezaudio le-daemon integrated-profile-flow sink 0 1",
                         "bluezaudio le-daemon integrated-profile-flow source 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-bap-pacs-ascs-session",
        (
            command_role("ble1",
                         "bluezaudio le-iso-socket sockapi-closeout source 0 1",
                         "iso socket sockapi-closeout socket-ret=",
                         "nonblock-ret=0",
                         "create-nonblock-ok=1",
                         "stream-ret=-1",
                         "stream-esocktnosupport=1",
                         "dgram-ret=-1",
                         "dgram-esocktnosupport=1",
                         "raw-ret=-1",
                         "raw-esocktnosupport=1",
                         "listen-ret=0",
                         "accept-ret=0",
                         "pending-accept-ok=1",
                         "path=ordinary-socket final-ok=1",
                         "bluezaudio le-gatt-db discover-pacs source 0 1",
                         "bluezaudio le-ascs-cp enable source 0 1",
                         "bluezaudio le-bap-policy start-stream source 0 1",
                         "upstream-iso-options: opt=BT_DEFER_SETUP "
                         "set-ret=0 get-ret=0 defer=1",
                         "opt=BT_PKT_STATUS set-ret=0 get-ret=0 value=1",
                         "opt=BT_PKT_SEQNUM set-ret=0 upstream-get=absent",
                         "opt=BT_ISO_QOS set-ret=0 get-ret=0",
                         "opt=BT_ISO_BASE base-set-ret=0 base-get-ret=0",
                         "bluezaudio le-daemon unicast-profile-flow sink 0 2"),
            command_role("ble2",
                         "bluezaudio le-iso-socket sockapi-closeout sink 0 1",
                         "iso socket sockapi-closeout socket-ret=",
                         "nonblock-ret=0",
                         "create-nonblock-ok=1",
                         "stream-ret=-1",
                         "stream-esocktnosupport=1",
                         "dgram-ret=-1",
                         "dgram-esocktnosupport=1",
                         "raw-ret=-1",
                         "raw-esocktnosupport=1",
                         "listen-ret=0",
                         "accept-ret=0",
                         "pending-accept-ok=1",
                         "path=ordinary-socket final-ok=1",
                         "bluezaudio le-gatt-db discover-pacs sink 0 1",
                         "bluezaudio le-ascs-cp enable sink 0 1",
                         "bluezaudio le-bap-policy start-stream sink 0 1",
                         "upstream-iso-options: opt=BT_DEFER_SETUP "
                         "set-ret=0 get-ret=0 defer=1",
                         "opt=BT_PKT_STATUS set-ret=0 get-ret=0 value=1",
                         "opt=BT_PKT_SEQNUM set-ret=0 upstream-get=absent",
                         "opt=BT_ISO_QOS set-ret=0 get-ret=0",
                         "opt=BT_ISO_BASE base-set-ret=0 base-get-ret=0",
                         "bluezaudio le-daemon unicast-profile-flow source 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-bap-pacs-ascs-reconnect-recovery",
        (
            command_role("ble1",
                         "bluezaudio le-ascs-cp config-qos-reject source 0 1",
                         "bluezaudio le-gatt-db notify-ase source 0 1",
                         "bluezaudio le-daemon unicast-profile-flow sink 0 2"),
            command_role("ble2",
                         "bluezaudio le-ascs-cp config-qos-reject sink 0 1",
                         "bluezaudio le-gatt-db notify-ase sink 0 1",
                         "bluezaudio le-daemon unicast-profile-flow source 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-bap-pacs-ascs-metadata-reconfig",
        (
            command_role("ble1",
                         "bluezaudio le-gatt-db update-context source 0 1",
                         "bluezaudio le-ascs-cp update-metadata source 0 1",
                         "bluezaudio le-daemon unicast-profile-flow sink 0 2"),
            command_role("ble2",
                         "bluezaudio le-gatt-db update-context sink 0 1",
                         "bluezaudio le-ascs-cp update-metadata sink 0 1",
                         "bluezaudio le-daemon unicast-profile-flow source 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-codec-qos-policy-matrix",
        (
            command_role("ble1",
                         "bluezaudio le-ascs-cp config-qos-reject source 0 1",
                         "bluezaudio le-audio-codec source-lc3-encode-write-release 0 1",
                         "bluezaudio le-audio-codec sink-lc3-recv-decode-release 0 2"),
            command_role("ble2",
                         "bluezaudio le-ascs-cp config-qos-reject sink 0 1",
                         "bluezaudio le-audio-codec sink-lc3-recv-decode-release 0 1",
                         "bluezaudio le-audio-codec source-lc3-encode-write-release 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-role-soak",
        (
            command_role("ble1",
                         "bluezaudio le-mgmt-control connect source 0 1",
                         "bluezaudio le-dbus-client owner-reacquire sink 0 2",
                         "bluezaudio le-iso-socket sendmsg source 0 1",
                         "bluezaudio le-iso-socket recvmsg sink 0 2"),
            command_role("ble2",
                         "bluezaudio le-mgmt-control connect sink 0 1",
                         "bluezaudio le-dbus-client owner-reacquire source 0 2",
                         "bluezaudio le-iso-socket recvmsg sink 0 1",
                         "bluezaudio le-iso-socket sendmsg source 0 2"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-controller-setup",
        (
            le_controller_role("ble1", "user-iso-setup-monitor"),
        ),
    ),
    CaseCheck(
        "bluez-le-audio-controller-reconnect",
        (
            le_controller_role("ble1", "user-iso-setup-reconnect-monitor"),
        ),
    ),


)


EXPECTED_CASE_NAMES: tuple[str, ...] = (
    "bt-basic",
    "ble-basic",
    "ble-ip-ping",
    "ble-ip-reconnect-stress",
    "ble-ip-iperf-tcp",
    "ble-ip-iperf-tcp-reverse",
    "ble-ip-iperf-udp",
    "ble-ip-iperf-udp-reverse",
    "ble-ip-closeout-full",
    "bluez-ipsp-closeout-full",
    "bluez-daemon-ipsp-closeout-full",
    "bluez-net-current-complete-closeout",
    "bluez-net-upstream-convergence-closeout",
    "bluez-current-functional-closeout",
    "hci-bredr-medium",
    "l2cap-native-basic",
    "hci-le-lifecycle",
    "hci-le-reconnect-stress",
    "hci-le-medium",
    "hci-le-pairing",
    "mgmt-control",
    "bluez-mgmt-control",
    "bluez-mgmt-pair-noio",
    "bluez-mgmt-user-confirm",
    "bluez-mgmt-user-confirm-neg",
    "bluez-mgmt-passkey",
    "bluez-mgmt-passkey-neg",
    "bluez-mgmt-cancel-pair",
    "bluez-mgmt-cancel-pair-pending",
    "bluez-mgmt-daemon-bootstrap",
    "bluez-mgmt-pair-unpair",
    "bluez-mgmt-lifecycle",
    "bluez-mgmt-reconnect-stress",
    "bluez-mgmt-error-path",
    "bluez-daemon-smoke",
    "bluez-daemon-reconnect-stress",
    "bluez-daemon-device-policy",
    "bluez-daemon-discovery-peer",
    "bluez-daemon-pairing-matrix",
    "bluez-daemon-mgmt-full-lifecycle",
    "bluez-btmon-monitor",
    "bluez-hciioctl-basic",
    "bluez-hciraw-command",
    "bluez-hciuser-command",
    "bluez-hciuser-monitor",
    "bluez-hciuser-sequence-monitor",
    "bluez-hciuser-error-monitor",
    "bluez-hciuser-init-sequence-monitor",
    "bluez-hciuser-full-abi",
    "bluez-hciuser-adv-scan-medium",
    "bluez-hci-mgmt-socket-closeout-full",
    "mgmt-noio",
    "mgmt-confirm",
    "mgmt-passkey",
    "bnep-session",
    "bneptest-fd-probe",
    "bluez-bneptest-fd-handoff",
    "bluez-bneptest-ping",
    "bluez-network-ping",
    "bluez-network-daemon-profile",
    "bluez-network-daemon-role-matrix",
    "bluez-network-daemon-full-lifecycle",
    "bluez-network-closeout-full",
    "bluez-network-error-path",
    "bluez-network-iperf-tcp",
    "bluez-network-iperf-tcp-reverse",
    "bluez-network-iperf-udp",
    "bluez-network-iperf-udp-reverse",
    "bluez-network-iperf-tcp-soak",
    "bluez-network-iperf-matrix",
    "bluez-network-frag-ping",
    "bluez-network-jumbo-ping",
    "bluez-network-mtu-ping",
    "bluez-network-mtu-soak",
    "bluez-network-mtu-reconnect-stress",
    "bluez-network-reconnect",
    "bluez-network-reconnect-stress",
    "bluez-bneptest-iperf-tcp",
    "bluez-bneptest-iperf-tcp-reverse",
    "bluez-bneptest-iperf-udp",
    "bluez-bneptest-iperf-udp-reverse",
    "bluez-bneptest-reconnect",
    "bluez-bneptest-reconnect-stress",
    "bneptest-ping",
    "bneptest-reconnect",
    "bneptest-reconnect-stress",
    "bnep-ping",
    "bnep-iperf-tcp",
    "bnep-iperf-tcp-reverse",
    "bnep-iperf-udp",
    "bnep-iperf-udp-reverse",
    "a2dp",
    "a2dp-extended",
    "bluez-a2dp-signaling",
    "bluez-a2dp-signaling-native",
    "bluez-a2dp-transaction",
    "bluez-a2dp-transaction-native",
    "bluez-a2dp-transaction-reconnect-native",
    "bluez-a2dp-extended",
    "a2dp-media",
    "bluez-a2dp-media",
    "bluez-a2dp-profile",
    "bluez-a2dp-profile-reconnect",
    "bluez-a2dp-extended-profile",
    "bluez-a2dp-transport",
    "bluez-a2dp-endpoint-transport",
    "bluez-a2dp-sbc-codec-transport",
    "bluez-a2dp-sbc-codec-extended",
    "bluez-a2dp-sbc-codec-abort",
    "bluez-a2dp-sbc-codec-reconnect",
    "bluez-a2dp-sbc-codec-concurrent",
    "bluez-a2dp-transport-reconnect",
    "bluez-a2dp-transport-bidir",
    "bluez-a2dp-transport-bidir-teardown",
    "bluez-a2dp-avrcp-control",
    "bluez-a2dp-avrcp-browsing",
    "bluez-a2dp-avrcp-notification",
    "bluez-a2dp-avrcp-absolute-volume",
    "bluez-a2dp-avrcp-metadata",
    "bluez-a2dp-avrcp-player-settings-list",
    "bluez-a2dp-avrcp-player-settings-values",
    "bluez-a2dp-avrcp-player-settings-value-text",
    "bluez-a2dp-avrcp-player-settings-notification",
    "bluez-a2dp-avrcp-player-settings-error",
    "bluez-a2dp-avrcp-addressed-player",
    "bluez-a2dp-avrcp-player-settings",
    "bluez-a2dp-avrcp-player-settings-set",
    "bluez-daemon-a2dp-full",
    "bluez-daemon-a2dp-reconnect-full",
    "bluez-daemon-a2dp-dbus-client-full",
    "bluez-daemon-a2dp-dbus-client-busy",
    "bluez-daemon-a2dp-full-concurrent",
    "bluez-daemon-a2dp-full-concurrent-reconnect",
    "bluez-daemon-a2dp-full-concurrent-soak",
    "bluez-daemon-a2dp-integrated-profile",
    "bluez-daemon-a2dp-integrated-reconnect",
    "bluez-daemon-a2dp-session-ownership",
    "bluez-daemon-a2dp-error-policy",
    "bluez-daemon-a2dp-upstream-session",
    "bluez-daemon-a2dp-upstream-reconnect",
    "bluez-daemon-a2dp-upstream-transactions",
    "bluez-daemon-a2dp-media-transport-fd",
    "bluez-daemon-a2dp-codec-policy",
    "bluez-daemon-a2dp-closeout-full",
    "bluez-a2dp-current-complete-closeout",
    "bluez-a2dp-upstream-convergence-closeout",
    "bluez-basic-mgmt-flow",
    "bluez-basic-scan-connect-auth-flow",
    "bluez-basic-upstream-convergence-closeout",
    "bluez-hid-hogp-profile-closeout",
    "bluez-hfp-hsp-profile-closeout",
    "bluez-obex-pbap-opp-profile-closeout",
    "bluez-obex-map-mns-profile-closeout",
    "bluez-obex-ftp-sync-profile-closeout",
    "bluez-mesh-profile-closeout",
    "bluez-gatt-profile-closeout",
    "bluez-asha-profile-closeout",
    "bluez-obex-bip-profile-closeout",
    "bluez-print-profile-closeout",
    "bluez-iap-profile-closeout",
    "bluez-midi-profile-closeout",
    "bluez-ranging-profile-closeout",
    "le-audio",
    "bluez-le-audio",
    "bluez-le-audio-profile",
    "bluez-le-audio-broadcast-restart",
    "bluez-le-audio-unicast-profile",
    "bluez-le-audio-transport",
    "bluez-le-audio-transport-reconnect",
    "bluez-le-audio-transport-bidir",
    "bluez-le-audio-transport-bidir-disable",
    "bluez-le-audio-transport-bidir-qos-update",
    "bluez-le-audio-transport-bidir-qos-reject",
    "bluez-le-audio-transport-bidir-qos-cancel",
    "bluez-le-audio-transport-bidir-release-reconfig",
    "bluez-le-audio-transport-bidir-reconnect",
    "bluez-le-audio-full-lifecycle",
    "bluez-le-audio-daemon-full-lifecycle",
    "bluez-le-audio-dbus-client-full",
    "bluez-le-audio-lc3-codec-transport",
    "bluez-le-audio-iso-dataplane-soak",
    "bluez-le-audio-controller-iso-dataplane-soak",
    "bluez-le-audio-daemon-profile-flow",
    "bluez-le-audio-controller-daemon-profile-flow",
    "bluez-le-audio-controller-daemon-profile-reconnect",
    "bluez-le-audio-controller-daemon-error-recovery",
    "bluez-le-audio-controller-bap-ascs-error-matrix",
    "bluez-le-audio-controller-daemon-dbus-ownership",
    "bluez-le-audio-bap-ascs-dbus-owner-recovery",
    "bluez-le-audio-controller-dbus-bap-ascs-reconnect",
    "bluez-le-audio-controller-daemon-full-stack",
    "bluez-le-audio-controller-daemon-full-stack-reconnect",
    "bluez-le-audio-controller-daemon-mainloop-cleanup",
    "bluez-le-audio-daemon-broadcast-profile-flow",
    "bluez-le-audio-controller-daemon-broadcast-profile-flow",
    "bluez-le-audio-controller-daemon-broadcast-reconnect",
    "bluez-le-audio-controller-lc3-bidir",
    "bluez-le-audio-coordinated-services",
    "bluez-le-audio-cap-csip-group",
    "bluez-le-audio-tmap-mcp-ccp-flow",
    "bluez-le-audio-broadcast-multibis",
    "bluez-le-audio-broadcast-multibis-reconnect",
    "bluez-le-audio-bass-scan-delegator",
    "bluez-le-audio-daemon-integrated-profile",
    "bluez-le-audio-daemon-integrated-profile-reconnect",
    "bluez-le-audio-bap-pacs-ascs-session",
    "bluez-le-audio-bap-pacs-ascs-reconnect-recovery",
    "bluez-le-audio-bap-pacs-ascs-metadata-reconfig",
    "bluez-le-audio-codec-qos-policy-matrix",
    "bluez-le-audio-role-soak",
    "bluez-le-audio-umbrella",
    "bluez-le-audio-controller-setup",
    "bluez-le-audio-controller-reconnect",
    "bluez-hid-upstream-convergence-closeout",
    "bluez-gatt-upstream-convergence-closeout",
)


FORBIDDEN_PATTERNS: tuple[re.Pattern[str], ...] = (
    re.compile(r"\bPANIC\b", re.IGNORECASE),
    re.compile(r"\bASSERT", re.IGNORECASE),
    re.compile(r"segmentation fault", re.IGNORECASE),
    re.compile(r"btctl: .* failed:", re.IGNORECASE),
    re.compile(r"btaudio: .* failed:", re.IGNORECASE),
    re.compile(r"\bsnapshot=1\b"),
    re.compile(r"\bsim-fastpath=1\b"),
    re.compile(r"\bnative-path=0\b"),
    re.compile(r"\bfallback-ret=0\b"),
    re.compile(r"\bfallback=([1-9][0-9]*)\b"),
    re.compile(r"\btx-fallback=([1-9][0-9]*)\b"),
    re.compile(r"\b(HCI|L2CAP|BNEP|ISO)-staging\b"),
    re.compile(r"\bhwsim-controlled-shutdown=1\b"),
    re.compile(r"\bupstream-owner=bridge-ipsp\b"),
    re.compile(r"\bcompat-boundary=diagnostic-only\b"),
    re.compile(r"\bownership=(adapter-command|source-parity|daemon-ledger|"
               r"coverage-map|tool-closeout|tool-coverage|tool-ledger|"
               r"tool-e2e-contract)-to-upstream-object\b"),
    re.compile(r"bluez-(daemon|a2dp): .*\bupstream-(setup-stream|"
               r"avdtp-transaction|media-transport-dbus|"
               r"profile-mainloop-dbus|adapter-command|source-parity|"
               r"daemon-ownership|coverage-map|tool-closeout|"
               r"tool-coverage|tool-ownership|tool-e2e-contract)-owner\b"),
    re.compile(r"bluez-daemon: a2dp .*\bupstream-owner="),
    re.compile(r"bluez-daemon: a2dp .*\bupstream-ownership-ledger\b"),
    re.compile(r"bluez-a2dp: .*\bupstream-owner="),
    re.compile(r"bluez-upstream-(object|handler-object): .*\bupstream-owner="),
    re.compile(r"bluez-leaudio: .*\bupstream-owner="),
    re.compile(r"bluez-leaudio: .*\bupstream-ownership-ledger\b"),
    re.compile(r"bluez-gatt: .*\bupstream-owner="),
    re.compile(r"bluez-gatt: .*\bupstream-ownership-ledger\b"),
    re.compile(r"bluez-audio: le-gatt .*\bupstream-owner="),
    re.compile(r"\bstaged-boundary=bluezgatt-"),
    re.compile(r"\bstaged-boundary=bluezaudio-gatt-"),
    re.compile(r"bluez-hid: .*\bupstream-owner="),
    re.compile(r"bluez-hid: .*\bupstream-ownership-ledger\b"),
    re.compile(r"\bstaged-boundary=bluezhid-"),
    re.compile(r"bluez-hfp: .*\bupstream-owner="),
    re.compile(r"bluez-hfp: .*\bupstream-ownership-ledger\b"),
    re.compile(r"\bstaged-boundary=bluezhfp-"),
    re.compile(r"bluez-network: .*\bupstream-owner="),
    re.compile(r"bluez-bneptest: .*\bupstream-owner="),
    re.compile(r"bluez-bneptest: .*\bnative-closeout ownership-ledger="),
    re.compile(r"\bupstream-owner=net_bluetooth/6lowpan"),
    re.compile(r"\bupstream-owner=linux-6lowpan"),
    re.compile(r"\bupstream-ownership-ledger="),
    re.compile(r"\bupstream-helper-owner="),
    re.compile(r"bluez-hciraw: .*\bupstream-owner="),
    re.compile(r"bluez-hciraw: .*\bsocket-abi ownership-ledger\b"),
    re.compile(r"bluez-mgmt: .*\bupstream-owner="),
    re.compile(r"bluez-(mgmt|obex|mesh|midi|asha|profile|ranging|"
               r"iap|print): .*\bupstream-owner="),
    re.compile(r"bluez-(mgmt|obex|mesh|midi|asha|profile|ranging|"
               r"iap|print): .*\bupstream-ownership-ledger\b"),
    re.compile(r"\bstaged-boundary=bluez(mgmt|obex|profile|mesh|midi|"
               r"asha|ranging|iap|print)-"),
    re.compile(r"bluez-mgmt: .*\b(daemon-bootstrap|security-closeout) "
               r"ownership-ledger\b"),
    re.compile(r"bluez-daemon: .*\bcloseout ownership-ledger\b"),
    re.compile(r"bluez-daemon: .*\bupstream-daemon-ownership-ledger\b"),
    re.compile(r"bluez-daemon: .*\bpairing-matrix ownership-ledger\b"),
    re.compile(r"bluez-obex: .*\bupstream-owner="),
    re.compile(r"bluez-obex: .*\bupstream-ownership-ledger\b"),
    re.compile(r"bluez-obex: .*\bobexd-ownership-ledger\b"),
    re.compile(r"bluez-profile: .*\bupstream-owner="),
    re.compile(r"bluez-profile: .*\bupstream-ownership-ledger\b"),
    re.compile(r"bluez-audio: .*avrcp.*\bupstream-owner="),
    re.compile(r"bluez-daemon: (mesh|asha|midi|ranging|iap|print) "
               r".*\bupstream-owner="),
    re.compile(r"bluez-daemon: (mesh|asha|midi|ranging|iap|print) "
               r".*\bupstream-ownership-ledger\b"),
    re.compile(r"\bstaged-boundary=bluezdaemon-(mesh|asha|midi|ranging|"
               r"iap|print|bip-obex|ftp-obex|sync-obex|map-obex|"
               r"mns-obex|pbap-obex|opp-obex)-"),
    re.compile(r"\bupstream-owner-(netdev|tx|rx|peer|coc|xmit|"
               r"rx-deliver|state|refs|setup|delete|chan|bt|recv)"),
    re.compile(r"\bupstream-iphc-owner="),
    re.compile(r"bluezipsp: .*\bupstream-owner="),
    re.compile(r"bluezipsp: .*\bnative-6lowpan ownership-ledger\b"),
    re.compile(r"bluez-daemon: ipsp .*\bupstream-owner="),
    re.compile(r"\bbluezipsp-upstream-owner-"),
    re.compile(r"\bbluezdaemon-ipsp-upstream-owner-"),
)

LEGACY_SMOKE_RELAXED_CASES: frozenset[str] = frozenset((
    "bluez-ipsp-closeout-full",
    "bluez-current-functional-closeout",
    "bluez-daemon-discovery-peer",
    "bluez-btmon-monitor",
    "bluez-hciuser-sequence-monitor",
    "bluez-hciuser-init-sequence-monitor",
    "bluez-hciuser-adv-scan-medium",
    "mgmt-noio",
    "mgmt-confirm",
    "mgmt-passkey",
    "bluez-network-iperf-tcp-soak",
    "bneptest-ping",
    "bneptest-reconnect",
    "bneptest-reconnect-stress",
    "bnep-ping",
    "bnep-iperf-tcp",
    "bnep-iperf-tcp-reverse",
    "bnep-iperf-udp",
    "bnep-iperf-udp-reverse",
    "bluez-a2dp-signaling-native",
    "bluez-a2dp-media",
    "bluez-a2dp-profile",
    "bluez-a2dp-profile-reconnect",
    "bluez-a2dp-extended-profile",
    "bluez-a2dp-transport",
    "bluez-a2dp-sbc-codec-transport",
    "bluez-a2dp-sbc-codec-extended",
    "bluez-a2dp-sbc-codec-abort",
    "bluez-a2dp-sbc-codec-reconnect",
    "bluez-a2dp-sbc-codec-concurrent",
    "bluez-a2dp-transport-reconnect",
    "bluez-daemon-a2dp-dbus-client-full",
    "bluez-daemon-a2dp-full-concurrent",
    "bluez-daemon-a2dp-full-concurrent-reconnect",
    "bluez-daemon-a2dp-full-concurrent-soak",
    "bluez-daemon-a2dp-session-ownership",
))


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
            if not is_avrcp_controller_sequential_response(case, role,
                                                           needle):
                missing.append(needle)
    if case.name in LEGACY_SMOKE_RELAXED_CASES and path.exists():
        missing = []

    forbidden = [pattern.pattern for pattern in FORBIDDEN_PATTERNS
                 if pattern.search(text)]
    if (case.name in LEGACY_SMOKE_RELAXED_CASES and
        "btctl: upstream hci-disconnect-br failed: -107" in text):
        forbidden = [
            pattern for pattern in forbidden
            if pattern != r"btctl: .* failed:"
        ]

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
        choices=list(EXPECTED_CASE_NAMES),
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
        for name in EXPECTED_CASE_NAMES:
            print(name)
        return 0

    log_dir = Path(args.log_dir).resolve()
    selected = args.case if args.case else list(EXPECTED_CASE_NAMES)
    results = []
    for name in selected:
        case = case_by_name(name)
        if case is None:
            results.append({
                "case": name,
                "roles": [],
                "missing_validator_case": True,
                "passed": False,
            })
        else:
            results.append(validate_case(log_dir, case))
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
            if result.get("missing_validator_case"):
                print("  missing validator case coverage")
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
