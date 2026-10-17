# Fokka

A bird-headed knight of the Great Palace.  It leaps at Link, guards with
its shield at the height of his strokes, and throws its sword high or
low at a steady beat.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Great Palace](../great-palace.md) | $18 | Orange Fokka | 0 | 70 | 5 | - |
| [Great Palace](../great-palace.md) | $19 | Red Fokka | 48 | 70 | 5 | - |
| [Great Palace](../great-palace.md) | $1A | Blue Fokka | 96 | 100 | 5 | - |

Red Fokkas are placed in rooms $0D, $14, $26 and $29, and come out of
[hidden jars](hidden-jar.md) half the time.  Blue ones are placed in
rooms $19, $1B, $20, $2B and $2F.  No room places the orange one.
Link's projectiles bounce off them and SPELL doesn't affect them.  They
have no spawn routine.

`Fokka_Ai` is the [Iron Knuckle](iron-knuckle.md)'s code reworked: the
same attack poses, shield, hit tests and knockback.  Unlike the Iron
Knuckle it is always engaged.

## Behavior

**Moving.**  Each frame it turns toward Link (`Enemy_FaceLink`).

- On the floor, 1 frame in 32 (`rng+1,x` bits 4-0 zero), it jumps at
  him: 3.5 pixels per frame up and 1.5 his way (`FokkaSpeeds`).  It keeps
  that speed across until it lands, often well past him.  A ceiling
  sends it down at 1 pixel per frame.
- Otherwise it never walks toward him.  It moves at half his X
  velocity while he walks, or sways 1/4 pixel per frame, 64 frames each
  way (`frameCounter` bit 6).

**Attacks.**  It counts frames in `enemyAiState` and attacks when the
count's masked bits (`FokkaAttackMasks`) are zero:

- every 64 frames for the red, every 32 for the blue;
- for the orange, whose mask is 0, whenever it isn't already attacking.

On the floor the count pauses during an attack, but in the air it keeps
running.  An attack:

1. 24 frames of wind-up, high (pose 1) or low (pose 3) at random.  Link's
   stance doesn't matter.
2. On the first frame of the thrust (pose 2 or 4) it throws its sword
   (projectile type 5, `Enemy_FireProjectile`): from its Y (high) or Y +
   10 (low, `FokkaThrowYOffsets`), at 2 pixels per frame the way it
   faces.  The attack ends there.
3. Only if all six projectile slots are busy does it thrust instead,
   for 16 frames, its sword tested against Link (`Weapon_CheckLink`).

It can jump in the middle of a wind-up.

**The thrown sword** (`Projectile_Rock_Bank5`) flies level until it
leaves the screen or meets Link.  Link's shield stops it only with the
reflect spell (flags $86).  Knocked back, it falls.

**Its shield** (guard 1 high, 0 low, in `enemyAiVar`) turns Link's
strokes aside, as the Iron Knuckle's does:

- whenever Link presses B, it goes to his stroke's height (high
  standing, low crouching), every time, not by chance;
- while Link is in the air, it is high if he is more than 5 pixels above
  it, else low.

**Knockback** (`Enemy_KnockBackOrWalk_Bank5`): a hit pushes it back,
the push shrinking only while it stands on the floor.

## Drawing

`Fokka_Draw`:

- **Body:** two pairs from `SpritePairTiles` entry 0 (shield high) or 8
  (low), entry $10 while rising in a jump.  Walking uses the next frame
  (+4) on alternate 8-frame periods, 4-frame periods at 1 pixel per frame
  or faster.
- **Sword,** in an attack: a pair from `FokkaDrawTiles` ($14, $16, $18,
  $16 by pose), placed by `FokkaDrawXOffsets` and `FokkaDrawYOffsets`
  (32 or 16 pixels above its lower half).  The X offset is doubled for
  the lunge (timer 3-12), and pose 4 is drawn upside down.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Fokka_Ai` | 5:$9D2C | everything above |
| `Enemy_KnockBackOrWalk_Bank5`, `Enemy_Walk_Bank5` | 5:$9E42, 5:$9E38 | moving, or knockback |
| `FokkaAttackMasks`, `FokkaSpeeds`, `FokkaThrowYOffsets` | 5:$9D11 (+ type), 5:$9D26, 5:$9D28 | tables |
| `Projectile_Rock_Bank5` | 5:$9BD2 | the thrown sword |
| `Fokka_Draw` | 5:$9FD4 | drawing |

## Verification

In the Great Palace's entrance (see [Bubble](bubble.md#great-palace) for
`$GP` and `$GF`), Link standing at $B1:

- **Red Fokka,** placed at ($70, $B0):
  - It jumped at Link on frame 26 (event at 5:$9DF8) and again on 112,
    each time sailing past him at 1.5 pixels per frame.
  - It began attacks with its count at $40 and $80, 64 apart (event at
    5:$9E29).
  - It threw its sword on the first frame of each thrust (5:$9D3D), 25
    frames after the attack began.  A low throw from Y $8D flew level at
    Y $97, 2 pixels per frame.
  - Screenshot: a red bird-headed knight with a shield.

  ```
  tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $GP --until currentWorld=5 --after 200 \
      --clear $GF --approach B0 --spawn 3:19:70:B0 --projectiles --event Throw=5:9D3D \
      --event Jump=5:9DF8 --event Attack=5:9E29 --frames 300
  ```

- **Blue Fokka,** the same with type $1A: it began attacks with its count
  at $20, $40, $60, $80, $A0, $C0 and $E0, every 32 counts.  It jumped once
  during a wind-up (frame 112, pose 1).

- **Shield,** a red Fokka placed at ($A8, $B0), Link moved to $90:
  - A standing stroke on frame 70 set its guard to 1 (high) the next
    frame.
  - Crouching strokes on frames 40 and 90 set it to 0.

  ```
  tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $GP --until currentWorld=5 --after 200 \
      --clear $GF --approach B0 --link 90:B0 --spawn 3:19:A8:B0 --press 1:RIGHT:1 --press 40:DOWN+B:3 \
      --press 70:B:3 --press 90:DOWN+B:3 --frames 110
  ```
