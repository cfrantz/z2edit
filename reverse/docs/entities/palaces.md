# Palace Enemies

Bank 4 holds two enemy groups: one for palaces 1, 2 and 5 (tables
`EnemyHp_Bank4` ... at $9400) and one for palaces 3, 4 and 6
(`EnemyHp_Bank4B` ... at $A900).  Many routines are shared by both.
Types $00-$04 and $11-$13 are [common enemies](common.md) ($11 and $12,
Octoroks, are the "Ropes" of the palaces).

## Palaces 1, 2 and 5

| Type | Enemy | HP | Exp | Damage | Drop | Notes | AI |
|------|-------|----|-----|--------|------|-------|----|
| $00 | [Fairy](enemies/fairy.md) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `Fairy_Ai` |
| $01 | [Item](enemies/item.md) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder | `Item_Fall` |
| $02 | [Door](enemies/locked-door.md) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `LockedDoor_Ai` |
| $03 | [Myu](enemies/myu.md) | 3 | 3 | 1 | 1 | comes back | `Myu_Ai` |
| $04 | [Bot](enemies/bot.md) | 4 | 2 | 1 | 1 | comes back | `Bot_Ai` |
| $05 | [Hidden jar](enemies/hidden-jar.md) | 255 | 0 | 0 | - | blocks Link's projectiles, immune to Thunder | `HiddenJar_Ai` |
| $06 | [Slow Bubble](enemies/bubble.md) | 250 | 50 | 0 | - | comes back | `Bubble_Ai` |
| $07 | [Orange Moa](enemies/orange-moa.md) | 16 | 50 | 3 | 2 | touch drains 10 exp, comes back | `OrangeMoa_Ai` |
| $08 | [Falling Block Generator](enemies/falling-block.md) | 0 | 0 | 1 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `FallingBlockGenerator_Ai` |
| $09 | [Falling Block](enemies/falling-block.md) | 0 | 0 | 1 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder | `FallingBlock_Ai` |
| $0A | [Blue Skull Head](enemies/skull-head.md) | 56 | 20 | 0 | - | touch drains 10 exp, comes back | `SkullHead_Ai` |
| $0B | [Tinsuit generator](enemies/tinsuit.md) | 2 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder | `TinsuitGenerator_Ai` |
| $0C | [Tinsuit](enemies/tinsuit.md) | 2 | 0 | 0 | - | touch drains 10 exp, comes back | `Tinsuit_Ai` |
| $0D | [Acid Dripper](enemies/acid-dripper.md) | 250 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `AcidDripper_Ai` |
| $0E | [Fast Bubble](enemies/bubble.md) | 250 | 50 | 0 | - | comes back | `Bubble_Ai` |
| $0F | [Bago Bago Generator](enemies/bago-bago.md) | 8 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `BagoBagoGenerator_Ai` |
| $10 | [Bago Bago](enemies/bago-bago.md) | 8 | 5 | 2 | 1 | touch drains 10 exp | `BagoBago_Ai` |
| $11 | [Hopping Octorok](enemies/octorok.md) | 12 | 20 | 1 | 1 |  | `OctorokHopping_Ai` |
| $12 | [Walking Octorok](enemies/octorok.md) | 12 | 20 | 1 | 1 |  | `OctorokWalking_Ai` |
| $13 | [Elevator](enemies/elevator.md) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `Elevator_Ai` |
| $14 | [Crystal Spot](enemies/crystal.md) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `CrystalSpot_Ai` |
| $15 | [Crystal](enemies/crystal.md) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `Crystal_Ai` |
| $16 | [Energy Ball Shooter (down/right)](enemies/energy-ball-shooter.md) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `EnergyBallShooter_Ai` |
| $17 | [Energy Ball Shooter (down/left)](enemies/energy-ball-shooter.md) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `EnergyBallShooter_Ai` |
| $18 | [Orange Iron Knuckle](enemies/iron-knuckle.md) | 16 | 50 | 2 | 2 | blocks Link's projectiles | `Ironknuckle_Ai` |
| $19 | [Red Iron Knuckle](enemies/iron-knuckle.md) | 24 | 100 | 3 | 2 | blocks Link's projectiles | `Ironknuckle_Ai` |
| $1A | [Blue Iron Knuckle](enemies/iron-knuckle.md) | 56 | 150 | 3 | - | blocks Link's projectiles | `Ironknuckle_Ai` |
| $1B | [Wolf Head Generator](enemies/wolf-head.md) | 2 | 5 | 1 | - | touch drains 10 exp, blocks Link's projectiles, immune to SPELL | `WolfHeadGenerator_Ai` |
| $1C | [Wolf Head](enemies/wolf-head.md) | 2 | 5 | 1 | - | touch drains 10 exp | `WolfHead_Ai` |
| $1D | [Wizzrobe](enemies/wizzrobe.md) | 32 | 200 | 3 | - | blocks Link's projectiles, immune to SPELL | `Mago_Ai` |
| $1E | [Hammer Thrower](enemies/hammer-thrower.md) | 16 | 50 | 2 | 2 | blocks Link's projectiles | `HammerThrower_Ai` |
| $1F | [Red Stalfos](enemies/stalfos.md) | 8 | 30 | 1 | 2 | blocks Link's projectiles | `Stalfos_Ai` |
| $20 | [Horse Head](enemies/horsehead.md) | 32 | 50 | 2 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, comes back | `HorseHead_Ai` |
| $21 | [Helmethead (Gooma in palace 5)](enemies/helmethead.md) | 48 | 150 | 3 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, comes back | `Helmethead_Ai` |
| $22 | [Helmethead Head](enemies/helmethead.md) | 80 | 0 | 1 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder | `HelmetheadHead_Ai` |
| $23 | [Blue Stalfos](enemies/stalfos.md) | 12 | 50 | 1 | 2 | blocks Link's projectiles | `Stalfos_Ai` |

