# Main Loop and NMI

Zelda II runs almost all game logic inside the NMI (vblank interrupt)
handler.  The main thread is idle except when it loads an area, which takes
longer than one frame.

## Startup

1. `Reset` ($FF70): disable interrupts and NMI, wait two vblanks, reset and
   configure the MMC1 (see [MMC1](../hardware/mmc1.md)), map bank 7 at
   $8000, and jump to `Main`.
2. `Main` ($C000): disable rendering, map bank 5 and call its
   initialization (`sub_5_A6A0`), which sets `nmiMode` to `NMI_MODE_BANK5`
   and enables NMI.
3. Fall into `Main_Loop`.

## Main_Loop

```
loop:
    wait until game state ($0736) is $08 or $14 and $076C is 1
    nmiMode = NMI_MODE_SOUND_ONLY
    load the area (sub_C4CB)
    nmiMode = NMI_MODE_GAME
```

While idle, the loop uses only `a` and the flags.

## NMI Dispatch

`Nmi` ($C07B) pushes the flags and tests `nmiMode` ($0100):

| nmiMode | Handler | Used for |
|---------|---------|----------|
| bit 7 clear (`NMI_MODE_SOUND_ONLY`, $40) | `Nmi_SoundOnly` | area loading |
| bit 7 set, bit 6 clear (`NMI_MODE_BANK5`, $80) | `Nmi_Bank5` -> `Bank5_Nmi` ($A610 in bank 5) | title and other bank 5 screens |
| bits 7, 6 set (`NMI_MODE_GAME`, $C0) | game NMI (rest of `Nmi`) | gameplay |

- `Nmi_SoundOnly` saves all registers (the main thread is running real
  code), maps bank 6, calls the sound update (`sub_6_9000`), and restores
  the area bank with `Mmc1_SetPrgBankArea`.
- The game NMI saves only `a` (the main thread is idle).  It updates the
  PPU (sprite DMA, VRAM updates, scroll), reads the controllers
  (`Joypad_Update`), updates sound (bank 6), runs the game for one frame
  (mostly bank 0 code), and returns with bank 0 mapped.  Its details are
  analyzed in Phase 3.

## PPU Shadow Registers

| Variable | Address | Meaning |
|----------|---------|---------|
| `ppuCtrlShadow` | $00FF | value last written to PPUCTRL |
| `ppuMaskShadow` | $00FE | value last written to PPUMASK |
| `ppuScrollX` | $00FD | first PPUSCROLL write |
| `ppuScrollY` | $00FC | second PPUSCROLL write |

## Controllers

`Joypad_Update` ($D346), called by the game NMI, reads both controllers
(re-reading joypad 1 until two reads agree) and sets:

- `joypadPressed,x` ($00F5, $00F6): buttons newly pressed this frame
- `joypadHeld,x` ($00F7, $00F8): buttons held

Button bits are the `JOY_*` constants in `hardware.inc` (A = bit 7 ...
Right = bit 0).
