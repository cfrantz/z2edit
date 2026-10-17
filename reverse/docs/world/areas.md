# Side View Areas

A side view area is a map of up to 4 screens (pages) of 16 x 13 tiles,
built in SRAM (`sramAreaTiles`, 4 pages of `AREA_PAGE_SIZE` = $D0 bytes)
from a compact description in its area bank.  `Main_Loop` builds it with
`Area_Load`, outside the NMI, in `GAME_MODE_LOAD_AREA` and
`GAME_MODE_LOAD_ROOM`.

This format is also what z2edit reads and writes (`Map::from_bytes` in
`rust/z2edit/src/zelda2/sideview.rs`).

## Finding an Area

`Area_Load` finds the area's data through `AreaDataPtrTables` (one table
of pointers per overworld region group, indexed by `areaId`) and its
enemy list through `AreaEnemyListTables` (pointing into `sramEnemyLists`).

## Format

| Byte | Contents |
|------|----------|
| 0 | length of the data, including the header |
| 1 | bit 7: object set; bits 6-5: width in pages - 1 (`areaLastPage`); bits 4-2: a row filled across the area (grass, bushes; `AreaRowFillTiles`, `AreaRowFillRows`) |
| 2 | bit 7: no ceiling (`areaNoCeiling`); bits 6-4: tile set; bits 3-0: starting floor pattern |
| 3 | bits 7-6: sprite palette; bits 5-3: background palette; bits 2-0: background map (0 = none) |
| 4... | objects, 2 bytes each (3 for items) |

An object is a position byte and a type byte:

- The position's high nibble is the row, the low nibble the column, which
  is *relative*: it is added to the previous object's column
  (`Area_AdvanceColumn`).
- Row 13 changes the floor pattern from this column on; the type byte is
  the new pattern (`Area_DrawFloors`).
- Row 14 skips to the start of page (column & 3) (`Area_CheckPageSkip`).
- Types $00-$0E are small objects (`Area_DrawSmallObject`).
- Type $0F is an item; its kind is in the next byte (`Area_PlaceItem`).
- Types $10 and up are large objects: the high nibble is the kind and the
  low nibble the size (`Area_DrawLargeObject`).

Objects are drawn by routines in the area bank (`AreaBank::DrawLargeObject`,
`AreaBank::DrawSmallObject`; the town bank has its own), so each world has
its own set of objects.

In every area bank the large objects are mostly the same few shapes, a
row, two rows, a column or a pillar of the object's size, filled with
tiles from a per-object table (`Area_GetObjectTiles_Bank1`,
`Palace_GetObjectTiles_Bank4`, ...); bit 7 of header byte 1 selects the
second table (z2edit's object set 1), which only banks 1 and 2 have.  The
other objects are drawn by small routines of their own: lava, cacti,
clouds, graves, doors and so on.  The objects of each bank are named
after z2edit's render configs:

| Banks | Areas | Drawers |
|-------|-------|---------|
| 1, 2 | overworld side views (caves, forests, ...) | `Area_Draw*_Bank1`, `Area_Draw*_Bank2` |
| 3 | towns | `Town_Draw*` (see [Towns](towns.md)) |
| 4, 5 | palaces, Great Palace | `Palace_Draw*` (see [Palaces](palaces.md)) |

A small object ($05, or $0E and $0F in towns) and the large extra $50
record the column of the area's locked door and elevator (`Area_PlaceLockedDoor_*`,
`Area_PlaceElevator_*`).

## Building the Tile Map

`Area_Load`:

1. fills the tile map with the empty tile ($40, or $42 depending on the
   tile set);
2. if the area has a background map, builds it first, from
   `AreaBank::BackgroundAreas`;
3. draws the area's floors and ceiling (`Area_DrawFloors`, a column at a
   time with `Area_DrawFloorColumn` and the world's floor tiles from
   `Area_SetFloorTiles`);
4. draws its objects (`Area_DecodeObjects`), which also notes the elevator
   and locked door positions;
5. sets the scroll limits for the starting page.

The game then reads the tile map for collision (`Obj_GetBgTile`,
`AreaBank::IsSolidTile`) and changes it when blocks are broken.  A tile
is solid if it is at or above its palette group's threshold
(`SolidTileThresholds_Bank1` ...; the group is bits 7-6 of the tile).
For group 0 the lookup wraps around and reads an unrelated byte, which
makes every group 0 tile solid in bank 2 and none in the other banks.

## Overworld Objects (Banks 1 and 2)

Banks 1 (West Hyrule, Death Mountain) and 2 (East Hyrule, Maze Island)
have the same object code at the same addresses.  A large object's size
is its type's low nibble.

