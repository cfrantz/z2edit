# Helmethead

The boss of palace 2: a knight in a tower of helmets.  Each blow to his
head knocks a helmet off, and the loose helmets fly about firing at Link.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Palaces 1, 2, 5](../palaces.md) | $21 | Helmethead | 48 | 200 | 2 | - |
| [Palaces 1, 2, 5](../palaces.md) | $22 | Helmethead's helmet | 80 | 0 | 1 | - |

He is placed in room $22 of palaces 1, 2 and 5.  Type $21 is
[Gooma](gooma.md) in palace 5: `Helmethead_Init` (after `Boss_Init`) sets
his hit points, and type $21's attributes, damage and hitbox, by region
(`HelmetheadHp`, `HelmetheadAttrs`, ...).  So in play he gives 200
experience and has damage class 2, not the table's 150 and 3.  The fight
is described with the [palace bosses](../palaces.md#bosses).

## Behavior

`Helmethead_Ai` (in palace 5, `Gooma_Ai`):

1. `Boss_UpdateIntro`, `Boss_UpdateCommon`, and `Boss_Pace` with 6 hit
   points per bar block: he paces like [Horsehead](horsehead.md), but
   never swings.  His guard (2) leaves only his head open.
2. **Energy balls.**  Every 128 frames he fires one (projectile type 2)
   from 8 pixels higher, sinking at 2/16 pixel per frame.
3. **Helmets.**  When a stroke gets through (`objCollision` bit 5) and
   `helmetheadHitDelay` is 0, if he still has a helmet (`objState+1`
   counts the lost ones, up to 2), one flies off.  A helmet ($22) is
   launched from 12 pixels above him, up at -3 and backward at 1 pixel
   per frame.  The delay is set to 16, counting down every 4 frames (64
   frames).

**A loose helmet** (`HelmetheadHead_Ai`) is drawn and harmful to touch,
but is never tested for hits: it can't be destroyed.

- It falls and bounces on the floor (Y $C4) three times, up at -2, -1
  and -1/2, its X speed halved each time, with a sound.
- It lies there until its 48-frame timer runs out, then rises 2 pixels
  per frame to Y $70.
- From then on it flies up and down in a wave (between Y $4A and $6E or
  so), drifting toward Link at 6/16 pixel per frame from each
  `frameCounter` 0.  Every 128 frames it fires an aimed ball
  (`Enemy_FireAimedBall`) when Link is in range.

## Drawing

`Helmethead_Draw`: his body, two pairs from entry $6C ($70 in part of
each walking step); then his head, a pair from entry $74 + 2 × helmets
lost, 12 pixels above, lowered by `helmetheadHitDelay` just after a hit.
A loose helmet (`HelmetheadHead_Draw`) is drawn from the entry it had on
his head.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Helmethead_Init` | 4:$BC7E | spawning, by region |
| `Helmethead_Ai` | 4:$BAC3 | everything above |
| `HelmetheadHead_Ai` | 4:$BCEF | a loose helmet |
| `Helmethead_Draw`, `HelmetheadHead_Draw` | 4:$BD75, 4:$BDD0 | drawing |

## Verification

Scenarios in palace 1's first room (see [Bubble](bubble.md#verification),
with Link's life level raised):

- Helmethead placed at X $1C8 (guard and hitbox poked in): he fired
  energy balls on frames 45, 173, 301, ... every 128 frames, and paced
  as Horsehead does.  The standing and jumping strokes scripted there
  only clinked on his guard (in the movie, some strokes did get through).
- **Knocking helmets off** (the same, with the downward thrust learned,
  `techniques=0x10`; Link jumping at him and holding Down, as in the
  movie's first hit):
  - The thrusts on frames 76 and 176 each took 2 hit points and launched
    a helmet, into enemy slots 5 and 4.  `objState+1` went to 1, then 2.
  - The thrusts on frames 626 and 776 only took hit points ($2C, then
    $28): no helmets were left.
  - Each loose helmet had 208 hit points.  The launch code stores its
    upward speed, $D0, into `enemyHp` as well, so the table's 80 is never
    used.  It changes nothing: a loose helmet is never tested for hits,
    and the 224-point hit when Helmethead bursts still kills it.

  ```
  P=""; for k in $(seq 10 50 900); do P="$P --press $k:A+RIGHT:14 --press $((k+14)):DOWN+RIGHT:6 --press $((k+20)):DOWN:20"; done
  tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $PAT,statLevel+2=8,statContainers+1=8,techniques=0x10 \
      --until currentWorld=3 --after 200 --clear $PFLAT --approach 1A0 --spawn 1:21:1D0:B0 \
      --poke enemyAiVar+1=2 --poke sramEnemyHitbox+33=7 $P --event Struck=4:BB05 --event Launch=4:BB4D \
      --frames 900 --dump 177:enemyHp:6 --dump 177:objState+2:1
  ```
- A loose helmet placed at ($1E0, $A0), flung up at -3 and right at
  $10: it bounced on frames 59, 93 and 111, rose at frame 160 to Y $70,
  then drifted toward Link in a wave.  It fired aimed balls on frames
  328, 456 and 584.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $PAT,statLevel+2=8,statContainers+1=8 \
    --until currentWorld=3 --after 200 --clear $PFLAT --approach 1A0 --spawn 1:22:1E0:A0 \
    --poke objYVelocity+2=0xD0 --poke objXVelocity+2=0x10 --projectiles \
    --event Bounce=4:BD07 --event Aimed=4:BD6F --frames 700
```

In the no-glitch movie (`../private/Zelda2_NoGlitch_2014_Final.fm2:0`): the first hit, on frame 55650, knocked a
helmet off, and the second came off on frame 55712, after the 64-frame
delay; the hits in between only took hit points (4 each).  The first hit
came from Link's downward thrust (`objState` 9), the second from a
stroke (`objState` 7).  When Helmethead burst (frame 55844), both loose
helmets took the 224-point hit (`Enemy_TakeDamage` for slots 3 and 4).