## Palaces 3, 4 and 6

| Type | Enemy | HP | Exp | Damage | Drop | Notes | AI |
|------|-------|----|-----|--------|------|-------|----|
| $00 | [Fairy](enemies/fairy.md) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `Fairy_Ai` |
| $01 | [Item](enemies/item.md) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder | `Item_Fall` |
| $02 | [Door](enemies/locked-door.md) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `LockedDoor_Ai` |
| $03 | [Myu](enemies/myu.md) | 3 | 5 | 1 | 1 | comes back | `Myu_Ai` |
| $04 | [Bot](enemies/bot.md) | 4 | 2 | 1 | 1 | comes back | `Bot_Ai` |
| $05 | [Hidden jar or Iron Knuckle](enemies/hidden-jar.md) | 255 | 0 | 0 | - | blocks Link's projectiles, comes back | `HiddenJarOrIronknuckle_Ai` |
| $06 | [Slow Bubble](enemies/bubble.md) | 250 | 50 | 0 | - |  | `Bubble_Ai` |
| $07 | [Orange Moa](enemies/orange-moa.md) | 16 | 50 | 3 | 2 | touch drains 10 exp | `OrangeMoa_Ai` |
| $08 | [Falling Block Generator](enemies/falling-block.md) | 0 | 0 | 1 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `FallingBlockGenerator_Ai` |
| $09 | [Falling Block](enemies/falling-block.md) | 0 | 0 | 1 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder | `FallingBlock_Ai` |
| $0A | [Rebonack Un-horsed](enemies/rebonack.md) | 8 | 301 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, comes back | `RebonackUnhorsed_Ai` |
| $0B | [Rathead Generator](enemies/rathead.md) | 2 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `RatheadGenerator_Ai` |
| $0C | [Rathead](enemies/rathead.md) | 2 | 0 | 0 | - | touch drains 10 exp, comes back | `Rathead_Ai` |
| $0D | [Acid Dripper](enemies/acid-dripper.md) | 2 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `AcidDripper_Ai` |
| $0E | [Fast Bubble](enemies/bubble.md) | 250 | 50 | 0 | - |  | `Bubble_Ai` |
| $0F | [Blue Skull Head](enemies/skull-head.md) | 56 | 20 | 0 | 1 | touch drains 10 exp, comes back | `SkullHead_Ai_Bank4B` |
| $10 | [Flame](enemies/flame.md) | 8 | 5 | 2 | - | blocks Link's projectiles, immune to SPELL, sword passes through, comes back | `Flame_Ai` |
| $11 | [Hopping Octorok](enemies/octorok.md) | 12 | 20 | 1 | 1 |  | `OctorokHopping_Ai` |
| $12 | [(unused)](enemies/octorok.md) | 12 | 20 | 1 | 1 |  | `OctorokWalking_Ai` |
| $13 | [Elevator](enemies/elevator.md) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `Elevator_Ai` |
| $14 | [Crystal Spot](enemies/crystal.md) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `CrystalSpot_Ai` |
| $15 | [Crystal](enemies/crystal.md) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `Crystal_Ai` |
| $16 | [Energy Ball Shooter (down/right)](enemies/energy-ball-shooter.md) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `EnergyBallShooter_Ai` |
| $17 | [Energy Ball Shooter (down/left)](enemies/energy-ball-shooter.md) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `EnergyBallShooter_Ai` |
| $18 | [Orange Iron Knuckle](enemies/iron-knuckle.md) | 16 | 50 | 2 | 2 | blocks Link's projectiles | `Ironknuckle_Ai` |
| $19 | [Red Iron Knuckle](enemies/iron-knuckle.md) | 24 | 100 | 3 | 2 | blocks Link's projectiles | `Ironknuckle_Ai` |
| $1A | [Blue Iron Knuckle](enemies/iron-knuckle.md) | 56 | 150 | 3 | - | blocks Link's projectiles | `Ironknuckle_Ai` |
| $1B | [Wolf Head Generator](enemies/wolf-head.md) | 2 | 5 | 0 | - | touch drains 10 exp, blocks Link's projectiles, immune to SPELL, immune to Thunder | `WolfHeadGenerator_Ai` |
| $1C | [Wolf Head](enemies/wolf-head.md) | 2 | 5 | 1 | - | touch drains 10 exp | `WolfHead_Ai` |
| $1D | [Wizzrobe](enemies/wizzrobe.md) | 1 | 100 | 3 | 2 | blocks Link's projectiles, immune to SPELL | `Wizrobe_Ai` |
| $1E | [Doom Knocker](enemies/doom-knocker.md) | 32 | 100 | 3 | - | blocks Link's projectiles | `DoomKnocker_Ai` |
| $1F | [Red Stalfos Knight](enemies/stalfos.md) | 24 | 50 | 2 | 2 | blocks Link's projectiles | `Stalfos_Ai` |
| $20 | [Rebonack](enemies/rebonack.md) | 72 | 301 | 3 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, comes back | `Rebonack_Ai` |
| $21 | [Barba](enemies/barba.md) | 192 | 700 | 5 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, comes back | `Barba_Ai` |
| $22 | [Carock](enemies/carock.md) | 16 | 301 | 3 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, comes back | `Carock_Ai` |
| $23 | [Blue Stalfos Knight](enemies/stalfos.md) | 32 | 70 | 3 | 2 | blocks Link's projectiles | `Stalfos_Ai` |

