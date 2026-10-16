# Boon

A buzzing flying insect that darts about at random and drops rocks.
Identified on screen in East Hyrule's area $33: a flying insect dropping
rocks.

## Where It Appears

| Group | Type | HP | Exp | Damage class | Drop group |
|-------|------|---:|----:|-------------:|-----------:|
| [East Hyrule](../east-hyrule.md) | $17 | 8 | 50 | 1 | 2 |

Only bank 2's group has it: East Hyrule areas $2F-$33 and Maze Island
areas ($0B-$0D, $10, $15, $17-$19, $30), placed high (Y $30).  It has no
spawn routine.

## Behavior

`Boon_Ai`:

1. **Rocks.**  Every 32 frames (`frameCounter`), while it is above Y $70,
   it drops a rock (`Enemy_DropRock_Bank2`, projectile type 8), even
   while stunned.  The rock (`Projectile_FallingRock_Bank2`) starts with
   no speed and falls under gravity of 1/16 pixel per frame per frame.
   On reaching the ground (`Projectile_HitsBg`, at Y $90 or lower) it
   bursts in a spark.
2. `Enemy_SkipAiIfStunned`, `Enemy_UpdateCommon` (no background) and
   `Enemy_HurtLinkOnContact`.  It faces Link.
3. **Coming on screen** (`enemyAiState` 0): at Y $30 and a random X on
   the screen, the next turn in 32 frames.  This is also how it starts.
4. **Flying.**  When it is off screen (`drawOffscreen+1` bits 7, 6, 3, 2),
   it comes back on next frame.  Every 8-23 frames
   (`timerFast+TIMER_ENEMY`) it picks a random direction, `objState+1`:
   0 down, 1 up, 2 right, 3 left.  It flies 2 pixels per frame that way
   (`BoonMoves`).

Nothing keeps it high: it wanders anywhere on the screen, and drops
rocks only while above Y $70.

## Drawing

`Boon_Draw`: a pair of sprites facing its `objFacing`, alternating every
frame between `sramEnemyTiles` entries $10 and $12.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Boon_Ai` | 2:$9989 | everything above |
| `BoonMoves` | 2:$99F4 | by direction: `Boon_MoveY`, `Boon_MoveY`, `Boon_MoveX`, `Boon_MoveX` |
| `BoonYSpeeds` | 2:$9987 | $20, $E0 (also the X velocities) |
| `Enemy_DropRock_Bank2` | 2:$9A11 | a rock |
| `Projectile_FallingRock_Bank2` | 2:$965B | the falling rock |
| `Boon_Draw` | 2:$9E01 | drawing |

## Verification

Neither test movie meets one.  Scenario in East Hyrule's area $33
(location $15), a Boon placed with Link at X $170:

- It appeared at once at Y $30 at a random X.  It changed direction every
  8-15 frames, 2 pixels per frame each way, and wandered down to Y $D4.
- It dropped a rock at frame 14, above Y $70.  The rock fell from Y $4A,
  speeding up by 1/16 per frame, and burst at Y $C8 on frame 79 (states
  $F3, $F4, and on).
- It dropped no more while it stayed lower than Y $70.

```
EAT=areaPrgBank=2,currentRegion=2,currentWorld=0,townCode=0,palaceCode=0,areaLocation=0x15,areaId=0x33,areaStartPage=0,areaEntrySide=0,prevRegion=2
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $EAT --until areaId=0x33 --after 60 \
    --clear --spawn 0:17:180:30 --projectiles --event Turn=2:99ED --event Drop=2:9A16 --frames 400
```
