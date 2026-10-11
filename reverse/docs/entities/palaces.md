# Palace Enemies

Bank 4 holds two enemy groups: one for palaces 1, 2 and 5 (tables
`EnemyHp_Bank4` ... at $9400) and one for palaces 3, 4 and 6
(`EnemyHp_Bank4B` ... at $A900).  Many routines are shared by both.
Types $00-$04 and $11-$13 are [common enemies](common.md) ($11 and $12,
Octoroks, are the "Ropes" of the palaces).

## Palaces 1, 2 and 5

| Type | Enemy | HP | Exp | Damage | Drop | Notes | AI |
|------|-------|----|-----|--------|------|-------|----|
| $00 | Fairy | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `Fairy_Ai` |
| $01 | Item | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder | `Item_Fall` |
| $02 | Door | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `LockedDoor_Ai` |
| $03 | Myu | 3 | 3 | 1 | 1 | comes back | `Myu_Ai` |
| $04 | Bot | 4 | 2 | 1 | 1 | comes back | `Bot_Ai` |
| $05 | Hidden jar | 255 | 0 | 0 | - | blocks Link's projectiles, immune to Thunder | `HiddenJar_Ai` |
| $06 | Slow Bubble | 250 | 50 | 0 | - | comes back | `Bubble_Ai` |
| $07 | Orange Moa | 16 | 50 | 3 | 2 | touch drains 10 exp, comes back | `OrangeMoa_Ai` |
| $08 | Falling Block Generator | 0 | 0 | 1 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `FallingBlockGenerator_Ai` |
| $09 | Falling Block | 0 | 0 | 1 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder | `FallingBlock_Ai` |
| $0A | Blue Skull Head | 56 | 20 | 0 | - | touch drains 10 exp, comes back | `SkullHead_Ai` |
| $0B | Tinsuit generator | 2 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder | `TinsuitGenerator_Ai` |
| $0C | Tinsuit | 2 | 0 | 0 | - | touch drains 10 exp, comes back | `Tinsuit_Ai` |
| $0D | Acid Dripper | 250 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `AcidDripper_Ai` |
| $0E | Fast Bubble | 250 | 50 | 0 | - | comes back | `Bubble_Ai` |
| $0F | Bago Bago Generator | 8 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `BagoBagoGenerator_Ai` |
| $10 | Bago Bago | 8 | 5 | 2 | 1 | touch drains 10 exp | `BagoBago_Ai` |
| $11 | Hopping Octorok | 12 | 20 | 1 | 1 |  | `OctorokHopping_Ai` |
| $12 | Walking Octorok | 12 | 20 | 1 | 1 |  | `OctorokWalking_Ai` |
| $13 | Elevator | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `Elevator_Ai` |
| $14 | Crystal Spot | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `CrystalSpot_Ai` |
| $15 | Crystal | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `Crystal_Ai` |
| $16 | Energy Ball Shooter (down/right) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `EnergyBallShooter_Ai` |
| $17 | Energy Ball Shooter (down/left) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `EnergyBallShooter_Ai` |
| $18 | Orange Iron Knuckle | 16 | 50 | 2 | 2 | blocks Link's projectiles | `Ironknuckle_Ai` |
| $19 | Red Iron Knuckle | 24 | 100 | 3 | 2 | blocks Link's projectiles | `Ironknuckle_Ai` |
| $1A | Blue Iron Knuckle | 56 | 150 | 3 | - | blocks Link's projectiles | `Ironknuckle_Ai` |
| $1B | Wolf Head Generator | 2 | 5 | 1 | - | touch drains 10 exp, blocks Link's projectiles, immune to SPELL | `WolfHeadGenerator_Ai` |
| $1C | Wolf Head | 2 | 5 | 1 | - | touch drains 10 exp | `WolfHead_Ai` |
| $1D | Wizzrobe | 32 | 200 | 3 | - | blocks Link's projectiles, immune to SPELL | `Mago_Ai` |
| $1E | Hammer Thrower | 16 | 50 | 2 | 2 | blocks Link's projectiles | `HammerThrower_Ai` |
| $1F | Red Stalfos | 8 | 30 | 1 | 2 | blocks Link's projectiles | `Stalfos_Ai` |
| $20 | Horse Head | 32 | 50 | 2 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, comes back | `HorseHead_Ai` |
| $21 | Helmethead (Gooma in palace 5) | 48 | 150 | 3 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, comes back | `Helmethead_Ai` |
| $22 | Helmethead Head | 80 | 0 | 1 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder | `HelmetheadHead_Ai` |
| $23 | Blue Stalfos | 12 | 50 | 1 | 2 | blocks Link's projectiles | `Stalfos_Ai` |

