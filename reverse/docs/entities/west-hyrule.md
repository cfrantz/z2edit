# West Hyrule Enemies

Bank 1's enemy group, used in West Hyrule's side view areas (encounters,
caves, bridges).  Types $00-$13 are the [common enemies](common.md).
The definition tables are `EnemyHp_Bank1` and following ($9400); see
[the definition tables](README.md#the-definition-tables) for the columns.

| Type | Enemy | HP | Exp | Damage | Drop | Notes | AI |
|------|-------|----|-----|--------|------|-------|----|
| $00 | Fairy | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `Fairy_Ai` |
| $01 | Red Jar | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder | `Item_Fall` |
| $02 | Door | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `LockedDoor_Ai` |
| $03 | Myu | 3 | 3 | 1 | 1 | comes back | `Myu_Ai` |
| $04 | Bot | 3 | 2 | 0 | 1 | comes back | `Bot_Ai` |
| $05 | Bit | 3 | 2 | 0 | 1 | comes back | `Bit_Ai` |
| $06 | Moa | 8 | 10 | 2 | 1 | touch drains 20 exp | `Moa_Ai` |
| $07 | Ache | 3 | 3 | 0 | 1 | comes back | `Ache_Ai` |
| $08 | (unused) | 0 | 0 | 1 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `$AB98` |
| $09 | (unused) | 0 | 0 | 1 | - | touch drains 10 exp, blocks Link's projectiles, immune to SPELL | `$ABE9` |
| $0A | Acheman | 8 | 10 | 2 | 1 | comes back | `Ache_Ai` |
| $0B | Bubble Generator | 2 | 0 | 0 | - | touch drains 10 exp, blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `BubbleGenerator_Ai` |
| $0C | Rock Generator | 2 | 0 | 0 | - | touch drains 10 exp, blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `RockGenerator_Ai` |
| $0D | Red Deeler | 3 | 2 | 0 | 1 |  | `Deeler_Ai` |
| $0E | Blue Deeler | 4 | 3 | 0 | 1 |  | `Deeler_Ai` |
| $0F | Bago Bago Generator | 3 | 3 | 1 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `BagoBagoGenerator_Ai` |
| $10 | Bago Bago | 3 | 3 | 1 | 1 | touch drains 10 exp | `BagoBago_Ai` |
| $11 | Red Octorok (jumping) | 4 | 10 | 1 | 1 |  | `OctorokHopping_Ai` |
| $12 | Red Octorok (moving) | 4 | 10 | 1 | 1 |  | `OctorokWalking_Ai` |
| $13 | Elevator | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `Elevator_Ai` |
| $14 | Orange Moblin | 4 | 10 | 1 | 1 |  | `MoblinGoriya_Ai` |
| $15 | Red Moblin | 12 | 20 | 1 | 1 |  | `MoblinGoriya_Ai` |
| $16 | Blue Moblin | 18 | 20 | 1 | 1 |  | `MoblinGoriya_Ai` |
| $17 | Orange Daira | 18 | 70 | 2 | 2 | blocks Link's projectiles | `Daira_Ai` |
| $18 | Red Daira | 24 | 100 | 2 | 2 | blocks Link's projectiles | `Daira_Ai` |
| $19 | Orange Goriya | 12 | 20 | 1 | 1 | blocks Link's projectiles | `MoblinGoriya_Ai` |
| $1A | Red Goriya | 14 | 20 | 1 | 1 | blocks Link's projectiles | `MoblinGoriya_Ai` |
| $1B | Blue Goriya | 18 | 30 | 1 | 2 | blocks Link's projectiles | `MoblinGoriya_Ai` |
| $1C | Lowder | 4 | 3 | 0 | 1 |  | `Lowder_Ai` |
| $1D | Moby Generator | 3 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL, immune to Thunder, sword passes through | `MobyGenerator_Ai` |
| $1E | Moby | 3 | 2 | 0 | - | touch drains 10 exp | `Moby_Ai` |
| $1F | Megmet | 4 | 5 | 1 | 1 |  | `Megmet_Ai` |
| $20 | Geldarm | 8 | 5 | 1 | 1 | immune to SPELL | `Geldarm_Ai` |
| $21 | Dumb Moblin Generator | 0 | 0 | 0 | - | blocks Link's projectiles, immune to SPELL | `DumbMoblinGenerator_Ai` |
| $22 | Dumb Moblin | 2 | 0 | 0 | 1 | touch drains 10 exp | `DumbMoblin_Ai` |

## Moblins and Goriyas

**Moblins** ($14 orange, $15 red, $16 blue) and **Goriyas** ($19 orange,
$1A red, $1B blue) share `MoblinGoriya_Ai` and the movement in
`Fighter_Move`:

- When Link is far, they walk toward him (16 pixels/16 frames).  How
  close they come depends on the type (`ram_05E5`): the red and blue
  Moblins come within 22 pixels; the orange Moblin and the Goriyas keep
  about 96 pixels away.
- Within that distance they stop and sway: they copy Link's movement
  when he walks, or step 4 back and forth every 64 frames.  At random
  intervals (masks by type) they attack (`Fighter_StartAttack`), high
  or low; the low attack is chosen when Link is crouching.
- Moblins thrust their spear; the orange and blue Moblins throw it
  (`Moblin_ThrowSpear`, projectile type 5, which flies in an arc).  The
  red Moblin only thrusts.
- Goriyas throw a boomerang (projectile type 6) that flies out and comes
  back, disappearing when it returns to the Goriya.
- A thrust is tested against Link like a projectile
  (`Projectile_CheckLinkReflect`): his shield can block it, otherwise it
  hurts him.  Goriyas also block the sword beam and fire (damage bit 5).
- After a hit they are knocked back (`Fighter_KnockBack`).

## Dairas

**Dairas** ($17 orange, $18 red; `Daira_Ai`, `Daira_Attack`):
axe-wielding lizardmen.  They move like Moblins (`Fighter_Move`, keeping
30 or 55 pixels away) and swing their axe, which Link's shield can
block; the red Daira throws its axe (projectile type 7, straight).
They block the sword beam and fire.

## Other Enemies

### Lowder

**Lowder** ($1C, `Lowder_Ai`, `Lowder_Draw`): a beetle that walks back
and forth, faster toward Link.  It has no spawn routine, so it starts
with the [spawn defaults](../systems/enemies.md#spawning): facing Link
and walking his way at 1/2 pixel per frame, `objState` 0.  Only West
Hyrule has Lowders.

Each frame, after `Enemy_SkipAiIfStunned`:

1. `Enemy_HurtLinkOnContact` (with last frame's contact bit), then
   `Enemy_UpdateWithBg`: background probes (probe set 6), drawing, hit
   tests.
2. **Walk.**  It moves by its X velocity, doubled for this frame if its
   facing (its direction of travel) is the side Link is on: 1/2 pixel per
   frame away from Link, 1 pixel per frame toward him.  It walks in the
   air too.
3. **Turn toward Link.**  If Link's Y + $10 equals its Y (as it does when
   Link stands on the same floor) and its timer
   (`timerFast+TIMER_ENEMY`) has run out, it faces Link, takes the speed
   1/2 pixel per frame his way (`LowderSpeeds`), and sets the timer to
   160 frames.  So it turns toward him as soon as he is on its level, and
   again every 160 frames while he stays there; the timer only restarts
   then.
4. **Floor and walls.**  On a floor with no wall ahead, it lands
   (`Enemy_Land`) and clears `objState`.  If there is a wall ahead, or no
   floor under it (it has walked past a ledge, or is falling):
   - if `objState` is 0, this is the first such frame: it sets `objState`
     to 1, turns around (`Enemy_TurnAround`), and moves twice at its new
     velocity, backing off 1 pixel.  At a ledge this puts its floor probe
     (8 pixels in from its left edge) back over the floor, so it turns at
     ledges as well as walls.
   - otherwise it falls (`Enemy_MoveYWithGravity` with gravity $20/256
     pixel per frame per frame, at most 3 pixels per frame), still
     walking.

A Lowder that spawns in mid-air turns around once as it starts to fall
([quirks](../quirks.md)).  It never jumps.

**Drawing** (`Lowder_Draw`): two sprites facing its `objFacing`,
alternating every 8 frames between tiles $DE, $E0 and $E2, $E0
(`sramEnemyTiles` entries $3C and $3E).

Observed in the no-glitch movie (frames 57500-58010, three Lowders on
the floor at Y $90 with Link at Y $80): each turned toward Link every
160 frames; one walked 160 pixels in 160 frames toward Link, and
another slowed from 1 to 1/2 pixel per frame when Link walked past it.
None met a wall or a ledge in the traces, so the turning in step 4 is
described from the code.

**Megmet** ($1F, `Megmet_Ai`): hops toward Link in small random hops,
under its own light gravity, turning at walls.

**Moby** ($1E, `Moby_Ai`) and its generator ($1D,
`MobyGenerator_Ai`): every 128 frames the generator sends a Moby (3 hit
points) diving from high near one side of the screen; it falls until it
reaches Link's height, then flies straight across.  Touching it drains
experience.

**Geldarm** ($20, `Geldarm_Ai`, `Geldarm_Init`): a desert worm that
stretches its neck up from the sand and back down, a pixel every 5
frames, over 64 pixels, pausing at the top.  A sword hit on its head
makes it shrink back down with a sound.  It is immune to the SPELL
spell.

**Walking Moblin** ($22, `DumbMoblin_Ai`) and its generator ($21,
`DumbMoblinGenerator_Ai`): every 128 frames the generator sends a Moblin
(2 hit points) from the edge of the screen, walking straight in its
direction until it falls or meets a wall.  It never attacks.

Neither test movie fights Mobys, Geldarms or walking Moblins; their code
was disassembled from the tables and read, not traced.

## Projectiles

| Type | Thrown by | Routine | Behavior |
|------|-----------|---------|----------|
| 1 | rock generator | `Projectile_Straight_Bank1` | flies straight |
| 2 | bubble generator | `Projectile_Bubble_Bank1` | rises, swaying every 16 frames |
| 3 | Octorok, Bago Bago | `Projectile_Rock_Bank1` | flies straight; falls once deflected |
| 4 | Acheman | `Projectile_Fireball_Bank1` | flies straight, with a sound |
| 5 | Moblin | `Projectile_Falling_Bank1` | thrown spear, in an arc |
| 6 | Goriya | `Projectile_Boomerang_Bank1` | out and back |
| 7 | red Daira | `Projectile_Straight_Bank1` | flies straight |

All of them move and are tested against Link's shield and body in
`Projectile_MoveAndCheck_Bank1` (`Projectile_CheckLinkAlternate`), and
are drawn by `Projectile_Draw_Bank1`.  Their flags
(`ProjectileFlags_Bank1`) say how the shield treats them; see
[the definition tables](README.md#the-definition-tables).
