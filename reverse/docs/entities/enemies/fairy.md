# Fairy

A fairy hovering in place.  Touching it refills Link's life while the
game stands still and the screen flashes.

## Where It Appears

Type $00 in every enemy group but the towns' (where $00 is also a fairy,
`Fairy_Ai`).  It has no hit points, can't be hurt (it blocks Link's
projectiles, the sword passes through, and it is immune to SPELL and
Thunder), and has no spawn routine.

- **Fairy encounters.**  An overworld encounter sprite of kind 3 sets
  `fairyEncounter`.  The encounter's area then spawns nothing from its
  enemy list (`Enemy_TrySpawn`).  `Area_PlaceElevatorAndDoor` puts a
  fairy in enemy slot 1 at X $180, 42 or 43 pixels above Link, empties
  the other slots, and makes the area one screen wide.
- **Enemy lists**, as type $00, as in some palace rooms.

## Behavior

`Fairy_Ai`:

1. **Sound.**  The first time it is on screen and the pulse 2 sound
   channel is free (`sq2Sfx`), it plays its sound ($C0), once
   (`objState+1`).
2. `Enemy_UpdateCommon` (drawing, contact).
3. **Touch.**  When Link touches it (`objCollision` bit 4), it sets
   `enemyAiState` to 1 and freezes the game (`gameFrozen`).  It asks for
   a full refill of his life (`statMeterPending+1` = $FF).
4. **Refill.**  Each frame after that it keeps the screen flashing
   (`spellFlash` bit 4).  When the refill is done
   (`statMeterPending+1` back to 0), it unfreezes the game and is
   removed (`Enemy_Remove`).

## Drawing

`Fairy_Draw`: one sprite in palette 1, 8 pixels below its Y, bobbing 0 to
4 pixels over a 64-frame cycle (`FairyYOffsets`, by `frameCounter` bits
5-3).  It flaps between tiles $68 and $6A every 4 frames.  It isn't drawn
when its column is off screen.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Fairy_Ai` | 7:$D91E | everything above |
| `Area_PlaceElevatorAndDoor` | 0:$90ED | the fairy encounter's fairy |
| `Fairy_Draw` | 7:$F0DD | drawing |
| `FairyYOffsets` | 7:$EBB0 | the bob: 1, 2, 3, 4, 3, 2, 1, 0 |

## Verification

Neither test movie touches one.  Scenario in the West Hyrule cave (area
$2D; see the [Myu](myu.md#verification) page for `$AT`), with the candle,
a fairy placed at X $2D0, Y $78, Link walking into it from frame 30:

- It played its sound on frame 0.
- Link touched it on frame 46: `enemyAiState` 1, the game frozen.  It was
  removed on frame 48, because the scenario's invincibility makes the
  life meter read full, so the refill ended at once.  The screen was
  still flashing at frame 70.
- Screenshot: a small fairy beside Link.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT,itemOwned=1 --until areaId=0x2D --after 200 \
    --clear --approach 2A0 --spawn 0:00:2D0:78 --event Sound=7:D92B --event Touch=7:D93E \
    --event Gone=7:D954 --press 30:RIGHT:30 --frames 300
```