## Palaces 3, 4 and 6

| Type | Enemy | HP | Exp | Damage | Drop | Notes | AI |
|------|-------|----|-----|--------|------|-------|----|
| $00 | Fairy | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `Fairy_Ai` |
| $01 | Item | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder | `Item_Fall` |
| $02 | Door | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `LockedDoor_Ai` |
| $03 | Myu | 3 | 5 | 1 | 1 | comes back | `Myu_Ai` |
| $04 | Bot | 4 | 2 | 1 | 1 | comes back | `Bot_Ai` |
| $05 | Hidden jar or Iron Knuckle | 255 | 0 | 0 | - | blocks Link's projectiles, comes back | `HiddenJarOrIronknuckle_Ai` |
| $06 | Slow Bubble | 250 | 50 | 0 | - |  | `Bubble_Ai` |
| $07 | Orange Moa | 16 | 50 | 3 | 2 | touch drains 10 exp | `OrangeMoa_Ai` |
| $08 | Falling Block Generator | 0 | 0 | 1 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `FallingBlockGenerator_Ai` |
| $09 | Falling Block | 0 | 0 | 1 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder | `FallingBlock_Ai` |
| $0A | Rebonack Un-horsed | 8 | 301 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, comes back | `RebonackUnhorsed_Ai` |
| $0B | Rathead Generator | 2 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `RatheadGenerator_Ai` |
| $0C | Rathead | 2 | 0 | 0 | - | touch drains 10 exp, comes back | `Rathead_Ai` |
| $0D | Acid Dripper | 2 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `AcidDripper_Ai` |
| $0E | Fast Bubble | 250 | 50 | 0 | - |  | `Bubble_Ai` |
| $0F | Blue Skull Head | 56 | 20 | 0 | 1 | touch drains 10 exp, comes back | `SkullHead_Ai_Bank4B` |
| $10 | Flame | 8 | 5 | 2 | - | blocks Link's projectiles, immune to SPELL, sword passes through, comes back | `Flame_Ai` |
| $11 | Hopping Octorok | 12 | 20 | 1 | 1 |  | `OctorokHopping_Ai` |
| $12 | (unused) | 12 | 20 | 1 | 1 |  | `OctorokWalking_Ai` |
| $13 | Elevator | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `Elevator_Ai` |
| $14 | Crystal Spot | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `CrystalSpot_Ai` |
| $15 | Crystal | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `Crystal_Ai` |
| $16 | Energy Ball Shooter (down/right) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `EnergyBallShooter_Ai` |
| $17 | Energy Ball Shooter (down/left) | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `EnergyBallShooter_Ai` |
| $18 | Orange Iron Knuckle | 16 | 50 | 2 | 2 | blocks Link's projectiles | `Ironknuckle_Ai` |
| $19 | Red Iron Knuckle | 24 | 100 | 3 | 2 | blocks Link's projectiles | `Ironknuckle_Ai` |
| $1A | Blue Iron Knuckle | 56 | 150 | 3 | - | blocks Link's projectiles | `Ironknuckle_Ai` |
| $1B | Wolf Head Generator | 2 | 5 | 0 | - | touch drains 10 exp, blocks Link's projectiles, immune to SPELL, immune to Thunder | `WolfHeadGenerator_Ai` |
| $1C | Wolf Head | 2 | 5 | 1 | - | touch drains 10 exp | `WolfHead_Ai` |
| $1D | Wizzrobe | 1 | 100 | 3 | 2 | blocks Link's projectiles, immune to SPELL | `Wizrobe_Ai` |
| $1E | Doom Knocker | 32 | 100 | 3 | - | blocks Link's projectiles | `DoomKnocker_Ai` |
| $1F | Red Stalfos Knight | 24 | 50 | 2 | 2 | blocks Link's projectiles | `Stalfos_Ai` |
| $20 | Rebonack | 72 | 301 | 3 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, comes back | `Rebonack_Ai` |
| $21 | Barba | 192 | 700 | 5 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, comes back | `Barba_Ai` |
| $22 | Carock | 16 | 301 | 3 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, comes back | `Carock_Ai` |
| $23 | Blue Stalfos Knight | 32 | 70 | 3 | 2 | blocks Link's projectiles | `Stalfos_Ai` |

