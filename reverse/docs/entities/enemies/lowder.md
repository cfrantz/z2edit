# Lowder

A beetle that walks back and forth, turning at walls and ledges, faster
when heading toward Link.

## Where It Appears

| Group | Type | HP | Exp | Damage class | Drop group |
|-------|------|---:|----:|-------------:|-----------:|
| [West Hyrule](../west-hyrule.md) | $1C | 4 | 3 | 0 | 1 |

Only West Hyrule has Lowders.  It uses background probe set 6 and has no
spawn routine, so it starts with the
[spawn defaults](../../systems/enemies.md#spawning): facing Link,
walking his way at 1/2 pixel per frame, `objState` 0.

## Behavior

`Lowder_Ai`, each frame after `Enemy_SkipAiIfStunned`:

1. `Enemy_HurtLinkOnContact`, with last frame's contact bit, then
   `Enemy_UpdateWithBg`: background probes, drawing, hit tests.
2. **Walk.**  It moves by its X velocity, doubled for this frame if its
   facing, which is its direction of travel, is the side Link is on: 1/2
   pixel per frame away from Link, 1 pixel per frame toward him.  It
   walks in the air too.
3. **Turn toward Link.**  If Link's Y + $10 equals its Y, as when Link
   stands on the same floor, and its timer (`timerFast+TIMER_ENEMY`) has
   run out, it faces Link, takes 1/2 pixel per frame his way
   (`LowderSpeeds`; doubled by step 2 from the next frame), and sets the
   timer to 160 frames.  So it turns toward Link as soon as he is on its
   level, and every 160 frames while he stays there.
4. **Floor and walls.**  On a floor with no wall ahead, it lands
   (`Enemy_Land`) and clears `objState`.  With a wall ahead, or no floor
   under it:
   - If `objState` is 0, this is the first such frame: it sets
     `objState` to 1, turns around (`Enemy_TurnAround`), and moves twice
     at its new speed, backing off 1 pixel.  At a ledge this puts its
     floor probe, 8 pixels in from its left edge, back over the floor, so
     it turns at ledges as well as at walls.
   - Otherwise it falls: `Enemy_MoveYWithGravity` with gravity $20/256
     pixel per frame per frame, at most 3 pixels per frame.  It keeps
     walking as it falls.

It never jumps.  Since the first frame without a floor always counts as
a ledge, a Lowder that spawns in mid-air turns around once and then
falls ([quirks](../../quirks.md)).

## Drawing

`Lowder_Draw`: two sprites facing its `objFacing`, alternating every 8
frames between tiles $DE, $E0 and $E2, $E0 (`sramEnemyTiles` entries $3C
and $3E).

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Lowder_Ai` | 1:$98C3 | everything above |
| `LowderSpeeds` | 1:$98C1 | X velocity: $08 right, $F8 left |
| `Lowder_Draw` | 1:$9DF1 | drawing |

## Verification

- No-glitch movie, frames 57500-58010, three Lowders at Y $90 with Link
  at Y $80: each turned toward Link every 160 frames; one walked 160
  pixels in 160 frames toward Link, and another slowed from 1 to 1/2
  pixel per frame when Link walked past it.
- Scenario (West Hyrule cave, area $2D; see the
  [Myu](myu.md#verification) page for `$AT`), aim timers poked to $FF so
  that Link doesn't draw them: a Lowder walking right turned at X $392,
  where its wall probe (X + 14) reaches the wall at $3A0, and backed off
  to $391 with `objState` 1 for one frame; another turned at the edges of
  the one-tile pits at $310 and $340, at X $316/$317 moving left and
  $338/$339 moving right (its X + 8 just inside the pit).
- Scenario: a Lowder placed in mid-air at X $300, Y $60, facing right,
  turned around on its first frame and fell with Y velocity 1, 2, then 3
  pixels per frame (8 frames each), walking left at 1 pixel per frame
  toward Link.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT --until areaId=0x2D --after 200 \
    --clear --approach 2A0 --spawn 0:1C:370:90:1 --spawn 1:1C:328:90:2 \
    --poke timerFast+3=0xFF --poke timerFast+4=0xFF --event Turn=1:991F --frames 140
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT --until areaId=0x2D --after 200 \
    --clear --approach 2A0 --spawn 0:1C:300:60:1 --poke timerFast+3=0xFF --event Turn=1:991F --frames 30
```
