About `rust/nes/mapper`

This subdirectory contains emulators for several NES mapper chips.

- cnrom.rs: An emulator for the "CNROM" mapper (mapper 3).
- mmc1.rs: An emulator for the Nintendo MMC1 mapper (mapper 1).
- mmc3.rs: An emulator for the Nintendo MMC3 mapper (confusingly given mapper ID 4).
- mmc5.rs: An emulator for the Nintendo MMC5 mapper (mapper 5).
- mod.rs: The module root and factory function for mappers.
- uxrom.rs: An emulator for the UxROM mapper (mapper 0).
- vrc7.rs: An emulator for the Konami VRC7 mapper (mapper 85).
- vrc7_audio/dsa_emu2413.c: A C implementation of the YM-2413 OPL-L chip.
- vrc7_audio/dsa_emu2413.h: The header file for the YM-2413 implementation.
- vrc7_audio/dsa_emu2413.rs: A rust binding to the C YM-2413 implementation.
- vrc7_audio/mod.rs: The vrc7_audio module root.
