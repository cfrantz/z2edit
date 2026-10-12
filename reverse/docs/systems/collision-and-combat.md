# Collision and Combat

An overview of how things touch in the side view, with links to the
pages that describe each part.

## Against the Background

Objects test the area's tile map at **probe points**, offsets from their
position (`Obj_GetBgTile`, with `ProbeOffsetX` and `ProbeOffsetY`).  Points
beyond the map or below the playfield read as tile $40.  A tile is solid
if it is at or above its palette group's threshold
(`AreaBank::IsSolidTile`; see [Side view areas](../world/areas.md), and
[Quirks](../quirks.md) for group 0 in bank 2).

- Link: `Link_CheckBgInBank` (walls, floors, ceilings; see
  [Link](link.md#walking)).
- Enemy projectiles: `Projectile_HitsBg` and its variants.
- The sword tip breaks blocks: `Sword_CheckBlocksInBank` (object slot 16,
  see [Object slots](../memory/objects.md)).

## Between Objects

All object-object tests are box overlaps (`Hitbox_Overlap`); see
[Object slots](../memory/objects.md#collision) for how the boxes are
set up.  Each frame, for every enemy, `Enemy_UpdateCommon` tests:

| Test | Routine | Result |
|------|---------|--------|
| The sword against the enemy | `Enemy_CheckSwordHit` | `Enemy_TakeDamage` ([Enemies](enemies.md#damage)) |
| Link's projectiles (beam, fire; slots 13-14) | `Enemy_CheckProjectileHits` | damage, and the beam turns into a spark |
| Link's body against the enemy | `Enemy_CheckLinkContact` (every other frame outside towns) | `Link_HurtByEnemy` ([Link](link.md#taking-damage)) |

Enemy projectiles (slots 7-12) are tested against Link by the area banks'
projectile routines:

- `Projectile_CheckLink`: first the shield (`Link_GetShieldHitbox`, a
  small box in front of Link, high standing or low crouching), which
  knocks the projectile back with a sound; then his body
  (`Projectile_CheckLinkBody`), which hurts him.
- `Projectile_CheckLinkReflect`: the shield only counts with the reflect
  spell, or for enemy types below $17.
- `Projectile_CheckLinkAlternate`: the same, on alternate frames by slot.

Each area bank's projectile flags (`ProjectileFlags_Bank1` ...) say how the
shield treats each projectile; see the [enemy pages](../entities/README.md).

## Damage and Its Effects

- To enemies: `SwordDamage` by attack level; hit enemies are stunned
  ([Enemies](enemies.md#damage)).  A killed enemy gives
  experience, or every 6th kill of a drop group an item instead
  ([Items and drops](items.md#drops)).
- To Link: `LinkDamage` by the enemy's damage class and Link's life
  level, halved by the shield spell; knockback and 80 frames of
  invulnerability ([Link](link.md#taking-damage)).  Running out of life
  starts the [death sequence](death.md).
