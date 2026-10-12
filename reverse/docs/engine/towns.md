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

## Conversations

Talking to someone (`Townsperson_StartTalk`) sets `dialogType`; the
conversation is run by `Dialog_Update` (from `Menu_UpdateTownDialog` in
bank 7) as a state machine on `pauseState`:

1. The window opens over the playfield, one row pair a frame
   (`Dialog_OpenRow`), drawn with bank 3's copies of the pause menu's
   window routines (`Menu_*_Bank3`; see [Menus](menus.md)).
2. `Dialog_ChooseText` runs the handler for the person's type
   (`DialogHandlers`), which may give Link something and decides whether
   the person's second text applies; `Dialog_SelectText` then finds the
   text.
3. `Dialog_PrintText` types it, a letter every 5 frames, each with the
   typing sound (`sfxRequest` $60).  B skips to the end (not for a
   wizard or a teacher).
4. `Dialog_Reward` follows a gift: the fanfare for a spell or a
   technique, or the refill in the life and magic ladies' houses.
5. After B, the window is erased (`Menu_CloseRow_Bank3`) and
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
| $0D | `Dialog_TechniqueMan` | Saria: the river man, once Bagu's note was read; Mido: downstab; elsewhere (not Rauru): upstab |
| $0E | `Dialog_Type0E` | Mido's man sends Link to Error of Ruto (`techniques` bit 1); Error then tells of the tunnel |
| $0F | `Dialog_Wizard` | spell `townCode`, if Link has `townCode` + 1 magic containers |
| $10-$12 | `Dialog_SetTalked` | remember the talk in `townFlags` |
| $13-$16 | `Dialog_CheckTownFlag` | second text if the town's flag is set (e.g. the mirror); the person leads Link to a door |
| $17, $18 | `Dialog_HouseLady` | the life and magic ladies |
| $1D-$20 | `Dialog_Type1D` | meant to set a flag, but its index misses its table (nothing set) |
| $21 | `Dialog_Mirror` | the mirror found (`townFlags` bit 0) |
| $0C, $19-$1C, $22 | - | one text |

`techniques` holds two quest flags besides the sword techniques: bit 0,
the river man of Saria will help Link cross; bit 1, Mido's man told him
to ask Error.

### Text Format

Each byte prints a two-tile column: a byte >= $C0 is the tile itself
(letters are $DA-$F3, digits $D0-$D9, space $F4); below $C0, the low six
bits are the tile and bits 7-6 put a mark above it.  $FD ends a line,
$FE ends a line with a longer pause, $FF ends the text.  A window holds
four lines of 10 characters.
