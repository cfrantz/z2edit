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
