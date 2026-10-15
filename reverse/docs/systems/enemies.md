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

## Spawning

`Enemy_Spawn` places an enemy from the area's list: X from the list's
column (a multiple of 16), Y from `EnemySpawnYPositions` ($30, $50, $60,
... $B0, by bits 6-4 of the list entry), `objYHi` 1, and the hit points
of its type.  It turns it toward Link (`Enemy_FaceLink`) and gives it X
velocity $08 or $F8 (1/2 pixel per frame) that way
(`EnemySpawnXVelocities`).  It clears `enemyAiState`, `objState`,
`objYVelocity`, `enemyStunTimer`, `enemyKnockBack`, `enemyVulnerability`
and `ram_04A0`, then runs the type's spawn routine (most types have none).
The enemy's timer (`timerFast+TIMER_ENEMY`) is not reset; by then it has
normally counted down to 0.

## Building Blocks

AI routines are built from a few helpers in bank 7.  Positions are in
pixels; velocities are signed 4.4 fixed point, so $10 is one pixel per
frame (see [object slots](../memory/objects.md#movement)).

| Helper | What it does |
|--------|--------------|
| `Enemy_SkipAiIfStunned` | While `enemyStunTimer` is nonzero, drops the AI's return address and runs only `Enemy_UpdateCommon`: the enemy freezes, and flashes while it is drawn. |
| `Enemy_MoveX` | X += `objXVelocity`. |
| `Enemy_MoveYAndFall` | Y += `objYVelocity`, then `objYVelocity` += 2: gravity of 1/8 pixel per frame per frame, with no limit.  Used by enemies that hop. |
| `Enemy_MoveYWithGravity` | Y += `objYVelocity` whole pixels, then adds `temp0`/256 to the velocity, up to `temp2` pixels per frame.  Here `objYVelocity` counts whole pixels. |
| `Enemy_Land` | Rounds Y down to a multiple of 16 and stops vertical movement (`Enemy_StopY`). |
| `Enemy_UpdateBgCollision` | Sets `objCollision` bits 3-0 from the background (see below). |
| `Enemy_UpdateCommon` | Draws the enemy, tests it against the sword, Link's projectiles and Link (setting `objCollision` bit 4 when it touches him), and despawns it if it fell below the screen or is far off it.  `Enemy_UpdateWithBg` does `Enemy_UpdateBgCollision` first. |
| `Enemy_HurtLinkOnContact` | Hurts Link if `objCollision` bit 4 is set. |
| `Enemy_FaceLink` | Sets `objFacing` to the side Link is on: 1 (right) when Link's X + 8 is at least the enemy's X + 1, else 2 (left); returns `y` = facing - 1, for indexing a (right, left) speed table.  The comparison is off by one depending on the carry ([quirks](../quirks.md)). |
| `Enemy_TurnAround` | Reverses `objFacing` and `objXVelocity`. |
| `Enemy_FireProjectile` | Takes a free projectile slot (searching 5 down to 0; carry set if none) and starts it at the enemy's position, moving horizontally in the enemy's facing at 2 pixels per frame (`ProjectileSpeedsByFacing`).  The caller sets its type in `objState+7`. |

Because `Enemy_UpdateCommon` runs partway through most AI routines, an
AI that tests `objCollision` before calling it sees the previous frame's
bits.

### Background Probes

`Enemy_UpdateBgCollision` tests points around the enemy against the
area's tile map (`Obj_GetBgTile`, `AreaBank::IsSolidTile`).  The type's
probe set *s* (bits 3-0 of `sramEnemyFlags`) picks the points from
`ProbeOffsetX`/`ProbeOffsetY`:

| Bit of `objCollision` | Meaning | Probe | Set 6 point (X, Y) |
|---|---|---|---|
| 3 | solid above | $15 + *s* | (+8, -31) |
| 2 | standing on a floor | $14 + *s* | (+8, -16) |
| 0 | wall ahead, moving right | $10 + *s*, then $12 + *s* | (+14, -27), (+14, -21) |
| 1 | wall ahead, moving left | $11 + *s*, then $13 + *s* | (+2, -27), (+2, -21) |

Only the side the enemy is moving toward (by the sign of
`objXVelocity`) is tested.  The floor probe at Y - 16 means that an
enemy resting on a floor has a Y that is a multiple of 16, which is
where `Enemy_Land` puts it; a falling enemy lands as soon as its Y
crosses into the next row of tiles.  Set 6 is used by the Myu, Bot,
Bit, Lowder and Octoroks, among others.

## Damage

A sword hit (`Enemy_TakeDamage`) takes `SwordDamage[attack level]` hit
points (2, 3, 4, 6, 9, 12, 18, 24 for levels 1-8) and stuns the enemy for
48 frames.  When its hit points run out, `Enemy_Kill` notes its
experience class and counts the kill in its item drop group; every 6th
kill in a group drops an item.  A downward thrust hit bounces Link up; an
upward thrust only hits enemies above him.

Touching an enemy hurts Link; see [Link](link.md#taking-damage).
