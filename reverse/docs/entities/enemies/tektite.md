# Tektite

A long-legged creature that crouches and stretches, leaps over Link
again and again, and fires straight shots.  Identified on screen in East
Hyrule's cave area $09: cyan, long-legged, hopping, spitting small shots.

## Where It Appears

| Group | Type | HP | Exp | Damage class | Drop group |
|-------|------|---:|----:|-------------:|-----------:|
| [East Hyrule](../east-hyrule.md) | $14 | 24 | 50 | 2 | 2 |

Only bank 2's group has it: caves and encounter areas of East Hyrule
($09, $0D, $11, $16, $22-$25) and of Maze Island (the second group).  Its
attribute bit 5 makes the sword clink off it and blocks the sword beam;
fire still works.  It has no spawn routine
([spawn defaults](../../systems/enemies.md#spawning)).  Its probe set
rests its Y on the floor's row, and it is drawn 32 pixels tall from Y
down.

## Behavior

`Tektite_Ai`, after `Enemy_SkipAiIfStunned` (drawn first, `Enemy_Draw`):

1. **The cycle.**  `enemyAiState` steps through $F1-$10, 32 steps.  It
   steps every frame on the ground and every other frame in the air (on
   frames with Y velocity bit 1 set).  Its distance from 0 crouches the
   body (see Drawing).  Through steps $F8-$07 the type's hitbox in SRAM,
   `sramEnemyHitbox+$14`, is $80; otherwise it is the tall $89.  It is
   rewritten every frame, so all Tektites on screen share the last one's.
2. **Leap.**  At step $F8, on the ground, half the time (`rng+1,x` bit
   0), it leaps: Y velocity -$3B under `Enemy_MoveYAndFall`'s gravity
   (2/16), about 50 pixels up and 60 frames long.
3. `Enemy_Fall_Bank2`: background probes, landing (`Enemy_Land`) when
   coming down on a floor.  At a wall it turns around.
4. **Moving.**  On the ground it faces Link (`Enemy_FaceLink`) and stands
   still.  In the air it moves 1 pixel per frame the way it faces
   (`TektiteSpeeds`) and doesn't turn, so it lands beyond Link and next
   leaps back over him.
5. **Firing.**  Every 64 frames (`frameCounter`), half the time, it fires
   projectile type 6 the way it faces, 2 pixels per frame straight
   (`Enemy_FireProjectile`, `Projectile_Straight_Bank2`), at the height
   of its top sprite.  In the air that is often away from Link.  When that
   sprite is hidden off screen, the shot starts at Y $F8 and is removed at
   once.
6. The rest of `Enemy_UpdateCommon` (without drawing it again), and
   `Enemy_HurtLinkOnContact`.

## Drawing

`Tektite_Draw`: two pairs of sprites facing its `objFacing`, from
`sramEnemyTiles` entry $00, or $04 while its hitbox is the tall one
(legs stretched).  The top pair is lowered by the step's distance from 0
(0-15 pixels), so the body bobs down and up over the cycle.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Tektite_Ai` | 2:$9805 | everything above |
| `TektiteSpeeds` | 2:$9803 | $10, $F0 |
| `Enemy_Fall_Bank2` | 2:$9B42 | probes, falling, landing |
| `Tektite_Draw` | 2:$9DAC | drawing |

## Verification

Scenario in East Hyrule's cave area $09 (location $0D, with the candle):

```
EAT=areaPrgBank=2,currentRegion=2,currentWorld=0,townCode=0,palaceCode=0,areaLocation=0x0D,areaId=0x09,areaStartPage=0,areaEntrySide=0,prevRegion=2,itemOwned=1
FLAT="--fill 0:3FF:40:0 --fill 0:3FF:50:0 --fill 0:3FF:60:0 --fill 0:3FF:70:0"
```

The hills above row $80 were cleared (`$FLAT`, not drawn), and Link
stood at X $80.  A Tektite placed at X $E0:

- It leapt at step $F8 every time it did, 62-188 frames apart.  It
  landed beyond Link and leapt back, between X $66 and $A3.
- It fired at `frameCounter` $00, $40, $80 and $C0 when `rng+1,x` bit 0
  was clear: 7 shots in 1000 frames.  Each flew at 2 pixels per frame
  from its top sprite's Y, sometimes away from Link.
- Placed at X $C0 with Link at the area's left end, it leapt toward him
  until it reached the left edge.  Its one shot there left from Y $F8,
  off screen, and vanished.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $EAT --until areaId=0x09 --after 60 \
    --clear $FLAT --approach 80 --spawn 0:14:0E0:80 --projectiles \
    --event Leap=2:9830 --event Fire=2:986E --frames 1000
```
