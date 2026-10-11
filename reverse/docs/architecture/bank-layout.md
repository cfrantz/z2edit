# PRG Bank Layout

The PRG ROM is 8 x 16K banks.  Bank 7 is fixed at $C000-$FFFF; banks 0-6
are switched into $8000-$BFFF (see [MMC1](../hardware/mmc1.md)).  Every
switchable bank ends with a copy of the reset code at $BF70-$BFFF
(`ResetStub`).

| Bank | Contents | Mapped when |
|------|----------|-------------|
| 0 | part of the per-frame game logic | in the game NMI, every gameplay frame |
| 1 | West Hyrule and Death Mountain overworlds and their side-view areas | `areaPrgBank` for world 0, regions 0 and 1 (from region 0) |
| 2 | East Hyrule and Maze Island overworlds and their side-view areas | `areaPrgBank` for world 0, regions 2 and 1 (from region 2) |
| 3 | towns, town NPCs and dialog | `areaPrgBank` for worlds 1 and 2 |
| 4 | palaces and palace enemies | `areaPrgBank` for worlds 3 and 4 |
| 5 | Great Palace (world 5); title screen and other non-gameplay screens | `areaPrgBank` for world 5; `nmiMode` = `NMI_MODE_BANK5` |
| 6 | music and sound effects | for the sound update in every NMI |
| 7 | fixed: reset, main loop, NMI, bank switching, most game logic | always |

Banks 1-5 are the *area banks*: the area loader and the per-frame code
reach them through `areaPrgBank` and the fixed addresses declared in
`src/interfaces.inc`.  Which one is mapped is chosen from the world and
region; see [The Area Bank](../hardware/mmc1.md#the-area-bank).

## Sound Entry Points (Bank 6)

| Entry | Called from | Used for |
|-------|-------------|----------|
| `sub_6_8000` | `sub_C032`, from `Bank5_Nmi` | title and other bank 5 screens |
| `sub_6_9000` | `Nmi` and `Nmi_SoundOnly` | gameplay, and area loading |

## Evidence

`tools/bank_usage.py` plays a movie and records, for each game state
(`$0736`) and world (`$0707`), how many CPU cycles execute in each bank.
Cycles in the main thread's idle loop are counted separately.  Over the
100AK movie (`private/Zelda2_100AK_schmidtty.fm2`):

- The main thread is idle for roughly 25-85% of all cycles, depending on
  the state.
- Of the remaining (NMI) cycles in gameplay, bank 7 executes about 80-90%,
  bank 0 about 5-10%, bank 6 about 4-6% and the area bank about 1-6%.
  Most game logic is therefore in bank 7; bank 0 holds a smaller set of
  per-frame routines.
- Every gameplay state runs bank 0 and bank 6 code.
- The only area bank that executes in each world is the one the table
  above gives: bank 1 or 2 in world 0, bank 3 in worlds 1 and 2, bank 4 in
  worlds 3 and 4, and bank 5 in world 5.
- Bank 5 runs outside world 5 only in states $00 and $01 in world 0
  (probably the title screens; about 24% of non-idle cycles), $11 in world
  0, and $05 in world 6.

The profiler reads the state and world once per 15-frame sample, so a few
cycles near a state change are attributed to the wrong state.  This
explains the small amounts of an unexpected bank in some rows (e.g. bank
3 or 4 in state $00, world 0).
