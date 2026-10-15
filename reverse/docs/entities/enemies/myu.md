# Myu

A spiked blob that crawls along the floor in little steps, resting now
and then.  Its movement, `Myu_Move`, is shared by the [Bot](bot.md) and
the [Bit](bit.md).

## Where It Appears

| Group | Type | HP | Exp | Damage class | Drop group |
|-------|------|---:|----:|-------------:|-----------:|
| [West Hyrule](../west-hyrule.md) | $03 | 3 | 3 | 1 | 1 |
| [East Hyrule](../east-hyrule.md) | $03 | 3 | 5 | 1 | 1 |
| [Palaces 1, 2, 5](../palaces.md) | $03 | 3 | 3 | 1 | 1 |
| [Palaces 3, 4, 6](../palaces.md) | $03 | 3 | 5 | 1 | 1 |
| [Great Palace](../great-palace.md) | $03 | 24 | 10 | 3 | 1 |

In every group it comes back after dying (flags bit 6) and uses
background probe set 6.  It has no spawn routine, so it starts with the
[spawn defaults](../../systems/enemies.md#spawning): X velocity 1/2
pixel per frame toward Link, all other state 0.

## Behavior

`Myu_Ai` sets the step speeds (`myuStepSpeeds`) to 2 pixels right and 2
left, then falls through into `Myu_Move`.  Each frame, after
`Enemy_SkipAiIfStunned` (no AI while stunned):

1. **Fall:** `Enemy_MoveYAndFall` (Y += Y velocity, then Y velocity +=
   2/16 pixel per frame), then `Enemy_UpdateWithBg` (background probes,
   drawing, hit tests) and `Enemy_HurtLinkOnContact`.
2. **In the air** (no floor under it, or Y less than $60): it moves by
   its X velocity unless there is a wall ahead, and nothing else happens
   this frame.  Floors are not tested at all while Y < $60, high on the
   screen, so a Myu standing on a floor up there falls through it
   ([quirks](../../quirks.md)).
3. **On the ground:** it lands (`Enemy_Land`: Y rounded down to a
   multiple of 16, Y velocity 0).  Then, by its phase (`enemyAiState`):
   - **Moving** (0): it counts its frames on the ground in `enemyAiVar`.
     On counts 3 and 15 of every 16 it takes a **step**: its X velocity
     is set from `myuStepSpeeds` by its direction (`enemyAiDir`: 0 right,
     1 left), and it moves once by it, unless there is a wall ahead.  A
     Myu therefore moves 2 pixels, then 2 more 12 frames later, then 2
     more 4 frames later: 4 pixels per 16 frames, 1/4 pixel per frame on
     average.
   - **Resting** (1): it stays where it is.
4. **Phase change:** when its timer (`timerFast+TIMER_ENEMY`) has run
   out, and only on the ground, it switches phase and reloads the timer
   with 0-63 at random (`rng+1,x` bits 5-0) ORed with 192 for a moving
   phase or 64 for a resting phase (`MyuPhaseTimes`).  So it moves for
   **192-255 frames** and rests for **64-127 frames**.  Bit 0 of the new
   timer becomes its direction, so each moving phase goes left or right
   at random, whatever side Link is on.

A new Myu has phase 0 and timer 0: on its first frame on the ground it
counts one moving frame, then switches to resting.

**Walls.**  It never turns around.  With a wall ahead (the probe on the
side it is moving toward), it simply doesn't step; it stays against the
wall until a later moving phase picks the other direction.

**Ledges.**  It doesn't test for them, and walks off.  In the air it
keeps the X velocity of its last step, 2 pixels per frame, so it falls
off a ledge in a fast, flat arc.  The step counter and phase don't
change in the air, but the timer keeps counting down; a phase that ends
in the air ends on the first frame back on the ground.

`objFacing` is never changed after spawning; it doesn't matter, since the
Myu is drawn symmetric.

## Drawing

`Myu_Draw`: a symmetric pair of sprites (the right one mirrored), tile
$24 (`SpritePairTiles` entry $2A) when `objState` is 0 and tile $B0
(entry $2C) otherwise.  `objState` is set by `Myu_Move`: bits 3-2 of the
step counter (0, 4, 8, 12) while moving, 1 while resting.  So while
moving it shows the first frame for 4 counts, ending with the step on
count 3, and the second frame for 12, ending with the step on count 15.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Myu_Ai` | 7:$DA47 | sets the step speeds, falls into `Myu_Move` |
| `Myu_Move` | 7:$DA54 | the movement above |
| `MyuPhaseTimes` | 7:$DA00 | minimum phase lengths: $C0 moving, $40 resting |
| `Myu_Draw` | 7:$EFC2 | drawing |

## Verification

- No-glitch movie, frames 42804-44334 (Bots, which move the same way):
  first rest 94 frames, then a moving phase of 246; steps of 2 pixels
  alternating 4 and 12 frames apart.  Over the first 100000 frames,
  every complete phase was 192-255 frames moving or 64-127 resting,
  except when a Bot's leap carried it past the end of a phase.
- Scenario (West Hyrule cave, area $2D; see
  [`tools/enemy_scenario.py`](../../../tools/ABOUT.md)): a Myu placed at
  X $388 next to the wall at $3A0 stepped right up to X $392 and stayed
  there through a whole moving phase, never turning; a later phase took
  it left.  Its phases lasted 226, 250 and 255 frames moving and 120, 98
  and 64 resting.
- Same scenario, with map row $40 made solid from X $2C0 to $2FF (a
  floor at Y $50): a Myu placed on it at Y $50 fell straight through,
  while a Lowder placed beside it walked on it and turned at its edge.

```
AT=areaPrgBank=1,currentRegion=0,currentWorld=0,townCode=0,palaceCode=0,areaLocation=3,areaId=0x2D,areaStartPage=0,areaEntrySide=0,prevRegion=0
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT --until areaId=0x2D --after 200 \
    --clear --approach 330 --fill 2C0:2FF:40 --spawn 0:03:388:90 --spawn 2:03:2D0:50 \
    --spawn 3:1C:2E8:50 --poke timerFast+6=0xFF --event Toggle=7:DAA9 --frames 1400
```
