# Stalfos

A skeleton knight that drops from the ceiling as it comes into view,
walks up to Link and slashes.  The blue one also leaps at him.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Palaces 1, 2, 5](../palaces.md) | $1F | Red Stalfos | 8 | 30 | 1 | 2 |
| [Palaces 1, 2, 5](../palaces.md) | $23 | Blue Stalfos | 12 | 50 | 1 | 2 |
| [Palaces 3, 4, 6](../palaces.md) | $1F | Red Stalfos Knight | 24 | 50 | 2 | 2 |
| [Palaces 3, 4, 6](../palaces.md) | $23 | Blue Stalfos Knight | 32 | 70 | 3 | 2 |

They appear in rooms of all six palaces.  They block Link's projectiles
and have no spawn routine.

## Behavior

`Stalfos_Ai`:

1. **Waiting above.**  Until it first comes on screen it waits at Y $40,
   not moving.  Then it is active for good and drops.
2. **Shield.**  Its guard is always 1 (`enemyAiVar`), a shield edge at
   head height.  Link's projectiles and strokes that meet it are turned
   aside; low strokes get through.
3. **Upward thrust.**  While Link does the upward thrust (`objState` 8),
   it removes his sword for the frame (`swordY` $F8).  The thrust can't
   hurt a Stalfos, nor any enemy handled after it that frame
   ([quirks](../../quirks.md)).
4. It falls (gravity 2/16), faces Link every frame, and is tested for
   hits.  A ceiling stops a jump.
5. **On the ground**, 30 pixels or more from Link, it walks toward him.
   The speeds are `StalfosSpeeds`: $0A (0.625 pixel per frame) to the
   right but $F2 (0.875) to the left ([quirks](../../quirks.md)).
6. **Near Link** (within 30), whenever it is idle, it picks at random:
   - a quarter of the time, the blue Stalfos leaps at him: Y velocity
     -$33, walking his way;
   - a quarter of the time it waits 15 frames;
   - otherwise it swings (`Enemy_StartSwing`): 15 frames of wind-up
     (pose 1), then the slash (pose 2) for 15 frames, its sword tested
     against Link once as it comes out (`Weapon_CheckLink`), then 15
     frames of rest.
7. In the air it keeps moving by its X velocity unless at a wall.

## Drawing

`Stalfos_Draw` (which also turns it toward Link): two pairs from
`sramEnemyTiles`:

- **In the air:** $34 rising.  Falling, $38 until it has first landed
  (`enemyLanded`), drawn front-on with its halves mirrored to match; after
  that $38, or $30 when 48 pixels or more from Link.
- **On the ground:** $2C, alternating with $30 every 8 frames; $42/$46
  in the wind-up; $3C slashing.  The slash adds its sword, a pair from
  entry $40, 16 pixels to the side it faces.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Stalfos_Ai` | 4:$965A | everything above |
| `StalfosSpeeds` | 4:$9657 | walking: $0A right, $F2 left |
| `Enemy_StartSwing` | 4:$BC23 | next pose, 15 frames |
| `Stalfos_Draw` | 4:$BDED | drawing |

## Verification

Scenarios in palace 1's first room (see [Bubble](bubble.md#verification),
with Link's life level raised), Link at X $1A0:

- A red Stalfos placed at ($1F0, $40) fell, landed at Y $B0 within 30
  pixels of Link, and swung: wind-up, slash, rest, every 45 frames or
  so.
- A blue Stalfos at $220, off screen, waited at Y $40 until it was in
  view, then dropped.  It walked left at $F2, 5 pixels per 6 frames.  A
  red one at $140 walked right at $0A, 4 pixels per 6 frames.
- The blue Stalfos leapt at Link 6 times in 1500 frames.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $PAT,statLevel+2=8,statContainers+1=8 \
    --until currentWorld=3 --after 200 --clear $PFLAT --approach 1A0 --spawn 2:23:220:40 \
    --spawn 3:1F:140:40 --event Jump=4:971B --frames 1500
```
