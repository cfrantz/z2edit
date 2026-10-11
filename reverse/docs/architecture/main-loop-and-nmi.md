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
    wait until gameState is GAME_STATE_PLAY and
               gameMode is GAME_MODE_LOAD_AREA or GAME_MODE_LOAD_ROOM
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

## Game State Machine

The game NMI runs `Game_RunState`, a hierarchy of state machines:

| Level | Variable | Changed means |
|-------|----------|---------------|
| state | `gameState` ($076C) | `gameMode`, `gameStep` and `gameSubStep` restart at 0 |
| mode | `gameMode` ($0736) | `gameStep` and `gameSubStep` restart at 0 |
| step | `gameStep` ($0738) | `gameSubStep` restarts at 0 |
| sub-step | `gameSubStep` ($073D) | |

Each level keeps the value it had when last checked (`gameStatePrev`,
`gameModePrev`, `gameStepPrev`); `Game_CheckStateChange`,
`Game_CheckModeChange` and `Game_CheckStepChange` compare and reset the
lower levels, then return the current value for an inline jump table.

`gameState` values (`GAME_STATE_*`):

| Value | State |
|-------|-------|
| 0 | begin: set `lives` to 3, go to state 1 |
| 1 | play: dispatch on `gameMode` |
| 2 | death: Link's death animation; then lose a life (go to state 6), or show the continue screen when no lives remain.  Pressing Up+A on controller 2 enters the continue screen directly. |
| 6 | respawn after losing a life, then back to state 1 (provisional) |

States 3, 4 and 5 have not been identified.  On the title screens bank 5
uses `gameState` for its own states.

Common `gameMode` sequences in state 1, from `tools/watch.py` over the
no-glitch movie:

- entering a side-view area: $07 -> $08 (`GAME_MODE_LOAD_AREA`, loaded by
  `Main_Loop`) -> $09 -> $0A -> $0B (`GAME_MODE_SIDE_VIEW`), then $10 when
  Link leaves the area, followed by $07 for the next area or $01 for the
  overworld;
- moving to a connected room (e.g. by elevator): $12 -> $13 -> $14
  (`GAME_MODE_LOAD_ROOM`) -> $15 -> $09;
- the overworld: $00/$01 -> ... -> $05 (`GAME_MODE_OVERWORLD`) -> $06 ->
  $07 when Link enters an area.

