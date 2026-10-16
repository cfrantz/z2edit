# Rebonack

The boss of palace 3: a knight on a flying horse who charges back and
forth past Link.  Knocked off his horse, he fights on foot as a blue Iron
Knuckle.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Palaces 3, 4, 6](../palaces.md) | $20 | Rebonack | 72 | 301 | 3 | - |
| [Palaces 3, 4, 6](../palaces.md) | $0A | Rebonack unhorsed | 48 (set when unhorsed) | 301 | 0 | - |

He is placed in rooms $0E, $2C and $35 of palaces 3, 4 and 6.
`Rebonack_Init` removes him once the room's flag shows him beaten, puts
him at X $220, Y $AC, and clears the unhorsed Rebonack's hitbox entry.
The fight is described with the [palace bosses](../palaces.md#bosses).

## Behavior

**Mounted** (`Rebonack_Ai`):

1. 9 hit points per bar block, once the fight is on.  Nothing else
   happens until 255 frames after it starts (`TIMER_BOSS`).
2. Guard 4 (his shield, `Enemy_CheckProjectileHitsSpecial`).  He bobs 1
   pixel every 4 frames, 16 frames each way.
3. `Boss_UpdateIntro_Bank4B`, `Boss_DrawAndCheck_Bank4B` (contact, hit
   tests).  His lance is tested against Link's body.
4. **Charging.**  Every other frame his X velocity moves 1/16 toward Link
   (`RebonackAccel`), up to 3 pixels per frame (`RebonackMaxSpeeds`).  So
   he flies past Link, slows, and comes back.
5. **Pauses.**  When his velocity passes through 0 he pauses for 83
   frames, drifting 1/4 pixel per frame back and forth by `frameCounter`
   bit 6.  If the drift leaves him moving away from Link, the next
   deceleration passes through 0 again and he pauses again.
6. **Turning.**  Found facing away from Link while slower than 9/16
   pixel per frame, he turns: `enemyAiState` counts 31 frames, and at 16
   he faces Link.  `Rebonack_Draw` slides his two halves past each other
   meanwhile.

**Unhorsed**, below 36 hit points:

- The rider, type $0A with 48 hit points and guard 1, appears in slot 3,
  24 pixels ahead, leaping (Y velocity -$16) backward at 2 pixels per
  frame.  Link's Y velocity is doubled at that moment.
- The horse (`objState+1` = 1) flies off forward at 1 pixel per frame and
  is removed once fully off screen.
- The rider (`RebonackUnhorsed_Ai`, 6 hit points per bar block) runs the
  Iron Knuckle's AI as a blue one: he throws his sword, and he is engaged
  wherever he is.  `enemyAiDir` is set to `frameCounter` & $40 each
  frame, so for 64 frames of every 128 he attacks again the moment an
  attack ends.  See [Iron Knuckle](iron-knuckle.md).

## Drawing

`Rebonack_Draw`: two columns of two pairs, the horse's front (entry $58)
and the rider (entry $5C, or $60 for the riderless horse).  The rider's
top row drops a pixel every other 16 frames.  `RebonackDrawOffsets`
slides the columns during a turn.  The unhorsed Rebonack is drawn as an
Iron Knuckle (`RebonackUnhorsed_Draw`).

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Rebonack_Init` | 4:$AF91 | spawning |
| `Rebonack_Ai` | 4:$AFBC | mounted |
| `RebonackUnhorsed_Ai` | 4:$B23F | on foot |
| `Rebonack_Draw`, `RebonackUnhorsed_Draw` | 4:$B3A8, 4:$AF12 | drawing |

## Verification

Scenarios at palace 3's entrance (see [Flame](flame.md#verification) for
`$P3`; here the floor fill runs to X $2FF), Rebonack placed at ($1F0,
$AC), Link at $1A0:

- He waited 255 frames, then charged left, past Link, to 2.3 pixels per
  frame.  He turned on frame 378 and paused on frame 393.  He charged
  back right to $233, turned (632), paused (647), and paused again at
  737 and 827 while drifting.  Then he charged left again.
- With his hit points poked to 35 (and the unhorsed hitbox cleared, as
  `Rebonack_Init` does): unhorsed on frame 255.  The horse flew left at
  $F0 and was removed on frame 470.  The rider leapt right at $1E,
  landed, and attacked at 288, 365, 390, 415, 440, ... throwing his sword
  each time, back to back in alternate 64-frame stretches.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $P3,statLevel+2=8,statContainers+1=8 \
    --until currentWorld=4 --after 200 --clear $F3 --approach 1A0 --spawn 0:20:1F0:AC \
    --poke enemyHp=0x23 --poke sramEnemyHitbox+10=0 --projectiles --event Unhorse=4:B00F \
    --event Attack=4:9DE6 --event Throw=4:9CAC --frames 700
```

In the no-glitch movie (`../private/Zelda2_NoGlitch_2014_Final.fm2:0`), Link's strokes at the mounted Rebonack
(frames 71860-71908) clinked off his shield three times and otherwise hit
for 4, mostly from jumps.
