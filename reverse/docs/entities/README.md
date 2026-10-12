# Enemies and Objects

Every world has its own set of up to 36 enemy types, defined by tables in
its area bank and copied to SRAM when the world is loaded
(`Mode_LoadWorld`); see [enemies](../systems/enemies.md) for how the
engine runs them.  The same type number means different enemies in
different groups.

| Group | Bank | Tables | Page |
|-------|------|--------|------|
| West Hyrule | 1 | `EnemyHp_Bank1` ... ($9400) | [West Hyrule](west-hyrule.md) |
| East Hyrule (and Death Mountain, Maze Island) | 2 | `EnemyHp_Bank2` ... ($9400) | [East Hyrule](east-hyrule.md) |
| Towns | 3 | `EnemyHp_Bank3` ... ($9400) | [Towns](towns.md) |
| Palaces 1, 2 and 5 | 4 | `EnemyHp_Bank4` ... ($9400) | [Palaces](palaces.md) |
| Palaces 3, 4 and 6 | 4 | `EnemyHp_Bank4B` ... ($A900) | [Palaces](palaces.md) |
| Great Palace | 5 | `EnemyHp_Bank5` ... ($9400) | [Great Palace](great-palace.md) |

Types $00-$13 are mostly shared by the outdoor groups and run routines in
bank 7: see [common enemies](common.md).  Names follow z2edit's config
(`config/vanilla/bankN.json5`).

## The Definition Tables

Each group's block (copied to `sramProjectilePtrs` and following, $6D00):

| Table | SRAM | Per | Contents |
|-------|------|-----|----------|
| `ProjectilePtrs_BankN` | `sramProjectilePtrs` | projectile type 1-11 | routine run each frame |
| `ProjectileFlags_BankN` | `sramProjectileFlags` | projectile type (indexed from 1) | bits 7-6, against Link's shield: %00 destroyed, %01 deflected, %10 passes through it (destroyed by it with the reflect spell), %11 destroyed (sent back with the reflect spell); bit 5: it survives hitting Link; bit 4: makes a different sound on the shield; bit 3: tall hitbox |
| `EnemyHp_BankN` | `sramEnemyHp` | enemy type | hit points |
| `EnemyInitPtrs_BankN` | `sramEnemyInitPtrs` | enemy type | routine run when it spawns (`Enemy_Spawn`) |
| `EnemyAiPtrs_BankN` | `sramEnemyAiPtrs` | enemy type | routine run every frame (`Enemy_RunAi`) |
| `EnemyAttr_BankN` | `sramEnemyAttr` | enemy type | bits 7-6: sprite palette; bit 5: the sword clinks off it and its beam is blocked (fire still works); bit 4: touching it drains experience; bits 3-0: experience class |
| `EnemyDamage_BankN` | `sramEnemyDamage` | enemy type | bits 7-6: item drop group; bit 5: Link's projectiles (beam and fire) are blocked; bit 4: immune to the SPELL spell; bits 3-0: damage class |
| `EnemyHitbox_BankN` | `sramEnemyHitbox` | enemy type | hitbox (`EnemyHitboxes`, bits 5-0); bit 7: a tall enemy, which dies as two rows of sprites; $FF: a boss, which drops a key (item kind 8) in the middle of the screen when it dies |
| `EnemyFlags_BankN` | `sramEnemyFlags` | enemy type | bit 7: immune to Thunder; bit 6: comes back after dying; bit 5: Thunder does only level 1 damage; bit 4: the sword passes through it; bits 3-0: background collision probe set |
| `EnemyDrawPtrs_BankN` | `sramEnemyDrawPtrs` | enemy type | routine that draws it |

After them come projectile tiles and draw routines (`sramProjectileTiles`,
`sramProjectileAttrs`, `sramProjectileDrawPtrs`) and the enemies' sprite tiles
(`sramEnemyTiles`).

The tables in each page below are generated from the ROM by
`tools/enemy_table.py --doc`.

## Experience

Killing an enemy gives the experience of its class (`Enemy_AwardExperience`):

| Class | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 |
|-------|---|---|---|---|---|---|---|---|---|---|----|----|----|----|----|----|
| Exp | 0 | 2 | 3 | 5 | 10 | 20 | 30 | 50 | 70 | 100 | 150 | 200 | 301 | 500 | 700 | 1000 |

Touching an enemy with attribute bit 4 takes 10 experience from Link, a
point per frame (20 for enemy type $06), besides the damage.

## Damage

Touching an enemy costs Link `LinkDamage[class][life level - 1]` (meter
units, 32 per block; halved by the shield spell), by its damage class:

| Class | Life 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|-------|-------:|--:|--:|--:|--:|--:|--:|--:|
| 0 | 16 | 12 | 12 | 12 | 8 | 4 | 4 | 4 |
| 1 | 32 | 28 | 20 | 16 | 12 | 12 | 8 | 8 |
| 2 | 48 | 40 | 36 | 32 | 24 | 20 | 16 | 12 |
| 3 | 96 | 72 | 56 | 48 | 40 | 32 | 28 | 24 |
| 4 | 144 | 120 | 96 | 72 | 56 | 48 | 40 | 32 |
| 5 | 224 | 160 | 128 | 112 | 96 | 80 | 64 | 48 |
| 6 | 224 | 192 | 160 | 144 | 128 | 112 | 96 | 80 |
| 7 | 255 | 255 | 255 | 255 | 255 | 255 | 255 | 255 |

Projectiles use their own damage (see each page).

## Item Drops

Enemies in drop groups 1-3 count their kills per group (`Enemy_Kill`,
`ram_05DE`); every 6th kill in a group leaves an item instead of the
experience: in groups 1 and 3 a blue magic jar (7 times in 8) or a
50-point experience bag; in group 2 a red magic jar or a 200-point bag
(about half each).  Items are objects (enemy type $01 with the item kind
in `enemyAiState`), falling to the ground (`Item_Fall`); see
`Item_Collect`.

## Hitting Enemies

A sword hit takes `SwordDamage[attack level]` hit points
(`Enemy_TakeDamage`) and stuns the enemy for 48 frames; most AI routines
do nothing while it is stunned (`Enemy_SkipAiIfStunned`).  The sword beam
and the fire spell use the same damage unless the enemy blocks them.
Thunder hits every enemy on the screen for 50 hit points
(`ThunderDamage`), or 2 (flags bit 5), unless it is immune (flags bit 7).  The SPELL spell
turns enemies into Bots outside towns, unless they are immune (damage
bit 4).
