# Skull Head

A flying skull that comes in from the side Link faces and swoops
through him in a wave, again and again: blue in the palaces, orange in
the Great Palace.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Palaces 1, 2, 5](../palaces.md) | $0A | Blue Skull Head | 56 | 20 | 0 | - |
| [Palaces 3, 4, 6](../palaces.md) | $0F | Blue Skull Head | 56 | 20 | 0 | 1 |
| [Great Palace](../great-palace.md) | $16 | Orange Skull Head | 96 | 20 | 3 | 1 |

It is placed in rooms $12, $26 and $3C of palaces 1, 2 and 5, and $16,
$22 and $31 of palaces 3, 4 and 6, and in pairs in the Great Palace's
rooms $05, $0C, $13, $17, $24, $27, $28 and $2D.  Touching it drains
experience.  It comes back after dying.  The three groups' routines
differ only in their numbers, except that the Great Palace's doesn't wait
at an area end (below).  The Great Palace's is also immune to Thunder.

## Behavior

**Spawning** (`SkullHead_Init`, `SkullHead_Init_Bank4B`,
`GpSkullHead_Init`):

- about 32 pixels above Link (his Y - $20; - $24 in palaces 3, 4, 6; -
  $30 in the Great Palace);
- at the edge of the screen he faces (`SkullHeadSpawnX`);
- flying inward at 1 pixel per frame (1.5 in palaces 3, 4, 6 and the
  Great Palace; `SkullHeadSpeeds`), facing Link.

**Flying** (`SkullHead_Ai`, `SkullHead_Ai_Bank4B`, `GpSkullHead_Ai`),
after `Enemy_SkipAiIfStunned`:

- Its Y velocity changes by 2/16 per frame, down while `enemyAiState` is
  even and up while it is odd.  At 3 pixels per frame (2.875 in palaces
  3, 4, 6; `SkullHeadMaxYSpeeds`) the state goes up by 1.
- So it dives from its start, 72 pixels down, through Link, and climbs
  back, every 96 frames (about 66 pixels, 92 frames, in palaces 3, 4, 6).
- The Great Palace's turns at 3.5 pixels per frame
  (`GpSkullHeadMaxYSpeeds`): it dives about 97 pixels, well below Link,
  every 112 frames.
- When it is partly off screen, it spawns again ahead of Link, unless the
  screen is held at an area end, when it waits there.  The Great
  Palace's doesn't test `scrollHeld`: it always spawns again at once.

## Drawing

`SkullHead_Draw` (`SkullHead_Draw_Bank4B`): a pair of sprites from
`sramEnemyTiles` entry $82 ($C0), with its facing flipped (the tiles face
the other way).  `GpSkullHead_Draw` is the same with entry $4A.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `SkullHead_Init`, `SkullHead_Init_Bank4B` | 4:$B9F4, 4:$ACAD | spawning |
| `SkullHead_Ai`, `SkullHead_Ai_Bank4B` | 4:$BA20, 4:$ACDA | flying |
| `SkullHead_Draw`, `SkullHead_Draw_Bank4B` | 4:$B833, 4:$ACA2 | drawing |
| `GpSkullHead_Init`, `GpSkullHead_Ai`, `GpSkullHead_Draw` | 5:$963B, 5:$9667, 5:$9695 | Great Palace |

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

In the Great Palace's entrance (see
[Bubble](bubble.md#great-palace) for `$GP` and `$GF`), an orange skull
head placed at ($C0, $80), Link standing at $B1 facing right:

- It dove from Y $80 to $E1 and back every 112 frames, flying left at 1/2
  pixel per frame (the spawn default).
- Partly off the left edge at X $49 on frame 237, it came again at the
  right edge, X $139, Y $7F (Link's Y - $31: the carry was clear), flying
  left at 1.5 pixels per frame.
- Screenshot: an orange winged skull.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $GP --until currentWorld=5 --after 200 \
    --clear $GF --approach B0 --spawn 1:16:C0:80 --log 1 --frames 260
```

