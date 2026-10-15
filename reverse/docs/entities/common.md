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

**Myu** ($03), **Bot** ($04) and **Bit** ($05): blobs that crawl in
little steps, alternating moving and resting phases; the Bot also leaps
at Link, and the Bit has periods of double speed.  See
[Myu](enemies/myu.md), [Bot](enemies/bot.md), [Bit](enemies/bit.md).

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

**Octoroks** ($11 hopping, $12 walking): hop every 128 frames and spit
rocks at Link from the top of a hop; the walking one also walks, turns
toward Link between hops and spits from the ground at random.  The
palaces' Ropes run the same code.  See [Octorok](enemies/octorok.md).

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
