# MMC1 Usage and Bank Switching

Zelda II uses the Nintendo MMC1 mapper with 128K PRG ROM (8 x 16K banks),
128K CHR ROM, and 8K of battery-backed PRG RAM ("SRAM") at $6000-$7FFF.

## Configuration

| Register | Value | Set by | Meaning |
|----------|-------|--------|---------|
| Control  | `$0F` | `Reset` | horizontal mirroring, PRG mode 3, 8K CHR mode |
| Control  | `$0E` | `Area_Load` (area load) | vertical mirroring, PRG mode 3, 8K CHR mode |
| CHR      | varies | `Mmc1_SetChrBank` | 8K CHR bank (the low bit of the 4K bank number is ignored) |
| PRG      | 0-7   | `Mmc1_SetPrgBank` and friends | 16K bank at $8000-$BFFF |

PRG mode 3 fixes bank 7 at $C000-$FFFF and switches the bank at
$8000-$BFFF.  The control register is written only twice: horizontal
mirroring at reset, and vertical mirroring by the area loader, which then
remains in effect.  CHR1 is only written at reset (it is unused in 8K CHR
mode).  Bit 4 of the PRG register (PRG RAM disable) is always clear, so
SRAM is always enabled.

## Writing MMC1 Registers

MMC1 registers are loaded serially: five writes, each supplying bit 0, LSB
first.  A write with bit 7 set resets the shift register.  The routines in
bank 7 are:

| Routine | Effect |
|---------|--------|
| `Mmc1_SetControl` | control register = a |
| `Mmc1_SetChrBank` | CHR bank = a |
| `Mmc1_SetPrgBank` | PRG bank = a |
| `Mmc1_SetPrgBank0` | PRG bank = 0 |
| `Mmc1_SetPrgBankArea` | PRG bank = `areaPrgBank` |

Each switchable bank (0-6) also contains a copy of the reset code and these
routines at $BF70 (`ResetStub` in `src/resetstub.inc`), so that reset works
whichever bank is mapped at power-on.  Banks 0 and 5 call their own copies
of `Mmc1_SetControl` and `Mmc1_SetChrBank`.

The serial write is not atomic: an interrupt handler that writes the MMC1
while the main thread is part-way through a write corrupts both.  The game
avoids this by never switching banks in the main thread while an NMI that
switches banks can occur; see [the main loop and NMI](../architecture/main-loop-and-nmi.md).

## Which Bank Is Mapped When

- **Title screen and other bank 5 screens** (`nmiMode` = `NMI_MODE_BANK5`):
  bank 5 is mapped and its NMI handler (`Bank5_Nmi`, $A610) runs.
- **In game** (`nmiMode` = `NMI_MODE_GAME`): the game runs inside the NMI
  handler.  It maps bank 6 to update sound and then bank 0, which is the
  "home" bank for its per-frame routines (most per-frame logic is in bank 7;
  see [the bank layout](../architecture/bank-layout.md)).  The NMI returns
  with bank 0 still mapped; the main thread is idle in bank 7 and does not
  care.
- **Area loading** (`nmiMode` = `NMI_MODE_SOUND_ONLY`): the main thread
  runs the area loader with the area bank mapped.  The NMI only updates
  sound, mapping bank 6 and then restoring `areaPrgBank`.

## The Area Bank

`areaPrgBank` ($0769) holds the bank containing the current world's areas.
It is set by the code at $CD40 from the world ($0707) and region ($0706):

| World | Bank | Contents |
|-------|------|----------|
| 0 (overworld), region 0, or region 1 entered from region 0 | 1 | West Hyrule, Death Mountain |
| 0 (overworld), region 2, or region 1 entered from region 2 | 2 | East Hyrule, Maze Island |
| 1, 2 | 3 | towns |
| 3, 4 | 4 | palaces |
| 5 | 5 | Great Palace |

The previous region ($070A) distinguishes Death Mountain from Maze Island,
which are both region 1.

## Trampolines

Code in bank 0 reaches the area bank through short bank 7 routines of this
form:

```
    jsr     Mmc1_SetPrgBankArea     ; map the area bank
    jsr     <routine>               ; call into it (or read its data)
    jmp     Mmc1_SetPrgBank0        ; return to bank 0
```

Examples: `Overworld_GetTerrain` (calls `OverworldBank::GetTerrain`), `Overworld_UpdatePalaceStones`
(`OverworldBank::UpdatePalaceStones`), `Overworld_QueueMapStrip` (`OverworldBank::QueueMapStrip`).

## Bank Interfaces

Because bank 7 calls and reads area banks at fixed addresses, the area
banks share a common layout.  These are declared in `src/interfaces.inc`:

- `AreaBank` (banks 1-5): pointer tables at $8000, $8500, $8508; data at
  $800E, $851A-$8522, $88A0, $89A0, $8AA0, $8BA0; a routine at $850C that
  is identical in all five banks.
- `OverworldBank` (banks 1, 2): routines at $83A1, $83CF, $879B; data at
  $8409.  Banks 1 and 2 are byte-identical at $800B-$8505 (apart from 2
  bytes) and $A0FC-$BFFF.
- `PalaceBank` (banks 4, 5): data at $84D8.

## Quirks

- The IRQ vector points into the internal ROM header ($FFF0); IRQs are
  never enabled.
- Bank 1 writes to $95E6 (the MMC1 control register) at $9D6B, probably a
  typo for $05E6.  It is harmless; see the comment in `bank1.s`.
