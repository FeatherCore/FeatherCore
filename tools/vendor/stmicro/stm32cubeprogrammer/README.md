# STM32CubeProgrammer Local CLI Cache

This directory can hold the small subset of STM32CubeProgrammer needed by local
firmware packaging scripts.

Expected local layout:

```text
bin/STM32_SigningTool_CLI
lib/libcrypto.so.3
lib/libQt6Core.so.6
lib/libicui18n.so.56
lib/libicuuc.so.56
lib/libicudata.so.56
```

`STM32_SigningTool_CLI` has a `$ORIGIN/../lib` runtime search path, so the
adjacent `lib` directory is required.  These files are ignored by Git because
they come from ST's STM32CubeProgrammer distribution.

Populate from a full local install:

```bash
install -d tools/vendor/stmicro/stm32cubeprogrammer/bin
install -d tools/vendor/stmicro/stm32cubeprogrammer/lib

cp -a /home/uan-wsl2/third/stm32cubeprogrammer/bin/STM32_SigningTool_CLI \
  tools/vendor/stmicro/stm32cubeprogrammer/bin/

cp -a /home/uan-wsl2/third/stm32cubeprogrammer/lib/libcrypto.so.3 \
  /home/uan-wsl2/third/stm32cubeprogrammer/lib/libQt6Core.so.6 \
  /home/uan-wsl2/third/stm32cubeprogrammer/lib/libicui18n.so.56 \
  /home/uan-wsl2/third/stm32cubeprogrammer/lib/libicuuc.so.56 \
  /home/uan-wsl2/third/stm32cubeprogrammer/lib/libicudata.so.56 \
  tools/vendor/stmicro/stm32cubeprogrammer/lib/
```
