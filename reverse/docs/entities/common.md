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

**Moa** ($06): a floating eye that weaves back and forth around Link,
keeping pace with him and drifting up and down, through walls; touching
it drains experience, and in East Hyrule it is invisible without the
cross.  See [Moa](enemies/moa.md).

**Ache** ($07) **and Acheman** ($0A): bats that perch at the top of
caves and swoop down and back up, in a random direction, when Link comes
within 32 pixels (on an area's first and last pages, at once and over
and over); at the bottom of each swoop the Acheman becomes a man for 48
frames and throws a fireball.  See [Ache and Acheman](enemies/ache.md).

**Deelers** ($0D red, $0E blue): forest spiders that follow Link under
the branches and drop on their thread when he is near; the blue one lets
go low down and hops after him.  See [Deeler](enemies/deeler.md).

**Bago Bago** ($10): a fish launched from below the screen by its
generator ($0F) every 128 frames; it lands on a bridge or floor, walks
toward Link without turning again, and spits rocks.  See
[Bago Bago](enemies/bago-bago.md).

**Octoroks** ($11 hopping, $12 walking): hop every 128 frames and spit
rocks at Link from the top of a hop; the walking one also walks, turns
toward Link between hops and spits from the ground at random.  The
palaces' Ropes run the same code.  See [Octorok](enemies/octorok.md).

## Generators

Generators are invisible enemies that spawn others.  Their spawn routine
(`Generator_Init_BankN`) removes any other generator of the same type,
so there is one at a time.

- **Bubble generator** ($0B): every 32 frames, a bubble (West Hyrule) or
  lava fireball (East Hyrule) rises from the bottom of the screen at a
  random X, swaying.  See [Bubble Generator](enemies/bubble-generator.md).
- **Rock generator** ($0C): every 32 frames, while one of four slots is
  free, a rock flies in from the left or right edge at about Link's
  height.  See [Rock Generator](enemies/rock-generator.md).
- **Bago Bago generator** ($0F): every 128 frames, a Bago Bago leaps up
  from below one side of the screen.  See [Bago Bago](enemies/bago-bago.md).

## Unused

Types $08 and $09 point to empty space ($AB98, $ABE9) in the outdoor
banks; z2edit's config notes that they crash in the Great Palace.