## Shared Enemies

**Bubbles** ($06 slow, $0E fast; `Bubble_Ai`, `SlowBubble_Init`,
`FastBubble_Init`): flaming skulls that fly diagonally (speed $10 or
$20), bouncing off walls, floors and ceilings and the screen edges.
While touching Link they drain a point of magic every frame, besides the
damage.  With 250 hit points they are practically indestructible.

**Falling blocks** ($08 generator, $09 block; `FallingBlockGenerator_Ai`,
`FallingBlock_Ai`): every 16 frames the generator drops a block from the
top of the screen at a random X (beyond the first $60 pixels).  A block
falls; if it hits Link it hurts him and shatters, as it does when the
sword hits it; when it lands it becomes a solid block in the area's tile
map, so the blocks pile up.  (Types $08 and $09 of the overworld groups
point to the same addresses, which are empty in those banks.)

**Crystal spot** ($14, `CrystalSpot_Ai`) **and crystal** ($15,
`Crystal_Ai`): the statue where Link places the palace's crystal.  When
he stands on it holding a crystal (`crystalsRemaining`), the game
freezes, the crystal rises with the palace's palette flashing
(`CrystalSpot_FlashPalette`), a fanfare plays, his meters refill, and he
gets exactly enough experience for his next level.  The area flag is
cleared and the spot becomes a crystal (`Crystal_Init` checks the flag on
later visits).  The crystal placed is recorded in `crystalPlaced`.

