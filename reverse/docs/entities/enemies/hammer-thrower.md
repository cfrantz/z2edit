# Hammer Thrower

An armored knight that paces back and forth and lobs hammers at Link in
high arcs.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Palaces 1, 2, 5](../palaces.md) | $1E | Hammer Thrower | 16 | 50 | 2 | 2 |

It is placed in rooms $02, $08, $0C, $1C, $1E, $1F, $2E and $36 of
palaces 1, 2 and 5.  It blocks Link's projectiles and has no spawn
routine.  Palaces 3, 4 and 6 have the [Doom Knocker](doom-knocker.md) in
its place.

## Behavior

`HammerThrower_Ai`.  `enemyAiState` counts every frame:

- **Pacing:** 1/2 pixel per frame, 64 frames right and 64 left (by bit 6
  of the count, `HammerThrowerSpeeds`), whichever way it faces.  Walls
  stop it (`Enemy_KnockBackOrWalk_Bank4`).
- **Facing:** it turns toward Link every 16 frames.
- **Jumping:** when `frameCounter` is 0, Y velocity -3.
- **Hammers:** at counts $00, $20, ... $C0 (7 in every 256 frames, none
  in the last 40), it raises a hammer (`Enemy_StartSwing`, 15 frames).
  When the timer reaches 1 it throws it: projectile type 6, 1 pixel per
  frame the way it faces, Y velocity -$2B.  The hammer falls under
  gravity of 1/16 (`Projectile_Fall_Bank4`), rising about 59 pixels.

It is drawn, tested for hits and contact (`Enemy_UpdateWithBg`), and
hurts Link on contact.

## Drawing

`HammerThrower_Draw`: two pairs from `sramEnemyTiles` entry $22, or $26
every other 16 frames on the ground.  While a hammer is raised it is
drawn over its back: a pair from entry $2A at its Y, 12 pixels behind it
(`HammerThrowerDrawOffsets`).

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `HammerThrower_Ai` | 4:$BA57 | everything above |
| `HammerThrowerSpeeds` | 4:$BA54 | pacing: $08, $F8 |
| `Projectile_Fall_Bank4` | 4:$9940 | the hammer |
| `HammerThrower_Draw` | 4:$97E1 | drawing |

## Verification

Scenario in palace 1's first room (see [Bubble](bubble.md#verification),
with Link's life level raised), a Hammer Thrower placed at X $1D0, Link
at $1A0:

- It raised hammers at counts $00, $20, ... $C0 and threw each 14 frames
  later, 7 per 256 frames, none at $E0.
- A hammer left at X velocity $F0 and Y velocity -$2B, peaked 43 frames
  later and fell.
- It paced between X $1D0 and $1F0, 64 frames each way, and jumped when
  `frameCounter` reached 0.
- Screenshot: a red knight with a hammer held up over its back.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $PAT,statLevel+2=8,statContainers+1=8 \
    --until currentWorld=3 --after 200 --clear $PFLAT --approach 1A0 --spawn 0:1E:1D0:90 --projectiles \
    --event Swing=4:BA6A --event Throw=4:BA7B --frames 600
```
