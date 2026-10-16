# Eye

A floating red eye, its lid narrowed, that drifts across, bobbing,
turning toward Link every 64 frames, and now and then stops to stare.
In flight it turns the sword aside; it can only be hurt while it stops
with its eye open.

## Where It Appears

| Group | Type | HP | Exp | Damage class | Drop group |
|-------|------|---:|----:|-------------:|-----------:|
| [East Hyrule](../east-hyrule.md) | $15 | 16 | 20 | 4 | 1 |

Only bank 2's group has it: many East Hyrule areas ($0B, $14, $18-$1A,
$26, $27, $2F-$32, $3B, $3C) and Maze Island ($02, $27, $28).  It has no
spawn routine ([spawn defaults](../../systems/enemies.md#spawning)).

## Behavior

`Eye_Ai` (no `Enemy_SkipAiIfStunned`):

1. `Enemy_HurtLinkOnContact`.  While its timer (`timerFast+TIMER_ENEMY`)
   runs, it is stopped: only `Enemy_UpdateCommon`.
2. **Bob.**  `enemyAiState` counts frames.  In the second half of every 64
   (bit 5), it drifts 1/2 pixel per frame down, then the next time up
   (bit 6; `EyeYSpeeds`): 16 pixels each way.
3. **Turn.**  Every 64 frames it turns toward Link (`Enemy_FaceLink`).
4. **Fly.**  1/2 pixel per frame the way it faces (`EyeXSpeeds`).  It
   ignores the background.
5. **Guard.**  It sets `enemyAiVar` to 5 and tests its guard
   (`Enemy_CheckProjectileHitsSpecial`): Link's projectiles that touch
   it are destroyed, and a sword stroke on it clinks and skips the hit
   test.  Then it is tested as usual (`Enemy_UpdateCommon`).  Guard 5 is
   a box 18 wide and 13 high, 2 pixels below its Y and 1 left of its X
   (`Enemy_GetSpecialHitbox`), nearly the whole eye.  While it is stopped
   (step 1) the guard isn't tested, so that is when it can be hurt.
6. **Stop.**  On 1 frame in 32 at random (`rng+1,x` bits 4-0 all 0), it
   stops for 63 frames.

## Drawing

`Eye_Draw`: a pair of sprites facing its `objFacing`, `sramEnemyTiles`
entry $0A while it flies, its eye a slit, and $08 while it is stopped,
wide open.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Eye_Ai` | 2:$98BC | everything above |
| `EyeYSpeeds` | 2:$98BA | $08 down, $F8 up |
| `EyeXSpeeds` | 2:$98B9 | by facing (1, 2): the two bytes of `EyeYSpeeds` |
| `Eye_Draw` | 2:$9DDE | drawing |

## Verification

Scenario in East Hyrule's cave area $09, set up as for the
[Tektite](tektite.md#verification), an Eye placed at X $E0, Y $60, Link
at X $24:

- It moved left at 1/2 pixel per frame.  It drifted down from Y $60 to
  $70 over frames 32-63, held, then up from frame 96.
- It stopped for 63 frames at frames 108 and 186 and on (the timer from
  $3F), then went on.
- Screenshots: a narrow slit of an eye in flight, wide open when stopped.
- Sword strokes (B every 6 frames, Link at X $17, the Eye placed at $50):
  in flight they clinked (`Enemy_SwordClink`, frames 97-100); 4 frames
  after it stopped one hit it (`Enemy_HitBySword`, frame 105), and every
  hit in 200 frames came while it was stopped.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $EAT --until areaId=0x09 --after 60 \
    --clear $FLAT --spawn 1:15:0E0:60 --log 1 --frames 300 --shot 100 --shot 130
```
