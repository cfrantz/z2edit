# Link in the Side View

Link is object slot 0 (see [object slots](../memory/objects.md)).  Each
side view frame, `Mode_SideView` runs his spells (`Spell_Update`) and
then his movement (`Link_Run`), both in bank 0.

## Modes

`Link_Run` dispatches on `linkMode` (`LINK_MODE_*`):

| Mode | Routine | |
|------|---------|---|
| 0 | `Link_EnterArea` | place Link in a new area, at the edge given by `areaEntrySide`, facing inward, or where an elevator or door puts him |
| 1 | `Link_Control` | under the player's control |
| 2 | `Link_Sink` | sink one pixel per frame (lava); below Y = $D0 he dies |

Link dies when `linkDeathPending` is set; `Mode_SideView` then starts the
death sequence (`GAME_STATE_DEATH`).

## Units

Velocities are signed fixed point.  Horizontal velocity (`objXVelocity`)
is 4.4: $10 is one pixel per frame.  Link's vertical velocity
(`objYVelocity`) is whole pixels per frame, and `objYFrac` accumulates
gravity in 1/256 pixel per frame per frame (`Obj_MoveYWithGravity`).

## Walking

`Link_UpdateXVelocity`:

- While entering an area, Link walks in at speed $10 (1 pixel per frame).
- While standing (`linkStanding`) with a direction held, he accelerates by
  1 per frame, or 2 when reversing, up to $18 (1.5 pixels per frame)
  (`LinkWalkAccel`, `LinkWalkMaxSpeed`).
- Otherwise, on the ground, he slows down by 1 per frame.  Crouching
  therefore brings him to a stop.

Walking into a wall (`objCollision` bits 1-0) stops him.  The walking
animation (`Link_AnimateWalk`) has three frames, advanced faster at higher
speeds (`LinkWalkFrameTimes`).

## Jumping

`Link_UpdateJump`: on the ground, pressing A jumps with an initial
velocity of

| | standing | running (\|X velocity\| >= $13) |
|---|---|---|
| jump spell | -4.5 | -5.0 |
| normal | -3.06 | -3.5 |

pixels per frame (`LinkJumpVelocity`, `LinkJumpVelocityFrac`).  Gravity is
$48/256 pixel per frame per frame, or $30/256 while A is held on the way
up, so holding A jumps higher.  The terminal velocity is 5 pixels per
frame.  Hitting a ceiling on the way up stops the rise.  Landing at full
speed makes Link crouch for a few frames (`linkLandingTimer`).  Once the
techniques are learned (`techniques`, `TECH_*`), holding Up on the way up
gives the upward thrust, and holding Down in the air the downward thrust.

## Taking Damage

When an enemy touches Link (`Enemy_CheckLinkContact`, then
`Link_HurtByEnemy`), he loses `LinkDamage[class][life level]` from his
life, where the class is the low nibble of the enemy type's
`sramEnemyDamage` entry.  Higher life levels take less damage; the shield
spell halves it.  He is knocked back (`TIMER_LINK_HURT`), bounced upward
(Y velocity -2) and stays invulnerable for 80 frames (`TIMER_INVULNERABLE`),
blinking at the end.  Some enemies (`sramEnemyAttr` bit 4) also drain
experience, one point per frame for 10 frames (`experienceDrain`).  If
his life runs out, `linkDeathPending` starts the death sequence.

## Experience and Levels

Killing an enemy adds its experience to `experiencePending`, which
`Mode_SideView` counts into `experience` at 10 points per frame.
`experienceNext` is the cheapest next level of the three stats
(`Exp_ComputeNext`, from `LevelUpCostHi`/`LevelUpCostLo`):

| Stat | 1->2 | 2->3 | 3->4 | 4->5 | 5->6 | 6->7 | 7->8 |
|------|-----:|-----:|-----:|-----:|-----:|-----:|-----:|
| attack | 200 | 500 | 1000 | 2000 | 3000 | 5000 | 8000 |
| magic | 100 | 300 | 700 | 1200 | 2200 | 3500 | 6000 |
| life | 50 | 150 | 400 | 800 | 1500 | 2500 | 4000 |

(Each table has an eighth entry, 9000.)  When `experience` reaches
`experienceNext`, `Hud_Update` opens the level up dialog
(`DIALOG_LEVEL_UP`).  Continuing after a game over resets the experience
to 0.

