# Skull Head

A blue flying skull that comes in from the side Link faces and swoops
through him in a wave, again and again.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Palaces 1, 2, 5](../palaces.md) | $0A | Blue Skull Head | 56 | 20 | 0 | - |
| [Palaces 3, 4, 6](../palaces.md) | $0F | Blue Skull Head | 56 | 20 | 0 | 1 |

It is placed in rooms $12, $26 and $3C of palaces 1, 2 and 5, and $16,
$22 and $31 of palaces 3, 4 and 6.  Touching it drains experience.  It
comes back after dying.  The two groups' routines differ only in their
numbers.

## Behavior

**Spawning** (`SkullHead_Init`, `SkullHead_Init_Bank4B`):

- about 32 pixels above Link (his Y - $20; - $24 in palaces 3, 4, 6);
- at the edge of the screen he faces (`SkullHeadSpawnX`);
- flying inward at 1 pixel per frame (1.5 in palaces 3, 4, 6;
  `SkullHeadSpeeds`), facing Link.

**Flying** (`SkullHead_Ai`, `SkullHead_Ai_Bank4B`), after
`Enemy_SkipAiIfStunned`:

- Its Y velocity changes by 2/16 per frame, down while `enemyAiState` is
  even and up while it is odd.  At 3 pixels per frame (2.875 in palaces
  3, 4, 6; `SkullHeadMaxYSpeeds`) the state goes up by 1.
- So it dives from its start, 72 pixels down, through Link, and climbs
  back, every 96 frames (about 66 pixels, 92 frames, in palaces 3, 4, 6).
- When it is partly off screen, it spawns again ahead of Link, unless the
  screen is held at an area end, when it waits there.

## Drawing

`SkullHead_Draw` (`SkullHead_Draw_Bank4B`): a pair of sprites from
`sramEnemyTiles` entry $82 ($C0), with its facing flipped (the tiles face
the other way).

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `SkullHead_Init`, `SkullHead_Init_Bank4B` | 4:$B9F4, 4:$ACAD | spawning |
| `SkullHead_Ai`, `SkullHead_Ai_Bank4B` | 4:$BA20, 4:$ACDA | flying |
| `SkullHead_Draw`, `SkullHead_Draw_Bank4B` | 4:$B833, 4:$ACA2 | drawing |

## Verification

Scenario in palace 1's first room (see [Bubble](bubble.md#verification),
with Link's life level raised), a skull head placed at ($1D0, $90), Link
standing at $1A0 facing right:

- It flew left 1/2 pixel per frame (the spawn default speed, as the
  scenario skips the spawn routine), diving from Y $90 to $D8 and back
  every 96 frames.  It hit Link as it passed through him.
- Off screen at X $143 on frame 288, it came again at the right edge,
  Y $90, flying left at 1 pixel per frame.
- Screenshot: a small blue winged skull.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $PAT,statLevel+2=8,statContainers+1=8 \
    --until currentWorld=3 --after 200 --clear $PFLAT --approach 1A0 --spawn 0:0A:1D0:90 --frames 300
```

In palace 3's entrance (see [Flame](flame.md#verification) for `$P3` and
`$F3`), a type $0F skull head placed at ($C0, $90) dove from Y $90 to
$D1 and back every 92 frames, flying left at 1/2 pixel per frame (the
spawn default).

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $P3,statLevel+2=8,statContainers+1=8 \
    --until currentWorld=4 --after 200 --clear $F3 --approach 80 --spawn 4:0F:C0:90 --frames 250
```
