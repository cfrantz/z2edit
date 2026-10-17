# Towns

The towns are side view areas of worlds 1 (West Hyrule's towns) and 2
(East Hyrule's), in bank 3.  `townCode` numbers them:

| townCode | Town | Wizard's spell |
|----------|------|----------------|
| 0 | Rauru | Shield |
| 1 | Ruto | Jump |
| 2 | Saria | Life |
| 3 | Mido | Fairy |
| 4 | Nabooru | Fire |
| 5 | Darunia | Reflect |
| 6 | New Kasuto | Spell |
| 7 | Old Kasuto | Thunder |

The people are bank 3's enemy group; see
[Towns: townspeople](../entities/towns.md) for how they move.  Each town
gives its people their own sprites: when a town loads, bank 7 copies its
row of `TownPersonTiles` (towns 1-6) over the first sprite tiles of types
$0D-$20 (`sramEnemyAttr+13`).

## Buildings

Town areas use the area format of [side view areas](areas.md), with
their own objects: `Area_DrawLargeObject` and `Area_DrawSmallObject`
send worlds 1 and 2 to `Town_DrawLargeObject` and
`Town_DrawSmallObject`, which draw roofs, walls, windows, doors,
chimneys, furniture, signs and the like (named after z2edit's town
render config), and place the locked door and the elevator like the
other area banks.

### Town Objects

Large objects (size = the type's low nibble):

| Type | Row | Routine | Drawn |
|------|-----|---------|-------|
| $1x, $2x | 0-$E | `Town_DrawCrossHatchedRoof` | 2 rows, size + 1 wide: $84 over $85 |
| $3x | 0-$E | `Town_DrawGrayRoof` | $CF over $D0 |
| $4x | 0-$E | `Town_DrawGreenRoof` | $57 over $52 |
| $5x-$9x | 0-$E | `Town_DrawWall` (brick, white brick, grass, white wall, logs) | down to row $A, size tiles of $80, $CD, $4E, $CB, $82, then the right edge $81, $CE, $4F, $CB, $83 |
| $Ax-$Cx | 0-$E | `Town_DrawWindows` (square, gray, brown) | a window ($86/$86, $D1/$D2, $87/$88) every 3 rows down to row 9 |
| $Dx | 0-$E | `Town_DrawUndergroundBricks` | 2 rows of $55 |
| $Ex | 0-$E | `Town_DrawPillar` | $94 over size of $93 |
| $Fx | 0-$E | `Town_DrawCastleBricks` | a row of size + 1 of $98 |
| $1x | $F | `Town_DrawBush` | bushes ($45) on row $A |
| $2x | $F | `Town_DrawBackgroundWall` | $D3 $CC $CC from row 4, size + 1 rows |
| $3x, $4x | $F | `Town_DrawSlantRight`, `Town_DrawSlantLeft` | a room ceiling's slopes: 3 rows of $44 from row 2 |
| $5x | $F | `Area_PlaceElevator_Bank3` | (unused) |
| $6x | $F | `Town_DrawBridge` | size + 1 wide: $51, $56, $92 on rows $A-$C |
| $7x | $F | `Town_DrawWater` | size + 1 wide: $40 on row $B, water $92 on row $C |

Small objects:

| Type | Row | Routine | Drawn |
|------|-----|---------|-------|
| $00 | 0-$E | `Town_DrawClosedDoor` | $89 $8A $8B down, and the floor tile $53 under it |
| $01 | 0-$E | `Town_DrawOpenDoor` | $8C $91 $91 down, and the door tile $54 under it |
| $03 | 0-$E | `Town_DrawOpenDoor3` | $8C $91 $91 $58 down (the church's high door) |
| $05 | 0-$E | `Town_DrawChair` | $4A |
| $06, $07 | 0-$E | `Town_DrawLargeCloud`, `Town_DrawSmallCloud` | $C4 $C5, $C4 $C6 (background layers) |
| $08 | 0-$E | `Town_DrawTable` | $4B $4B over $4C $4D |
| $09 | 0-$E | `Town_DrawBench` | $46 $46 $47 $48 $48 |
| $0A, $0D | 0-$E | `Town_DrawCross`, `Town_DrawChurchCross` | $DC, $DD |
| $0B, $0C | 0-$E | `Town_DrawChimneyEntrance`, `Town_DrawChimney` | $59 $59, $5A $5A |
| $00-$03 | $F | `Town_DrawDoorLeft` ... `Town_DrawWallRight` | a room's left and right ends (`TownWallEdgeTiles`) |
| $04 | $F | `Town_DrawFireplace` | 2 x 2 from row 9, over the door tiles $EA $EA |
| $05 | $F | `Town_DrawFountain` | $D9 over $D7 $D8 $DA $D8 $DB |
| $06 | $F | `Town_DrawSign` | $E3 over $E4 on rows 9-$A |
| $07-$09 | $F | `Town_DrawHeadstone`, `Town_DrawTombLeft`, `Town_DrawTombRight` | $95, $96, $97 on row 9 |

`Town_DrawOpenDoor2` ($02) and `Town_DrawStonehenge` ($04) are unused.
The open door's $54 and the fireplace's $EA are door tiles
(`AreaBank::SpecialTiles`): Up on them goes through, so a fireplace is a
way through too.  Floors and ceilings come from `TownFloorRecords`, 9
bytes per background type.

These tiles were checked against the tile maps the game builds.  The
areas were the street areas $00, $06, $07, $0A, $0C and $10, the rooms
$19 and $1A, the underground $24 and the graveyard $3C (`tools/area_dump.py`;
see [Side view areas](areas.md#checking-objects)):

```
TA=areaPrgBank=3,currentRegion=0,currentWorld=1,townCode=0,palaceCode=0xfe,areaLocation=0x32,areaId=0,areaStartPage=0,areaEntrySide=0,prevRegion=0
tools/area_dump.py --movie build/movies/file_select.fm2 --at $TA --until currentWorld=1 \
    --areas 0x00,0x06,0x07,0x0A,0x0C,0x10,0x19,0x1A,0x24,0x3C
```

## Conversations

Talking to someone (`Townsperson_StartTalk`) sets `dialogType`; the
conversation is run by `Dialog_Update` (from `Menu_UpdateTownDialog` in
bank 7) as a state machine on `pauseState`:

1. The window opens over the playfield, one row pair a frame
   (`Dialog_OpenRow`), drawn with bank 3's copies of the pause menu's
   window routines (`Menu_*_Bank3`; see [Menus](../systems/menus.md)).
2. `Dialog_ChooseText` runs the handler for the person's type
   (`DialogHandlers`), which may give Link something and decides whether
   the person's second text applies; `Dialog_SelectText` then finds the
   text.
3. `Dialog_PrintText` types it, a letter every 5 frames, each with the
   typing sound (`sfxRequest` $60).  B skips to the end (not for a
   wizard or a teacher).
4. `Dialog_Reward` follows a gift: the fanfare for a spell or a
   technique, or the refill in the life and magic ladies' houses.
5. After B (`Dialog_WaitForB`; no second B when B skipped the text),
   the window is erased (`Menu_CloseRow_Bank3`) and
   `Dialog_Finish` ends the conversation.  In Saria and Darunia a walking
   townsperson ($19-$1C) may then turn into a bat (`Dialog_MaybeTurnToBat`,
   103 in 256).

### What People Say

A text is chosen by the person's type ($0A-$22) and the town:
`WestTextIds` / `EastTextIds` hold, for each type, a text number per
town of the world (`townCode & 3`), and `WestTextIdsMet` /
`EastTextIdsMet` (types $0A-$19) the second text, used when the handler
found its condition met.  When the gift was already given
(`dialogAlreadyDone`), row $0F is used instead.  The number indexes the
world's text pointer table (`WestTextPtrs`, `EastTextPtrs`, through
`TownTextTables`) into `TownText`.

The handlers (`Dialog_*`, with `y` = `townCode`):

| Type | Handler | |
|------|---------|-|
| $0A, $0B | `Dialog_SleepyCreature` | talking Ache and Bit: "ZZZ..." until Link has talked to them 4 times (`townspersonTalkCount`) |
| $0D | `Dialog_TechniqueMan` | Saria: the river man, once Link has talked to Bagu; Mido: downstab; elsewhere (not Rauru): upstab; a technique already learned: "I CANNOT HELP YOU ANYMORE." |
| $0E | `Dialog_Error` | Mido's man sends Link to Error of Ruto (`techniques` bit 1); Error then tells of the tunnel; elsewhere the cross is checked, to no effect |
| $0F | `Dialog_Wizard` | spell `townCode`, if Link has `townCode` + 1 magic containers |
| $10-$12 | `Dialog_SetTalked` | remember the talk in `townFlags` (bits 3-1); only Bagu's ($10, bit 3 of townCode 3) is checked |
| $13-$16 | `Dialog_CheckTownFlag` | second text if the town's flag is set (see below); the person leads Link to a door |
| $17, $18 | `Dialog_HouseLady` | the life and magic ladies |
| $1D-$20 | `Dialog_SetWalkerFlag` | walkers: set `townFlags` bit 7-4, which no town of theirs checks |
| $21 | `Dialog_Examine` | signs and things examined: `townFlags` bit 0, Saria's mirror and Nabooru's water |
| $0C, $19-$1C, $22 | - | one text |

`techniques` holds two quest flags besides the sword techniques: bit 0,
the river man of Saria will help Link cross; bit 1, Mido's man told him
to ask Error.  Saria's bridge clears bit 0 once it is built
(`Saria_BuildBridge`), so Link must talk to the river man on each visit.

The flags `Dialog_CheckTownFlag` reads (`TownFlagChecks`), and what sets
them:

| Town | Type | `townFlags` bit | Set by |
|------|------|-----------------|--------|
| Rauru | $14 | - | always met |
| Ruto | $16 | 4 | the trophy (`Item_CollectSpecial`) |
| Saria | $14 | 0 | the mirror under the table (type $21) |
| Mido | $15 | 6 | the water of life (`Item_CollectSpecial`) |
| Nabooru | $16 | 0 | the fountain (type $21) |
| Darunia | $15 | 5 | the child (`Item_CollectSpecial`) |
| New Kasuto | $15 | 3 | the 7th magic container (`Item_Collect`) |

(`TownFlagChecks` also has bit 6 for type $13 in Ruto, but no area holds
a type $13.)  The movies show each of them: the woman who lost her
mirror, the mother of the sick girl, the trophy's owner, the child's
mother and the New Kasuto woman lead Link to their doors once the flag
is set.

Talking to Bagu (type $10 in his house, entered as townCode 3) sets bit 3
of townCode 3's flags, which the river man of Saria checks.  Each town's
streets are areas 3n to 3n + 2, n = `townCode` (the right entrances lead
to area 3n + 2: `Locations_Bank1`, `Locations_Bank2`).

### Text Format

Each byte prints a two-tile column: a byte >= $C0 is the tile itself
(letters are $DA-$F3, digits $D0-$D9, space $F4, '.' $CF); below $C0,
the low six bits are the tile ('!' $36, '?' $34) and bits 7-6 put a mark
above it.  $FD ends a line,
$FE ends a line with a longer pause, $FF ends the text.  A window holds
four lines of 10 characters.

### Checking Conversations

`tools/enemy_scenario.py --dialog` prints each text as it starts, with
`townFlags` and `techniques`.  Scenarios start in a town as on the
[Townsperson](../entities/enemies/townsperson.md#verification) page, with
`townCode` (and for East Hyrule `currentWorld=2`, `currentRegion=2`)
chosen for the town, and B pressed every 20 frames:

- **Teachers** (room $2C, Link walked to X $1B8): with `townCode` 3 the
  knight said "WHEN YOU JUMP PRESS DOWNWARD TO STAB." and set
  `techniques` bit 4, with the flash and fanfare.  With `townCode` 5 in
  East Hyrule: "WHEN YOU JUMP PRESS UP TO STAB." and bit 2.  With both
  already learned (`techniques=0x14` in `--at`): "I CANNOT HELP YOU
  ANYMORE. GO NOW.", with no fanfare (`Dialog_StartReward` went on to
  `Dialog_EndText`).
- **Error** (room $20, Link walked to X $140): in Ruto "I AM ERROR.",
  or with `techniques=2` "SOUTH OF KINGS TOMB IN MIDO IS A TUNNEL."; in
  Mido the same person sets bit 1: "ASK ERROR OF RUTO ABOUT THE
  PALACE."  In Nabooru, with or without the cross, "WITH BOOTS I COULD
  WALK ON THE WATER."
- **A walker's flag** (area $12 with `townCode` 3, its people cleared, a
  type $15 woman and a type $1E walker spawned): she said "PLEASE. MY
  DAUGHTER IS SICK. HELP HER!"; the walker's talk set `townFlags+3` bit 6;
  then she said "THE WATER OF LIFE! QUICK COME WITH ME."  (Area $12 is
  New Kasuto's, where bit 6 means nothing.)

  ```
  BS=""; for k in $(seq 4 20 900); do BS="$BS --press $k:B:2"; done
  tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at ${TA/townCode=0/townCode=3},areaId=0x12 \
      --until currentWorld=1 --after 200 --clear --spawn 0:15:30:B0 --spawn 1:1E:90:B0 $BS \
      --press 300:RIGHT:40 --log 0,1 --frames 700 --dialog
  ```

- **Closing** (the Bit's room): a B 14 frames into "ZZZ..." skipped it
  and closed the window (`Dialog_Finish` 9 frames later); a text left to
  finish waited in `Dialog_WaitForB` until the next B, 210 frames on.

Both movies were also replayed with every handler hooked:
- **The no-glitch movie:** 11 conversations.
- **The 100AK movie:** 23 conversations, including all eight wizards and
  Nabooru's fountain ("WANT TO GET SOME WATER?").

Each matched the code: Bagu's talk, then the river man's help; the mirror,
then its owner; the fountain, then the thirsty woman; the Water of Life, the trophy, the child and the 7th magic
container, each followed by its person leading Link to a door; the
technique teachers; and the wizards' spells and the house ladies' refills.
