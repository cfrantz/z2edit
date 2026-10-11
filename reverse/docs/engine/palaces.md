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
are drawn by `Area_DrawLargeObject_Bank4` / `_Bank5` and
`Area_DrawSmallObject_Bank4` / `_Bank5`.  Most large objects are shapes
filled with tiles from a per-object table (`PalaceObjectTiles_Bank4`,
`Palace_GetObjectTiles_Bank4`): a row (`Palace_DrawObjectRow_*`), two
rows, a column, or a pillar.  The others are lava, pits, the elevator and
locked door positions (`Area_PlaceElevator_*`, `Area_PlaceLockedDoor_*`),
two-tile decorations (window, unicorn and rat heads, the Ironknuckle or
bird knight statue), clouds and the crystal statue
(`Palace_DrawCrystalStatue`).  The Great Palace adds the elevator shaft
and Zelda's bed (`Palace_DrawSleepingZelda`, in the ending's room).  The
names follow z2edit's palace render configs.

## Bosses and the Crystal

A palace's boss (`Boss_UpdateIntro`) freezes the scroll and starts the
boss music when Link reaches page 1 of its room; when it dies, sparks fly
from it (`Boss_SpawnDeathSpark_Bank4`, from `Enemy_DieStep`), the other
enemies are destroyed and the music stops (song $80, which every world
leaves empty).

The crystal statue (`CrystalSpot_Ai`) takes the palace's crystal; see
[Palace enemies](../entities/palaces.md).  Once placed, the area flag of
the spot is cleared, `crystalsRemaining` drops, and on the overworld the
palace is shown as stone (`Overworld_UpdatePalaceStones`).
