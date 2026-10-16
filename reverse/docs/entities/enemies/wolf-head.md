# Wolf Head

A flying wolf's head that crosses the screen in steps, firing energy
balls ahead of it; an invisible generator sends one every 128 frames.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class |
|-------|------|-------|---:|----:|-------------:|
| [Palaces 1, 2, 5](../palaces.md) | $1B | Wolf Head Generator | 2 | 5 | 1 |
| [Palaces 1, 2, 5](../palaces.md) | $1C | Wolf Head | 2 | 5 | 1 |
| [Palaces 3, 4, 6](../palaces.md) | $1B | Wolf Head Generator | 2 | 5 | 0 |
| [Palaces 3, 4, 6](../palaces.md) | $1C | Wolf Head | 2 | 5 | 1 |

The generator is placed in room $3E of palaces 1, 2 and 5 and $07, $09,
$20 and $3B of palaces 3, 4 and 6.  Touching a wolf head drains
experience (attribute bit 4).

## Behavior

**The generator** (`WolfHeadGenerator_Ai`): every 128 frames
(`frameCounter` $00, $80), while the screen follows Link (`scrollHeld`
0), it launches one (`Enemy_SpawnFlyer_Bank4`):

- in a free enemy slot from 5 down, with 2 hit points;
- at Link's Y less 2 (or 3);
- at the edge of the screen he is heading for: the right edge, flying
  left, if he holds Right; the left edge if he holds Left; either at
  random if neither;
- flying inward at 1/2 pixel per frame (`FlyerSpeeds_Bank4`), with no
  list entry.

**A wolf head** (`WolfHead_Ai`, no `Enemy_SkipAiIfStunned`) counts frames
in `enemyAiState`, from $FF:

- For 32 frames it flies across at 1/2 pixel per frame.  For the next 32
  it moves straight down 16 pixels; then across again, then up 16, and so
  on (bit 5: vertical; bit 6: up).
- At the start of each 64 (count bits 5-0 all 0), if it faces Link, it
  fires an energy ball straight ahead at 2 pixels per frame
  (projectile type 1, `Enemy_FireProjectile`).

It ignores the background and never turns, so it crosses the screen and
is despawned beyond it.

## Drawing

`WolfHead_Draw`: a pair of sprites from `sramEnemyTiles` entry $16,
facing its way (`Ironknuckle_DrawPair`).

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `WolfHeadGenerator_Ai` | 4:$B861 | the generator |
| `Enemy_SpawnFlyer_Bank4`, `Enemy_InitFlyer_Bank4` | 4:$B869, 4:$B876 | launching one |
| `FlyerSpeeds_Bank4` | 4:$B85F | $08, $F8 |
| `WolfHead_Ai` | 4:$B8BB | the head |
| `WolfHead_Draw` | 4:$B904 | drawing |

## Verification

Scenario in palace 1's first room (see [Bubble](bubble.md#verification)),
a generator placed with Link at X $1A0:

- Wolf heads came at `frameCounter` $00 and $80, at Y $AE (Link's $B0
  less 2).
- One crossed at 1/2 pixel per frame for 32 frames, went down 16 pixels
  (Y $AE to $BE), across, then up 16, and so on.  It fired straight
  ahead at the start of each 64 frames.
- With Right held over the spawn frame, the head came from the right edge
  (X $23B) flying left.  With Left held, it came from the left edge (X
  $149) flying right.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $PAT --until currentWorld=3 --after 200 \
    --clear $PFLAT --approach 1A0 --spawn 3:1B:1A0:80 --press 40:RIGHT:10 --frames 50
```
