# Great Palace Enemies

Bank 5's enemy group (`EnemyHp_Bank5` ... at $9400), for the Great
Palace.  Types $00-$13 are mostly the [common enemies](common.md) with
tougher stats; the Bago Bagos and Octoroks are "fire" versions with their
own colors (`FireOctorok_Draw`).

| Type | Enemy | HP | Exp | Damage | Drop | Notes | AI |
|------|-------|----|-----|--------|------|-------|----|
| $00 | [Fairy](enemies/fairy.md) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `Fairy_Ai` |
| $01 | [Item](enemies/item.md) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder | `Item_Fall` |
| $02 | [Door](enemies/locked-door.md) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `LockedDoor_Ai` |
| $03 | [Myu](enemies/myu.md) | 24 | 10 | 3 | 1 | comes back | `Myu_Ai` |
| $04 | [Bot](enemies/bot.md) | 48 | 10 | 3 | 1 | comes back | `Bot_Ai` |
| $05 | Hidden jar | 255 | 10 | 3 | 1 | blocks Link's projectiles, comes back | `HiddenJar_Ai_Bank5` |
| $06 | [Moa](enemies/moa.md) | 6 | 5 | 1 | 1 | touch drains 20 exp | `Moa_Ai` |
| $07 | [Ache](enemies/ache.md) | 4 | 3 | 0 | 1 | comes back | `Ache_Ai` |
| $08 | (unused) | 0 | 0 | 3 | 1 | immune to SPELL, immune to Thunder, sword passes through | `$AB98` |
| $09 | (unused) | 0 | 0 | 3 | 1 | touch drains 10 exp, immune to SPELL, immune to Thunder | `$ABE9` |
| $0A | [Acheman](enemies/ache.md) | 2 | 0 | 2 | 1 |  | `Ache_Ai` |
| $0B | [Bubble Generator](enemies/bubble-generator.md) | 2 | 0 | 0 | 1 | immune to SPELL, immune to Thunder, sword passes through | `BubbleGenerator_Ai` |
| $0C | [Rock Generator](enemies/rock-generator.md) | 2 | 0 | 0 | 1 | immune to Thunder, sword passes through | `RockGenerator_Ai` |
| $0D | [Red Deeler](enemies/deeler.md) | 4 | 3 | 0 | 1 |  | `Deeler_Ai` |
| $0E | [Blue Deeler](enemies/deeler.md) | 6 | 5 | 0 | 1 |  | `Deeler_Ai` |
| $0F | [Fire Bago Bago Generator](enemies/bago-bago.md) | 8 | 20 | 3 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `BagoBagoGenerator_Ai` |
| $10 | [Fire Bago Bago](enemies/bago-bago.md) | 8 | 20 | 3 | 1 | touch drains 10 exp | `BagoBago_Ai` |
| $11 | [Fire Hopping Octorok](enemies/octorok.md) | 12 | 20 | 4 | 1 |  | `OctorokHopping_Ai` |
| $12 | [Fire Walking Octorok](enemies/octorok.md) | 12 | 20 | 4 | 1 |  | `OctorokWalking_Ai` |
| $13 | [Elevator](enemies/elevator.md) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `Elevator_Ai` |
| $14 | Slow Bubble | 255 | 20 | 3 | - | immune to Thunder, comes back | `GpBubble_Ai` |
| $15 | Fast Bubble | 255 | 20 | 3 | - | immune to Thunder, comes back | `GpBubble_Ai` |
| $16 | Orange Skull Head | 96 | 20 | 3 | 1 | touch drains 10 exp, immune to Thunder, comes back | `GpSkullHead_Ai` |
| $17 | Large Bubble | 255 | 0 | 3 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, comes back | `GpBubble_Ai` |
| $18 | Orange Fokka | 0 | 70 | 5 | - | blocks Link's projectiles, immune to SPELL | `Fokka_Ai` |
| $19 | Red Fokka | 48 | 70 | 5 | - | blocks Link's projectiles, immune to SPELL | `Fokka_Ai` |
| $1A | Blue Fokka | 96 | 100 | 5 | - | blocks Link's projectiles, immune to SPELL | `Fokka_Ai` |
| $1B | (unused) | 0 | 0 | 0 | - | touch drains 10 exp | `$9E66` |
| $1C | (unused) | 2 | 0 | 0 | - | touch drains 10 exp | none (its AI pointer, $9E67, is a byte of `FokkeruSpeeds`) |
| $1D | Fokkeru | 144 | 200 | 4 | - | blocks Link's projectiles, immune to SPELL | `Fokkeru_Ai` |
| $1E | King Bot | 56 | 0 | 3 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder | `KingBot_Ai` |
| $1F | (unused) | 0 | 3 | 1 | - |  | `$0000` |
| $20 | Barrier | 0 | 3 | 6 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `Barrier_Ai` |
| $21 | (unused) | 0 | 3 | 1 | - |  | `$0000` |
| $22 | Thunderbird | 192 | 1000 | 5 | - | blocks Link's projectiles, immune to SPELL, Thunder weak | `Thunderbird_Ai` |
| $23 | Dark Link Trigger | 248 | 150 | 4 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder | `DarkLink_Ai` |

