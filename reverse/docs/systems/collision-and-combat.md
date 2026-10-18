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

- Link: `Link_CheckBgInBank` (see [Link](link.md#walking)) sets
  `objCollision` from his probe points (offsets from his top left): head
  (+12, 0), (+20, 0) for bit 3; feet (+12, +32), (+20, +32) for bit 2;
  and in the direction he faces, a wall at the upper point (+23 or +9,
  +6) and the lower point (+23 or +9, +26) for bit 0 or 1.  The fairy
  has its own set.  The tile at (+15, +32), under him, can be special
  (`AreaBank::SpecialTiles`): lava sinks him (`LINK_MODE_SINK`, with
  his cry); swamp tiles slow him and put his feet behind the background
  once his Y is $A5 or more; a crumbling floor starts breaking; Up on a
  door, on the ground, enters it (gameMode $16); crouching on a chimney
  sinks him into it.  In a room that doesn't scroll, the screen edge he
  faces is a wall; in a scrolling one, walking his body off the screen
  leaves the area (gameMode $10), and falling below it falls into the
  area below (gameMode $0C).
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
projectile routines with `Projectile_CheckLinkAlternate`, on alternate
frames by slot: first the shield (`Link_GetShieldHitbox`, a small box in
front of Link, high standing or low crouching), which deflects or
destroys the projectile as its flags say; then his body, which hurts him.

Hand-held weapons (swords, spears, maces) are drawn as the enemy's fifth
sprite, and the enemy's AI tests them itself.  `Enemy_GetWeaponHitbox`
makes a line 14 pixels wide, 9 pixels below that sprite's top.

- `Weapon_CheckLink`: first Link's shield, which knocks the enemy back
  with a sound; then his body (`Weapon_CheckLinkBody`), which hurts him.
  A boss (guard 2 or more) also knocks him up and back.
- `Weapon_CheckLinkReflect`: the shield only counts with the reflect
  spell, or for enemy types below $17.

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
