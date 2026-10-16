# Tinsuit

A small armored creature that hops along the floor.  An invisible
generator sends them in from the edge of the screen every 128 frames.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Palaces 1, 2, 5](../palaces.md) | $0B | Tinsuit generator | 2 | 0 | 0 | - |
| [Palaces 1, 2, 5](../palaces.md) | $0C | Tinsuit | 2 | 0 | 0 | - |

The generator is placed in rooms $04 and $05 of palaces 1, 2 and 5;
Tinsuits themselves in rooms $01, $02, $03, $1C, $20, $32 and $3A.
Touching one drains experience.  They come back.  Palaces 3, 4 and 6
have the [Rathead](rathead.md) in the same type numbers.

## Behavior

**The generator** (`TinsuitGenerator_Ai`) launches one as a
[wolf head generator](wolf-head.md) would (`WolfHeadGenerator_Ai`):

- every 128 frames, while the screen follows Link;
- in a free enemy slot from 5 down, with 2 hit points;
- at the edge of the screen he is heading for.

It then puts it on the floor (Y $B0) as type $0C, walking inward at 3/4
or 5/4 pixel per frame at random (`TinsuitSpeeds`).  A Tinsuit from an
enemy list has its default speed doubled (`Tinsuit_Init`): 1 pixel per
frame.

**A Tinsuit** (`Tinsuit_Ai`, after `Enemy_SkipAiIfStunned`):

- Each time it lands on the floor it hops again: Y velocity -1 under
  gravity of 2/16, about 4 pixels high, every 16 frames.
- It turns around at walls.  It doesn't turn toward Link.

## Drawing

`Tinsuit_Draw`: two pairs of sprites, 1 pixel low, from `sramEnemyTiles`
entry $7A, or $7E after odd hops, so it steps as it hops.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `TinsuitGenerator_Ai` | 4:$9EC1 | the generator |
| `TinsuitSpeeds` | 4:$9EBC (+ facing, + 0 or 2) | $0C, $F4, $14, $EC |
| `Tinsuit_Init` | 4:$98B6 | a list's Tinsuit |
| `Tinsuit_Ai` | 4:$BCBC | hopping |
| `Tinsuit_Draw` | 4:$97CC | drawing |

## Verification

Scenario in palace 1's first room (see [Bubble](bubble.md#verification),
with Link's life level raised), a generator placed with Link standing at
X $1A0:

- Tinsuits were launched on frames 45, 173 and 301, at Y $B0.  The first
  came from the left edge at $14 (5/4 pixel per frame), the others from
  the right at $F4 (3/4) and $EC (5/4).
- Each hopped every 16 frames, up to 5 pixels, and pushed Link back when
  it reached him.
- Screenshot: a red and white figure, 32 pixels tall.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $PAT,statLevel+2=8,statContainers+1=8 \
    --until currentWorld=3 --after 200 --clear $PFLAT --approach 1A0 --spawn 0:0B:1D0:B0 \
    --event Tin=4:9ECE --frames 400
```
