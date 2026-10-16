# Moblin

A spearman that walks up to Link and attacks high or low, aiming where
his shield isn't.  The orange and blue Moblins throw their spear in a
high attack.  Moblins, [Goriyas](goriya.md) and [Dairas](daira.md) are
the *fighters*: they share their movement and attack timing
(`Fighter_Move`), described here.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [West Hyrule](../west-hyrule.md) | $14 | Orange Moblin | 4 | 10 | 1 | 1 |
| [West Hyrule](../west-hyrule.md) | $15 | Red Moblin | 12 | 20 | 1 | 1 |
| [West Hyrule](../west-hyrule.md) | $16 | Blue Moblin | 18 | 20 | 1 | 1 |

Only bank 1's group has them: red Moblins in the West Hyrule forests
($28-$2A) and area $04, with a blue one there; orange Moblins on Death
Mountain (areas $22, $23, $2F, $31-$33 of the second group).  They use
background probe set 0, which rests them with their Y on the floor's
row, and are drawn 32 pixels tall from that Y down.  Their spawn routine
(`Fighter_Init_Bank1`) sets `enemyAiState` to $FF.

## Movement: Fighter_Move

Each fighter's AI ends in `Fighter_Move`, with a distance r to keep
from Link: $16 for the red and blue Moblins, $60 for the orange Moblin
and the Goriyas, $1E and $37 for the orange and red Dairas.  Each frame:

1. `Enemy_UpdateWithBg` and `Enemy_HurtLinkOnContact`; it falls (gravity
   2/16 pixel per frame per frame) and lands on floors.
2. `Fighter_KnockBack` moves it by last frame's X velocity, unless there
   is a wall ahead.  While it is knocked back (`enemyKnockBack`, as when
   Link's shield blocks its thrust), it moves by the knockback instead,
   which shrinks by 1/16 pixel per frame, and the rest of the frame is
   skipped.
3. It turns toward Link and takes dx = Link's X + 8 - its X, the **low
   byte only** ([quirks](../../quirks.md)): a fighter 128 or more pixels
   away may take Link for near.
4. **Beyond r** it walks toward Link at 1 pixel per frame
   (`FighterSpeeds` doubled).  Within $50 pixels of him (so not when r is
   $60), it also starts an attack whenever it isn't attacking (not the
   red Daira): the red and blue Moblins and the orange Daira attack over
   and over as they come.
5. **Within r** it stops coming closer.  It counts frames in
   `enemyAiState` and starts an attack when the count AND its period mask
   (`FighterAttackPeriods`: $3F, 64 frames; $1F for the blue Moblin; $7F
   for the orange Goriya) is 0.  Since the spawn routine sets the count to
   $FF, the first frame near Link brings an attack.  When an attack starts
   and `rng+1,x` AND its quick mask (`FighterQuickAttackMasks`: 3, 1, 1
   for the Moblins, 3, 7 for the Dairas, $0F, 7, 3 for the Goriyas) is 0,
   the count jumps to $E0 and the next attack comes 32 frames later.
   Between attacks it holds its ground (`Fighter_Stop`):
   - stopped, if Link is against a wall;
   - moving at half Link's X velocity while he walks, so that it keeps
     its distance;
   - otherwise swaying 1/8 pixel per frame, one way for 64 frames and
     back (`frameCounter` bit 6), 8 pixels each way.

**Starting an attack** (`Fighter_StartAttack`): pose (`objState+1`) 1,
a low attack, or 3, a high one, held for 24 frames (the wind-up).  The
orange Moblin always attacks high and the Dairas always low.  The others
choose by `rng+1,x` bit 0: half the time at random (bit 2), half the
time where Link's shield isn't, high if he crouches (`linkStanding` 0),
low if he stands.

## The Moblin's Attack

`MoblinGoriya_Ai` plays the attack out.  When the 24-frame wind-up is
over:

