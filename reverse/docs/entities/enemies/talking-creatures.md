# Talking Creatures

A sleeping Ache and a sleeping Bit, kept in town houses, that talk once
Link has woken them by asking enough times.

## Where It Appears

| Group | Type | Creature | AI | Drawing |
|-------|------|----------|----|---------|
| [Towns](../towns.md) | $0A | talking Ache | `Townsperson_StandAi` | `Ache_Draw` |
| [Towns](../towns.md) | $0B | talking Bit | `TalkingBit_Ai` | `Bot_Draw` |

The Ache hangs in room $20, page 2, at Y $30.  The Bit lies on a table in
room $1E, page 2.  Neither can be hurt.

## Behavior

- **Both are standing people** (`Townsperson_StandAi`; see
  [Townsperson](townsperson.md)).  They turn toward Link every 64
  frames, and can be talked to.
- **The Ache can be reached from the air.**  `Townsperson_CheckTalk`
  makes an exception for its type ($0A): Link may talk to it in mid-air,
  since it hangs high.
- **Waking them.**  They answer "ZZZ..." until `townspersonTalkCount`
  reaches 4.  The fourth conversation and later ones give the second
  text, "LET ME BE! MASTER IS IN WOODS N OF RIVER" (`Dialog_SleepyCreature`).
  Their count starts again when they spawn.
- **The Bit breathes** (`TalkingBit_Ai`).  For 128 of every 256 frames
  (`frameCounter` bit 7 clear) its `objState+1` follows `frameCounter`
  bits 4-3, which `Bot_Draw` uses to squash it.  The rest of the time it
  holds still.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `TalkingBit_Ai` | 3:$9783 | the Bit's breathing, then `Townsperson_StandAi` |
| `Townsperson_StandAi` | 3:$9793 | both |
| `Dialog_SleepyCreature` | 3:$B5B9 | their text |

## Verification

Town scenarios: see [Townsperson](townsperson.md#verification).  In room
$1E, starting on page 2 from the left, the Bit (slot 5, X $2A0) lay on a
table.  Its `objState+1` was $18, then 0 from frame 64, when
`frameCounter` bit 7 was set.  Screenshots on frames 4, 12, 20 and 70
show it slightly squashed and stretched.

```
tools/enemy_scenario.py --movie build/movies/file_select.fm2 \
    --at $TA,areaId=0x1e,areaStartPage=2 --until currentWorld=1 --after 200 --log 5 --frames 100 \
    --shot 4 --shot 12 --shot 20 --shot 70
```

- **Waking the Bit** (same room, Link walked to X $298, B every 20
  frames; `--dialog` prints each text): "ZZZ..." three times, then
  "LET ME BE! MASTER IS IN WOODS N OF RIVER" on the fourth talk and every
  later one.

  ```
  BS=""; for k in $(seq 4 20 900); do BS="$BS --press $k:B:2"; done
  tools/enemy_scenario.py --movie build/movies/file_select.fm2 \
      --at ${TA/areaStartPage=0/areaStartPage=2},areaId=0x1e --until currentWorld=1 --after 200 \
      --approach 298 $BS --log 5 --frames 900 --dialog
  ```

- **The Ache from mid-air** (room $20, the Ache at X $280, Y $70): Link
  walked to X $278 and jumped, with B every 4 frames.  The conversation
  started at Y $8D (event at 3:$9A33).  It snapped Link up to the tile row
  ($80), where he hung until the window closed, then fell to the floor.

  ```
  BS=""; for k in $(seq 14 4 40); do BS="$BS --press $k:B:2"; done
  tools/enemy_scenario.py --movie build/movies/file_select.fm2 \
      --at ${TA/areaStartPage=0/areaStartPage=2},areaId=0x20 --until currentWorld=1 --after 200 \
      --approach 278 --press 10:A:30 $BS --event Talk=3:9A33 --log 5 --frames 400 --dialog
  ```
