# STM32H7S78-DK ESP-Hosted NG BT/BLE over SPI

This target uses an ESP32-C5 running ESP-Hosted NG firmware as the BT/BLE
controller. The STM32H7S78-DK runs NuttX and owns the Bluetooth host stack.

For the full board bring-up checklist, wiring expectations, phone/nRF Connect
steps, and pass/fail criteria, see:

```text
tools/firmware/stm32h7s78-dk/BT_BLE_ESP_HOSTED_NG_VALIDATION.md
```

## Target architecture

```text
NSH / bt command
  -> NuttX Bluetooth host + AF_BLUETOOTH socket/ioctl path
  -> ESP-Hosted NG bt_driver_s HCI lower-half
  -> ESP-Hosted NG SPI payload ESP_HOSTED_NG_HCI_IF
  -> ESP32-C5 ESP VHCI controller firmware
  -> BLE controller radio
```

Wi-Fi and BT/BLE HCI share the ESP-Hosted NG SPI transport. Bluetooth does not
use UART H4 in this target.

## Build STM32H7S78-DK host firmware

```sh
cd /home/uan/Feather-develop-BT/FeatherCore_ESP
./tools/firmware/stm32h7s78-dk/build-nsh-esp_hosted_ng_bt.sh
```

Optional host-side validation after the build:

```sh
./tools/firmware/stm32h7s78-dk/validate-nsh-esp_hosted_ng_bt-host.sh
```

Outputs:

```text
build/stm32h7s78-dk-nxboot.bin
build/stm32h7s78-dk-nsh-esp_hosted_ng_bt.bin
```

The NSH app image includes:

```text
ESP-Hosted NG SPI host driver
NuttX Bluetooth host stack
NET_BLUETOOTH socket/ioctl path
btsak as the NSH command `bt`
BLE advertising command `ble_adv`
BLE GATT server command `ble_gatt`
Combined BLE peripheral command `ble_periph`
```

## Build ESP32-C5 companion firmware

The ESP32-C5 firmware is treated as an external prerequisite for this target.
Do not rebuild or flash it from this STM32H7S firmware flow.

Expected external companion state:

```text
ESP32-C5 has ESP-Hosted NG firmware already flashed.
The firmware exposes BT/BLE HCI over the ESP-Hosted NG SPI interface.
The STM32H7S side owns the Bluetooth host stack.
```

Debug logging in the ESP32-C5 firmware can be adjusted separately when needed,
but functional ESP32-C5 firmware changes are outside this STM32H7S build target.

## NSH BLE advertising smoke test

After flashing both sides and booting NuttX:

```sh
ifconfig
bt bnep0 info
ble_periph check
ble_periph start
```

Expected behavior:

```text
The ESP32-C5 controller receives HCI commands over ESP_HOSTED_NG_HCI_IF.
A phone or BLE central should be able to scan the device.
The advertising name defaults to Feather-ESP-BLE.
After connecting, the central can discover the demo GATT service.
```

Stop advertising:

```sh
ble_adv stop
```

## Basic GATT/client smoke commands

For phone-as-central validation, the preferred single-command path is:

```sh
ble_periph check
ble_periph start
ble_periph status
```

`ble_periph check` verifies the STM32H7S Bluetooth host path before enabling
advertising. It opens the NuttX Bluetooth socket and reads controller
information with `SIOCGBTINFO`; failure here means the issue is below the GATT
or advertising demo layer.

The combined peripheral demo service is:

```text
Service UUID:  6df18a2c-7d8d-4b5d-9f9e-465354484552
Value UUID:    6df18a2d-7d8d-4b5d-9f9e-465354484552
Value handle:  0x0202
Properties:    read, write, write-without-response
Default value: Feather peripheral ready
```

The lower-level split debug commands remain available:

```sh
ble_gatt start
ble_gatt status
ble_adv start
ble_adv stop
```

The split GATT debug service is:

```text
Service UUID:  6df18a2a-7d8d-4b5d-9f9e-465354484552
Value UUID:    6df18a2b-7d8d-4b5d-9f9e-465354484552
Value handle:  0x0102
Properties:    read, write, write-without-response
Default value: Feather GATT ready
```

Expected phone/nRF Connect flow:

```text
Scan for Feather-ESP-BLE
Connect
Discover 6df18a2c-7d8d-4b5d-9f9e-465354484552
Read 6df18a2d-7d8d-4b5d-9f9e-465354484552
Write bytes or text to the same characteristic
Observe `ble_periph: read ...` and `ble_periph: write ...` on the NSH console
```

The demo deliberately avoids depending on private NuttX Bluetooth connection
callback internals. A successful central-side GATT read/write and the matching
NSH log are the board-level evidence that advertising, connection, ATT/GATT
data delivery, and the ESP-Hosted NG HCI path are working end to end.

After capturing the NSH console log from a board run, validate it with:

```sh
./tools/firmware/stm32h7s78-dk/validate-nsh-esp_hosted_ng_bt-board-log.sh \
  board-console.log
```

From NSH, `bt` also exposes scan and GATT client operations for peer-device
testing:

```sh
bt bnep0 scan start
bt bnep0 scan get
bt bnep0 scan stop
bt bnep0 gatt connect <addr> public
bt bnep0 gatt discover <addr> public <uuid16>
bt bnep0 gatt read <addr> public <handle>
bt bnep0 gatt write <addr> public <handle> <byte> [byte...]
```

## Current validation status

Host-side build was validated with:

```text
./tools/firmware/stm32h7s78-dk/build-nsh-esp_hosted_ng_bt.sh
```
