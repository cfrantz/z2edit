# Orange Moa

A flying eye that swoops in from the side Link is facing, turns back
while climbing, and drops flames on its way out.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Palaces 1, 2, 5](../palaces.md) | $07 | Orange Moa | 16 | 50 | 3 | 2 |
| [Palaces 3, 4, 6](../palaces.md) | $07 | Orange Moa | 16 | 50 | 3 | 2 |

It is placed in rooms $19, $2A and $2B of palaces 1, 2 and 5, and $03,
$06, $0F, $25, $2B, $2F, $34 and $3D of palaces 3, 4 and 6.  Touching it
drains experience.  In palaces 1, 2 and 5 it comes back.  It has no spawn
routine; its list position matters only for which page it is on.  It is
drawn by `Moa_Draw`, as the overworld [Moa](moa.md).

## Behavior

`OrangeMoa_Ai`.  `enemyAiState` & 3 is the state:

1. **Waiting** (`OrangeMoa_Appear`).  When its timer ends (at once the
   first time), it appears at the edge of the screen Link is facing
   (`OrangeMoaSpawnX`), at Y $7D.  It flies inward at 2 pixels per frame
   (`OrangeMoaSpeeds`), for 16 frames, or 80 if Link is standing still.
2. **Coming in** (`OrangeMoa_Wait`).
3. **Rising** (`OrangeMoa_Rise`), for 64 frames: up 1 pixel per frame,
   while its X velocity changes by 1/16 per frame toward the side it came
   from (`OrangeMoaAccel`).  It slows, stops and turns back, ending at 2
   pixels per frame the other way, about 64 pixels higher.
4. **Flying out** (`OrangeMoa_Fly`).  It drops a flame
   (`Enemy_DropFlame_Bank4`) on frames where `frameCounter` & $2F is 0
   (2 in every 64), unless it is touching anything.  Once off screen it
   waits 191 frames and comes again.

In states 1-3 it bobs (Y velocity ±4/16, 16 frames each way), is drawn,
and is tested for hits and contact.  While waiting it is neither drawn
nor tested.

**Flames** (projectile type 7, `Projectile_Flame_Bank4`): they have no
speed, fall under gravity of 1/16 and stop on the floor, and burn for 255
frames, with a sound.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `OrangeMoa_Ai` | 4:$B909 | bobbing, states |
| `OrangeMoa_Appear`, `OrangeMoa_Wait`, `OrangeMoa_Rise`, `OrangeMoa_Fly` | 4:$B93C, 4:$B96D, 4:$B97C, 4:$B991 | the states |
| `Enemy_DropFlame_Bank4` | 4:$B9A4 | a flame |
| `Projectile_Flame_Bank4` | 4:$98B9 | the flame burning |
| `Moa_Draw` | 7:$EFF2 | drawing |

## Verification

Scenario in palace 1's first room (see [Bubble](bubble.md#verification),
with Link's life level raised), an Orange Moa placed at X $1D0, Link
standing at $1A0 facing right:

- It appeared at once at the right edge (X $222), Y $7D, flying left at
  2 pixels per frame.  After 16 frames it rose to Y $40 over 64 frames,
  its X velocity going from -$20 to +$1F, and flew out to the right.
- Off screen on frame 102, it came again 191 frames later, this time
  coming in for 80 frames (Link standing still).
- Its second pass dropped flames at `frameCounter` $90, $C0 and $D0.  The
  first fell from Y $3C at X $1A0 and burnt for 255 frames.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $PAT,statLevel+2=8,statContainers+1=8 \
    --until currentWorld=3 --after 200 --clear $PFLAT --approach 1A0 --spawn 0:07:1D0:90 --projectiles \
    --event Appear=4:B957 --event Rise=4:B972 --event Fly=4:B98E --event Drop=4:B9A9 --frames 700
```
