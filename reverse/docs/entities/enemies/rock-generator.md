# Rock Generator

An invisible enemy that throws rocks across the screen, at about Link's
height, from the left and right edges.

## Where It Appears

| Group | Type | HP | Exp | Damage class | Drop group |
|-------|------|---:|----:|-------------:|-----------:|
| [West Hyrule](../west-hyrule.md) | $0C | 2 | 0 | 0 | - |
| [East Hyrule](../east-hyrule.md) | $0C | 2 | 0 | 0 | - |
| [Great Palace](../great-palace.md) | $0C | 2 | 0 | 0 | 1 |

Outdoors it is invisible (no draw routine), can't be hurt, and has the
generators' spawn routine (`Generator_Init_BankN`), which removes any other
generator of the same type.  The lists place it in the desert encounter
areas with Geldarms (West Hyrule $1D-$20 and Death Mountain $1D-$1E),
and in areas $1D-$1E of Maze Island.  No Great Palace area
places one.

## Behavior

`RockGenerator_Ai`, every 32 frames (`frameCounter` bits 4-0 all 0, so all
generators fire together), if one of projectile slots 3-0 is free
(`Projectile_FindFree`), throws a rock (projectile type 1).  A random r
(0-7, `rng+1,x`) picks the side, r & 1 (`RockSpawnX`), and the height
from Link's Y, `objY` (`RockSpawnY`):

| r | 0, 6 | 2 | 4 | 1, 7 | 3 | 5 |
|---|------|---|---|------|---|---|
| from | left | left | left | right | right | right |
| Y | +3 | -13 | -45 | +19 | -29 | -61 |

From the left it starts at `scrollX` + $0C and flies right; from the right
at `scrollX` + $F4 and flies left, at 1 pixel per frame (`RockSpeeds`).
`projectileTimer` is set to the side, but nothing reads it.

The rock (`Projectile_Straight_Bank1`, `Projectile_Straight_Bank2`) flies
straight through walls and is removed off screen.  It crosses the screen
in about 230 frames, so four slots allow only about one rock every 64
frames once the first four are out.  Its flags ($00) make Link's shield
destroy it.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `RockGenerator_Ai` | 7:$DC4F | the generator |
| `RockSpeeds` | 7:$DBF9 | $10 right, $F0 left |
| `RockSpawnX` | 7:$DC45 | $0C, $F4 from the screen's left edge |
| `RockSpawnY` | 7:$DC47 | heights from Link's Y, by r |
| `Projectile_Straight_Bank1` | 1:$9639 | the rock (also the Daira's axe) |

## Verification

Neither test movie has one.  Scenario (West Hyrule cave, area $2D; see the
[Myu](myu.md#verification) page for `$AT`), a rock generator placed with
Link at Y $80, 400 frames (`--projectiles`):

- The first four rocks came at frames 5, 37, 69 and 101 (`frameCounter`
  $80, $A0, $C0, $E0).  The next four came only as slots freed, at 165,
  229, 293 and 389.
- Rocks from the left flew at Y $73, $83 and $53, and from the right at
  $43 and $63 (Link's Y + -13, +3, -45, -61, -29), 1 pixel per frame.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT --until areaId=0x2D --after 200 \
    --clear --approach 2A0 --spawn 5:0C:2A0:80 --projectiles --event Rock=7:DC8B --frames 400
```
