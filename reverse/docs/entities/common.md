# Common Enemies and Objects

Enemy types $00-$13 of the West Hyrule, East Hyrule and Great Palace
groups (and some of the palace groups) run routines in bank 7, shared by
all of them; their hit points, experience and damage differ by group
(see each group's page).  All AI routines here start with
`Enemy_SkipAiIfStunned` (no AI while stunned) and end with
`Enemy_UpdateCommon` (draw, collisions, despawning), unless noted.

## Objects

**Fairy** ($00, `Fairy_Ai`, `Fairy_Draw`).  Bobs in place, flapping, and
plays its sound when it first appears.  When Link touches it, his life
meter is refilled (`statMeterPending`) while the game is frozen and the
screen flashes; then it disappears.  It cannot be hurt.

**Items** ($01, `Item_Fall`, `Item_Draw`).  An item (its kind in
`enemyAiState`) falls with gravity, bouncing a little if it lands fast,
and is collected by touch (`Item_Collect`): inventory items (kinds 0-7),
keys (8), experience bags ($0A-$0D: 50, 100, 200, 500), magic jars ($10:
one block, $11: full), a 1-up doll ($12).  The red jar placed in areas is
this type, made a full magic jar by its spawn routine
(`RedJar_Init_BankN`).

**Locked door** ($02, `LockedDoor_Ai`, `LockedDoor_Draw`).  Pushes Link
back when he walks into it.  If he has the magic key, or a key (which is
used up), it opens: its area flag is cleared so that it stays open, and
the game freezes while it slides up for 17 frames.

**Elevator** ($13, `Elevator_Ai`, `Elevator_Draw`).  Placed by the area
(`Elevator_Place`, from `elevatorPos`), not by the enemy list.  While
Link stands on it, Up and Down move it and him, with a sound; at the
bottom of the screen it takes him to the room below
(`Mode_FindConnectedRoom`).

## Enemies

### Myu, Bot and Bit

**Myu** ($03, `Myu_Ai`), **Bot** ($04, `Bot_Ai`) and **Bit** ($05,
`Bit_Ai`) share their movement, `Myu_Move`; they differ only in step
size and in the Bot's leap.  None of them has a spawn routine, so each
starts with the [spawn defaults](../systems/enemies.md#spawning): X
velocity 1/2 pixel per frame toward Link, everything else 0.  The
building blocks are described in
[enemies](../systems/enemies.md#building-blocks).

| Type | AI | Draw | Step | Extra |
|------|----|------|------|-------|
| Myu | `Myu_Ai` | `Myu_Draw` (tiles $24, $B0) | 2 pixels | |
| Bot | `Bot_Ai` | `Bot_Draw` (tiles $B2, $B4) | 2 pixels | leaps at Link |
| Bit | `Bit_Ai` | `Bot_Draw` (tiles $B2, $B4) | 2 pixels, or 4 in its fast periods | |

Myus are used outdoors and in all the palaces; Bots outdoors, in towns
and in all the palaces; Bits outdoors and in towns.

**Each frame** (`Myu_Move`), after `Enemy_SkipAiIfStunned`:

1. Fall: `Enemy_MoveYAndFall` (Y += velocity, then velocity += 2/16
   pixel per frame), then `Enemy_UpdateWithBg` (background probes,
   drawing, hit tests) and `Enemy_HurtLinkOnContact`.
2. If its Y is less than $60 (high on the screen), or there is no floor
   under it, it is **in the air**: it moves by its X velocity unless there is a wall ahead, and
   nothing else happens this frame.  (Floors are not tested at Y < $60,
   so one standing that high would fall through.)
3. **On the ground** it lands (`Enemy_Land`: Y rounded down to a multiple
   of 16, Y velocity 0).  Then, by its phase (`enemyAiState`):
   - **Moving** (0): it counts its frames on the ground in
     `enemyAiVar`.  On counts 3 and 15 of every 16 it takes a
     **step**: its X velocity is set to the step speed for its direction
     (`myuStepSpeeds` by `enemyAiDir`: right or left) and it moves once by
     it, unless there is a wall ahead.  So a Myu moves 2 pixels, then 2
     more 12 frames later, then 2 more 4 frames later: 4 pixels per 16
     frames, 1/4 pixel per frame on average.
   - **Resting** (1): it stays in place.
4. When its timer (`timerFast+TIMER_ENEMY`) has run out, and only on the
   ground, it switches phase and reloads the timer with 0-63 at random
   (`rng+1,x` bits 5-0) plus 192 for a moving phase or 64 for a resting
   phase (`MyuPhaseTimes`): it moves for **192-255 frames** and rests for
   **64-127 frames**.  Bit 0 of the new timer is its new direction
   (`enemyAiDir`: 0 right, 1 left), so each moving phase goes left or
   right at random, whatever side Link is on.

A new Myu, Bot or Bit has phase 0 and timer 0, so on its first frame on
the ground it counts one moving frame and then switches to resting.

**Walls and ledges.**  It does not turn at walls: against one it simply
doesn't step until its next moving phase picks a direction.  It walks
off ledges.  In the air it keeps the X velocity of its last step (2
pixels per frame for a Myu, 4 for a fast Bit), its leap, or its spawn,
so it falls off a ledge in a fast, flat arc.

**The Bot's leap** (`Bot_Ai`).  Before moving, on any frame on which last
frame's probe found a floor under it, in either phase, a Bot leaps if
bits 6-0 of its random byte (`rng+1,x`) are all 0, about 1 frame in 128:
Y velocity -$1B (1.69 pixels per frame up) and X velocity 1 pixel per
frame toward Link (`BotJumpSpeeds`, by `Enemy_FaceLink`).  The leap rises
13 pixels in 14 frames and lands about 27 frames later, about 27 pixels
over (less if it hits a wall, which stops it moving sideways).  Its
step counter and phase timer don't change while it is in the air, except
that the timer keeps counting down.

**The Bit's fast periods** (`Bit_Ai`).  When `frameCounter` OR (enemy
index * 32) is $C0 or more, the step speed is doubled to 4 pixels.  Since
the index is ORed in rather than added, the fast periods are not evenly
staggered: a Bit in enemy index 0 or 1 is fast for frames $C0-$FF of
every 256, in index 2 or 3 for $80-$FF, and in index 4 or 5 for $40-$7F
and $C0-$FF ([quirks](../quirks.md)).

**Drawing.**  `objState` is the animation frame: while moving, bits 3-2
of the step counter (0, 4, 8, 12); while resting, 1.  Each type has two
frames (sprite tiles in the table above, from `SpritePairTiles`), one
for `objState` 0 and one for anything else, so while moving
it shows the first frame for 4 counts (ending with the step on count 3)
and the second for 12 (ending with the step on count 15).  Each is drawn
as a symmetric pair of sprites (the right one mirrored), never facing a
direction.  A resting Bot or Bit quivers: on alternate 4-frame periods
(`frameCounter` bit 2 clear) its two sprites are drawn a pixel closer
together.

Observed in the no-glitch movie (frames 42804-44334, two Bots;
`Bot_Ai`, `Myu_Ai` and event breakpoints): the first rest lasted 94
frames and the next moving phase 246; steps of 2 pixels alternated 4
and 12 frames apart; a leap from Y $B0 rose to $A3 and landed 27 frames
and 27 pixels later; a Bot leapt while resting.

**Moa** ($06, `Moa_Ai`).  A floating eye (ghost) that drifts toward Link,
weaving as its speed swings back and forth, and drifts up or down at
random, turning back at Y $40 and $C0.  It passes through walls.
Touching it drains experience.  In East Hyrule it is invisible unless
Link has the cross.

**Ache** ($07) **and Acheman** ($0A) (`Ache_Ai`).  Bats that hang from
the ceiling until Link comes within 32 pixels horizontally, then swoop
down at him in an arc, moving 8 pixels/16 frames across, and fly back up
to Y $30 to perch again.  After each swoop, the Acheman turns into a man
for 48 frames and throws a fireball (projectile type 4) at Link.  On an
area's first and last pages they swoop at once, toward the middle.

**Deelers** ($0D red, $0E blue, `Deeler_Ai`, `Deeler_Draw`).  Spiders
hanging from trees in forests, moving sideways under the branch to stay
above Link.  When he is within 64 pixels they may drop on their thread
(1 frame in 32, at random), down to 96 pixels below the branch, then
climb back up.  A blue Deeler that drops low enough lets go and then
hops after Link.  (Neither test movie fights them.)

**Bago Bago** ($10, `BagoBago_Ai`).  A fish that leaps out of the water
(spawned below the screen by the generator, $0F) in a high arc, moving
toward Link once it lands; every 112 frames, when within 16 pixels of
his height and facing him, it may spit a rock.  Touching it drains
experience.

### Octoroks

The **hopping Octorok** ($11, `OctorokHopping_Ai`) and the **walking
Octorok** ($12, `OctorokWalking_Ai`) share `Enemy_Hop` and
`Enemy_ShootIfFacingLink`, and are drawn by `Octorok_Draw`.  Neither has
a spawn routine ([spawn defaults](../systems/enemies.md#spawning): X
velocity 1/2 pixel per frame toward Link, facing him, `enemyAiState` 0).
They are used outdoors and in all the palaces.

**Hopping** (`Enemy_Hop`, every frame for both).  `enemyAiState` counts
frames.  When its low 7 bits wrap to 0, every 128 frames (the first 128
frames after it spawns), the Octorok jumps: Y velocity -$20 (2 pixels
per frame up).  It moves and falls with `Enemy_MoveYAndFall` (gravity
2/16 pixel per frame per frame), then `Enemy_UpdateWithBg` and
`Enemy_HurtLinkOnContact`.  On a floor it lands (`Enemy_Land`).  In the
air, on the frame its Y velocity reaches 0, the top of the hop, 16
frames and 17 pixels up, it spits a rock if it is facing Link
(`Enemy_ShootIfFacingLink`).  It lands 32 frames after jumping.

**Spitting** (`Enemy_ShootIfFacingLink`).  If the Octorok's facing is
the side Link is on (`Enemy_FaceLink`, without changing the facing), it
fires projectile type 3, the rock (`Enemy_FireProjectile`): it starts at
the Octorok's position and flies horizontally at 2 pixels per frame in
its facing.  If all six projectile slots are busy, nothing happens.  In
the outdoor groups the rock is `Projectile_Rock_Bank1` and `Projectile_Rock_Bank2`: it flies
straight, and falls if Link's shield knocks it back.

**The hopping Octorok** turns toward Link every frame (`Enemy_FaceLink`)
and never moves sideways, so it hops in place and spits a rock at the
top of every hop.

**The walking Octorok**, each frame after `Enemy_Hop`:

1. If there is a wall ahead it turns around (`Enemy_TurnAround`).
2. It moves by its X velocity, 1/2 pixel per frame, on the ground and in
   the air alike.  It does not test for ledges, so it walks off them.
3. When its frame count is 64 more than a multiple of 128 (`enemyAiState`
   = $40 or $C0), halfway between hops, it turns toward Link and walks
   his way (`OctorokSpeeds`).
4. Its spit timer (`timerFast+TIMER_ENEMY`) counts down; when it reaches
   0, it is reloaded with 128-255 frames at random (`rng+1,x` bits 6-0,
   plus 128), and if the Octorok is on the ground it spits a rock at Link
   if facing him.  Its facing is its direction of travel, so it shoots
   only while walking toward him.

So the walking Octorok spits at the top of each hop when walking toward
Link, and also from the ground at random times.

**Drawing** (`Octorok_Draw`): two sprites, alternating between tiles
$C6, $C8 and $CA, $CC (`SpritePairTiles` entries $12 and $14) every 8
frames (`frameCounter` bit 3), mirrored when it faces right.  When it is low on the screen (Y $B5 or more) its
sprite attributes also get `extraSpriteAttrs`, the bits added to Link's
sprites (provisional: presumably so that it is drawn behind the
background like Link where he wades).

Observed in the no-glitch movie (frames 57994-59500, two hopping
Octoroks on a floor at Y $A0, with Link held still from frame 57990):
the first hop came 128 frames after spawning and then every 128; each
hop rose to Y $8F, spat a rock on its 16th frame, and landed on its
32nd.  The walking Octorok was not in the traced frames (the test movies do
run `OctorokWalking_Ai`, later); it is described from the code.

## Generators

Generators are invisible enemies that spawn others.  Their spawn routine
(`Generator_Init_BankN`) removes any other generator of the same type,
so there is one at a time.

- **Bubble generator** ($0B, `BubbleGenerator_Ai`): every 32 frames, a
  fireball (projectile type 2) rises from the bottom of the screen at a
  random X, swaying as it goes (lava areas).
- **Rock generator** ($0C, `RockGenerator_Ai`): every 32 frames, a rock
  (projectile type 1) flies in from the left or right edge at about
  Link's height.  (Not seen in the test movies.)
- **Bago Bago generator** ($0F, `BagoBagoGenerator_Ai`): every 128 frames,
  a Bago Bago (with 3 hit points, whatever the table says) leaps up from
  below the left or right side of the screen.

## Unused

Types $08 and $09 point to empty space ($AB98, $ABE9) in the outdoor
banks; z2edit's config notes that they crash in the Great Palace.