## The Sword

B starts an attack (`Link_ReadInput`, `Link_UpdateAttack`), standing or
crouching (`linkAttackStanding`).  On the ground it stops Link (unless he
is recoiling from a wall) and he keeps still until it ends.

| Frame (`linkAttackFrame`) | Standing | Crouching |
|---|---|---|
| 1 | winding up (pose 4), 6 frames | the stroke (pose 7), 6 frames, with the sound |
| 2 | the stroke (pose 5), 6 frames, with the sound | crouching (pose 6), 6 frames |
| 3 | standing (pose 3), 6 frames | |
| 4 | the sword held back (pose 4), 32 frames | |

While the sword is held back, B strikes again at once (back to frame 2),
and A, Down, Left or Right lowers it.  Landing ends an attack made in the
air.  The sword's position (`swordX`, `swordY`, `Sword_UpdatePosition`)
and hitbox (`Sword_GetHitbox`) follow the pose.  In the air, holding Up
on the way up gives the upward thrust (pose 8) and holding Down the
downward thrust (pose 9), once learned (`techniques` bits
`TECH_UPWARD_THRUST`, `TECH_DOWNWARD_THRUST`, taught in Darunia and Mido);
the thrusts use their own hitbox offset.

At full life (`LinkProjectile_Update`) each stroke also fires a beam,
in object slot 13, on the stroke's first frame, unless the fire spell is
active, which fires its flame instead (slots 13-14 from attack level 4).
The beam starts 13 pixels ahead of Link, at his Y (10 pixels lower
crouching), and flies 2 pixels per frame for 32 frames, with a changing
palette.  Then it sparks for 7 frames (`LinkProjectile_Spark`), as it does
when it hits something.  The flame is tile $84, flipped vertically every 4
frames, and flies until it leaves the screen, with its sound each frame.
A scripted attack at full life from North Castle showed the beam's 32
frames, the spark's 7 and the attack's frame times above.

`techniques` also holds two quest flags (bits 0 and 1; see
[Towns](../world/towns.md)): bit 0 makes the bridge in Saria build itself when
Link reaches it (`Saria_BuildBridge`), and is then cleared.

## Spells

`Spell_Update` (each frame in side view) casts the spell at
`spellCursor` when Select is pressed, if Link owns it, it is not the spell
last cast (`spellLastCast`, cleared when the pause menu opens), and he has
the magic: the cost (`SpellCosts`, by spell and magic level) is taken from
the magic meter (32 units per container), the spell's bit is set in
`spellActive` (until the next area), and Link's palette flashes for 32
frames (`spellFlash`, `Link_UpdatePalette`); for Reflect, Spell and
Thunder the backdrop flashes too.  A spell costing one more than the
magic left is still cast and leaves 0 (QUIRK; checked with a scripted
cast of Shield).  The effects run each frame from `Spell_RunEffect`:

| Spell | Effect |
|---|---|
| Shield | Link's shield color, and damage halved (`shieldSpellActive`) |
| Jump | higher jumps (`jumpSpellActive`) |
| Life | once: 48 more life to refill (`statMeterPending`; checked with a scripted cast) |
| Fairy | Link becomes the fairy once he is below Y $20 |
| Fire | a flame with each stroke |
| Reflect | the shield reflects shots (`reflectSpellActive`) |
| Spell | once: the enemies on screen become Bots (`Spell_TurnEnemiesToBots`), except in towns; on New Kasuto's east street (area $14) it opens the big door (`BigDoor_Update`) |
| Thunder | once: removes the enemy projectiles and hits every enemy on screen for 50, or 2 (flags bit 5), unless immune (flags bit 7) |

Costs, by magic level:

| Spell | Level 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|-------|--:|--:|--:|--:|--:|--:|--:|--:|
| Shield | 64 | 48 | 48 | 32 | 32 | 32 | 32 | 32 |
| Jump | 96 | 80 | 64 | 64 | 40 | 32 | 24 | 16 |
| Life | 140 | 140 | 120 | 120 | 100 | 100 | 100 | 100 |
| Fairy | 160 | 160 | 120 | 120 | 80 | 80 | 80 | 80 |
| Fire | 240 | 160 | 120 | 60 | 32 | 32 | 32 | 32 |
| Reflect | 240 | 240 | 160 | 96 | 80 | 64 | 48 | 32 |
| Spell | 240 | 224 | 192 | 160 | 96 | 64 | 48 | 32 |
| Thunder | 240 | 240 | 240 | 240 | 240 | 240 | 200 | 128 |

