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
| $05 | [Hidden jar](enemies/hidden-jar.md) | 255 | 10 | 3 | 1 | blocks Link's projectiles, comes back | `HiddenJar_Ai_Bank5` |
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
| $14 | [Slow Bubble](enemies/bubble.md) | 255 | 20 | 3 | - | immune to Thunder, comes back | `GpBubble_Ai` |
| $15 | [Fast Bubble](enemies/bubble.md) | 255 | 20 | 3 | - | immune to Thunder, comes back | `GpBubble_Ai` |
| $16 | [Orange Skull Head](enemies/skull-head.md) | 96 | 20 | 3 | 1 | touch drains 10 exp, immune to Thunder, comes back | `GpSkullHead_Ai` |
| $17 | [Large Bubble](enemies/bubble.md) | 255 | 0 | 3 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, comes back | `GpBubble_Ai` |
| $18 | [Orange Fokka](enemies/fokka.md) | 0 | 70 | 5 | - | blocks Link's projectiles, immune to SPELL | `Fokka_Ai` |
| $19 | [Red Fokka](enemies/fokka.md) | 48 | 70 | 5 | - | blocks Link's projectiles, immune to SPELL | `Fokka_Ai` |
| $1A | [Blue Fokka](enemies/fokka.md) | 96 | 100 | 5 | - | blocks Link's projectiles, immune to SPELL | `Fokka_Ai` |
| $1B | (unused) | 0 | 0 | 0 | - | touch drains 10 exp | `$9E66` |
| $1C | (unused) | 2 | 0 | 0 | - | touch drains 10 exp | none (its AI pointer, $9E67, is a byte of `FokkeruSpeeds`) |
| $1D | [Fokkeru](enemies/fokkeru.md) | 144 | 200 | 4 | - | blocks Link's projectiles, immune to SPELL | `Fokkeru_Ai` |
| $1E | [King Bot](enemies/king-bot.md) | 56 | 0 | 3 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder | `KingBot_Ai` |
| $1F | (unused) | 0 | 3 | 1 | - |  | `$0000` |
| $20 | [Barrier](enemies/barrier.md) | 0 | 3 | 6 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `Barrier_Ai` |
| $21 | (unused) | 0 | 3 | 1 | - |  | `$0000` |
| $22 | [Thunderbird](enemies/thunderbird.md) | 192 | 1000 | 5 | - | blocks Link's projectiles, immune to SPELL, Thunder does only 2 | `Thunderbird_Ai` |
| $23 | [Dark Link](enemies/dark-link.md) | 248 | 150 | 4 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder; `DarkLink_Init` gives it 8 hit points, and each hit takes 1 | `DarkLink_Ai` |

## Enemies

**[Bubbles](enemies/bubble.md)** ($14 slow, $15 fast, $17 large;
`GpBubble_Ai`): like the palace bubbles, they bounce diagonally and drain
magic while touching Link.  A large bubble brought below 192 hit points
splits into two fast bubbles.

**[Skull head](enemies/skull-head.md)** ($16, `GpSkullHead_Ai`): like
the palace skull heads, with 96 hit points and a deeper dive.

**[Fokkas](enemies/fokka.md)** ($18 orange, $19 red, $1A blue;
`Fokka_Ai`): bird knights built on the Iron Knuckle's code.  They leap
at Link at random, guard at the height of his strokes, and throw their
sword high or low every 64 frames (red) or 32 (blue).

**[Fokkeru](enemies/fokkeru.md)** ($1D, `Fokkeru_Ai`): a bird that hops
about at random and spits a fireball every 64 frames (projectile type 7).
The fireball arcs to the floor, then burns in place or rolls toward Link
(`Projectile_RollingFlame_Bank5`).

**[King Bot](enemies/king-bot.md)** ($1E, `KingBot_Ai`): a giant Bot,
hidden until Link comes near, that drops and hops after him.  Below 48
hit points it bursts into five Bots.

**[Hidden jar](enemies/hidden-jar.md)** ($05, `HiddenJar_Ai_Bank5`): an
invisible object; striking it reveals a red jar, or half the time a red
Fokka.

**[Barrier](enemies/barrier.md)** ($20, `Barrier_Ai`): the flashing wall
at the entrance.  It hurts Link on every contact, and kills a fairy Link.
When all six crystals are placed and Link walks up to it, it dissolves
from the bottom up.

## Bosses

- **[Thunderbird](enemies/thunderbird.md)** ($22, `Thunderbird_Ai`):
  - It appears when Link's screen reaches page 1 of room $35, and the
    scroll freezes.
  - It flies after Link, bobbing, and spits fireballs every 32 frames, or
    16 below 96 hit points.
  - With 192 hit points it has no hit tests at all, until the Thunder
    spell takes 2 points.  Its body never hurts Link.
- **[Dark Link](enemies/dark-link.md)** ($23, `DarkLink_Ai`,
  `DarkLink_Init`): in the last room.
  - An old man sits on a pillar.  When Link arrives the game freezes, the
    old man vanishes, and Link's shadow steps out of him.
  - The shadow fights with Link's own sprites: it takes his stance when
    he strikes, attacks where his shield isn't, backs off and jumps.
  - It takes 8 hits, whatever the attack level.  Then it sinks through
    the floor, Link walks to the middle of the room, and the ending
    begins (`gameState` 3).

## Projectiles

| Type | Routine | Thrown by |
|------|---------|-----------|
| 1, 2 | `Projectile_MoveAndCheck_Bank5` | |
| 3 | `Projectile_Straight_Bank5` | Octoroks |
| 4, 8 | `Projectile_Fireball_Bank5` | Thunderbird (8) |
| 5 | `Projectile_Rock_Bank5` | [Fokkas](enemies/fokka.md)' swords |
| 7 | `Projectile_RollingFlame_Bank5` | [Fokkeru](enemies/fokkeru.md) |
