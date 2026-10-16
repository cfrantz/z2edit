# Rock Thrower

A creature behind a tall fence that peeks over the top, throws rocks in
an arc at Link, and ducks back down to come up somewhere else.  z2edit
calls it the "Fence Lizalfos".

## Where It Appears

| Group | Type | HP | Exp | Damage class | Drop group |
|-------|------|---:|----:|-------------:|-----------:|
| [East Hyrule](../east-hyrule.md) | $1D | 1 | 50 | 2 | - |

Only bank 2's group has it: East Hyrule areas $02-$05 (location $05 is
area $05), three to an area, along the fence.  It blocks Link's
projectiles (damage bit 5).  It has no spawn routine.

## Behavior

`RockThrower_Ai` turns it toward Link (`Enemy_FaceLink`), plays out a
throw, draws it, and moves it by `enemyAiState` bits 1-0:

| State | What it does |
|-------|--------------|
| 0, hidden | when its timer runs out, moves to a spot ahead of where Link is heading (`scrollX` + $40 + 0-127 + 3 times Link's X velocity) and waits 36-67 frames |
| 1, rising | rises 1 pixel per frame to Y $4C until the wait is over |
| 2, rising | rises on to Y $45, then stays up for 64-127 frames |
| 3, up | throws, one throw after another, while more than 16 frames are left; in the last 16 it sinks 16 pixels; then it waits 64 frames hidden |

**A throw** (`objState+1`, steps of 16 frames counted in `enemyAiVar`):

1. a wind-up (1), rock in hand;
2. the rock (projectile type 8, `Enemy_DropRock_Bank2`), and a
   follow-through (2).

The rock's X velocity is a quarter of Link's distance, in 1/16 pixels
per frame, plus Link's own X velocity, so it leads him.  Its Y velocity
depends on Link's height (`RockThrowYSpeeds`, by his Y / 32: -$24, -$1C,
-$14, -$0C, -8, 0, 0, 0), throwing harder up the higher he is.  The rock
falls under gravity of 1/16 (`Projectile_FallingRock_Bank2`) and bursts
on the ground.  Its flags ($94) let it pass through Link's shield.

Its single hit point means any hit kills it, but it shows only above
the fence.

## Drawing

`RockThrower_Draw`: a pair of sprites from `sramEnemyTiles` entry $2C,
with the priority bit set, so the fence (background) hides it.  While it
throws it adds:

- its arm, tile $23, 16 (wind-up) or 13 (follow-through) pixels above
  its Y.  In the follow-through the arm is mirrored and flipped.  It is
  shifted by `RockThrowerDrawTiles`, doubled in the wind-up's last 7
  frames and in the follow-through;
- in the wind-up, the rock, tile $AE, beside the arm.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `RockThrower_Ai` | 2:$9730 | everything above |
| `RockThrowYSpeeds` | 2:$9728 | rock Y velocity by Link's Y / 32 |
| `Enemy_DropRock_Bank2` | 2:$9A11 | the rock |
| `RockThrower_Draw` | 2:$9D32 | drawing |

## Verification

Neither test movie meets one.  In East Hyrule's area $05 (location $05),
with the area's own enemies, Link at the right end (X $3C8, Y $B0):

- One thrower rose from Y $54 to $4B and on to $44.  It threw at frames
  49 and 83, each about 17 frames after its wind-up began.  It sank to
  $54 by frame 105 and waited.
- At frame 169 it chose a new spot, X $3AD, and repeated; then $34D.
- Screenshots: a tall wooden fence; the thrower's head and arm over its
  top while it throws, a rock flying toward Link; nothing above the fence
  while it is down.

```
EAT=areaPrgBank=2,currentRegion=2,currentWorld=0,townCode=0,palaceCode=0,areaLocation=0x05,areaId=0x05,areaStartPage=0,areaEntrySide=0,prevRegion=2
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $EAT --until areaId=0x05 --after 60 \
    --log 0,1,2,3,4,5 --projectiles --event Throw=2:974D --event Spot=2:97A2 --frames 500
```
