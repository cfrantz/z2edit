# Horsehead

The boss of palace 1: a horse-headed knight who paces before Link and
swings a mace.  Only his head can be hurt.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Palaces 1, 2, 5](../palaces.md) | $20 | Horsehead | 32 | 50 | 2 | - |

He is placed in room $0D of palaces 1, 2 and 5 (palace 1's boss room).
His spawn routine (`Boss_Init`) removes him once the room's flag shows
him beaten, and gives him guard 2 and hitbox entry 7.  The fight itself
(music, frozen scroll, the hit point bar, the death that clears the room)
is described with the [palace bosses](../palaces.md#bosses).

## Behavior

`HorseHead_Ai`:

1. `Boss_UpdateIntro` and `Boss_UpdateCommon`: drawn, moved, hit tests.
2. **Guard.**  His guard (2, `Enemy_GetSpecialHitbox`) covers Y+5 to
   Y+37, 16 pixels wide; strokes and projectiles there clink.  His own
   hitbox (entry 7) is 8 pixels wide from Y-10 to Y+22, so only strokes
   above Y+5 reach him: his head.
3. **The mace** (his fifth sprite) is tested against Link's body
   (`Weapon_CheckLinkBody`): a hit knocks Link up and back.
4. **The swing** (`HorseHead_Attack`): pose 1, the mace raised behind
   him, 15 frames; pose 2, swung in front, 15 frames; then 15 frames of
   rest.
5. **Pacing** (`Boss_Pace`, 4 hit points per bar block):
   - He turns toward Link only every 32 frames, never mid-swing.
   - `enemyAiState` bits 6-0 count walking frames / 2; bit 7 set means
     backing away.  He walks 1/2 pixel per frame, but only while facing
     Link.
   - 96 pixels or more from Link, the count restarts toward him
     (`Boss_ResetState`).
   - From 32 to 95 pixels, every 64 frames, 3 times in 4, likewise.
   - Under 32 pixels (`Boss_ChooseAttack`): facing Link and not
     swinging, 1 time in 8 each frame he swings.  Once the count is out,
     1 time in 8 each frame a new 32-frame count starts, toward or away
     at random.

## Drawing

`HorseHead_Draw`: three pairs, 48 pixels tall, from `sramEnemyTiles`
entry $58, or $5E in part of each walking step.  In a swing his middle
pair is redrawn from `HorseHeadFrames` (his arm) and the mace is drawn
16 pixels behind or in front of him (`HorseHeadDrawOffsets`).  Facing
away from Link, his head is turned to look at him.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Boss_Init` | 4:$BCA1 | spawning |
| `HorseHead_Ai` | 4:$BB5F | everything above |
| `Boss_Pace`, `Boss_ResetState`, `Boss_ChooseAttack` | 4:$BB71, 4:$BBBC, 4:$BBC5 | pacing |
| `HorseHead_Attack` | 4:$BC11 | the swing |
| `Boss_UpdateCommon` | 4:$BC4E | update, death |
| `HorseHead_Draw` | 4:$981D | drawing |

## Verification

Scenario in palace 1's first room (see [Bubble](bubble.md#verification),
with Link's life level raised), Horsehead placed at X $1E0 with
`enemyAiVar` 2 and hitbox entry 7 poked in (the scenario skips
`Boss_Init`), Link standing at $1A0:

- With Link past page 1, the bar of 8 blocks appeared at the left of the
  screen.
- He paced in and out (counts $0A, $82, $8E ...), swinging when close:
  15 frames per pose.
- Placed at Y $A0 with Link striking from the floor: the strokes clinked
  off his guard (`Enemy_SwordClink`, frames 60-65 and 125-129), and he
  was never stunned.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $PAT,statLevel+2=8,statContainers+1=8 \
    --until currentWorld=3 --after 200 --clear $PFLAT --approach 1A0 --spawn 0:20:1E0:90 \
    --poke enemyAiVar=2 --poke sramEnemyHitbox+32=7 --event Swing=4:BBF5 --frames 900
```
