# ESP32-C6 Rust no_std watchdog test

Default ESP Bootloader turns on RWDT.

Default ESP-HAL turns off RWDT and all other watchdogs.

This example verifies the scenario, plus prints registers.

## Expected output

```
ESP32-C6 Watchdog Test - Checking WDT status before and after esp_hal::init()
======================================================================

Checking watchdog status BEFORE esp_hal::init():
RWDT enabled: true, config0: 0xc007ea00
TIMG0 MWDT enabled: false, config0: 0x00048000
TIMG1 MWDT enabled: false, config0: 0x0004c000

==> Calling esp_hal::init() with default config...

Checking watchdog status AFTER esp_hal::init():
RWDT enabled: false, config0: 0x00000000
TIMG0 MWDT enabled: false, config0: 0x00000000
TIMG1 MWDT enabled: false, config0: 0x00000000

Changes after esp_hal::init():
  RWDT status changed: true -> false
  RWDT config0 changed: 0xc007ea00 -> 0x00000000
  TIMG0 MWDT status unchanged: false
  TIMG0 MWDT config0 changed: 0x00048000 -> 0x00000000
  TIMG1 MWDT status unchanged: false
  TIMG1 MWDT config0 changed: 0x0004c000 -> 0x00000000

Watchdog control registers inspected:
- LP_WDT base: 0x600B1C00 (RWDT control)
- TIMG0 base: 0x60008000 (MWDT0 control)
- TIMG1 base: 0x60009000 (MWDT1 control)
- Key register field: wdtconfig0.wdt_en (bit 31)
```
