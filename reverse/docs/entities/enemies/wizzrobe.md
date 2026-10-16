# Wizzrobe

A robed wizard that blinks into view, casts at Link, and vanishes, again
and again.  The two palace groups have different Wizzrobes with their own
code: the first group's (`Mago_Ai`) casts a flame and can be cut down
with the sword; the second group's (`Wizrobe_Ai`) fires a beam and can
only be hurt by that beam, sent back with the reflect spell.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Palaces 1, 2, 5](../palaces.md) | $1D | Wizzrobe (Mago) | 32 | 200 | 3 | - |
| [Palaces 3, 4, 6](../palaces.md) | $1D | Wizzrobe | 1 | 100 | 3 | 2 |

The first is placed in rooms $2C, $30, $35, $37, $39, $3A and $3E of
palaces 1, 2 and 5; the second in rooms $12, $15, $21, $27, $29, $2B,
$2E, $32 and $3C of palaces 3, 4 and 6.  Both block Link's projectiles
and are immune to SPELL.  Neither has a spawn routine, and both always
face Link.

## Behavior: Palaces 1, 2 and 5

`Mago_Ai` runs a 256-frame cycle, `enemyAiState` counting down:

| Count | Frames | What it does |
|-------|-------:|--------------|
| $FF-$E0 | 32 | blinking in: drawn every other frame, not tested |
| $DF-$C0 | 32 | solid: `Enemy_UpdateCommon` (hits, contact) and contact damage; casts at $D0 |
| $BF-$A0 | 32 | blinking out |
| $9F-$00 | 160 | gone; each frame its X is set to a random place on the screen (`scrollX` + `rng+1,x`), so it reappears anywhere along its own Y |

**The flame** (at $D0): projectile type 7 (`Enemy_DropFlame_Bank4`) on
the floor (Y $C0), 8 pixels to the side it faces.  It runs along the
floor at 3/4 pixel per frame, slowing by 1/16 every 8 frames, and burns
for 255 frames (`Projectile_Flame_Bank4`).

`Mago_Draw`: two pairs from `sramEnemyTiles` entry $1A, or $1E from count
$D2 on (casting).

## Behavior: Palaces 3, 4 and 6

`Wizrobe_Ai` runs a 193-frame cycle, `enemyAiState` counting down:

| Count | Frames | What it does |
|-------|-------:|--------------|
| $FF-$C0 | 64 | blinking in |
| $BF-$80 | 64 | solid (`Wizrobe_Animate`, `objState+1` = 1): drawn, harmful to touch; fires a beam at $B0 |
| $7F-$40 | 64 | blinking out |
| $3F | 1 | `Wizrobe_Restart`: a new random X ($00-$DF) on page 1, and the cycle starts again |

**It can't be hit.**  Neither Link's sword nor his projectiles are tested
against it; only Link's touch is.

**The beam** (`Wizrobe_FireBeam`, at $B0, only while it is on screen):
projectile type 9 along the floor (Y $C0), 2 pixels per frame the way it
faces, from 13 pixels ahead (right) or 6 (left).  `Projectile_CarockBeam`
flies it level and tests it against Link's shield and body.  When Link
has the reflect spell, his shield sends it back (`projectileTimer` 0).
It then flies back at its thrower (`projectileSpeed` holds the slot), and
if it meets the Wizzrobe while it is solid it does 1 hit, which is enough
(1 hit point).  The boss Carock shares this code.

**Page 1 only** ([quirks](../../quirks.md)).  `Wizrobe_Restart` always
puts it on page 1 (`objXHi` 1).  A Wizzrobe placed on another page of its
room appears there once, then moves to page 1 for good.  Most rooms place
them on pages 2 and 3 as well.

`Wizrobe_Draw`: two pairs from entry $1E while blinking in, else $1A.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Mago_Ai` | 4:$B7C5 | palaces 1, 2, 5 |
| `MagoShotXOffsets`, `MagoShotXHiOffsets`, `MagoShotSpeeds` | 4:$B7BE, 4:$B7C0, 4:$B7C2 | its flame |
| `Mago_Draw` | 4:$BE7B | drawing |
| `Wizrobe_Ai` | 4:$AE4F | palaces 3, 4, 6 |
| `Wizrobe_Animate`, `Wizrobe_Restart` | 4:$AEB2, 4:$AEBD | solid phase, moving |
| `Wizrobe_FireBeam` | 4:$AED3 | the beam |
| `Projectile_CarockBeam` | 4:$AD29 | the beam flying, reflected |
| `Wizrobe_Draw` | 4:$B42C | drawing |

## Verification

**Palaces 1, 2 and 5.**  Scenario in palace 1's first room (see
[Bubble](bubble.md#verification), with Link's life level raised), a
Wizzrobe placed at X $1C0, Link at $1A0:

- It cast on frames 47, 303 and 559, every 256 frames.  Between casts its
  X jumped about the screen ($1D1, $1F8, $180, ...) and came to rest at
  $19F, then $1C6.  The flame burnt Link and knocked him back.
- Placed at Link's height ($1C4, Y $B0), with Link striking every 6
  frames: his sword hit it on frame 32 (count $DF, the first solid frame)
  and again on 45.

**Palaces 3, 4 and 6.**  At palace 3's entrance (see
[Flame](flame.md#verification) for `$P3` and `$F3`):

- A Wizzrobe at $160 fired beams on frames 79, 272 and 465, every 193
  frames, reappearing at $16B, then $17E.  The beams burst on Link's
  shield.
- With Link striking every 6 frames 18 pixels away, no hit was ever
  registered (`Enemy_HitBySword` never ran), through the solid phase.
- With `reflectSpellActive` set, the beam came back off Link's shield at
  2 pixels per frame, met the Wizzrobe on frame 97 (count $9F, still
  solid), and killed it.
- Placed on page 0 at $A0, Link on page 0: its first beam came at frame
  79; after that it moved to $123, then $1AF, both on page 1, off the
  screen, and fired no more.
- Screenshots: the first group's Wizzrobe in an orange robe, casting a
  small flame at Link's feet; the second's in white and red with its
  beam.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $PAT,statLevel+2=8,statContainers+1=8 \
    --until currentWorld=3 --after 200 --clear $PFLAT --approach 1A0 --spawn 0:1D:1C0:90 \
    --projectiles --event Cast=4:B7DF --frames 600
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $P3,statLevel+2=8,statContainers+1=8 \
    --until currentWorld=4 --after 200 --clear $F3 --approach 140 --spawn 0:1D:180:C0 \
    --poke reflectSpellActive=1 --projectiles --event Dmg=7:E726 --event Beam=4:AE6A --frames 140
```
