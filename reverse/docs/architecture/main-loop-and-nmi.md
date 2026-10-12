# Main Loop and NMI

Zelda II runs almost all game logic inside the NMI (vblank interrupt)
handler.  The main thread is idle except when it loads an area, which takes
longer than one frame.

## Startup

1. `Reset` ($FF70): disable interrupts and NMI, wait two vblanks, reset and
   configure the MMC1 (see [MMC1](../hardware/mmc1.md)), map bank 7 at
   $8000, and jump to `Main`.
2. `Main` ($C000): disable rendering, map bank 5 and call its
   initialization (`Game_PowerOnInit`), which sets `nmiMode` to `NMI_MODE_BANK5`
   and enables NMI.
3. Fall into `Main_Loop`.

## Main_Loop

```
loop:
    wait until gameState is GAME_STATE_PLAY and
               gameMode is GAME_MODE_LOAD_AREA or GAME_MODE_LOAD_ROOM
    nmiMode = NMI_MODE_SOUND_ONLY
    load the area (Area_Load)
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
  code), maps bank 6, calls the sound update (`Sound_Engine`), and restores
  the area bank with `Mmc1_SetPrgBankArea`.
- The game NMI saves only `a` (the main thread is idle).  It updates the
  PPU (sprite DMA, VRAM updates, scroll), reads the controllers
  (`Joypad_Update`), updates sound (bank 6), runs the game for one frame
  (`Game_RunState`, mostly bank 0 code), runs the menus when the mode
  asked for them (`menusPending`, `Game_UpdateMenus`; see
  [menus](../systems/menus.md)), and returns with bank 0 mapped.

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

Handlers are reached through inline jump tables (`JumpTable`); when done
they usually jump to `Game_NextMode`, or to `Game_BlankAndNextMode` /
`Game_UnblankAndNextMode` to switch rendering off or on (`renderingOff`)
on the way.

Each level keeps the value it had when last checked (`gameStatePrev`,
`gameModePrev`, `gameStepPrev`); `Game_CheckStateChange`,
`Game_CheckModeChange` and `Game_CheckStepChange` compare and reset the
lower levels, then return the current value for an inline jump table.

`gameState` values (`GAME_STATE_*`):

| Value | State |
|-------|-------|
| 0 | `GameState_Begin`: start or continue a game at North Castle (`Game_StartAtNorthCastle`), `lives` = 3, go to state 1.  (On the title screens, bank 5 uses `gameState` for its own states.) |
| 1 | `GameState_Play`: dispatch on `gameMode` (below) |
| 2 | `GameState_Death`: Link's death, and the continue screen |
| 3 | `GameState_StartEnding`: map bank 5 and go to state 4 |
| 4 | `GameState_Ending`: bank 5's ending sequence, which ends by going to state 5 |
| 5 | `GameState_SaveAfterEnding`: save the game (`Mode_SaveAfterEnding`) and return to the title screen (`Game_ReturnToTitle`) |
| 6 | `GameState_Respawn`: after losing a life, show the lives screen (`Mode_ShowLives`' steps) and go back to state 1 in gameMode $07 |

`gameMode` values in state 1 (`GameState_Play`):

| Mode | Handler | |
|------|---------|---|
| $00 | `Mode_LoadWorld` | copy the world's tables from its area bank into SRAM |
| $01 | `Mode_ReturnToOverworld` | put Link back on the overworld at the location he left |
| $02 | `Mode_OverworldSetup` | |
| $03 | `Mode_IndexOverworldMap` | index the compressed overworld map (runs for many frames) |
| $04 | `Game_UnblankAndNextMode` | |
| $05 | `Mode_Overworld` | `GAME_MODE_OVERWORLD`: see [the overworld](../world/overworld.md) |
| $06 | `Mode_EnterLocation` | enter the location (or encounter) under Link |
| $07 | `Mode_PrepareLoad` | blank the screen |
| $08 | `Game_Wait` | `GAME_MODE_LOAD_AREA`: `Main_Loop` loads the area |
| $09 | `Mode_DrawArea` | draw the area, a step per frame |
| $0A | `Mode_StartSideView` | sprite 0, scroll, music; turn rendering on |
| $0B | `Mode_SideView` | `GAME_MODE_SIDE_VIEW`: see [the side view frame](../world/side-view.md) |
| $0C | `Mode_FallSaveScroll` | falling into an area (a hole on the overworld, or a pit) ... |
| $0D | `Mode_FallBlankScreen` | ... blank screen ... |
| $0E | `Mode_FallStart` | ... Link appears at the top ... |
| $0F | `Mode_FallAnimate` | ... and falls, then $08 or $10 |
| $10 | `Mode_LeaveArea` | Link left the area by an edge: the next area ($07) or the overworld ($01) |
| $11 | `Mode_ShowLives` | the lives screen (name and lives), before entering a palace |
| $12 | `Mode_EnterArea` | enter the area set up from the overworld |
| $13 | `Mode_FindConnectedRoom` | find the room connected by an elevator |
| $14 | `Game_Wait` | `GAME_MODE_LOAD_ROOM`: `Main_Loop` loads the room |
| $15 | `Mode_PlaceLink` | put Link on the ground, then $09 |
| $16 | `Mode_EnterDoor` | Link went through a town door |
| $17 | `Mode_RaftStart` | the raft ride |
| $18 | `Mode_Raft` | |

Common sequences, from `tools/watch.py` over the no-glitch movie:

- entering a side view area from the overworld: $05 -> $06 -> $12 ($11
  first for a palace) -> $14 -> $15 -> $09 -> $0A -> $0B;
- walking off an edge: $0B -> $10 -> $07 -> $08 -> $09 -> $0A -> $0B, or
  $10 -> $01 -> $02 -> ... -> $05 back to the overworld;
- an elevator to another room: $0B -> $13 -> $14 -> $15 -> $09;
- falling down a pit: $0B -> $0C -> $0D -> $0E -> $0F -> $10.

`gameMode` values in state 2 (`GameState_Death`):

| Mode | Handler |
|------|---------|
| $00 | `Game_BlankAndNextMode` |
| $01 | `Death_ClearScreen` |
| $02 | `Death_StartAnimation` |
| $03 | `Death_Animate`: the death pose, flashing |
| $04 | `Death_LoseLife`: red background (`Death_QueueRedPalette`), then a life is lost: state 6, or the continue screen with no lives left |
| $05 | `Game_BlankAndNextMode` (Up+A on controller 2 in the pause menu enters here, `Game_ShowContinueScreen`) |
| $06 | `Death_DrawContinueScreen` |
| $07 | `Game_UnblankAndNextMode` |
| $08 | `Game_NextMode` |
| $09 | `Death_ContinueMenu`: Select moves the cursor; Start continues, or ... |
| $0A | `Death_SaveAndQuit`: ... with Save, blinks the choice, saves (`Death_SaveGame`) ... |
| $0B | `Game_ReturnToTitle`: ... and returns to the title screen |
