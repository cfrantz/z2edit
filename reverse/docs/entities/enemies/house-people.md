# House People

People who live in a town house: they pace in front of it, and go in
and out through its door.  Two of them, the life lady and the magic
lady, lead Link into their house to restore his life or magic.

## Where It Appears

| Group | Type | Person | Spawn |
|-------|------|--------|-------|
| [Towns](../towns.md) | $13-$16 | idle people | `Townsperson_InitIdle`: inside the house |
| [Towns](../towns.md) | $17 | life lady | `Townsperson_Init`: outside |
| [Towns](../towns.md) | $18 | magic lady | `Townsperson_Init`: outside |

All of them use `HouseLady_Ai`.  The idle people start inside, in the
doorway, 8 pixels up and 14 back from where they are placed
(`Townsperson_InitIdle`, `Townsperson_InitAtDoor`).  What they say is
chosen by the town code: an idle person's second text depends on a town
flag (`Dialog_CheckTownFlag`), and a lady's on whether Link is outside
or inside (`Dialog_HouseLady`).

## Behavior

`HouseLady_Ai`, by `enemyAiState`:

**Inside ($80).**

- Hidden.  Once `townspersonTimer` is 0, and Link comes within 96 pixels
  across (by screen X), the person comes out.
- It finds the door (`Townsperson_StartDoor`), steps back to it, and the
  door opens halfway.

**Coming out ($81-$AF), a frame per step.**

- The door is fully open at $90, and the person is drawn from then on.
- The door is half open at $A0, and closes at $B0.
- At $B0 the person is outside (state 0), with `townspersonTimer` $90.

**Outside (0, `HouseLady_Idle`).**

- Out of the doorway it steps down 4 pixels every 16 frames, to Y $B0.
- It paces: it walks while `townspersonTimer` is $40 or more, and
  stands while it is below.
- At 0 it turns around, with the timer at $E0: 160 frames walking, then
  64 standing.
- It walks at the spawn speed of 1/2 pixel per frame, past its own door
  (`Townsperson_WalkAndTalk`).
- Link can talk to it.  While it talks it keeps its facing and doesn't
  turn to Link (`Townsperson_FaceLinkWhileTalking`).

**Going in (1-$7F).**  When Link asks a lady for help, she turns back
toward her door (`Townsperson_TurnToDoor`, from `Dialog_HouseLady`).
`townspersonToDoor` then makes her walk with `Townsperson_WalkAi`, which
looks for doors.  At the door (the tile ahead is $8B) she:

| State | What happens |
|-------|--------------|
| 1-$1F | waits (second frame), the door half open at $10 (`Door_QueueHalfOpen`) |
| $20-$3F | the door open (`Door_QueueOpen`); she walks in, stepping up 4 pixels at $27 and $37 |
| $40 | the ground tile under the door becomes $54, a door (`AreaBank::SpecialTiles`) |
| $40-$7D | stands in the doorway |
| $70 | the door half open |
| $7E | the door closed (`Door_QueueClosed`), the tile back to $53 |

After $7E a lady waits inside again (state $80, with `townspersonTimer`
255).  Any other person would be removed, but only the ladies are ever
sent to their door.

**Following her in.**  Between $40 and $7D the doorway is a door tile,
so Link can go through it (Up on it, `Link_CheckBg`, `gameMode` $16).
`Townsperson_CheckTalk` catches him touching her there:

- Instead of the door's area, her house conversation starts on the spot
  (`dialogType` 4).
- The game goes back to side view (`gameMode` $0B), with `doorDepth` and
  `renderingOff` undone.
- The enemies of the area's list in the slots are marked so as not to
  come back.

The conversation's second text and `Dialog_Reward` then restore his life
or magic.

**The door** (`Door_QueueTiles`): two vertical PPU strings of 6 tiles,
the door's left and right columns (`DoorTilesLeft`, `DoorTilesRight`).
They are half open (left column dark), open (both dark) or closed.

## Drawing

`Townsperson_Draw` (see [Townsperson](townsperson.md#drawing)).  Inside
(`enemyAiState` $40 or more) a person isn't drawn; coming out it is drawn
from $90.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Townsperson_InitIdle`, `Townsperson_InitAtDoor` | 3:$9756, 3:$975D | starting inside |
| `HouseLady_Ai`, `HouseLady_Idle` | 3:$97D4, 3:$983F | everything above |
| `Townsperson_TurnToDoor` | 3:$9A50 | a lady heading for her door |
| `Townsperson_StartDoor`, `Townsperson_NextState`, `Door_CheckTile` | 3:$989F, 3:$98BC, 3:$990A | going in |
| `Door_QueueHalfOpen`, `Door_QueueOpen`, `Door_QueueClosed`, `Door_SetClosedTile` | 3:$9946, 3:$994A, 3:$994E, 3:$98F9 | the door |

## Verification

Town scenarios: see [Townsperson](townsperson.md#verification) for `$TA`.

- **Pacing** (area 2, the life lady at X $AC, Link far away):
  - She turned around on frames ~210 and ~434.
  - In between she walked 160 frames at 1/2 pixel per frame, then stood
    64.
  - She walked past her door at X $D0 without using it.

  ```
  tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $TA,areaId=2 --until currentWorld=1 \
      --after 200 --log 5 --frames 600
  ```

- **Coming out** (area 8, an idle person, type $14, inside at X $15E;
  Link walked to $108):
  - Screenshots: the door half open (state $89), then fully open with
    her in the doorway (state $9B).
  - The door closed at $B0.  She stepped down from Y $A8 to $B0 and
    walked left for 80 frames, then stood.

  ```
  tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $TA,areaId=8 --until currentWorld=1 \
      --after 200 --approach 108 --log 3 --event HalfOpen=3:9946 --event Open=3:994A --event Close=3:994E \
      --frames 200 --shot 2 --shot 20
  ```

- **Going in, and following her** (area 2, Link walked to $9E, B
  pressed every 20 frames):
  - She turned to her door after each conversation, and found it on
    frame 430 (`Townsperson_StartDoor`).
  - She walked in from X $C2 to $D1, stepping up to Y $A8.  She waited
    inside at X $C3 from state $80.
  - With Link walked to X $CA and Up held from frame 495 (state $41),
    her house conversation started at once (`dialogType` 4, event at
    3:$99EA), without `Mode_EnterDoor`.
  - The window opened over the street, and `Dialog_Reward` ran on
    frame 561.
  - The doorway tile: `--dump` showed $60BD (row 11, column $D, under
    the door) $53 before and $54 during.

  ```
  BS=""; for k in $(seq 40 20 400) $(seq 560 20 900); do BS="$BS --press $k:B:2"; done
  tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $TA,areaId=2 --until currentWorld=1 \
      --after 200 --approach 90 --press 1:RIGHT:6 --press 10:B:2 $BS --press 450:RIGHT:30 --press 495:UP:50 \
      --event StartDoor=3:989F --event Lady4=3:99EA --event Reward=3:B75B --log 5 --frames 900 \
      --dump 510:sramAreaTiles:D0
  ```
