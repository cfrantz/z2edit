# The Overworld

On the overworld (`GAME_MODE_OVERWORLD`), the game NMI runs
`Mode_Overworld` once per frame (bank 0, with the overworld's area bank
mapped for map access).  The overworld reuses zero page $73-$7E and the
$0400-$04FF area (`overworldBuffer`) that side view uses for objects.

## Each Frame

`Mode_Overworld`:

1. `Overworld_CheckEncounter` and `Overworld_UpdateEncounters`: spawn
   and move the encounter sprites (slots 2-7).
2. `Overworld_DrawLink`.
3. If a move is under way (`overworldMoveFrames`), continue it
   (`Overworld_ContinueMove`).
4. Otherwise read the controller (`Overworld_HandleInput`: the hammer on
   rocks, the flute) and, when Link has just finished a step onto a
   location (`sramLocY`, `sramLocX`), enter it (`Overworld_EnterLocation`;
   on a raft dock, the raft ride instead); else start a move in the
   direction held (`Overworld_StartMove`).

## Terrain

The map's squares are terrain types 0-$F: 0 town, 1 cave, 2 palace,
3 bridge, 4 desert, 5 grass, 6 forest, 7 swamp, 8 graveyard, 9 road,
$A lava, $B mountain, $C water, $D water the boots walk on, $E rock,
$F the river devil.  (Link stands on road, 9, in the no-glitch movie's
first overworld frames.)  Off the map, rows and columns are water ($C),
or mountain ($B) along the west edge of regions 0 and 1.

## Moving

A move is one map square in `overworldDirection` (1 = right, 2 = left,
4 = down, 8 = up), allowed when the terrain there lets Link walk
(`Overworld_CanWalk`: terrain $0B and up blocks, except $0D with the
boots).  It takes 16 frames (`overworldMoveFrames`), twice as long in a
swamp (terrain 7, moving every other frame).  Each frame the screen
scrolls a pixel and one step of drawing the map coming into view runs:

- moving up or down: `Overworld_DrawRowStep` decodes the new row of the
  map (`Overworld_DecodeRowAt`), queues its two rows of tiles
  (`Overworld_QueueRowTiles`) and its attribute bytes
  (`Overworld_QueueRowAttrs`);
- moving left or right: `Overworld_DrawColumnStep` decodes the new column
  (`Overworld_DecodeColumn`), queues its tiles in six parts
  (`Overworld_QueueColumnTiles`), decodes the neighboring column for the
  attribute bytes (`Overworld_DecodeNextColumns`) and queues them.

Map squares are drawn as 2x2 tiles (`OverworldTileMetatiles`) with a
palette per terrain (`OverworldTerrainPalettes`).  The map is
run-length compressed in the area bank; `Mode_IndexOverworldMap` builds
a table of row pointers in SRAM when the overworld is set up, and
`Overworld_GetRowPtr` / `Overworld_ReadMapRun` decode from it.

## Encounters

`Overworld_CheckEncounter` starts a group of encounter sprites if the
terrain under Link is one of `EncounterTerrains`: desert (4), grass (5),
forest (6), swamp (7), graveyard (8) and lava ($A); not roads.  It
starts one when the encounter timer (`timerSlow+8`, 160 frames after
the overworld is set up) runs out, or while the walking counter
(`overworldWalkFrames`, one per frame of walking, wrapping) is 0, once
every 256 walking frames; north of row $3C in West Hyrule only the timer
counts.  In the movies, 27 of 70 and 32 of 82 groups came from the
walking counter.  By the terrain it resets the timer (`EncounterDelays`),
sets how long the sprites stay (`EncounterLifetimes`), and picks one of
two sets of four starting positions (`EncounterPositionSets`,
`EncounterPositions`); a random one of the four is left out.  A random
number against the terrain's four thresholds (`EncounterChances`)
chooses the strength (0-3), which sets the kinds of the three sprites
(`EncounterKinds`: 1 weak, 2 strong, 3 a fairy; drawn with
`EncounterSpriteTiles` and `EncounterSpriteAttrs`).  There are 8 slots.

Every 16 frames each sprite takes a step for the next 16 frames
(`Overworld_UpdateEncounters`): toward Link when it is in his row or
column (within 16 pixels), otherwise in a random direction; fairies, and
everyone during frames 0-$3F of each 256, wander.  When one touches Link
on terrain where encounters happen, `Overworld_DrawEncounters` sets
`areaLocation` to $FF and enters it like a location; `Location_EnterEncounter` then
makes a temporary location (slot $3E) from the encounter area for the
terrain (`OverworldBank::Encounters`).

## Entering Locations

`Mode_EnterLocation` and `Location_Enter` find what the location leads
to: another region, a town, a palace, or a side view area
(`Location_GetArea`), and how Link enters it (`Location_SetupEntry`,
`Location_CheckFallEntry` for a hole that he falls into).  Returning,
`Mode_ReturnToOverworld` puts him back beside the location he left.

## Palaces as Stone

`Overworld_UpdatePalaceStones` (`OverworldBank::UpdatePalaceStones`, run
when the overworld is set up) turns a palace into stone on the map once
Link has both its item (`itemOwned`, same index as the palace: candle,
glove, raft, boots, flute, cross) and placed its crystal
(`crystalPlaced`): it writes tile $0B at the palace's place in the map
(`PalaceStoneMapPtrs_Bank1`, `PalaceStonePalaces_Bank1`; bank 2 has its
own).

## The Raft

On a raft dock with the raft (`Overworld_EnterLocation`), Link rides
across the water (`Mode_Raft`), moving by `RaftSteps` in `raftDirection`
until he reaches the other dock (`RaftStopX`).  Both movies ride only
from West Hyrule's dock (X $3D, rightward): the raft leaves from Link's
X $80 until it is off the screen, the other region is loaded, and it
comes in for $80 frames to X $7C.

## The Screen's Right Edge

`Overworld_DrawLink` also places 15 sprites (tile $FF) down the screen's
right edge, X $F8, which hide the map column being redrawn there: the
overworld always shows a black strip 8 pixels wide on the right.

## The Hammer and the Flute

A (with the hammer) on a rock ($E) or forest (6) square ahead of Link,
and the flute (B) on the river devil ($F) or on East Hyrule's desert
spot (Y $64, X $2D), change the map (`Overworld_CheckSpecialTerrain`):
rocks and the river devil become desert, the desert spot reveals palace
6, and one forest square in East Hyrule (Y $51, X $3E) reveals New
Kasuto; other forest is left as it is (the hammer's sound still
plays).
