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

Link's attack (`Link_UpdateAttack`) runs through `linkAttackFrame`,
standing or crouching; the sword's position (`swordX`, `swordY`,
`Sword_UpdatePosition`) and hitbox (`Sword_GetHitbox`) follow his pose.
In the air, holding Up on the way up gives the upward thrust (objState
8) and holding Down the downward thrust (objState 9), once learned
(`techniques` bits `TECH_UPWARD_THRUST`, `TECH_DOWNWARD_THRUST`, taught
in Darunia and Mido); the thrusts use their own hitbox offset.

At full life the sword also fires a beam (`LinkProjectile_Update`,
object slot 13; slot 14 too from attack level 4), unless the fire spell
is active, which fires its flame instead.  A beam that hits something
turns into a spark (`LinkProjectile_Spark`).

`techniques` also holds two quest flags (bits 0 and 1; see
[Towns](towns.md)): bit 0 makes the bridge in Saria build itself when
Link reaches it (`Saria_BuildBridge`), and is then cleared.

## Spells

`Spell_Update` (each frame in side view) casts the spell at
`spellCursor` when Select is pressed, if Link owns it, it is not the spell
last cast, and he has the magic: the cost (`SpellCosts`, by spell
and magic level) is taken from the magic meter (32 units per container),
the spell's bit is set in `spellActive`, and the screen flashes.  A spell
costing one more than the magic left is still cast (QUIRK).  The effects
run each frame from `Spell_RunEffect`.

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
head for 112 frames (`itemHoldTimer`, the item in `heldItem`, drawn by
`Sword_UpdateHitPosition` from `HeldItemTiles`), with the item fanfare.


As a fairy (`linkFairy` = 8, from the fairy spell), Link flies in the
direction held on the D-pad.  He does not move in a direction that is
blocked (`objCollision`), nor up above Y = $18.  His collision probes use
the fairy's offsets.

## Status

The numbers above are read from the code.  Over the first 20000 frames of
the no-glitch movie (`tools/watch.py`), Link's X velocity stays within
$18 either way and his Y velocity within -4 to 5, as expected; the jump
spell's velocities were not exercised there.
