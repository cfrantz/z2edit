# Leever

A desert creature that pops out of the sand at a random spot near Link,
slides along toward him, and sinks back, again and again.

## Where It Appears

| Group | Type | HP | Exp | Damage class | Drop group |
|-------|------|---:|----:|-------------:|-----------:|
| [East Hyrule](../east-hyrule.md) | $16 | 16 | 10 | 4 | 1 |

Only bank 2's group has it: the desert encounter areas $1D and $21 of
East Hyrule and area $07 of Maze Island.  Its spawn routine,
`Leever_Init`, hides it $1C below its spawn Y (its depth, kept in
`objState+1`) for 0-255 frames at random.

## Behavior

`Leever_Ai`, after `Enemy_SkipAiIfStunned`, `Enemy_UpdateCommon` and
`Enemy_HurtLinkOnContact`, by `enemyAiState` bits 1-0:

| State | What it does | Next |
|-------|--------------|------|
| 0, hidden | waits for its timer | moves to a random X on the screen (`scrollX` + `rng+1,x`) at its depth, turns toward Link with 1 pixel per frame his way (`LeeverSpeeds`), and waits 80 frames |
| 1, rising | during the last 24 of those frames rises 1 pixel every other frame, 12 pixels | moves for 48-127 frames |
| 2, moving | 1 pixel per frame straight on, through walls | sinks for 32 frames |
| 3, sinking | 1 pixel every other frame, 16 pixels | hidden for 48-127 frames |

It is drawn behind the background, so it shows only where the
background is empty, above the sand.  It is tested for hits all the
time, hidden or not.

## Drawing

`Leever_Draw`: a symmetric pair of sprites (the right one mirrored),
alternating between `sramEnemyTiles` entries $0C and $0E every 8 frames,
in palette 3 with the priority bit set (attribute $23).

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Leever_Init` | 2:$97F3 | spawn routine |
| `Leever_Ai` | 2:$9910 | the four states |
| `LeeverSpeeds` | 2:$990E | $10 right, $F0 left |
| `Leever_Draw` | 2:$9DEB | drawing |

## Verification

Neither test movie meets one.  Scenario in East Hyrule's cave area $09,
set up as for the [Tektite](tektite.md#verification).  A Leever was
placed hidden at Y $8C (depth $8C, as `Leever_Init` would for a spawn Y
of $70), with its timer at $10 and Link at X $24.

- It appeared at X $D2 on frame 15, rose from Y $8C to $80 over frames
  72-95, and moved left at 1 pixel per frame for 58 frames.
- It sank to Y $90 over 32 frames, stayed hidden for 52, and reappeared
  at X $63 at its depth.  The cycle repeated.
- Screenshots: hidden behind the rock while over the cave wall, visible
  against the open cave.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $EAT --until areaId=0x09 --after 60 \
    --clear $FLAT --spawn 0:16:0C0:8C --poke objState+1=0x8C --poke timerFast+3=0x10 \
    --event Appear=2:994D --event Move=2:9960 --event Sink=2:9974 --frames 500
```