## Holding Up an Item

Collecting a special item (`Item_Collect`) makes Link hold it over his
head for up to 112 frames (`itemHoldTimer`, the item in `heldItem`, drawn
by `Sword_UpdateHitPosition` from `HeldItemTiles`), with the item
fanfare; he regains control when the fanfare ends.  His Y is rounded to
the tile row.

## The Fairy

As a fairy (`linkFairy` = 8, from the fairy spell), Link flies 1.5
pixels per frame in the direction held on the D-pad, diagonals too
(`LinkControlXVelocities`, `LinkControlYVelocities`), requesting pulse 1
effect $02 (the "item dropped" sound) each frame.  He does not move in a
direction that is blocked (`objCollision`, `blockedDirections`), nor up
above Y = $18.  His collision probes use the fairy's offsets.

## Poses and Drawing

Link's pose is `objState` of slot 0 (`LINK_POSE_*`; checked over the
100% movie and screenshots):

| Pose | |
|------|---|
| 0-2 | walking frames; 2 also falling after a jump |
| 3 | standing (`LINK_POSE_STAND`) |
| 4 | standing stroke, sword raised (`LINK_POSE_WIND_UP`), drawn 8 pixels back |
| 5 | standing stroke, sword out (`LINK_POSE_STROKE`) |
| 6 | crouching, rising in a jump, and landing hard (`LINK_POSE_CROUCH`); 4 pixels higher in the air |
| 7 | crouching stroke (`LINK_POSE_CROUCH_STROKE`) |
| 8 | upward thrust (`LINK_POSE_UP_THRUST`) |
| 9 | downward thrust (`LINK_POSE_DOWN_THRUST`) |
| $0A | hurt (`LINK_POSE_HURT`) |
| $0B | after the Dark Link fight (`LINK_POSE_ENDING`): the right column mirrored |
| $0C-$0E | holding up an item (`LINK_POSE_HOLD_UP`): $0C in side view areas, $0D in towns (never seen), $0E in palaces |

`Link_Draw` draws him as two rows of three 8x16 sprites at `oamBuffer`
$08-$1F (`Link_DrawSprites`, one call per row), with the tiles of his pose
(`LinkBodyFrames`, `LinkSpriteTiles`).  The tiles face left; facing right
they are mirrored and drawn 8 pixels further right.  A blank tile ($F5)
hides its sprite.  While hurt, the palette cycles every 2 frames; while
entering an area or sinking, and on the bottom row while wading, he is
behind the background.

In poses 5 and 7 a seventh sprite, the sword (tile $32 at the top row,
or $54 at the bottom row), goes at `oamBuffer` $20, which is also the
first enemy projectile's slot.  It is 20 pixels beyond the body (8
pixels to the left of it facing left), hidden if that wraps past the
screen edge, and shares Link's attributes.  Its position becomes the
sword's hit position (`swordX`, `swordY`); for the thrusts the hit
position is computed instead (`Sword_UpdatePosition`: 4 pixels above
him for pose 8; below his feet for pose 9).  Holding up an item, the item
is two sprites at `oamBuffer` $20 and $24, 16 pixels above him.

Dark Link is drawn by the same code with `swordOwner` 1, his own sprite
offset and `extraSpriteAttrs` for his palette.  Some steps still read
Link's variables: the vertical off-screen test, the blank tile check
(Link's six sprites), and the fairy.

## Status

The numbers above are read from the code.  Over the first 20000 frames of
the no-glitch movie (`tools/watch.py`), Link's X velocity stays within
$18 either way and his Y velocity within -4 to 5, as expected; the jump
spell's velocities were not exercised there.

Over both movies (exec hooks on `Link_Control` and its helpers): jumps
rise in pose 6 and fall in pose 2; hard landings (5 pixels per frame)
crouch Link; the ceiling clamp, re-strikes from the held-back sword and
its cancels, the up and down thrusts, the fairy's flight and the
one-short spell cast (three times in each movie) all occur.  Holding up
an item uses pose $0C and $0E, never $0D.
