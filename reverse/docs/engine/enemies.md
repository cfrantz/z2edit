# Enemies

Up to six enemies (object slots 1-6) and six enemy projectiles (slots
7-12) are active at once; see [object slots](../memory/objects.md).  Each
enemy and its behavior is described in [enemies and objects](../entities/README.md).

## Per-Type Tables

Each world's enemy types are described by tables in its area bank, copied
into SRAM when the world is loaded (`Mode_LoadWorld`).  The engine in bank
7 reaches enemy behavior only through these tables:

| Table | Contents |
|-------|----------|
| `sramProjectilePtrs` | routine for each projectile type |
| `sramProjectileFlags` | projectile flags: reflectable, survives hitting Link, large hitbox |
| `sramEnemyHp` | hit points |
| `sramEnemyAiPtrs` | routine run every frame (the enemy's AI) |
| `sramEnemyInitPtrs` | routine run when it spawns |
| `sramEnemyAttr` | bits 7-6 sprite palette, bit 5 sword clinks, bit 4 drains experience, bits 3-0 experience class |
| `sramEnemyDamage` | bits 7-6 item drop group, bit 5 blocks Link's projectiles, bit 4 immune to SPELL, bits 3-0 damage class |
| `sramEnemyHitbox` | hitbox, an index into `EnemyHitboxes`; $FF for a boss |
| `sramEnemyFlags` | bit 7 immune to Thunder, bit 6 comes back after dying, bit 5 Thunder weak, bit 4 not hit by the sword, bits 3-0 background collision probe set |
| `sramEnemyDrawPtrs` | routine that draws it |
| `sramEnemyTiles` | sprite tiles |

## Each Frame

`Enemies_Update` (with the area bank mapped) runs, for each enemy:

- `Enemy_Update`: compute its screen position and run the handler for its
  state (`enemyActive`, `ENEMY_STATE_*`):
  - `ENEMY_STATE_EMPTY` (`Enemy_TrySpawn`): spawn the area's next enemy
    from its list in SRAM when the screen scrolls far enough;
  - `ENEMY_STATE_ACTIVE` (`Enemy_RunAi`): run its AI routine;
  - `ENEMY_STATE_DYING` (`Enemy_Die`): wait out the death timer;
  - `ENEMY_STATE_SHOW_EXP` (`Enemy_ShowExperience`): show the experience
    it was worth (`enemyExpClass`, `ExperienceTilesLeft/Right`).
- `Projectile_Update`: run its projectile's routine.

Sprite slots (`objOamOffset`) are handed out in a different rotation each
frame (`oamRotation`), so that when too many sprites share a line,
different ones flicker.

AI routines start with `Enemy_SkipAiIfStunned`, which skips the AI while
the enemy is stunned, and end with `Enemy_UpdateCommon`: draw it
(`Enemy_Draw`), test it against the sword, Link's projectiles and Link
(see [collision](../memory/objects.md#collision)), and despawn it when it
falls off the screen or is far from it (`Enemy_Despawn`; it can then
spawn again).

## Damage

A sword hit (`Enemy_TakeDamage`) takes `SwordDamage[attack level]` hit
points (2, 3, 4, 6, 9, 12, 18, 24 for levels 1-8) and stuns the enemy for
48 frames.  When its hit points run out, `Enemy_Kill` notes its
experience class and counts the kill in its item drop group; every 6th
kill in a group drops an item.  A downward thrust hit bounces Link up; an
upward thrust only hits enemies above him.

Touching an enemy hurts Link; see [Link](link.md#taking-damage).
