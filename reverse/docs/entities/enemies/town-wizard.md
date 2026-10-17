# Town Wizard

The old man in a house of each town who teaches Link the town's spell.
He speaks first: Link doesn't need to touch him or press B.

## Where It Appears

| Group | Type | Person | AI | Spawn |
|-------|------|--------|----|-------|
| [Towns](../towns.md) | $0F | wizard | `Wizard_Ai` | `Townsperson_Init` |

He stands on page 1 of rooms $24-$2B, one per town, at X $1C0.  He can't
be hurt.  What he gives is chosen by the town code (`Dialog_Wizard`: the
town's spell, if Link has enough magic containers; see
[Towns](../../world/towns.md#conversations)).

## Behavior

`Wizard_Ai`:

1. While on screen he faces Link (`Enemy_FaceLink`).
2. The first time Link stands on the ground within 32 pixels of him
   (Link's X + 8 within 32 of his X), he starts the conversation himself
   (`Townsperson_StartTalk`).
3. He never calls `Townsperson_CheckTalk`, so touching him does nothing.
   After that first conversation (`townspersonTalkCount`) he is silent
   until the room is entered again.
4. The 3 bytes after the routine (`inc townspersonTalking,x`) are never
   run.

## Drawing

- `Wizard_Draw`: as a townsperson (`Townsperson_Draw`).
- When he gives a spell, `Dialog_TextDone` sets `wizardFlashTimer` and
  the screen flashes.  For those 64 frames he is drawn as a small white
  figure instead (`SpritePairTiles` entry $4C, palette 1, 4 pixels
  higher).
- Meanwhile his `townspersonTalking` is cleared each frame.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Wizard_Ai` | 3:$97A9 | everything above |
| `Wizard_Draw` | 3:$9AC8 | drawing |

## Verification

Town scenarios: see [Townsperson](townsperson.md#verification) for
`$TB`.  In room $24 Link arrives at the top right, beside the wizard at
X $1C0:

- The conversation was already open at the takeover, with no B pressed:
  "IN PARAPA DESERT USE THIS MAGIC TO SURVIVE".
- B pressed every 20 frames ran it on.  The screen flashed, and from
  frame 170 he was drawn as a small white figure for 64 frames (event at
  3:$9ACD on 170-233).

```
BS=""; for k in $(seq 100 20 700); do BS="$BS --press $k:B:2"; done
tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $TB,areaId=0x24 --until currentWorld=1 \
    --after 200 $BS --event Talk=3:9A33 --event Flash=3:9ACD --log 5 --frames 300 --shot 60 --shot 175
```
