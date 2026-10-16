# Goriya

A [fighter](moblin.md#movement-fighter_move) that keeps 96 pixels from
Link and throws boomerangs, high or low, which fly out to about him and
come back.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [West Hyrule](../west-hyrule.md) | $19 | Orange Goriya | 12 | 20 | 1 | 1 |
| [West Hyrule](../west-hyrule.md) | $1A | Red Goriya | 14 | 20 | 1 | 1 |
| [West Hyrule](../west-hyrule.md) | $1B | Blue Goriya | 18 | 30 | 1 | 2 |

Only bank 1's group has them, in West Hyrule caves and encounter areas
($09, $0A, $0C, $10, $13, $18, $19, $21, $26, $38) and on Death Mountain.
They block the sword beam and fire (damage bit 5).  Spawn routine,
probe set and drawing height are as for the [Moblin](moblin.md).

## Behavior

`MoblinGoriya_Ai`, with `Fighter_Move` keeping r = $60 (96 pixels).
Beyond 96 pixels a Goriya walks toward Link and never attacks.  Within
96 pixels it holds and attacks every 128 frames (orange), or every 64
(red, blue), sometimes sooner (see
[Fighter_Move](moblin.md#movement-fighter_move)).  Since the distance is
taken from the low byte only, a Goriya 160 or more pixels away (up to
352) also counts as near.

**The throw.**  After the 24-frame wind-up (pose 1 low, 3 high) it
throws a boomerang (projectile type 6, `Enemy_FireProjectile`) and the
attack is over.  The boomerang flies 2 pixels per frame the way the
Goriya faces:

- **low**, from its Y + $10 (its feet), with Y velocity 0 or 1 pixel per
  frame down at random;
- **high**, from its Y (its head), 1 pixel per frame up.

It remembers its thrower (`projectileSpeed`), the thrower's direction
(`boomerangDir`) and the height (`boomerangHigh`).  A Goriya can have
several boomerangs out at once.

**The flight** (`Projectile_Boomerang_Bank1`), with a sound every other
frame:

1. **Out**, 32 frames (`projectileTimer` from $20): 64 pixels; its Y
   velocity shrinks toward 0 by 1/16 every other frame.
2. **Turn**: its X velocity changes by 1/16 pixel per frame back toward
   the Goriya (`BoomerangXAccel_Bank1`), and its Y velocity is 1/4 pixel
   per frame up after a low throw, down after a high one
   (`BoomerangYSpeeds_Bank1`).  It turns at 96 pixels from the Goriya,
   the distance it keeps from Link.  After 64 frames it is back at 2
   pixels per frame (`projectileTimer` $FF).
3. **Back**: straight at 2 pixels per frame, its Y velocity shrinking.
   From the turn on, it disappears when it touches its thrower, if that
   Goriya is still alive (`Hitbox_SetB`, `Hitbox_Overlap`); otherwise it
   flies on off the screen.

Hitting Link removes it.  Knocked back by his shield (flags $49:
deflected), it falls (`Projectile_FallIfDeflected_Bank1`).

## Drawing

`Goriya_Draw`: two pairs of sprites facing its `objFacing`, from
`sramEnemyTiles` entry $00 standing, alternating with $04 every 8 frames
while it moves, $08 in a low wind-up and $0C in a high one.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `MoblinGoriya_Ai` | 1:$9972 | the throw, then `Fighter_Move` |
| `MoblinThrowYOffsets` | 1:$9970 | boomerang Y: +$10 low, 0 high |
| `Projectile_Boomerang_Bank1` | 1:$9664 | the boomerang |
| `BoomerangYSpeeds_Bank1` | 1:$9662 | turning Y velocity: $FC low, $04 high |
| `BoomerangXAccel_Bank1` | 1:$965F | turning acceleration by direction (1: $FF, 2: $01) |
| `Goriya_Draw` | 1:$9C6F | drawing |

## Verification

Scenarios in the West Hyrule cave, set up as for the
[Moblin](moblin.md#verification); a red Goriya placed at X $310, 104
pixels from Link:

- It came within 96 pixels and attacked at once, then every 64 frames
  (or 32).  It threw on the 24th frame of each wind-up.
- A low boomerang left at Y $91 and flew 64 pixels in 32 frames.  It
  turned over 64 frames with Y velocity $FC, 96 pixels from the Goriya,
  where it struck Link.
- With Link crouching (Down held) every throw was high: from Y $81, 1
  pixel per frame up, turning with Y velocity $04.
- Placed 200 pixels from Link (with `--link 240:80`), the Goriya took him
  for near and threw.  A high boomerang went out 96 pixels, came back at
  2 pixels per frame, and vanished on reaching the Goriya, 126 frames
  after the throw.
- A Goriya off the right edge of the screen threw boomerangs that were
  removed at once, being off screen.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT --until areaId=0x2D --after 200 \
    --clear --approach 2A0 --fill 2E0:2EF:80 --fill 310:31F:80 --fill 340:34F:80 \
    --link 240:80 --spawn 0:1A:310:80 --poke enemyAiState=0xFF --projectiles \
    --event Throw=1:99C1 --event Caught=1:96E6 --frames 250
```