## Enemies

**Bubbles** ($14 slow, $15 fast, $17 large; `GpBubble_Ai`): like the
palace bubbles, they bounce diagonally and drain magic while touching
Link.  When a large bubble is hit, it splits into two fast bubbles
(255 hit points each).

**Skull head** ($16, `GpSkullHead_Ai`): like the palace skull heads, with
96 hit points.

**Fokkas** ($18 orange, $19 red, $1A blue; `Fokka_Ai`): bird knights
that fight like Iron Knuckles: they guard at the height Link attacks or
jumps, attack high or low, throw their sword (projectile type 5), and
jump at random (1 frame in 32 on the ground).

**Fokkeru** ($1D, `Fokkeru_Ai`): a bird that bounces about at random and
spits a fireball every 64 frames (projectile type 7), which rolls along
the floor and then shoots off to a side (`Projectile_RollingFlame_Bank5`).

**King Bot** ($1E, `KingBot_Ai`): a giant Bot that wakes when Link is
within 48 pixels and hops toward him.  When its hit points fall below 48,
it bursts into five Bots.

**Hidden jar** ($05, `HiddenJar_Ai_Bank5`): an invisible object;
striking it reveals a magic jar (or, half the time, a red Fokka; not
seen in the test movies).

**Barrier** ($20, `Barrier_Ai`): the barrier at the Great Palace's
entrance, flashing.  It hurts Link on contact, and kills him outright if
he is a fairy.  When all six crystals are placed (`crystalsRemaining` =
0) and Link walks up to it, it dissolves a column at a time.

## Bosses

- **Thunderbird** ($22, `Thunderbird_Ai`): appears when Link reaches the
  second page of its room (the scroll freezes).  It flies, bobbing,
  turning toward Link every 32 frames, and spits fireballs (projectile
  type 8, at random speeds) every 32 frames, every 16 once below 96 hit
  points.  With 192 hit points it cannot be hurt until the Thunder spell
  (50 points) brings it below 192.
- **Dark Link** ($23, `DarkLink_Ai`, `DarkLink_Init`): in the last room.
  When Link arrives the scroll freezes, the screen flashes and his shadow
  steps out of him (`DarkLink_Appear`, `DarkLink_Separate`).  The shadow
  is drawn with Link's own sprites (`DarkLink_Draw`) and fights like him
  (`DarkLink_Fight`): close to Link it reacts to his attacks, raising its
  shield to his height, backing off or jumping away; farther away it
  approaches.  Its sword hurts Link; his sword bounces off its shield; it
  takes 8 hits.  When it is defeated, Link walks to the middle of the
  room, the Triforce appears, and the ending begins (`gameState` 3,
  `DarkLink_StartEnding`).

## Projectiles

| Type | Routine | Thrown by |
|------|---------|-----------|
| 1, 2 | `Projectile_Straight_Bank5` | |
| 3 | `Projectile_Arc_Bank5` | Octoroks |
| 4, 8 | `Projectile_Fireball_Bank5` | Thunderbird (8) |
| 5 | `Projectile_Rock_Bank5` | Fokkas' swords |
| 7 | `Projectile_RollingFlame_Bank5` | Fokkeru |
