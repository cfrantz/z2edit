# Moa

A floating eye (a ghost) that weaves back and forth around Link, drifting
up and down, through walls.  Touching it drains experience.  In East
Hyrule it is invisible without the cross.

## Where It Appears

| Group | Type | HP | Exp | Damage class | Drop group |
|-------|------|---:|----:|-------------:|-----------:|
| [West Hyrule](../west-hyrule.md) | $06 | 8 | 10 | 2 | 1 |
| [East Hyrule](../east-hyrule.md) | $06 | 48 | 50 | 3 | 1 |
| [Great Palace](../great-palace.md) | $06 | 6 | 5 | 1 | 1 |
| [Towns](../towns.md) | $06 | - | - | - | - |

Its attribute bit 4 makes touching it take experience: 20 points, twice
the usual (see [the definition tables](../README.md#experience)).  It
appears in graveyard encounter areas ($34-$38) of both Hyrules, and as
the "Purple Moa" in three areas of Old Kasuto ($15-$17).  No Great Palace
area places one.  The palaces' Orange Moa is a different enemy
(`OrangeMoa_Ai`).  It has no spawn routine
([spawn defaults](../../systems/enemies.md#spawning): X velocity 1/2
pixel per frame toward Link).

## Behavior

`Moa_Ai`, after `Enemy_SkipAiIfStunned`:

1. **Hit tests and drawing** (`Enemy_CheckCollisions`, `Enemy_Draw`,
   `Enemy_HurtLinkOnContact`).  In East Hyrule (`currentRegion` 2) it
   isn't drawn unless Link has the cross, but it can still be hit and
   still hurts.  It never probes the background, so walls and floors
   don't stop it.
2. **Move.**  Its own X velocity is a swing, below.  Each frame it moves
   by that velocity plus Link's X velocity plus 1/2 pixel per frame
   toward Link (`MoaSpeeds`).  So it keeps pace when Link walks, and
   creeps toward him.  It faces the way its own velocity points.
3. **Swing** (`enemyAiState` 1).  Its velocity changes by 1/16 pixel per
   frame every frame, away from the direction it faced when the swing
   began (`enemyAiDir`), until it reaches 2 pixels per frame the other
   way.  Then it holds (`enemyAiState` 0) for 32 frames
   (`timerFast+TIMER_ENEMY`), and the next swing goes back.  A full
   swing from +2 to -2 takes 64 frames; a new Moa starts with 40, from
   its 1/2 pixel per frame toward Link.
4. **Drift.**  At the start of each swing it picks a Y velocity of 1/2
   pixel per frame, down or up at random.  It is always down above Y $40
   and always up at Y $C0 or more.  It drifts only during the swing: 32
   pixels per full swing, none while holding.

Around Link, standing still, it sweeps about 64 pixels each way: while
holding at 2 pixels per frame for 32 frames, with the swings in between
adding nothing net.

## Drawing

`Moa_Draw`: a pair of sprites, tiles $B6, $B8 (`SpritePairTiles` entry
$1E), facing the way its own velocity points.  It is flipped upside down
every other 16 frames (`frameCounter` bit 4 clear).  When its velocity is
small, as it turns, the right sprite is drawn over the left one
(`MoaWobble`):

| abs(X velocity) | under 1/2 | 1/2 to 1 | 1 to 1.5 | 1.5 or more |
|-----------------|-----------|----------|----------|-------------|
| overlap (pixels) | 3 | 2 | 1 | 0 |

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Moa_Ai` | 7:$DACF | everything above |
| `MoaSpeeds` | 7:$DACD | $08, $F8: the push toward Link, and the drift |
| `Moa_Draw` | 7:$EFF2 | drawing |
| `MoaWobble` | 7:$EFE9 | the overlap, by abs(velocity) / 4 |

## Verification

Scenario (West Hyrule cave, area $2D; see the [Myu](myu.md#verification)
page for `$AT`), Link at X $2A0; Moas placed at (X $340, Y $60), ($220,
$C8) and ($300, $30); Link walked right for 40 frames from frame 300 and
left for 40 from 400.

- Velocity swings ran from -2 to +2 and back, 64 frames each (the first
  40), with 32-frame holds between.
- Each swing drifted 32 pixels.  The Moa at Y $C8 drifted up first, and
  the one at $30 down; one at Y $34 later drifted down.
- At 2 pixels per frame with Link on its left, a Moa moved 1.5 pixels
  per frame.  Its X followed Link's walk.
- It passed through the cave's walls and floor.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT --until areaId=0x2D --after 200 \
    --clear --approach 2A0 --spawn 0:06:340:60 --spawn 1:06:220:C8 --spawn 2:06:300:30 \
    --event Drift=7:DB0B --event Hold=7:DB47 --press 300:RIGHT:40 --press 400:LEFT:40 --frames 600
```
