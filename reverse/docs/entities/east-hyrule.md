# East Hyrule Enemies

Bank 2's enemy group (`EnemyHp_Bank2` ... at $9400), used in East
Hyrule and Maze Island (the second group of bank 2's areas; Death
Mountain is bank 1's).  Types $00-$13 are the
[common enemies](common.md), with tougher stats than in West Hyrule.

The names in the table are z2edit's.  The test movies fight few of these
enemies; types $14, $15 and $17-$19 were matched to the screen by
starting a game in an area that holds them (`tools/play.py --at`, with
the candle for the caves) and logging `enemyType`:

| Type | Area (location) | On screen |
|------|-----------------|-----------|
| $14 | cave $09 ($0D) | a cyan, long-legged hopping creature that spits small shots |
| $15 | location $00's area | a round red flying eye |
| $17 | $33 ($15) | a flying insect dropping rocks |
| $18 | cave $08 ($0C, page 3) | a cyan crested lizard |
| $19 | lava cave $3D ($16) | a cyan scorpion, half sunk in the rock |

| Type | Enemy | HP | Exp | Damage | Drop | Notes | AI |
|------|-------|----|-----|--------|------|-------|----|
| $00 | Fairy | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `Fairy_Ai` |
| $01 | Red Jar | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder | `Item_Fall` |
| $02 | Door | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `LockedDoor_Ai` |
| $03 | [Myu](enemies/myu.md) | 3 | 5 | 1 | 1 | comes back | `Myu_Ai` |
| $04 | [Bot](enemies/bot.md) | 4 | 2 | 1 | 1 | comes back | `Bot_Ai` |
| $05 | [Bit](enemies/bit.md) | 4 | 2 | 0 | 1 | comes back | `Bit_Ai` |
| $06 | [Moa](enemies/moa.md) | 48 | 50 | 3 | 1 | touch drains 20 exp | `Moa_Ai` |
| $07 | [Ache](enemies/ache.md) | 8 | 10 | 2 | 1 | comes back | `Ache_Ai` |
| $08 | (unused) | 0 | 0 | 1 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `$AB98` |
| $09 | (unused) | 0 | 0 | 1 | - | touch drains 10 exp, blocks Link's projectiles, immune to SPELL, immune to Thunder | `$ABE9` |
| $0A | [Acheman](enemies/ache.md) | 8 | 10 | 2 | 1 | comes back | `Ache_Ai` |
| $0B | [Bubble Generator](enemies/bubble-generator.md) | 2 | 0 | 0 | - | touch drains 10 exp, blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `BubbleGenerator_Ai` |
| $0C | [Rock Generator](enemies/rock-generator.md) | 2 | 0 | 0 | - | touch drains 10 exp, blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `RockGenerator_Ai` |
| $0D | [Red Deeler](enemies/deeler.md) | 12 | 5 | 2 | 1 |  | `Deeler_Ai` |
| $0E | [Blue Deeler](enemies/deeler.md) | 12 | 10 | 2 | 1 |  | `Deeler_Ai` |
| $0F | [Bago Bago Generator](enemies/bago-bago.md) | 8 | 0 | 2 | - | touch drains 10 exp, blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `BagoBagoGenerator_Ai` |
| $10 | [Bago Bago](enemies/bago-bago.md) | 8 | 5 | 2 | 1 | touch drains 10 exp | `BagoBago_Ai` |
| $11 | [Blue Octorok (jumping)](enemies/octorok.md) | 12 | 20 | 1 | 1 |  | `OctorokHopping_Ai` |
| $12 | [Blue Octorok (moving)](enemies/octorok.md) | 12 | 20 | 1 | 1 |  | `OctorokWalking_Ai` |
| $13 | Elevator | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `Elevator_Ai` |
| $14 | [Tektite](enemies/tektite.md) | 24 | 50 | 2 | 2 | sword clinks, beam blocked | `Tektite_Ai` |
| $15 | [Eye](enemies/eye.md) | 16 | 20 | 4 | 1 |  | `Eye_Ai` |
| $16 | [Leever](enemies/leever.md) | 16 | 10 | 4 | 1 |  | `Leever_Ai` |
| $17 | [Boon](enemies/boon.md) | 8 | 50 | 1 | 2 |  | `Boon_Ai` |
| $18 | [Basilisk](enemies/basilisk.md) | 48 | 50 | 3 | 2 | sword clinks, beam blocked | `Basilisk_Ai` |
| $19 | [Scorpion](enemies/scorpion.md) | 32 | 150 | 4 | 2 | blocks Link's projectiles | `Scorpion_Ai` |
| $1A | Red Lizalfos | 48 | 100 | 4 | - | blocks Link's projectiles, immune to SPELL | `Lizalfos_Ai` |
| $1B | Orange Lizalfos | 32 | 150 | 3 | 2 | blocks Link's projectiles, immune to SPELL | `OrangeLizalfos_Ai` |
| $1C | Blue Lizalfos | 56 | 200 | 4 | - | blocks Link's projectiles, immune to SPELL | `Lizalfos_Ai` |
| $1D | Fence Lizalfos | 1 | 50 | 2 | - | blocks Link's projectiles | `RockThrower_Ai` |

## Enemies

**Tektite** ($14): crouches and stretches, leaps over Link again and
again (1 pixel per frame across, about 50 pixels high), and every 64
frames, half the time, fires a straight shot the way it faces.  See
[Tektite](enemies/tektite.md).

**Eye** ($15): drifts across at 1/2 pixel per frame, bobbing, turning
toward Link every 64 frames, and now and then stops for 63 frames with
its eye open; only then can it be hurt.  See [Eye](enemies/eye.md).

**Leever** ($16): comes up out of the sand at a random spot, slides
toward Link at 1 pixel per frame for 48-127 frames and sinks back, again
and again.  See [Leever](enemies/leever.md).

**Boon** ($17): darts about at random, 2 pixels per frame in one of four
directions, and drops rocks every 32 frames while high on the screen.
See [Boon](enemies/boon.md).

**Basilisk** ($18): crawls back and forth on a 256-frame pace, rears up
for 80 frames every 256, and breathes fireballs, low while crawling and
high while reared.  See [Basilisk](enemies/basilisk.md).

**Scorpion** ($19): paces like the Basilisk, stings with fireballs, and
guards its body against low strokes and projectiles; it can only be hurt
in the 32 frames when it lowers its guard.  See
[Scorpion](enemies/scorpion.md).

**Lizalfos** ($1A red, $1B orange, $1C blue; `Lizalfos_Ai`,
`OrangeLizalfos_Ai`, `Lizalfos_Attack`): fight like Iron Knuckles: they
keep their distance, guard at the height Link attacks or jumps, and
attack high or low.  The blue Lizalfos throws its weapon (projectile type
7); the orange one thrusts twice.  They block the sword beam and fire.

**$1D** (z2edit: Fence Lizalfos; `RockThrower_Ai`): with a single hit
point, it pops up ahead of where Link is going, throws a rock in an arc
aimed at him (projectile type 8), and sinks back.

## Projectiles

| Type | Routine | Behavior |
|------|---------|----------|
| 1, 5, 6, 7 | `Projectile_Straight_Bank2` | flies straight |
| 2 | `Projectile_Bubble_Bank2` | lava fireball, rising and swaying |
| 3 | `Projectile_Rock_Bank2` | rock; falls once deflected |
| 4 | `Projectile_Fireball_Bank2` | fireball, with a sound |
| 8 | `Projectile_FallingRock_Bank2` | dropped or thrown rock, falling until it hits the ground |
