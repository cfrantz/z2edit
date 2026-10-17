# Carock

The boss of palace 4: a wizard who blinks in and out all over the room,
firing beams.  Only his own beams, sent back by the reflect spell, can
hurt him.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Palaces 3, 4, 6](../palaces.md) | $22 | Carock | 16 | 301 | 3 | - |

He is placed in room $1C of palaces 3, 4 and 6.  `Carock_Init` removes
him once the room's flag shows him beaten.  The fight is described with
the [palace bosses](../palaces.md#bosses).

## Behavior

`Carock_Ai` runs the [Wizzrobe](wizzrobe.md)'s code on a faster cycle,
49 frames, `enemyAiState` counting down.  He doesn't wait for
`TIMER_BOSS`.

| Count | Frames | What he does |
|-------|-------:|--------------|
| $FF-$F0 | 16 | blinking in |
| $EF-$E0 | 16 | solid (`Wizrobe_Animate`): harmful to touch; a beam at $E8 |
| $DF-$D0 | 16 | blinking out |
| $CF | 1 | a new random X on page 1 (`Wizrobe_Restart`) |

- **The beam** (`Wizrobe_FireBeam` with a = 2): projectile type 9 along
  the floor, from 26 pixels ahead facing right, 3 facing left.
- **No hit tests.**  `Boss_CheckDeath_Bank4B` skips him, so the sword and
  Link's projectiles pass through him.  A beam sent back by the reflect spell
  (`Projectile_CarockBeam`) does 1 hit if it meets him while he is solid
  (2 hit points per bar block: 16 in all).  When it kills him, he is kept
  as a dying boss for `Boss_UpdateIntro_Bank4B`.

## Drawing

`Carock_Draw`: 32 pixels wide, a column of three pairs (entry $A0 while
blinking in, else $AC) and a pair beside it ($A6 or $B2), on the side
away from his facing.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Carock_Init` | 4:$B107 | spawning |
| `Carock_Ai` | 4:$AE7B | everything above |
| `Wizrobe_FireBeam`, `Projectile_CarockBeam` | 4:$AED3, 4:$AD29 | the beam |
| `Carock_Draw` | 4:$B439 | drawing |

## Verification

Scenario at palace 3's entrance (see [Rebonack](rebonack.md#verification)
for the fill), Carock placed at ($1C0, $A0), Link at $1A0 with
`reflectSpellActive` set:

- He fired at count $E8 every 49 frames (frames 24, 73, 122, ...), and
  reappeared at a new X on page 1 each time ($12C, $11E, $1A5, ...).
- On frame 275 his beam, fired with Link 9 pixels away, came straight
  back off the shield and hit him (`Enemy_TakeDamage`) while still solid.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $P3,statLevel+2=8,statContainers+1=8 \
    --until currentWorld=4 --after 200 --clear $F3 --approach 1A0 --spawn 0:22:1C0:A0 \
    --poke reflectSpellActive=1 --projectiles --event Beam=4:AEA1 --event Dmg=7:E726 --frames 700
```

In the no-glitch movie (`../private/Zelda2_NoGlitch_2014_Final.fm2:0`), every hit on Carock (frames 95477-95820,
2 hit points each) came from a projectile (`temp11` 1): his reflected
beams.  He burst on frame 95948.

**Drawing.**  Palace 3 has no graphics for Carock (he is invisible
there).  Started at palace 4's entrance instead
(`currentRegion=1,palaceCode=0,areaLocation=0x34`, the fill to X $25F),
the screenshots on frames 170-225 show him as a red caped figure about
32 pixels wide, appearing at each new place.
