# Basilisk

A crested lizard that crawls back and forth, rears up every few
seconds, and breathes fireballs, low while crawling and high while
reared.  Identified on screen in East Hyrule's cave area $08: a cyan
crested lizard.

## Where It Appears

| Group | Type | HP | Exp | Damage class | Drop group |
|-------|------|---:|----:|-------------:|-----------:|
| [East Hyrule](../east-hyrule.md) | $18 | 48 | 50 | 3 | 2 |

Only bank 2's group has it: East Hyrule areas $00, $01, $03 and $08,
and Maze Island areas $09, $0F, $13, $15, $23, $26, $2F, $31 and $33.  Its
attribute bit 5 makes the sword clink off it and blocks the sword beam;
fire still works.  It has no spawn routine.

## Behavior

`Basilisk_Ai` (no `Enemy_SkipAiIfStunned`):

1. **Breath.**  Every 64 frames (`frameCounter`), half the time, it sets
   a 10-frame countdown (`objState+1`).  When it runs out it fires
   projectile type 6, 2 pixels per frame the way it faces
   (`Projectile_Straight_Bank2`):
   - crawling, from Y + 16, near the floor;
   - reared, from Y + 4 and further ahead (+14 facing right, -8 facing
     left, `BasiliskFireXOffsets`), from its raised head.

   Link's shield destroys the fireball (flags $01).
2. `Enemy_Fall_Bank2` (probes, falling, landing).  Its type's hitbox
   entry (`sramEnemyHitbox+$18`) is set to $89 while it crawls and $80
   while it is reared, then `Enemy_UpdateCommon` and
   `Enemy_HurtLinkOnContact`.
3. It moves by last frame's X velocity unless at a wall, or by its
   knockback (`Enemy_KnockBackOrWalk_Bank2`).  Every 64 frames it turns
   toward Link.
4. **Rearing.**  At `frameCounter` 0, every 256 frames, it rears up and
   stands for 80 frames (`timerFast+TIMER_ENEMY`).
5. **Pacing**, while not reared (`Enemy_Pace_Bank2`): `enemyAiState`
   counts calls, and its bits 7-6 pick the quarter.  Each quarter is 64
   calls:
   - 0: backs away from where it faces at 1/2 pixel per frame;
   - 1: stands;
   - 2: advances at about 1 pixel per frame ($0F, $EF);
   - 3: stands.

   So it backs off 32 pixels and comes 60 forward every cycle.  It
   doesn't test for ledges and walks off them.

## Drawing

`Basilisk_Draw`, with the extra attribute bits `extraSpriteAttrs` ($20,
behind the background, once Link has waded in the area's swamp; see
[Octorok](octorok.md#drawing)):

- **Reared:** two pairs of sprites from `sramEnemyTiles` entry $18.
- **Crawling:** one pair 16 pixels lower, entry $14, alternating with $16
  while it moves (every 4 frames, or 8 when slower than 1/2 pixel per
  frame).  Beside it is a head sprite, tile $DC, 16 pixels to the right
  or 8 to the left (`SegmentXOffsets_Bank2`).  The head is drawn 1 pixel
  back while it is about to breathe.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Basilisk_Ai` | 2:$9A2A | everything above |
| `Enemy_Pace_Bank2` | 2:$9B09 | the pace (also the Scorpion's) |
| `Enemy_KnockBackOrWalk_Bank2` | 2:$9B24 | moving, or knockback |
| `BasiliskFireXOffsets`, `BasiliskFireXHiOffsets` | 2:$9A25, 2:$9A27 | the reared fireball's offset |
| `Basilisk_Draw` | 2:$9E13 | drawing |

## Verification

Neither test movie meets one.  In its own area, cave $08 (location $0C,
starting on page 3), with the candle, 300 frames:

- It backed away at 1/2 pixel per frame, stood, and advanced at $0F.
- Screenshots: crawling on all fours with its head out in front (frames
  30, 150), reared up at frame 250 (`frameCounter` 0 at frame 204).

```
EAT=areaPrgBank=2,currentRegion=2,currentWorld=0,townCode=0,palaceCode=0,areaLocation=0x0C,areaId=0x08,areaStartPage=0,areaEntrySide=0,prevRegion=2,itemOwned=1
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $EAT --until areaId=0x08 --after 60 \
    --log 0,1,2,3,4,5 --frames 300 --shot 30 --shot 150 --shot 250
```
