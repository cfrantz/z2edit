# Palaces

The six palaces are side view areas of worlds 3 and 4, in bank 4; the
Great Palace is world 5, in bank 5.  The enemies and bosses are described
in [Palace enemies](../entities/palaces.md) and
[Great Palace enemies](../entities/great-palace.md).

## Which Palace

Entering a palace's overworld location sets `palaceCode` = location -
$34 (bank 7, $CB88).  With the region it indexes bank 7's palace tables
(index region * 4 + `palaceCode`):

| Index | Palace | CHR bank (`PalaceChrBanks`, halved) | Palette offset (`PalacePaletteOffsets`) |
|-------|--------|------|---------|
| 0 | 1 (Parapa) | 4 | $00 |
| 1 | 2 (Midoro) | 5 | $10 |
| 2 | 3 (Island) | 9 | $20 |
| 4 | 4 (Maze Island) | 10 | $30 |
| 8 | 5 (Ocean) | 11 | $40 |
| 9 | 6 (Three-Eye Rock) | 12 | $50 |
| 10 | Great Palace | 6 | $60 (its own palettes) |

The palette offset selects 16 bytes of `PalaceOutdoorPalettes` and 16 of
`PalaceIndoorPalettes` in bank 4, which bank 7 copies to
`sramAreaPalettes` when the area loads.

Bank 4 holds two enemy groups, one for palaces 1, 2 and 5 (`*_Bank4`) and
one for palaces 3, 4 and 6 (`*_Bank4B`).

## Area Objects

Palace areas use the [side view area](areas.md) format; their objects
are drawn by `Area_DrawLargeObject_Bank4` / `Area_DrawLargeObject_Bank5`
and `Area_DrawSmallObject_Bank4` / `Area_DrawSmallObject_Bank5`.  Most large objects are shapes
filled with tiles from a per-object table (`PalaceObjectTiles_Bank4`,
`Palace_GetObjectTiles_Bank4`): a row (`Palace_DrawObjectRow_*`), two
rows, a column, or a pillar.  The others are lava, pits, the elevator and
locked door positions (`Area_PlaceElevator_*`, `Area_PlaceLockedDoor_*`),
two-tile decorations (window, unicorn and rat heads, the Ironknuckle or
bird knight statue), clouds and the crystal statue
(`Palace_DrawCrystalStatue`).  The Great Palace adds the elevator shaft
and Zelda's bed (`Palace_DrawSleepingZelda`, in the ending's room).  The
names follow z2edit's palace render configs.

| Large | Bank 4 (palaces 1-6) | Bank 5 (Great Palace) |
|-------|----------------------|-----------------------|
| $1x-$5x | row (size + 1 wide) | row |
| $6x, $7x | two rows | row |
| $8x-$Ax | two rows | two rows |
| $Bx | column (size + 1 high) | two rows |
| $Cx | pillar: the main tile over size of the second | two rows |
| $Dx | pillar | column |
| $Ex | pillar | column |
| $Fx | pit: $40, size + 1 wide, down to row $C | pillar ($46 over $47) |
| $1x-$4x on row $F | lava: $83 on row $B, $84 on row $C | lava: $83 on row $A, $84 below |
| $5x on row $F | the elevator's column | the elevator's column |

| Small | Bank 4 | Bank 5 |
|-------|--------|--------|
| $00 | window, $80 over $81 | window, $80 over $81 |
| $01 | "unicorn head" $46 | "unicorn head" $44 |
| $02 | "rat head" $45 | "rat head" $43 ($02-$04) |
| $03, $04 | the crystal statue (4 x 5, `CrystalStatueTiles`) | (rat head) |
| $05 | the locked door's column (also $06 in bank 4) | the locked door's column |
| $06 | | the elevator shaft: $40 down to row $C |
| $07 | large cloud, $C2 $C3 | large cloud |
| $08, $0A-$0F | small cloud, $C2 $C4 | small cloud ($08, $0B-$0F) |
| $09 | statue, $49 over $4A | Zelda asleep, $86 $87 |
| $0A | | statue, $4A over $4B |

The two banks' head decorations are named after z2edit, which swaps them
between the banks.  Each bank's `PalaceObjectTiles_*` holds the tiles of
its large objects, a pair per type.  Floors come from
`PalaceFloorRecords_*`, 9 bytes per background type.

These were checked against the tile maps the game builds
(`tools/area_dump.py`; see [Side view areas](areas.md#checking-objects)).
The areas were palaces 1, 2 and 5's areas $00, $04, $09, $0D, $1A and
$27; palaces 3, 4 and 6's $02, $05 and $0A (`--set currentWorld=4`); and
the Great Palace's $00, $03, $08, $09, $0A and $3D:

```
PAT=areaPrgBank=4,currentRegion=0,currentWorld=3,townCode=4,palaceCode=0,areaLocation=0x34,areaId=0,areaStartPage=0,areaEntrySide=0,prevRegion=0
tools/area_dump.py --movie build/movies/file_select.fm2 --at $PAT --until currentWorld=3 --areas 0x00,0x04,0x09,0x0D,0x1A,0x27
tools/area_dump.py --movie build/movies/file_select.fm2 --at $PAT --until currentWorld=3 --set currentWorld=4 --areas 0x0A,0x05,0x02
GP=areaPrgBank=5,currentRegion=2,currentWorld=5,townCode=4,palaceCode=2,areaLocation=0x36,areaId=0,areaStartPage=0,areaEntrySide=0,prevRegion=2
tools/area_dump.py --movie build/movies/file_select.fm2 --at $GP --until currentWorld=5 --areas 0x00,0x03,0x08,0x09,0x0A,0x3D
```

## Bosses and the Crystal

A palace's boss (`Boss_UpdateIntro`) freezes the scroll and starts the
boss music when Link reaches page 1 of its room; when it dies, sparks appear
around it (`Boss_SpawnDeathSpark_Bank4`, from `Enemy_DieStep`), the other
enemies are destroyed and the music stops (song $80, which every world
leaves empty).

A boss's death lasts 126 frames (`Enemy_DieStep`, its death timer from
$7E): the boss stays drawn for 23 frames, then the screen flashes
(`BossDeathPalette`) with the boss dying sound, and for the remaining 103
frames a copy of Link, in Dark Link's palette, stands 8 pixels from him
in his pose while the sparks burst where the boss was.

The crystal statue (`CrystalSpot_Ai`) takes the palace's crystal; see
[Palace enemies](../entities/palaces.md).  Once placed, the area flag of
the spot is cleared, `crystalsRemaining` drops, and on the overworld the
palace is shown as stone (`Overworld_UpdatePalaceStones`).