## Shared Enemies

**Bubbles** ($06 slow, $0E fast): flaming skulls that fly diagonally,
bouncing off everything, and burn away Link's magic while touching him.
See [Bubble](enemies/bubble.md).

**Falling blocks** ($08 generator, $09 block): blocks dropped from the
ceiling every 16 frames that pile up into walls.  See
[Falling Block](enemies/falling-block.md).

**Crystal spot** ($14) **and crystal** ($15): the statue where Link sets
the palace's crystal, for a fanfare, full meters and a level.  See
[Crystal Spot](enemies/crystal.md).

**Energy ball shooters** ($16, $17): wall statues that fire energy balls
down at Link every 32 frames.  See
[Energy Ball Shooter](enemies/energy-ball-shooter.md).

**Iron Knuckles** ($18 orange, $19 red, $1A blue): armored knights that
walk up to Link, hold, and trade blows, following his strokes with their
shield; the blue one throws its sword.  See
[Iron Knuckle](enemies/iron-knuckle.md).

**Stalfos** ($1F red, $23 blue): skeleton knights that drop from the
ceiling as they come into view, walk up to Link and slash; the blue one
also leaps at him.  Their shield guards their head.  See
[Stalfos](enemies/stalfos.md).

**Wolf heads** ($1C) and their generator ($1B): heads that cross the
screen in steps, firing straight ahead, one every 128 frames from the
edge Link is heading for.  See [Wolf Head](enemies/wolf-head.md).

**Hidden jar** ($05): an invisible object that a sword stroke turns
into a red jar, or in palaces 3, 4 and 6 half the time into a red Iron
Knuckle.  See [Hidden Jar](enemies/hidden-jar.md).

**Acid dripper** ($0D): a ceiling spout that drips acid every 64 frames;
1 drop in 8 becomes a Bot where it lands.  See
[Acid Dripper](enemies/acid-dripper.md).

**Orange Moa** ($07): a flying eye that swoops in from the side Link
faces, turns back while climbing, and drops flames on its way out.  See
[Orange Moa](enemies/orange-moa.md).

**Skull heads** ($0A in palaces 1, 2, 5; $0F in 3, 4, 6): blue flying
skulls that come in from the side Link faces and swoop through him in a
wave.  See [Skull Head](enemies/skull-head.md).

## Palaces 1, 2 and 5