- **Low**: a thrust (pose 2) for 16 frames.
- **High**: the red Moblin thrusts (pose 4) for 16 frames.  The orange
  and blue Moblins throw their spear instead (`Moblin_ThrowSpear`, pose
  5 for 16 frames).

During a thrust the spear is drawn at once (an extra `Enemy_Draw`) and
tested against Link's shield and body like a projectile
(`Projectile_CheckLinkReflect`, with his shield checked first for these
types).  A blocked thrust knocks the Moblin back.

**The thrown spear** (projectile type 5, `Projectile_Falling_Bank1`)
leaves from the Moblin's X and Y + 5.  It flies 2.5 pixels per frame
across (`MoblinSpearSpeeds`) and starts 1.5 pixels per frame up, under
gravity of 2/16, so it rises 9 pixels and falls until it leaves the
screen.  It is drawn as a pair of tile $48 (`Projectile_DrawLarge_Bank1`),
in palette 1 for the orange Moblin (`projectileSpeed` 1).  Its flags
($81) let it pass through Link's shield
([the definition tables](../README.md#the-definition-tables)).

## Drawing

`Moblin_Draw`:

- **Body:** two pairs of sprites from `sramEnemyTiles` entry
  `MoblinDrawTiles[pose]`: $24 standing, alternating with $1E every 8
  frames while it moves; $24, $2A, $30, $36 in poses 2-5.
- **Spear**, in an attack: the pair from the entry after the body's.  It
  is beside the legs (Y + 8) in a low attack and at the head (Y) in a high
  one.
  - In a wind-up it points back, 4 pixels behind; a low wind-up is drawn
    mirrored.
  - In a thrust it is 8 pixels ahead, 16 while the timer is 3-12, the
    lunge.
  - The last 8 frames of a low thrust show the wind-up again.
  - In the high wind-up the front spear sprite is moved 12-13 pixels
    further ahead.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Fighter_Init_Bank1` | 1:$9631 | spawn routine: `enemyAiState` $FF |
| `MoblinGoriya_Ai` | 1:$9972 | the attack, then `MoblinGoriya_Move` |
| `Fighter_Move` | 1:$9A22 | movement and attack timing (all fighters) |
| `Fighter_StartAttack`, `Fighter_Stop` | 1:$9A82, 1:$9AAB | start an attack; hold near Link |
| `Fighter_KnockBack` | 1:$9B0A | move, or be knocked back |
| `FighterSpeeds` | 1:$9A13 | $08, $F8, doubled |
| `FighterQuickAttackMasks`, `FighterAttackPeriods` | 1:$99EF, 1:$99F7 (+ type) | attack timing masks |
| `Moblin_ThrowSpear` | 1:$97F2 | the thrown spear |
| `Moblin_Draw` | 1:$9CA4 | drawing |

## Verification

Scenarios in the West Hyrule cave (area $2D; see the
[Myu](myu.md#verification) page for `$AT`, here with `itemOwned=1` added
for the candle), its three pits filled (`--fill 2E0:2EF:80 --fill
310:31F:80 --fill 340:34F:80`), Link at X $2A0:

- A red Moblin placed at X $340 walked toward Link at 1 pixel per frame.
  It started attacking 80 pixels from him, one attack after another (24
  frames of wind-up, 16 of thrust).
  - Within 22 pixels it held, swaying by $02/$FE (1/8 pixel per frame).
  - It was knocked back (X velocity $0C, shrinking) when Link's shield
    blocked it.
- Screenshots of poses 1-4: spear back at knee height, thrust at knee
  height, spear back at head height, thrust at head height.
- An orange Moblin at $320 attacked high every time.  It threw its
  spear, which rose from Y $85 to $7C and fell, 2.5 pixels per frame
  across, until it left the screen at the bottom.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $AT --until areaId=0x2D --after 200 \
    --clear --approach 2A0 --fill 2E0:2EF:80 --fill 310:31F:80 --fill 340:34F:80 \
    --spawn 0:15:340:80 --poke enemyAiState=0xFF --event Attack=1:9AA4 --frames 500
```
