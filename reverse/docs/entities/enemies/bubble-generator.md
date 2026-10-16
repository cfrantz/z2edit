# Bubble Generator

An invisible enemy that sends projectiles up from the bottom of the
screen every 32 frames.  In West Hyrule they are bubbles rising from the
water by a bridge; in East Hyrule they are lava fireballs.

## Where It Appears

| Group | Type | HP | Exp | Damage class | Drop group |
|-------|------|---:|----:|-------------:|-----------:|
| [West Hyrule](../west-hyrule.md) | $0B | 2 | 0 | 0 | - |
| [East Hyrule](../east-hyrule.md) | $0B | 2 | 0 | 0 | - |
| [Great Palace](../great-palace.md) | $0B | 2 | 0 | 0 | 1 |

Outdoors it is invisible (no draw routine), can't be hurt, and has the
generators' spawn routine (`Generator_Init_BankN`), which removes any other
generator of the same type.  The area lists place one at each end of an
area: West Hyrule's areas $02, $05 and $06 (bridges) and East Hyrule's
area $06.  No Great Palace area places one.

## Behavior

`BubbleGenerator_Ai` counts frames in its `enemyAiState`.  Every 32
frames, if one of projectile slots 3-0 is free (`Projectile_FindFree`),
it launches projectile type 2:

- at Y $E0, at the bottom of the screen;
- at the left edge of a random 16-pixel column of the screen (`scrollX`
  + `rng+1,x`, rounded down to a multiple of 16);
- rising at 1.75 pixels per frame (Y velocity -$1C).

The projectile, `Projectile_Bubble_Bank1` (West) or
`Projectile_Bubble_Bank2` (East), keeps rising at that speed and sways:
1/4 pixel per frame right, then left, 16 frames each way, so about 4
pixels across.  Its sway counter (`projectileTimer`) isn't reset for a
new projectile, so each one starts its sway at a different point.  It is
removed when it has risen past the top of the screen and its Y wraps
around to $F0, 137 frames after launch, so up to four are up at a time.
In West Hyrule it is drawn by `Projectile_DrawFlash_Bank1`, which sets
its attributes from `frameCounter`.

Projectile type 2 has flags $00 in both banks
([the definition tables](../README.md#the-definition-tables)), so Link's
shield destroys it.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `BubbleGenerator_Ai` | 7:$DC15 | the generator |
| `Projectile_FindFree` | 7:$DBFB | a free slot of 3-0 |
| `Projectile_Bubble_Bank1` | 1:$9647 | the bubble |
| `Projectile_Bubble_Bank2` | 2:$9637 | the lava fireball |

## Verification

Scenario in West Hyrule's area $02 (location $14, a bridge over water),
with the area's own generators (`--projectiles` logs each projectile):

- A bubble was launched every 32 frames.  It rose from Y $DE at 1.75
  pixels per frame, swaying between X $39F and $3A3 with its velocity
  switching every 16 frames.
- It rose past the top and was removed at Y $F0, 137 frames after
  launch.  The next one's sway counter carried on from $8B.

A screenshot showed the bubbles as rings rising past the bridge.

```
AT=areaPrgBank=1,currentRegion=0,currentWorld=0,townCode=0,palaceCode=0,areaLocation=0x14,areaId=0x02,areaStartPage=0,areaEntrySide=0,prevRegion=0
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT --until areaId=0x02 --after 60 \
    --log 3 --projectiles --frames 200
```
