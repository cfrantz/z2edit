# Townsperson

The people standing and walking in the towns' streets and houses, the
signs and mirror, and the generator that keeps sending walkers along the
street.  Link talks to them by touching them and pressing B.

## Where It Appears

| Group | Type | Person | AI | Spawn |
|-------|------|--------|----|-------|
| [Towns](../towns.md) | $08, $09 | unused walkers | `Townsperson_WalkAi` | `Townsperson_Init` |
| [Towns](../towns.md) | $0C | sign (invisible) | `Sign_Ai` | `Townsperson_Init` |
| [Towns](../towns.md) | $0D, $0E, $10-$12 | standing people | `Townsperson_StandAi` | `Townsperson_Init` |
| [Towns](../towns.md) | $19-$1C | walkers sent by the generator | `Townsperson_WalkAi` | `Townsperson_Init` |
| [Towns](../towns.md) | $1D-$20 | walkers | `Townsperson_WalkAi` | `Townsperson_InitWalking` |
| [Towns](../towns.md) | $21, $22 | mirror, sign (invisible) | `Sign_Ai` | `Townsperson_Init` |
| [Towns](../towns.md) | $23 | townsperson generator (invisible) | `TownspersonGenerator_Ai` | `Generator_Init_Bank3` |

Hit points, experience and damage don't apply: none of them can be hurt
(flags bits 7 and 4).  Each street area places two generators, on pages 0
and 3.  The second removes the first when it spawns
(`Generator_Init_Bank3`), so one runs at a time.

The other people have their own pages: the [house people](house-people.md)
($13-$18), the [town wizard](town-wizard.md) ($0F), the
[talking creatures](talking-creatures.md) ($0A, $0B) and the
[town bat](town-bat.md) ($03).  What they all say is chosen by the town
code ([Towns: conversations](../../world/towns.md#conversations)).

## Behavior

**Spawning** (`Townsperson_Init`):

- Each starts not yet talked to (`townspersonTalkCount` 0), with no
  door, and with `townspersonTimer` $90, counting down a frame at a time.
- While it is between $AB and $65, for about 43 frames, the person
  can't be talked to.
- `Townsperson_InitWalking` ($1D-$20) first turns toward Link and sets
  the type's speed: 1, 0.75, 0.5 and 0.75 pixels per frame
  (`TownspersonSpeedBase`, into `GeneratorSpeeds`).

**Standing people** (`Townsperson_StandAi`) stand where they are placed,
and turn toward Link whenever `frameCounter` is a multiple of 64.

**Walkers** (`Townsperson_WalkAi`):

- They walk at their speed and never turn around.  They leave the
  screen and are removed when far from it (`Enemy_DespawnIfFar`).
- `Townsperson_Walk` moves them.  Every 16 frames it sets their facing
  from the velocity, and counts the step (`townspersonWalkCounter`).
- Unlike the [house people](house-people.md), they never go through
  doors.

**The generator** (`TownspersonGenerator_Ai`):

- It is invisible.  Every 256 frames (`enemyAiState`), if an enemy slot
  is free (searching from 5 down), it sends a walker onto the street at
  Y $B0.
- The walker's type is $19-$1C at random (`TownspersonGeneratorTypes`,
  whose last entry is the first byte of `GeneratorSpeeds`).  Its speed
  is 1, 0.75, 0.5 or 0.75 pixels per frame.
- It starts just off one edge of the screen, at `scrollX` - 32 or +
  264, walking inward.
- The side is the one Link is walking toward while he holds left or
  right; otherwise it is random.
- It has no list entry, so it never comes back once gone.

**Signs and the mirror** (`Sign_Ai`): invisible objects that can be read
(talked to) at once, since their `townspersonTimer` is kept at 0.  They
are removed when far off screen.

**Talking** (`Townsperson_CheckTalk`, `Townsperson_StartTalk`):

- A person can be talked to while on screen and while no conversation
  is on.  Link must be on the ground, except for the talking Ache.
- Link must touch it (`Enemy_CheckLinkContact`) and press B.  The
  person must not be in a door sequence, and its timer must not be in
  the $65-$AB window.
- The conversation (`dialogType` 2) freezes the game.  Link stops his
  stroke, stands (state 3), and is put on the tile row.
- The person's `townspersonTalking` and `townspersonTalkCount` go up.
- While it talks (`Townsperson_FaceLinkWhileTalking`), Link and the
  person face each other, and the rest of its AI is skipped.

## Drawing

`Townsperson_Draw`:

- **Sprites:** two pairs of sprites.  Their tiles are `sramEnemyAttr`'s
  byte for the type, which towns 1-6 replace for $0D-$20
  (`TownPersonTiles`).  The palette is `sramEnemyDamage` bits 1-0.
- **Pose:** a person with `sramEnemyDamage` bit 7 has one pose.  The
  others use their first frame while standing or talking.  Walking, they
  show the next frame (+4) on alternate 8-frame periods, or 16-frame
  periods below 1 pixel per frame.
- **Signs and the generator** have no drawing routine.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Townsperson_Init`, `Townsperson_InitWalking` | 3:$976B, 3:$9743 | spawning |
| `Townsperson_StandAi` | 3:$9793 | standing people |
| `Townsperson_WalkAi`, `Townsperson_Walk` | 3:$9878, 3:$9A68 | walkers |
| `TownspersonGenerator_Ai`, `Generator_Init_Bank3` | 3:$96E0, 3:$96B2 | the generator |
| `Sign_Ai` | 3:$9ABD | signs and the mirror |
| `Townsperson_CheckTalk`, `Townsperson_StartTalk`, `Townsperson_FaceLinkWhileTalking` | 3:$99B9, 3:$9A33, 3:$9A9B | talking |
| `Townsperson_DrawOrDespawn`, `Townsperson_Draw` | 3:$9A8A, 3:$9AE8 | drawing |

## Verification

Town scenarios start a game in a town area chosen by `areaId` (bank 3,
`--until currentWorld=1`).  The street area 0 is grass with a sign;
area 2 has a house and its life lady; area $1F is a house interior:

```
TA=areaPrgBank=3,currentRegion=0,currentWorld=1,townCode=0,palaceCode=0xfe,areaLocation=0x32,areaStartPage=0,areaEntrySide=0,prevRegion=0
```

- **Walkers and the generator** (area 0, Link standing at $17):
  - A type $20 walker walked left at 0.75 pixels per frame and never
    turned.
  - The generator's count reached 0 on frame 104.  It sent a type $1A
    walker in at X $108 (the scroll + 264), walking left at 0.75 pixels
    per frame: it was at $E6 on frame 150.

  ```
  tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $TA,areaId=0 --until currentWorld=1 \
      --after 200 --log 0,1,2,3,4,5 --frames 300
  ```

- **A standing person** (area $1F, starting on page 1 from the right,
  the kid at X $130, Link moved to $140):
  - It turned toward Link at `frameCounter` $C0, $00 and $40.
  - B on frame 30 started a conversation (`Townsperson_StartTalk`), as
    did each B after the last conversation closed.

  ```
  TB=areaPrgBank=3,currentRegion=0,currentWorld=1,townCode=0,palaceCode=0xfe,areaLocation=0x32,areaStartPage=1,areaEntrySide=1,prevRegion=0
  tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $TB,areaId=0x1f --until currentWorld=1 \
      --after 200 --link 140:B0 --press 1:LEFT:1 --press 30:B:2 --event Talk=3:9A33 --event Turn=7:DC91 \
      --log 5 --frames 260
  ```
