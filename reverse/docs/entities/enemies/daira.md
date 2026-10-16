# Daira

An axe-wielding lizardman, a [fighter](moblin.md#movement-fighter_move).
The orange Daira swings as it comes; the red one keeps its distance and
throws its axe.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [West Hyrule](../west-hyrule.md) | $17 | Orange Daira | 18 | 70 | 2 | 2 |
| [West Hyrule](../west-hyrule.md) | $18 | Red Daira | 24 | 100 | 2 | 2 |

Only bank 1's group has them: West Hyrule areas $05 and $0F, and many
areas of Death Mountain.  They block the sword beam and fire (damage bit
5).  Spawn routine, probe set and drawing height are as for the
[Moblin](moblin.md).

## Behavior

`Daira_Ai` plays out a swing (`Daira_Attack`), then moves with
`Fighter_Move` keeping r = $1E (30 pixels, orange) or $37 (55, red):

- The **orange Daira** walks up to Link, attacking again and again from
  80 pixels on (as the red Moblin does).  Within 30 pixels it holds and
  attacks every 64 frames.
- The **red Daira** walks up to 55 pixels and never attacks on the way.
  Within 55 pixels it holds and attacks every 64 frames, or 32 after a
  quick one (1 time in 8).

**The swing.**  Dairas always attack low.  `Fighter_StartAttack` sets
pose 1, the wind-up, for 24 frames.  Then every 5 frames `Daira_Attack`
advances the pose:

- **Pose 2**, the swing: the axe is drawn at once and tested against
  Link's shield and body like a projectile
  (`Weapon_CheckLinkReflect`; types $17 and up have the body tested
  only, unless Link has the reflect spell).
- The red Daira also throws its axe then (projectile type 7,
  `Enemy_FireProjectile`): from its position, 2 pixels per frame straight
  ahead (`Projectile_Straight_Bank1`), passing through walls.
- The swing ends at pose `DairaAttackFrames`: 4 for the orange Daira
  (poses 2 and 3, 5 frames each), 3 for the red (pose 2 only).

The thrown axe's flags ($93) let it pass through Link's shield; with the
reflect spell the shield destroys it
([the definition tables](../README.md#the-definition-tables)).

## Drawing

`OrangeDaira_Draw` and `RedDaira_Draw` both run `Daira_DrawWeapon`, so
the two differ only in palette.  Each first stores $F6 or $F5 to ROM
address $95E6, a write that only reaches the MMC1 and does nothing
([quirks](../../quirks.md)).

- **Body:** two pairs of sprites from `sramEnemyTiles` entry $1A, or $16
  on alternate periods while it walks on the ground.  The periods are 8
  frames, or 4 when it moves faster than 1/2 pixel per frame.
- **Axe**, in a swing (pose p): a pair from `DairaAxeEntries[p-1]` ($10,
  $12, $14), at Y + `DairaAxeYOffsets` (0, 2, $11) and shifted by
  `DairaAxeXOffsets` ($0D, $F0, $FB facing left; $F3, $10, $05 facing
  right).  In the wind-up it is drawn mirrored and upside down, raised
  behind.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Daira_Ai` | 1:$9A15 | the swing, then `Fighter_Move` |
| `Daira_Attack` | 1:$9AD1 | poses 2-3, the throw |
| `DairaAttackFrames` | 1:$9AB8 (+ type) | last pose + 1: 4 orange, 3 red |
| `Daira_DrawWeapon` | 1:$9D6B | drawing (both) |

## Verification

Scenarios in the West Hyrule cave, set up as for the
[Moblin](moblin.md#verification); a Daira placed at X $320, Link at
$2A0:

- The orange Daira attacked first at frame 22 as it walked, 80 pixels
  from Link.  It swung 24 frames later and ended 10 frames after the
  swing (poses 2 and 3), then attacked again at once, until near.
- The red Daira walked to 55 pixels without attacking.  It attacked
  there, swung and threw its axe 24 frames later and ended 5 frames
  after.  The axe flew left at 2 pixels per frame at the Daira's Y.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT --until areaId=0x2D --after 200 \
    --clear --approach 2A0 --fill 2E0:2EF:80 --fill 310:31F:80 --fill 340:34F:80 \
    --spawn 0:18:320:80 --poke enemyAiState=0xFF --projectiles --event Attack=1:9AA4 \
    --event Swing=1:9AE9 --event Axe=1:9AFA --event Done=1:9B05 --frames 300
```
