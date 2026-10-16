# Scorpion

A scorpion of the lava caves that paces back and forth with its tail
raised, stings with fireballs, and guards its body; it can only be hurt
when it lowers its guard.

## Where It Appears

| Group | Type | HP | Exp | Damage class | Drop group |
|-------|------|---:|----:|-------------:|-----------:|
| [East Hyrule](../east-hyrule.md) | $19 | 32 | 150 | 4 | 2 |

Only bank 2's group has it, in East Hyrule areas $0A, $0C, $12, $13,
$1E-$20 and the lava cave $3D (location $16), where it sits half sunk in
the rock.  It blocks
Link's projectiles (damage bit 5).  It has no spawn routine.

## Behavior

`Scorpion_Ai` (no `Enemy_SkipAiIfStunned`):

1. `Enemy_Fall_Bank2` (probes, falling, landing).
2. **Guard.**  Unless its guard is down (`objState+1` = 0), it tests
   guard 3 (`enemyAiVar`, `Enemy_CheckProjectileHitsSpecial`).  The guard
   is its body, 24 pixels wide, from 16 to 32 pixels below its Y, 4 left
   of its X.  Link's projectiles touching it are destroyed, and a
   crouching sword stroke clinks off it.  A standing stroke passes over
   it.  While its guard is down it counts `objState+1` down instead.
3. `Enemy_UpdateCommon`, `Enemy_HurtLinkOnContact`, and moving by last
   frame's X velocity (`Enemy_KnockBackOrWalk_Bank2`).  Every 64 frames
   it turns toward Link.
4. **Dropping its guard.**  On 1 frame in 64 at random (`rng+1,x` bits
   7-2 all set) it drops its guard for 32 frames, when any stroke that
   reaches it hits.
5. **Stinging.**  On 1 frame in 64 at random (`rng+2,x` bits 5-0 all 0)
   it raises its tail for 16 frames (`timerFast+TIMER_ENEMY`).  8 frames
   in, it fires projectile type 4 the way it faces: a fireball with a
   sound, 2 pixels per frame straight (`Projectile_Fireball_Bank2`).
6. **Pacing** (`Enemy_Pace_Bank2`), as the [Basilisk](basilisk.md):
   back off 1/2 pixel per frame, stand, advance about 1 pixel per frame,
   stand, 64 frames each.  It walks off ledges.

## Drawing

`Scorpion_Draw`:

- its tail, a pair from `sramEnemyTiles` entry $20, or $22 (raised)
  while it stings;
- its body below, entry $1C, alternating with $1E every 4 frames while it
  moves;
- its head, a fifth sprite in front of the body (16 pixels right or 8
  left of its X), tile $EE, or $FA while its guard is down.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Scorpion_Ai` | 2:$9AAE | everything above |
| `Enemy_Pace_Bank2` | 2:$9B09 | the pace |
| `Enemy_CheckProjectileHitsSpecial` | 7:$E617 | the guard |
| `Scorpion_Draw` | 2:$9E84 | drawing |

## Verification

Neither test movie meets one.  Scenarios in the lava cave $3D (location
$16), with the candle, Link at X $170 on a platform:

- A Scorpion placed at X $1E0 paced as described.  It dropped its guard
  at frame 72 for 32 frames, and fired at frames 273, 309 and 371, each
  8 frames after raising its tail.
- Screenshot: a cyan scorpion, tail curled over its back, head toward
  Link.
- Placed on Link's right, it backed off the platform's edge and fell to
  its death.
- Placed at X $15A on Link's left, with Link facing it, B pressed every
  6 frames:
  - standing strokes neither hit nor clinked (over it);
  - crouching strokes (Down held) clinked on frames 6-11 with its guard
    up;
  - with `objState+1` poked to $20 (guard down), the same strokes hit it
    on frames 6 and 7.

```
EAT=areaPrgBank=2,currentRegion=2,currentWorld=0,townCode=0,palaceCode=0,areaLocation=0x16,areaId=0x3D,areaStartPage=0,areaEntrySide=0,prevRegion=2,itemOwned=1
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $EAT --until areaId=0x3D --after 60 \
    --clear --spawn 0:19:15A:80 --poke enemyHp=0x7F --poke objState+1=0x20 \
    --event Clink=7:E654 --event Hit=7:E6AE --press 1:LEFT:2 --press 3:DOWN:60 \
    --press 5:DOWN+B:3 --press 11:DOWN+B:3 --press 17:DOWN+B:3 --frames 55
```