| Type | Row | Routine | Drawn |
|------|-----|---------|-------|
| $1x | 0-$E | none in bank 1 ($0000); `Area_DrawPillarWall_Bank2` | bank 2: pillars of $Ex's tiles, the object's size, in every column to the end of the area |
| $2x-$4x | 0-$E | `Area_DrawObjectTwoRows_*` | a row of size + 1 of the main tile, the second tile below |
| $5x-$Cx | 0-$E | `Area_DrawObjectRow_*` | a row of size + 1 of the main tile |
| $Dx | 0-$E | `Area_DrawObjectColumn_*` | a column of size + 1 (unused) |
| $Ex, $Fx | 0-$E | `Area_DrawObjectPillar_*` | the main tile over size of the second |
| $1x | $F | `Area_DrawObjectPillar_*` | as above |
| $2x | $F | `Area_DrawLava_*` | size + 1 wide: $80 on row $A, $89 on rows $B-$C |
| $3x | $F | `Area_DrawCactus_*` | size of $47 up from row 9, topped by $46 |
| $4x | $F | `Area_DrawCactus2_*` | $49 on row 9, $48 on row 8 |
| $5x | $F | `Area_PlaceElevator_*` | the elevator's column |

The main and second tiles come from `AreaObjectTiles_*` (object set 0)
or `AreaObjectTiles2_*` (set 1, header byte 1 bit 7), a pair per type.
For example, set 1's pillar $Fx is $59 over $5A, and set 0's $Ex is $4C
over $4B.

| Small | Routine | Drawn |
|-------|---------|-------|
| $00 | `Area_DrawHeadstone_*` | $C5 |
| $01 | `Area_DrawCross_*` | $C6 |
| $02 | `Area_DrawAngledCross_*` | $C7 |
| $03 | `Area_DrawTreeStump_*` | $5B over $5C |
| $04 | `Area_DrawStonehenge_*` | a 4 x 3 stone arch (`StonehengeTiles_*`) |
| $05 | `Area_PlaceLockedDoor_*` | the locked door's column |
| $06, $07 | `Area_DrawSleepingZelda_*` | $4E $4F (North Castle) |
| $08 | `Area_DrawPit_*` | $40 down to row $C |
| $09 | `Area_DrawLargeCloud_*` | $C2 $C3 |
| $0A-$0F | `Area_DrawSmallCloud_*` | $C2 $C4 |

Only the background layers place clouds.  Floors and ceilings come from
`AreaFloorRecords_*`, 9 bytes per tile set: 8 tiles chosen by the floor
pattern, then `floorTile`.

### Checking Objects

`tools/area_objects.py BANK [--group 2] [--by-routine]` decodes every
area's objects and names the routine that draws each.  `--by-routine`
lists which areas use each routine.  `tools/area_dump.py` loads areas in
the emulator (`areaId` with `gameMode` $14, from a start made with
`--at`) and prints the tile maps the game builds.  `--set
currentRegion=1` selects the second group.

These dumps matched every routine above except the unused column:

- West Hyrule: cacti in area $1D, stonehenge in $26, lava in $06, tree
  stumps in $14, graves in $34, Zelda in $00.
- Death Mountain: the pit and elevator in $10.
- East Hyrule: the pillar wall in $02, cacti in $0F, stonehenge in $2C.

```
AT=areaPrgBank=1,currentRegion=0,currentWorld=0,townCode=0,palaceCode=0,areaLocation=0x13,areaId=0x01,areaStartPage=0,areaEntrySide=0,prevRegion=0
tools/area_dump.py --movie build/movies/file_select.fm2 --at $AT --until areaId=0x01 --areas 0x1D,0x26,0x06,0x14,0x34,0x00
tools/area_dump.py --movie build/movies/file_select.fm2 --at $AT --until areaId=0x01 --set currentRegion=1 --areas 0x10
AT2=areaPrgBank=2,currentRegion=2,currentWorld=0,townCode=0,palaceCode=0,areaLocation=0x0D,areaId=0x09,areaStartPage=0,areaEntrySide=0,prevRegion=2
tools/area_dump.py --movie build/movies/file_select.fm2 --at $AT2 --until areaId=0x09 --areas 0x02,0x0F,0x2C
```

## Scrolling

While Link moves, `Scroll_Update` scrolls the screen to follow him,
between the area's page limits (`sramScrollMinPage`, `sramScrollMaxPage`,
by `areaLastPage`); single-screen areas and boss rooms (`scrollFrozen`)
do not scroll.  At the end of each frame `Scroll_DrawColumns` draws the
columns coming into view from the tile map, a little at a time (8 steps
a column), with their attributes.

## Moving Between Areas

Each area has four connections, one per page edge, in
`sramRoomConnections` (index `areaId` * 4 + page):

- Walking off the left or right edge (`Mode_LeaveArea`, gameMode $10)
  takes the connection of that edge: $FC-$FF return to the overworld
  (gameMode $01); anything else enters the connected area (gameMode $07),
  at the other side (`areaEntrySide`).
- An elevator leaving the screen (`Elevator_Ai`: Y >= $D8, which going up
  is reached by wrapping past 0) goes to the area above or below, by the
  direction held (`Mode_FindConnectedRoom`, gameMode $13).
- Entering from the overworld, `Location_SetupEntry` and `Mode_EnterArea`
  (gameMode $12) set up the area and its start page.

Leaving blanks the screen and removes the enemies (`Area_Exit`); the new
area is loaded (`Area_Load`), drawn a step per frame (`Mode_DrawArea`), and
`Link_EnterArea` places Link at the edge he came from, or where the
elevator or a door puts him, and starts the area's music.
