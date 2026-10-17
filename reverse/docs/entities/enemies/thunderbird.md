# Thunderbird

The Great Palace's guardian: a huge bird that floats about over Link
spitting fireballs.  Until the Thunder spell strikes it, nothing hurts it.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Great Palace](../great-palace.md) | $22 | Thunderbird | 192 | 1000 | 5 | - |

It is placed alone on page 2 of room $35, the room before Dark Link's.
`Thunderbird_Init` removes it if the area's flag says it was beaten, and
otherwise starts it drifting down at 1/2 pixel per frame.  Link's
projectiles bounce off it, SPELL doesn't affect it, and Thunder does it
only 2 points (flags bit 5; see [Enemies](../README.md)).

## Behavior

`Thunderbird_Ai` has the palace bosses' intro and death built in, as
`Boss_UpdateIntro` does for palaces 1, 2 and 5 (see
[Palace enemies](../palaces.md)):

1. **Intro.**  Until Link's screen reaches page 1 of the room
   (`scrollXHi`), it is only drawn.  Then the scroll freezes
   (`scrollFrozen`), and it waits 144 frames (`timerFast+TIMER_ENEMY`).
2. **Invulnerable.**  While it has 192 hit points or more it is only
   drawn, with no hit tests at all.  Thunder (2 points) brings it to 190,
   and from then on Link's sword hurts it normally.
3. **Harmless body.**  It never calls `Enemy_HurtLinkOnContact`, so
   touching it doesn't hurt Link.  Its damage class is unused; only its
   fireballs hurt.
4. **Link's sword.**  While Link isn't mid-stroke (`linkAttackFrame` 0)
   it moves his sword out of the hit tests (`swordY` = $F8).
5. **Flying.**
   - Every 32 frames it turns toward Link and flies his way at 1 pixel
     per frame (`ThunderbirdSpeeds`).
   - It bobs: every 4 frames its Y velocity changes by 1/16 toward 1
     pixel per frame up or down, turning at either.  That carries it 64
     pixels up and down every 256 frames.
6. **Fireballs.**  Every 32 frames, or every 16 below 96 hit points, it
   spits one (projectile type 8):
   - from 12 pixels right of its X, 1 pixel per frame up;
   - 1/8 to 1 pixel per frame either way at random (`rng+2`,
     `ThunderbirdShotSpeeds`).

   The fireball (`Projectile_Fireball_Bank5`) falls with gravity (1/16
   pixel per frame per frame) and bursts on the background.  Link's
   shield stops it only with the reflect spell.  It survives hitting him,
   and its damage class is 4 (flags $A4).
7. **Death.**  When its hit points run out (`Enemy_Kill` makes it
   dying), it is kept active with its type's hitbox set to $FF.  It
   flashes for 127 frames.  Then every projectile is removed, every other
   living enemy takes 224 points, and the music stops (`musicRequest`
   $80).

**Its hit point bar** (`Boss_DrawHpBar_Bank5`) counts 24 points a unit.

A test of `frameCounter` that branches to the next instruction either
way has no effect.

**Enemies_Update.**  In the intro and death paths it drops a return
address with `pla`/`pla`, as `Boss_UpdateIntro` does.  But here the
code is in the AI routine itself, so it drops `Enemies_Update`'s.  Those
frames end the enemy loop early: the enemy slots after the Thunderbird's
(it spawns in slot 5, so slots 4-0), and every projectile slot including
its own, are not updated or drawn.  Its room has no other enemies, so
this has no visible effect ([quirks](../../quirks.md)).

## Drawing

`Thunderbird_Draw`: a large bird made of many sprite pairs.  Its face
(the tile it writes to `sramEnemyTiles+47`) shows whether it can be hurt:
$29 in palette 2 at exactly 192 hit points; below that $2B for 8 frames
of every 32 and otherwise $19, in palette 3.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Thunderbird_Init` | 5:$A33B | spawn |
| `Thunderbird_Ai` | 5:$A359 | everything above |
| `Projectile_Fireball_Bank5` | 5:$9BC8 | its fireballs |
| `Thunderbird_Draw` | 5:$9EBF | drawing |

## Verification

**In the no-glitch movie**
(`../private/Zelda2_NoGlitch_2014_Final.fm2:0`, events hooked as on the
[Horsehead](horsehead.md) page):

- The scroll froze on frame 162602 with Link at X $171.  The first
  fireball came on frame 162767, 165 frames later: the 144-frame wait,
  then the next multiple of 32.
- On frame 162782 `Enemy_TakeDamage` took it from 192 to 190: the
  Thunder spell.  The next hit, in the same frame, was a sword stroke
  (12 points, attack level 6).
- Link, jumping, then hit it 16 more times by frame 162846, 2 or 3
  frames apart.
- It burst on frame 162974, 128 frames after the last hit.
- Link was never hurt by touching it (no `Link_HurtByEnemy`).

**In the Great Palace's entrance** (see [Bubble](bubble.md#great-palace)
for `$GP` and `$GF`), the Thunderbird placed by `--spawn` with
`scrollFrozen` poked to start the fight:

- **Flying.**  Placed at ($90, $70) with its Y velocity 8, it flew
  between Y $38 and $78 every 256 frames, and between X $B8 and $D6 around
  Link.  It spat fireballs every 32 frames.

  ```
  tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $GP --until currentWorld=5 --after 200 \
      --clear $GF --approach B0 --spawn 5:22:90:70 --poke scrollFrozen=1 --poke objYVelocity+6=8 \
      --projectiles --event Fire=5:A410 --frames 420
  ```

- **Hit points.**  Placed at ($C4, $A8) with 191 hit points, a standing
  stroke on frame 3 hurt it on frame 12 (`Enemy_TakeDamage`).  With 192
  the same stroke did nothing.

  ```
  tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $GP --until currentWorld=5 --after 200 \
      --clear $GF --approach B0 --spawn 5:22:C4:A8 --poke scrollFrozen=1 --poke enemyHp+5=0xBF \
      --press 3:B:3 --press 18:B:3 --event Dmg=7:E726 --frames 30
  ```

- **Contact.**  Placed on Link at ($B4, $A0), it touched him every other
  frame (`objCollision` bit 4), but `Link_HurtByEnemy` never ran.

  ```
  tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $GP --until currentWorld=5 --after 200 \
      --clear $GF --approach B0 --spawn 5:22:B4:A0 --poke scrollFrozen=1 --poke enemyHp+5=0xBF \
      --event Hurt=7:E2EF --log 5 --frames 20
  ```

- **The skipped slots.**  Placed at ($90, $70) without the fight
  started, beside a bubble in slot 2: the bubble's AI never ran (no log
  lines for slot 2).

  ```
  tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $GP --until currentWorld=5 --after 200 \
      --clear $GF --approach B0 --spawn 5:22:90:70 --spawn 2:14:60:60 --log 2,5 --frames 20
  ```
