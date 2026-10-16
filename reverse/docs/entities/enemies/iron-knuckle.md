# Iron Knuckle

An armored knight of the palaces.  It walks up to Link and trades blows,
following his strokes with its shield.  The blue one throws its sword.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Palaces 1, 2, 5](../palaces.md) | $18 | Orange Iron Knuckle | 16 | 50 | 2 | 2 |
| [Palaces 1, 2, 5](../palaces.md) | $19 | Red Iron Knuckle | 24 | 100 | 3 | 2 |
| [Palaces 1, 2, 5](../palaces.md) | $1A | Blue Iron Knuckle | 56 | 150 | 3 | - |
| [Palaces 3, 4, 6](../palaces.md) | $18 | Orange Iron Knuckle | 16 | 50 | 2 | 2 |
| [Palaces 3, 4, 6](../palaces.md) | $19 | Red Iron Knuckle | 24 | 100 | 3 | 2 |
| [Palaces 3, 4, 6](../palaces.md) | $1A | Blue Iron Knuckle | 56 | 150 | 3 | - |

The orange ones are in palaces 1, 2 and 5 only; red and blue in all six
palaces.  They block Link's projectiles and have no spawn routine.  Like
the overworld [fighters](moblin.md) they rest their Y on the floor's row
and are drawn 32 pixels tall from Y down.  The unhorsed Rebonack (palaces
3, 4, 6) runs the same routine as a blue Iron Knuckle, and palaces 3, 4
and 6 also hide Iron Knuckles in their type $05 (`HiddenJarOrIronknuckle_Ai`).

## Behavior

`Ironknuckle_Ai` draws it, plays out its attack, tests its shield
(`Enemy_CheckProjectileHitsSpecial`) and the usual hits, falls, and moves
by last frame's velocity (`Enemy_KnockBackOrWalk_Bank4`).
`ironknuckleKind` (its type, or $1A for the unhorsed Rebonack) indexes
its tables.

**Engaged.**  It is engaged while it is on screen and Link is level with
it or up to 41 pixels above it.  Then:

- It faces Link and walks toward him at 1 pixel per frame until within
  28 pixels (orange, red) or 96 (blue).
- There it holds: it moves at Link's X velocity while he walks, or else
  sways 1/2 pixel per frame (64 frames each way).  The speed is halved,
  always for the blue, and for the others unless Link is jumping.
- It attacks every 64 frames.  1 time in 16 it attacks again 32 frames
  later.  After a hit, it attacks 4-7 frames after its stun ends
  (`enemyAiDir`).

**Attacks.**  Each has 24 frames of wind-up (pose 1 high, 3 low), then
16 frames of thrust (pose 2 or 4).  The sword is tested against Link's
shield and body every frame of the thrust (`Projectile_CheckLink`).  The
blue Iron Knuckle throws its sword at the thrust instead (projectile type
5, 2 pixels per frame, from its Y or Y + 10) and is done.  The height:

- half the time at random;
- half the time by Link's stance: high if he stands, low if he crouches.
  That is where his shield is (high standing, low crouching).  The rule
  is the one the Moblins use to aim *away* from it, whose poses are the
  other way round ([quirks](../../quirks.md)).

**Its shield** (guard 0 low, 1 high, in `enemyAiVar`):

- When Link presses B, 1 time in 2 for the orange, 3 in 4 for the red and
  blue (`IronknuckleGuardMasks`), it goes to his stroke's height: high
  standing, low crouching.
- While Link isn't level with it, it is high if he is more than 10 pixels
  above, else low.

Strokes and projectiles that meet it are turned aside.

**Pacing**, when not engaged: it walks 1/2 pixel per frame for 80 frames,
stands 80, turns around, and so on (`Ironknuckle_PauseThenNext`).

## Drawing

`Ironknuckle_Draw`:

- **Body:** two pairs from `sramEnemyTiles` entry $00 (shield high) or
  $08 (low), 4 further on alternate walking periods.
- **Sword,** in an attack: a pair from `IronknuckleDrawTiles` ($10, $12,
  $14, $12 by pose), at its Y (high) or Y + 16 (low).  It is shifted by
  `IronknuckleDrawXOffsets`, doubled during the lunge, and drawn upside
  down in pose 4.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Ironknuckle_Ai` | 4:$9C8C | everything above |
| `Ironknuckle_PauseThenNext` | 4:$9E0F | pacing: 80 frames, next state |
| `Enemy_KnockBackOrWalk_Bank4` | 4:$9E17 | moving, or knockback |
| `IronknuckleGuardMasks`, `IronknuckleRangeOffsets`, `IronknuckleRanges` | 4:$9C65, 4:$9C6C, 4:$9C70 (+ kind) | shield chance, distance |
| `Ironknuckle_Draw` | 4:$9E45 | drawing |

## Verification

Scenario in palace 1's first room (see [Bubble](bubble.md#verification);
Link's life level raised to 8, as on the [Lizalfos](lizalfos.md) page), an
orange Iron Knuckle dropped at X $1D0 with Link standing at $1A0:

- It walked up to Link and attacked on frames 85, 190 and 298: pose 1
  then 2, 3 then 4, 1.  Each wind-up lasted 24 frames and each thrust
  16.
- Screenshots: in pose 1 its sword is raised at head height, in pose 3
  low.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $PAT,statLevel+2=8,statContainers+1=8 \
    --until currentWorld=3 --after 200 --clear $PFLAT --approach 1A0 --spawn 0:18:1D0:90 \
    --event Attack=4:9DE6 --frames 300
```
