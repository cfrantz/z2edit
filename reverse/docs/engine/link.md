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
speed makes Link crouch for a few frames (`linkLandingTimer`).

## Fairy

As a fairy (`linkFairy` = 8, from the fairy spell), Link flies in the
direction held on the D-pad.  He does not move in a direction that is
blocked (`objCollision`), nor up above Y = $18.  His collision probes use
the fairy's offsets.

## Status

The numbers above are read from the code.  Over the first 20000 frames of
the no-glitch movie (`tools/watch.py`), Link's X velocity stays within
$18 either way and his Y velocity within -4 to 5, as expected; the jump
spell's velocities were not exercised there.
