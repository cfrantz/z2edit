# Barba

The boss of palace 6: a dragon who rises out of the lava at one of three
places, breathes a burst of fireballs at Link, and sinks again.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Palaces 3, 4, 6](../palaces.md) | $21 | Barba | 192 | 700 | 5 | - |

He is placed in room $3A of palaces 3, 4 and 6.  `Barba_Init` starts him
hidden below the screen (Y $F0) and, through `Carock_Init`, removes him
once the room's flag shows him beaten.  The fight is described with the
[palace bosses](../palaces.md#bosses).

## Behavior

`Barba_Ai`: `Boss_UpdateIntro_Bank4B`, drawing and hit tests
(`Barba_DrawAndCheck`), 24 hit points per bar block, facing Link.
Nothing more until 255 frames after the fight starts (`TIMER_BOSS`).
Then, by `barbaStep`:

| Step | Routine | What he does |
|------|---------|--------------|
| 0 | `Barba_Wait` | when his timer runs out, moves to X $18, $6B or $B8 (`BarbaXPositions`: 3, 2 and 3 chances in 8) |
| 1 | `Barba_Rise` | rises 3 pixels per frame to Y $80 |
| 2 | `Barba_Breathe` | sways for 160 frames; while the timer is $40-$4F, a fireball every other frame |
| 3 | `Barba_Sink` | sinks 2 pixels per frame to Y $F0, then waits 96 frames |

**Fireballs:** projectile type 4, aimed at Link (`Enemy_FireAimedBall`)
at four times the usual speed, from his head, 3 pixels higher.  At most
8 per rise; the aimed ball is cancelled when Link is behind him or too
far.

## Drawing

`Barba_Draw`: his neck, a column of pairs from his Y down to the lava
(Y $E8), swaying (`BarbaNeckXOffsets`, doubled while he breathes); then
his head beside the top segment (`barbaHeadX`, `BarbaHeadXOffsets`).
While he breathes fire the mouth opens (`BarbaHeadFrames`,
`BarbaNeckFrames`).

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Barba_Init` | 4:$B0FE | spawning |
| `Barba_Ai` | 4:$B11F | the steps |
| `Barba_Wait`, `Barba_Rise`, `Barba_Breathe`, `Barba_Sink` | 4:$B140, 4:$B154, 4:$B16A, 4:$B1C8 | each step |
| `Barba_Draw` | 4:$B290 | drawing |

## Verification

Scenario at palace 3's entrance (see [Rebonack](rebonack.md#verification)
for the fill), Barba placed at ($1B8, $F0), Link at $1A0:

- He picked X $1B8 on frame 255 (after the wait), rose until frame 294,
  and fired 6 fireballs on frames 376-386.
- He sank on frame 454, waited from 520, and rose again at $118 (frame
  655).  He fired 6 more on frames 736-746.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $P3,statLevel+2=8,statContainers+1=8 \
    --until currentWorld=4 --after 200 --clear $F3 --approach 1A0 --spawn 0:21:1B8:F0 --poke barbaStep=0 \
    --projectiles --event Pick=4:B14E --event Fire=4:B19A --event Sink=4:B1BF --frames 1000
```

**Drawing.**  In palace 3 Barba is drawn with the wrong graphics (palace
3's sprite tiles).  The same scenario started at palace 6's entrance
(`currentRegion=2,palaceCode=1,areaLocation=0x35` in place of palace 3's
values; the fill to X $25F) showed him right on frame 378, as a column of
neck segments rising from the lava with his head turned toward Link,
breathing fire.  With `--projectiles`, his fireballs flew down at Link and, meeting the
floor at Y $C9 (frame 393), became sparks (type $F2 counting up;
`Projectile_Fireball_Bank4B`).
