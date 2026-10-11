# About `reverse/src`

The ca65 disassembly of Zelda II.  These files were generated once by
`tools/bootstrap.py` and are now maintained by hand following
`../CONVENTIONS.md`.  `make verify` (in `reverse/`) must always pass.

- `zelda2.cfg`: ld65 linker configuration (header, 8 PRG banks, CHR).
- `header.s`: iNES header; includes CHR ROM extracted from the reference ROM.
- `bank0.s` ... `bank7.s`: one file per 16K PRG bank.
- `zelda2.inc`: included by every bank; includes the files below.
- `hardware.inc`: NES hardware and MMC1 registers.
- `ram.inc`: RAM, zero page and SRAM variables.
- `constants.inc`: game-wide constants and enumerations.
- `interfaces.inc`: bank interfaces (see `../CONVENTIONS.md`).
- `globals.inc`: `.global` declarations for labels referenced across banks.
- `resetstub.inc`: the `ResetStub` macro: the reset code and vectors present
  at $BF70 in every switchable bank.
