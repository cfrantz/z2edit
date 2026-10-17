# Octorok

A hopping creature that spits rocks at Link.  It comes in two kinds: the
**hopping Octorok** stays put, and the **walking Octorok** walks back
and forth.  The palaces' Ropes and the Great Palace's Fire Ropes run the
same code.

## Where It Appears

| Group | Type | Name (z2edit) | HP | Exp | Damage class | Drop group |
|-------|------|---------------|---:|----:|-------------:|-----------:|
| [West Hyrule](../west-hyrule.md) | $11, $12 | Red Octorok | 4 | 10 | 1 | 1 |
| [East Hyrule](../east-hyrule.md) | $11, $12 | Blue Octorok | 12 | 20 | 1 | 1 |
| [Palaces 1, 2, 5](../palaces.md) | $11, $12 | Rope | 12 | 20 | 1 | 1 |
| [Palaces 3, 4, 6](../palaces.md) | $11 | Rope | 12 | 20 | 1 | 1 |
| [Great Palace](../great-palace.md) | $11, $12 | Fire Rope | 12 | 20 | 4 | 1 |

Type $11 is the hopping kind (`OctorokHopping_Ai`), $12 the walking kind
(`OctorokWalking_Ai`).  All use background probe set 6 and have no spawn
routine ([spawn defaults](../../systems/enemies.md#spawning): facing Link,
X velocity 1/2 pixel per frame his way, `enemyAiState` 0).  In palaces 3, 4
and 6 the walking type is unused (z2edit's config calls it a crash),
though its table entries point to the same routines as elsewhere.

## Behavior

### Hopping (both kinds)

`Enemy_Hop` runs every frame:

1. `enemyAiState` counts frames.  When its low 7 bits wrap to 0, every
   128 frames, the first time 128 frames after it spawns, it jumps: Y
   velocity -$20 (2 pixels per frame up).
2. It moves and falls (`Enemy_MoveYAndFall`: gravity 2/16 pixel per frame
   per frame), then `Enemy_UpdateWithBg` and `Enemy_HurtLinkOnContact`.
3. On a floor it lands (`Enemy_Land`).
4. In the air, on the frame its Y velocity reaches 0, it is at the top
   of the hop, 17 pixels up after 16 frames, and it spits a rock if it
   faces Link (`Enemy_ShootIfFacingLink`).

A hop lands 32 frames after it began.

### Spitting

`Enemy_ShootIfFacingLink` compares the Octorok's facing with the side
Link is on (`Enemy_FaceLink`, with the facing put back afterward).  If
they match, it fires projectile type 3 (`Enemy_FireProjectile`): from the
Octorok's position, horizontally at 2 pixels per frame in its facing.
If all six projectile slots are busy nothing happens.  Type 3 is each
group's own routine: a rock that flies straight and falls if Link's
shield knocks it back (`Projectile_Rock_Bank1`, `Projectile_Rock_Bank2`,
`Projectile_Rock_Bank4`, `Projectile_Rock_Bank4B`), or in the Great
Palace `Projectile_Straight_Bank5`, which flies straight and is never
knocked back: Link's shield destroys it (flags $03).

### The Hopping Octorok

`OctorokHopping_Ai` turns it toward Link every frame (`Enemy_FaceLink`),
then falls into `Enemy_Hop`.  It never moves sideways: it hops in place
and spits at the top of every hop.

### The Walking Octorok

`OctorokWalking_Ai`, each frame after `Enemy_Hop`:

1. If there is a wall ahead, it turns around (`Enemy_TurnAround`).
2. It moves by its X velocity, 1/2 pixel per frame, on the ground and in
   the air alike.  It doesn't test for ledges, so it walks off them.
3. When its frame count is $40 or $C0, halfway between hops, it turns
   toward Link and walks his way (`OctorokSpeeds`).
4. Its spit timer (`timerFast+TIMER_ENEMY`) counts down.  When it reaches
   0, it is reloaded with 128-255 frames at random (`rng+1,x` bits 6-0
   ORed with $80), and if the Octorok is on the ground then, it spits if
   it faces Link.  The timer starts at 0, so this happens on its first
   frame too.

Its facing is its direction of travel, so it spits, at the top of a hop
or from the ground, only while walking toward Link.  When the spit timer
runs out in the air, it doesn't spit and the chance is lost until the
timer runs out again.  In a pit one tile wide it turns at each wall,
shuttling back and forth, until a hop or a turn toward Link takes it
out.

## Drawing

`Octorok_Draw` (outdoors and in palaces): a pair of sprites facing its
`objFacing` (mirrored when facing right), alternating every 8 frames
(`frameCounter` bit 3) between tiles $C6, $C8 and $CA, $CC
(`SpritePairTiles` entries $12 and $14).  When it is low on the screen
(Y $B5 or more) its attributes also take `extraSpriteAttrs`, the extra
bits of Link's sprites (provisional: presumably so that it goes behind
the background where Link would).

`FireOctorok_Draw` (Great Palace): the same animation from
`sramEnemyTiles` entries $46 and $48.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `OctorokHopping_Ai` | 7:$D842 | faces Link, falls into `Enemy_Hop` |
| `Enemy_Hop` | 7:$D848 | hop timing, falling, landing, the shot at the top |
| `Enemy_ShootIfFacingLink` | 7:$D86C | fires a rock (projectile type 3) |
| `OctorokWalking_Ai` | 7:$D888 | walking, turns, the spit timer |
| `OctorokSpeeds` | 7:$D886 | walking X velocity: $08 right, $F8 left |
| `Octorok_Draw` | 7:$F0A8 | drawing |
| `FireOctorok_Draw` | 5:$96A0 | drawing in the Great Palace |

## Verification

- No-glitch movie, frames 57994-59500, two hopping Octoroks in pits (Y
  $A0) of a West Hyrule cave, with Link held still from frame 57990: the
  first hop came 128 frames after spawning, then every 128; each hop
  rose to Y $8F, spat on its 16th frame and landed on its 32nd.
- Scenario (the same cave, area $2D; see the [Myu](myu.md#verification)
  page for `$AT`): two walking Octoroks placed at Y $90, X $370 and
  $2C0, facing right.  They turned toward Link at frames 63, 191, 319 and
  447 after placement and hopped at 127, 255, 383 and 511.  The one at
  $2C0 walked off the edge into the pit at $2E0, turned at each of its
  walls every 9 frames, and left it on a hop.  Both spat at the top of
  hops only when facing Link; their spit timer ran out every 142-190
  frames, giving a shot when on the ground facing Link (frames 307, 335)
  and none in the air (145) or facing away (0, 459).

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT --until areaId=0x2D --after 200 \
    --clear --approach 2A0 --spawn 0:12:370:90:1 --spawn 1:12:2C0:90:1 \
    --event Hop=7:D850 --event Shoot=7:D880 --event WallTurn=7:D894 \
    --event FaceLink=7:D8A1 --event SpitTimer=7:D8AE --frames 520
```
