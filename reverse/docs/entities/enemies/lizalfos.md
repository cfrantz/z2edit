# Lizalfos

A lizard warrior with a shield, a [fighter](moblin.md#movement-fighter_move)
that guards wherever Link strikes.  The red one thrusts, the orange one
thrusts high or low around Link's shield, and the blue one throws its
weapon.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [East Hyrule](../east-hyrule.md) | $1A | Red Lizalfos | 48 | 100 | 4 | - |
| [East Hyrule](../east-hyrule.md) | $1B | Orange Lizalfos | 32 | 150 | 3 | 2 |
| [East Hyrule](../east-hyrule.md) | $1C | Blue Lizalfos | 56 | 200 | 4 | - |

Only bank 2's group has them, in East Hyrule (red: $0E, $10, $19, $2C;
orange: $02, $04, $0E, $18, $1A, $28-$2A; blue: $10, $12, $13) and Maze
Island.  They block Link's projectiles (damage bit 5) and are immune to
the SPELL spell.  Like the fighters they rest their Y on the floor's row
and are drawn 32 pixels tall from Y down.  They have no spawn routine,
so their first attack comes 64 frames after Link gets near.

## Behavior

`Lizalfos_Ai` (red, blue) plays out its attack (`Lizalfos_Attack`), then
`Lizalfos_Guard`.  `OrangeLizalfos_Ai` plays out its own attack, then the
same.

**Moving** (as `Fighter_Move`, with `Lizalfos_KnockBack`):

- It keeps a distance r from Link (`LizalfosRanges`: 24 pixels for the
  red and orange, 64 for the blue).  The distance is taken from the low
  byte of the X difference, as for the fighters.
- Beyond r it walks toward him at 1 pixel per frame; within $50 pixels it
  also attacks as it comes (`Lizalfos_ChooseGuard`).
- Within r it holds: stopped if Link is against a wall, keeping pace at
  half his speed while he walks, else swaying 1/8 pixel per frame.  It
  attacks every 64 frames, or 32 for the orange (`LizalfosAttackPeriods`).
- While knocked back it moves by the knockback and does nothing else.

**Guarding** (guard 0 low or 1 high in `enemyAiVar`, tested by
`Enemy_CheckProjectileHitsSpecial`): each guard is a 2-pixel-wide bar,
the shield's edge.

- When Link presses B, 3 times in 4 (`rng+1,x` bits 1-0 not 0) it moves
  the guard to the height of his stroke: high if he stands
  (`linkStanding`), low if he crouches.
- While Link is in the air (`linkJumpState`), when it isn't attacking:
  high if he is more than 10 pixels above it, else low.

Link's projectiles and strokes that meet the guard are turned aside.  So
strokes at one height always clink.  Only a change of height, the
quarter of the time the guard doesn't follow, gets through.

**Attacking.**  The red and blue always attack low (pose 1).  The orange
attacks high (pose 4) half the time at random, the other half away from
Link's shield: low if he stands, high if he crouches.  After a 24-frame
wind-up:

- **Red** (`Lizalfos_Attack`): thrust (pose 2) for 5 frames, its weapon
  tested against Link like a projectile, then pose 3 for 5.
- **Blue**: at pose 2 throws its weapon and is done.  The weapon is
  projectile type 7, 2 pixels per frame straight
  (`Projectile_Straight_Bank2`), from 16 pixels right or 8 left
  (`LizalfosShotXOffsets`); Link's shield destroys it (flags $03).
- **Orange** (`OrangeLizalfos_Ai`): thrust (pose 2 or 5) for 12 frames,
  tested every frame (`Projectile_CheckLink`), then 12 frames (pose 3
  or 6).

## Drawing

`Lizalfos_Draw`:

- **Body:** two pairs of sprites from `sramEnemyTiles` entry $24 (shield
  high) or $2C (shield low), 4 entries further on alternate 8-frame
  periods while it walks on the ground.
- **Weapon,** in an attack: a pair from `LizalfosFrames` ($34, $34, $36
  for poses 1-3; the orange's $38, $3A, $3A, drawn 8 lower for its high
  attack).  It is shifted by `LizalfosDrawTiles` (8, -6, -2 facing left;
  -8, 6, 2 facing right).  The shift is doubled for the red, and for the
  others while their timer is 3-8, the lunge.  For the red and blue,
  pose 2 is drawn mirrored and upside down and pose 3 13 pixels lower.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `OrangeLizalfos_Ai` | 2:$9B56 | the orange's attack, then `Lizalfos_Guard` |
| `Lizalfos_Ai` | 2:$9B90 | red and blue |
| `Lizalfos_Guard` | 2:$9B96 | guard, hit tests, movement |
| `Lizalfos_ChooseGuard` | 2:$9C1C | guard against jumps, start attacks, hold |
| `Lizalfos_Attack` | 2:$9CBE | red and blue attacks, the blue's throw |
| `Lizalfos_KnockBack` | 2:$9D13 | move, or be knocked back |
| `LizalfosRanges`, `LizalfosAttackPeriods` | 2:$9B73, 2:$9B6E (+ type) | distance, attack period |
| `Lizalfos_Draw` | 2:$9EE1 | drawing |

## Verification

Neither test movie meets one.  Scenarios in East Hyrule's cave area $09,
set up as for the [Tektite](tektite.md#verification), with Link's life
level and heart containers raised to 8 (`statLevel+2=8,statContainers+1=8`
in `$EAT`; at level 1 a class 4 hit kills him outright), Link at X $80:

- **Red, struck at one height.**  Standing strokes every 8 frames all
  clinked, its guard high (`enemyAiVar` 1).  From frame 210, crouching
  strokes all clinked too, its guard now low (0).
- **Red, alternating heights** (a standing then a crouching stroke every
  24 frames): 4 hits (frames 190, 298, 418, 465) among about 100 clinks.
- **Orange:** wound up high (pose 4) for 24 frames.  Its weapon was
  tested on each of the 12 frames of pose 5; pose 6 followed.  It
  attacked as it came.
- **Blue:** attacked every 87 frames from 64 pixels.  It threw on the
  24th frame: projectile type 7 at its Y, 2 pixels per frame toward Link.

```
EAT=areaPrgBank=2,currentRegion=2,currentWorld=0,townCode=0,palaceCode=0,areaLocation=0x0D,areaId=0x09,areaStartPage=0,areaEntrySide=0,prevRegion=2,itemOwned=1,statLevel+2=8,statContainers+1=8
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $EAT --until areaId=0x09 --after 60 \
    --clear $FLAT --approach 80 --spawn 0:1A:0C0:70 --poke enemyHp=0x7F --poke enemyAiState=0xFF \
    --event Clink=7:E654 --event Hit=7:E6AE --press 20:B:3 --press 30:DOWN:10 --press 32:DOWN+B:3 ... --frames 600
```
