# Doom Knocker

An armored knight that paces and throws its mace at Link.  The mace flies
out, turns, and comes back past it.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Palaces 3, 4, 6](../palaces.md) | $1E | Doom Knocker | 32 | 100 | 3 | - |

It is placed in rooms $01, $04, $0C, $10, $11, $18, $19, $1E, $23, $27 and
$2A of palaces 3, 4 and 6.  It blocks Link's projectiles and has no spawn
routine.  It is the [Hammer Thrower](hammer-thrower.md) of these palaces.

## Behavior

`DoomKnocker_Ai`.  `enemyAiState` counts only the frames it stands on the
floor:

- **Pacing:** 1/2 pixel per frame, 64 counts each way
  (`WalkSpeeds_Bank4B`).  It always faces Link.
- **Jumping:** when `frameCounter` is 0, Y velocity -3.
- **The mace:** every 128 counts it raises the mace for 15 frames, then
  throws it (`Enemy_FireProjectile`): projectile type 6, 2 pixels per
  frame at Link, level at its own Y.

**The mace** (`Projectile_Mace_Bank4B`) flies level.  After 64 frames
(128 pixels) it turns around, and then every 128 frames after that.  The
Doom Knocker doesn't catch it: it flies back past its thrower and on until
it leaves the screen or meets Link.  It is tested against Link's shield
and body.

## Drawing

`DoomKnocker_Draw`: as the Hammer Thrower's: two pairs from entry $22, or
$26 every other 16 frames on the ground, and the raised mace (entry $2A)
12 pixels behind it (`DoomKnockerDrawOffsets`).

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `DoomKnocker_Ai` | 4:$B0AD | everything above |
| `WalkSpeeds_Bank4B` | 4:$B0AA | pacing: $08, $F8 |
| `Projectile_Mace_Bank4B` | 4:$AD0D | the mace |
| `DoomKnocker_Draw` | 4:$AF26 | drawing |

## Verification

Scenario at palace 3's entrance (see [Flame](flame.md#verification) for
`$P3` and `$F3`), a Doom Knocker placed at X $C0, Link at $17 (176 pixels
away):

- It threw its mace on frames 38, 214 and 390.  It jumped in between, and
  the count stood still while it was in the air.
- The mace flew left at 2 pixels per frame for 64 frames, to X $59.  It
  turned and flew right at the same speed, past the Doom Knocker
  (X $E9 on frame 178), and off the screen.
- With Link close by ($80), each mace stopped on him and knocked him
  back.
- Screenshot: a blue knight.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $P3,statLevel+2=8,statContainers+1=8 \
    --until currentWorld=4 --after 200 --clear $F3 --spawn 0:1E:C0:A0 --projectiles \
    --event Throw=4:B0CD --frames 400
```
