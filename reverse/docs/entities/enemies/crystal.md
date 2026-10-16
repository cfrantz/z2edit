# Crystal Spot

The statue in each palace where Link sets the palace's crystal, sealing
it.  It is rewarded with a fanfare, full meters and a level.

## Where It Appears

| Group | Type | Enemy |
|-------|------|-------|
| [Palaces 1, 2, 5](../palaces.md) | $14, $15 | Crystal Spot, Crystal |
| [Palaces 3, 4, 6](../palaces.md) | $14, $15 | Crystal Spot, Crystal |

There is one spot per palace: rooms $0D, $22 and $29 of palaces 1, 2 and
5, and $0E, $1C and $3A of palaces 3, 4 and 6, at the statue on page 3.
Its spawn routine (`Crystal_Init`) puts it at X $7C, Y $62 on its page.
If the crystal is already placed (area flag 3 clear) it becomes the
crystal itself, type $15.

## Behavior

`CrystalSpot_Ai` keeps the palette flash going
(`CrystalSpot_FlashPalette`), draws the crystal once it is out, and runs
four steps by `enemyAiState`:

1. **Waiting** (`CrystalSpot_Wait`).  When it is on screen, Link has a
   crystal (`crystalsRemaining`), touches the spot and stands on the
   ground:
   - the game freezes (`gameFrozen`);
   - Link takes the holding-up pose (`objState` 3);
   - the crystal appears at Y $A0, and Link has one fewer;
   - a sound and the fanfare play;
   - the palace is marked done in `crystalPlaced` (by `palaceCode`, + 2
     past West Hyrule).
2. **Rising** (`CrystalSpot_Rise`).  The crystal rises 1 pixel per frame
   to Y $62, 62 frames.  Then background palette 3 flashes for 98 frames
   (`crystalFlashTimer`), cycling one color through
   `CrystalSpotFlashColors`, and the next music plays.
3. **The music** (`CrystalSpot_WaitMusic`).  When it ends, both of Link's
   meters are filled (`statMeterPending` $FF, $FF).
4. **Finish** (`CrystalSpot_Finish`).  When they are full:
   - the game unfreezes;
   - Link gets exactly the experience he lacks for his next level
     (`experiencePending` = `experienceNext` - `experience`);
   - the spot becomes the crystal, also in the area's enemy list (type
     byte $D5);
   - area flag 3 is cleared and the area's music returns.

**The crystal** (`Crystal_Ai`, type $15): one sprite, tile $3D, its
palette cycling every frame.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Crystal_Init` | 4:$9A73 | spawn: spot or crystal |
| `Crystal_Ai` | 4:$9A8B | the crystal |
| `CrystalSpot_Ai` | 4:$9AD4 | the steps |
| `CrystalSpot_Wait`, `CrystalSpot_Rise`, `CrystalSpot_WaitMusic`, `CrystalSpot_Finish` | 4:$9AEB-$9B56 | the steps |
| `CrystalSpot_FlashPalette` | 4:$9B95 | the flash |

## Verification

The no-glitch movie places crystals.  Scenario in palace 1's first room
(see [Bubble](bubble.md#verification)), a spot placed at Link's feet (X
$1A8, Y $A0) with `crystalsRemaining` 1:

- Placed on frame 0, risen to Y $62 on frame 62.
- The music ended at frame 375 and the step finished at 377.
- Screenshots: Link holding his arm up, the crystal rising beside the
  column.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $PAT --until currentWorld=3 --after 200 \
    --clear $PFLAT --approach 1A0 --spawn 0:14:1A8:A0 --poke crystalsRemaining=1 \
    --event Placed=4:9B03 --event Risen=4:9B35 --event MusicDone=4:9B4C --event Finish=4:9B5E --frames 700
```