**Tinsuits** ($0C) and their generator ($0B): small knights that hop
along the floor, one sent in every 128 frames from the edge Link is
heading for.  See [Tinsuit](enemies/tinsuit.md).

**Wizzrobe** (Mago, $1D): blinks in at a random place, casts a flame
along the floor, and vanishes, every 256 frames.  See
[Wizzrobe](enemies/wizzrobe.md).

**Hammer Thrower** ($1E): paces and lobs hammers in high arcs.  See
[Hammer Thrower](enemies/hammer-thrower.md).

### Bosses

The bosses of palaces 1, 2 and 5 share their fight
(`Boss_UpdateIntro`, `Boss_UpdateCommon`, `Boss_Init`):

- A boss is spawned only while its room's flag shows it unbeaten, so it
  stays dead.  `Boss_Init` gives it guard 2 and hitbox entry 7.
- Until Link's screen reaches page 1 of the room, the boss is only drawn.
  Then the boss music starts and the scroll freezes.
- Its hit points show as a column of 8 blocks at the left of the screen
  (`Boss_DrawHpBar`), each worth a set number of hit points.
- When the hits kill it, it is kept alive as a dying boss: hitbox $FF,
  flashing for 127 frames.  Then it bursts into sparks
  (`Boss_SpawnDeathSpark_Bank4`), every enemy projectile is removed,
  every other enemy is killed by a 224-point hit, and the music stops.

- **Horsehead** ($20, palace 1): paces and swings his mace; only his head
  can be hurt.  See [Horsehead](enemies/horsehead.md).
- **Helmethead** ($21, palace 2): paces and fires energy balls; blows to
  his head knock off his helmets, which fly about firing at Link.  See
  [Helmethead](enemies/helmethead.md).
- **Gooma** ($21 in palace 5): whirls and hurls a ball and chain.  See
  [Gooma](enemies/gooma.md).

## Palaces 3, 4 and 6

**Ratheads** ($0C) and their generator ($0B): the Tinsuits of these
palaces.  See [Rathead](enemies/rathead.md).

**Flame** ($10): a fire that falls to the floor and burns there for good.
See [Flame](enemies/flame.md).

**Wizzrobe** ($1D): comes and goes like the Mago, firing a beam; only
its own beam, reflected, can hurt it.  See
[Wizzrobe](enemies/wizzrobe.md).

**Doom Knocker** ($1E): paces and throws its mace, which turns and
flies back past it.  See [Doom Knocker](enemies/doom-knocker.md).

### Bosses

The bosses of palaces 3, 4 and 6 have their own copy of the fight
(`Boss_UpdateIntro_Bank4B`, `Boss_CheckDeath_Bank4B`).  It differs from
the first group's in three ways:

- the boss isn't drawn before Link reaches page 1;
- once the fight starts, Rebonack and Barba wait 255 frames
  (`TIMER_BOSS`);
- the boss's death doesn't kill the other enemies.

- **Rebonack** ($20, palace 3): charges back and forth on a flying horse;
  below 36 hit points he is unhorsed and fights on as a blue Iron
  Knuckle ($0A).  See [Rebonack](enemies/rebonack.md).
- **Barba** ($21, palace 6): a dragon rising from the lava at one of three
  places to breathe fireballs.  See [Barba](enemies/barba.md).
- **Carock** ($22, palace 4): a fast Wizzrobe whom only his own reflected
  beams can hurt.  See [Carock](enemies/carock.md).

## Projectiles

| Type | Palaces 1, 2, 5 | Palaces 3, 4, 6 | Thrown by |
|------|-----------------|-----------------|-----------|
| 1 | `Projectile_Straight_Bank4` | `Projectile_Straight_Bank4B` | energy ball shooters, wolf heads |
| 2 | `Projectile_Straight_Bank4` | `Projectile_Straight_Bank4B` | Helmethead |
| 3 | `Projectile_Rock_Bank4` | `Projectile_Rock_Bank4B` | Octoroks |
| 4 | `Projectile_Fireball_Bank4` | `Projectile_Fireball_Bank4B` | Barba (and Acheman) |
| 5 | `Projectile_Rock_Bank4` | `Projectile_Rock_Bank4B` | Iron Knuckles' swords |
| 6 | `Projectile_Fall_Bank4` | `Projectile_Mace_Bank4B` | hammer, mace |
| 7 | `Projectile_Flame_Bank4` | `Projectile_Flame_Bank4` | flames (Orange Moa, Mago) |
| 8 | `Projectile_AcidDrop_Bank4` | `Projectile_AcidDrop_Bank4` | acid drops |
| 9 | | `Projectile_CarockBeam` | Wizzrobe, Carock |
