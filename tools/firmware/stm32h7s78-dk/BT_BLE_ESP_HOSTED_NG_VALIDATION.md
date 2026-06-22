# STM32H7S78-DK ESP-Hosted NG BT/BLE validation checklist

This checklist validates the STM32H7S78-DK host firmware path for an
ESP32-C5 that has already been manually flashed with ESP-Hosted NG firmware.
The ESP32-C5 firmware is an external prerequisite for this flow.

## Goal

Prove the end-to-end BLE peripheral path:

```text
STM32H7S78-DK NuttX NSH
  -> NuttX Bluetooth host socket/ioctl/GATT API
  -> ESP-Hosted NG HCI lower-half
  -> ESP-Hosted NG SPI transport
  -> ESP32-C5 controller firmware
  -> BLE radio
  -> phone / nRF Connect scan, connect, GATT read/write
```

## Host firmware build

Build only the STM32H7S78-DK host firmware:

```sh
cd /home/uan/Feather-develop-BT/FeatherCore_ESP
./tools/firmware/stm32h7s78-dk/build-nsh-esp_hosted_ng_bt.sh
```

Expected outputs:

```text
build/stm32h7s78-dk-nxboot.bin
build/stm32h7s78-dk-nsh-esp_hosted_ng_bt.bin
```

Run the host-side validation gate:

```sh
./tools/firmware/stm32h7s78-dk/validate-nsh-esp_hosted_ng_bt-host.sh
```

This checks the host build artifacts, important Kconfig selections, NSH demo
sources, and ESP-Hosted NG HCI lower-half evidence. It does not replace real
board validation.

The app image contains:

```text
bt
ble_adv
ble_gatt
ble_periph
ESP-Hosted NG SPI host driver
NuttX Bluetooth host stack
```

## ESP32-C5 prerequisite

The ESP32-C5 must already be flashed by the user. The expected ESP firmware
mode is:

```text
ESP-Hosted NG
SPI transport
BT/BLE enabled
HCI over SPI enabled
UART HCI not used by this STM32H7S target
```

Expected ESP32-C5 serial log hints:

```text
BT/BLE
HCI over SPI
BLE
```

Do not rebuild or modify ESP32-C5 firmware from the STM32H7S host flow.
Only ESP-side debug logs should be adjusted separately if needed.

## STM32H7S host SPI settings

The STM32H7S firmware build currently enables:

```text
CONFIG_STM32H7RS_SPI4=y
CONFIG_WIRELESS_ESP_HOSTED_NG_SPI=y
CONFIG_WL_ESP_HOSTED_NG_SPI_DEV=4
CONFIG_WL_ESP_HOSTED_NG_SPI_FREQUENCY=10000000
CONFIG_WL_ESP_HOSTED_NG_SPI_MODE=2
CONFIG_WL_ESP_HOSTED_NG_HANDSHAKE_PIN=2
CONFIG_WL_ESP_HOSTED_NG_DATA_READY_PIN=4
CONFIG_WL_ESP_HOSTED_NG_RESET_PIN=7
CONFIG_WL_ESP_HOSTED_NG_IFNAME="wlan0"
```

Wire the ESP32-C5 companion to the STM32H7S board according to the board-level
pinmux for:

```text
SPI4 SCK
SPI4 MOSI
SPI4 MISO
SPI4 CS
handshake pin
data_ready pin
reset / EN pin
GND
compatible I/O voltage
```

The numeric handshake/data_ready/reset values above are the logical host
configuration values used by the ESP-Hosted NG host driver. Match them to the
actual board GPIO mapping before hardware validation.

## Flashing expectations

Program:

```text
stm32h7s78-dk-nxboot.bin
  internal Flash 0x08000000

stm32h7s78-dk-nsh-esp_hosted_ng_bt.bin
  XSPI2 NOR 0x70000000
```

After reset, NSH should boot from the STM32H7S app image.

## NSH preflight

Run:

```sh
ifconfig
bt bnep0 info
ble_periph check
```

Expected:

```text
ble_periph: host path ok ifname=bnep0 ...
ble_periph: buffers cmd=... acl=... mtu_acl=...
```

Meaning:

```text
PF_BLUETOOTH socket creation works.
SIOCGBTINFO ioctl works.
NuttX Bluetooth host can see a controller path.
```

If this fails, debug below GATT/advertising:

```text
ESP32-C5 firmware boot mode
SPI wiring
handshake/data_ready/reset wiring
ESP-Hosted NG boot/capability event
HCI if_type=2 RX dispatch
ESP-Hosted NG HCI lower-half registration
```

## Start BLE peripheral mode

Run:

```sh
ble_periph start
ble_periph status
```

Expected:

```text
ble_periph: host path ok ifname=bnep0 ...
ble_periph: advertising started on bnep0 as "Feather-ESP-BLE"
ble_periph: GATT service=6df18a2c-7d8d-4b5d-9f9e-465354484552 ...
```

`ble_periph start` performs:

```text
SIOCGBTINFO preflight
bt_gatt_register()
SIOCBTADVSTART connectable advertising
```

Stop advertising:

```sh
ble_periph stop
```

## Phone / nRF Connect validation

On a phone or BLE central:

```text
Scan for Feather-ESP-BLE
Connect
Discover service 6df18a2c-7d8d-4b5d-9f9e-465354484552
Read characteristic 6df18a2d-7d8d-4b5d-9f9e-465354484552
Write text or bytes to the same characteristic
```

Expected NSH output after write:

```text
ble_periph: write handle=0x0202 offset=0 len=... count=... data=... text="..."
```

Expected NSH output after read:

```text
ble_periph: read handle=0x0202 offset=0 len=... returned=... count=...
```

This proves:

```text
Phone scan sees advertising.
Phone connects.
GATT service discovery works.
GATT read path reaches STM32H7S NSH-visible host code.
GATT write path reaches STM32H7S NSH-visible host code.
```

The NuttX connection callback registration API is internal to the Bluetooth
host implementation in this tree. The board-level proof therefore treats
successful central-side GATT read/write logs as the connection evidence instead
of adding a private callback dependency to this NSH demo.

Validate a captured NSH console log with:

```sh
./tools/firmware/stm32h7s78-dk/validate-nsh-esp_hosted_ng_bt-board-log.sh \
  board-console.log
```

The log validator requires:

```text
ble_periph: host path ok ...
ble_periph: advertising started ...
ble_periph: GATT service=6df18a2c-7d8d-4b5d-9f9e-465354484552 ...
ble_periph: read handle=0x0202 ...
ble_periph: write handle=0x0202 ...
```

## Split debug commands

Use these only when isolating failures:

```sh
ble_gatt start
ble_gatt status
ble_adv start
ble_adv stop
bt bnep0 scan start
bt bnep0 scan get
bt bnep0 scan stop
```

`ble_periph` is the preferred combined validation command.

## Pass criteria

The goal is complete only when all of these are true on real hardware:

```text
Host firmware builds.
ESP32-C5 is manually flashed and logs HCI-over-SPI-capable BT/BLE mode.
STM32H7S receives ESP-Hosted NG boot/capability traffic.
ble_periph check succeeds.
ble_periph start succeeds.
Phone scans Feather-ESP-BLE.
Phone connects.
Phone reads the demo GATT characteristic.
Phone writes the demo GATT characteristic.
NSH prints the received write data.
```

Build success alone is not sufficient for final completion.