**Energy ball shooters** ($16, $17; `EnergyBallShooter_Ai`): statues
that, every 32 frames while on screen, fire an energy ball (projectile
type 1) aimed at Link, if he is on the side they face
(`Enemy_FireAimedBall`, which picks the ball's speed by his distance).

**Iron Knuckles** ($18 orange, $19 red, $1A blue; `Ironknuckle_Ai`):
armored knights.  When Link is on their level they face him and keep
their distance, mirroring his attacks: when he presses B they may raise
their shield to the height he is attacking (crouching or standing), and
they follow his height when he jumps; they attack high or low at random,
and the blue one throws swords (projectile type 5).  Out of Link's reach
they pace back and forth, turning every 80 frames.  Their shield blocks
the sword except where it is open (`enemyVulnerability`), and they block
the beam and fire.

**Stalfos** ($1F red, $23 blue; `Stalfos_Ai`): skeleton knights that
drop from the ceiling when they come into view, then face Link and walk
to within 30 pixels, slashing (`Enemy_StartSwing`); the blue Stalfos
also jumps at him (1 time in 4).  Only their unshielded part can be hit.

**Wolf heads** ($1C) and their generator ($1B; `WolfHead_Ai`,
`WolfHeadGenerator_Ai`): every 128 frames a wolf head (2 hit points)
flies in from an edge at Link's height, weaving up and down, and fires an
energy ball (projectile type 1) at him every 64 frames when facing him.

**Hidden jar** ($05; `HiddenJar_Ai`, `HiddenJarOrIronknuckle_Ai`): an
invisible object in a wall; striking it with the sword reveals a red
magic jar.  In palaces 3, 4 and 6 it is, half the time, a red Iron
Knuckle (24 hit points) instead.

**Acid dripper** ($0D, `AcidDripper_Ai`): drips acid (projectile type 8)
every 64 frames.  A drop that lands on the floor turns into a Bot
(`Projectile_AcidDrop_Bank4`).

## Palaces 1, 2 and 5

**Orange Moa** ($07, `OrangeMoa_Ai`): flies in from the side Link is
facing, at Y $7D, rises away while drifting, and drops flames
(projectile type 7, which burn on the floor for a while) as it passes.

**Skull head** ($0A, `SkullHead_Ai`, `SkullHead_Init`): appears above
Link and to the side he faces, and flies across in a wave, rising and
falling; it comes back when it leaves the screen.

**Tinsuit** ($0C) and its generator ($0B; `Tinsuit_Ai`,
`TinsuitGenerator_Ai`): small knights that appear on the floor and hop
along at double speed, turning at walls.

**Wizzrobe** (Mago, $1D, `Mago_Ai`): appears at a random place on the
screen, blinks for 32 frames, casts a flame (projectile type 7, at Y $C0)
that runs along the floor in the direction it faces, stays for a while
(touching it hurts), then vanishes and reappears elsewhere.

**Hammer thrower** ($1E, `HammerThrower_Ai`): paces and jumps (every
256 frames), swings, and throws hammers in a high arc (projectile type
6).

### Bosses

Bosses (hitbox $FF in their table entry) share `Boss_UpdateIntro`: when
Link reaches the second page of the boss room, the scroll freezes and the
boss music starts; when the boss dies, every other enemy on the screen is
destroyed and the music stops.  Their hit points are shown as a bar
(`Boss_DrawHpBar`).  A boss only appears while its area flag is set
(`Boss_Init`), so it stays dead.

- **Horsehead** ($20, palace 1; `HorseHead_Ai`): paces toward and away
  from Link (`Boss_Pace`) and swings his mace when facing him
  (`HorseHead_Attack`); only his head can be hurt.
- **Helmethead** ($21, palace 2; `Helmethead_Ai`): paces like Horsehead
  and fires an energy ball every 128 frames.  Each time his head is hit,
  one of his helmets flies off ($22, `HelmetheadHead_Ai`, up to two):
  it bounces about and fires energy balls at Link.
- **Gooma** ($21 in palace 5, `Gooma_Ai`, set up by `Helmethead_Init`
  with its own stats): paces and swings a ball and chain in long and
  short swings (`Gooma_Attack`).

## Palaces 3, 4 and 6

**Skull head** ($0F, `SkullHead_Ai_Bank4B`): as in palaces 1, 2 and 5.

**Rathead** ($0C) and its generator ($0B; `Rathead_Ai`,
`RatheadGenerator_Ai`): like Tinsuits.

**Flame** ($10, `Flame_Ai`): a flame that falls to the floor and burns
there; touching it hurts.

**Wizzrobe** ($1D, `Wizrobe_Ai`): like the Mago above, firing a beam
(projectile type 9) at $B0 of its cycle (`Wizrobe_FireBeam`).

**Doomknocker** ($1E, `DoomKnocker_Ai`): jumps every 256 frames and
throws a mace (projectile type 6) that comes back after a while
(`Projectile_Mace_Bank4B`).

### Bosses

- **Rebonack** ($20, palace 3; `Rebonack_Ai`): rides a flying horse,
  bobbing, charging back and forth at Link.  When his hit points fall
  below 36 he is knocked off: an unhorsed Rebonack ($0A, 48 hit points,
  `RebonackUnhorsed_Ai`) jumps away and fights like an Iron Knuckle,
  while the horse leaves.
- **Barba** ($21, palace 6; `Barba_Ai`): a dragon living in lava.  He
  waits, rises from one of eight places (`Barba_Wait`, `Barba_Rise`),
  breathes fireballs at Link at four times the normal speed for a while
  (`Barba_Breathe`), and sinks back (`Barba_Sink`).
- **Carock** ($22, palace 4; `Carock_Ai`): appears and vanishes like a
  Wizzrobe, firing beams (projectile type 9).  He cannot be hurt by the
  sword: only his own beams, sent back by Link's shield with the reflect
  spell, damage him (`Projectile_CarockBeam`).

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
