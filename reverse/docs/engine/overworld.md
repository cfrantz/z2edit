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

Encounter sprites wander toward Link.  When one touches him on terrain
where encounters happen, `Overworld_DrawEncounters` sets `areaLocation`
to $FF and enters it like a location; `Location_EnterEncounter` then
makes a temporary location (slot $3E) from the encounter area for the
terrain (`OverworldBank::Encounters`).

## Entering Locations

`Mode_EnterLocation` and `Location_Enter` find what the location leads
to: another region, a town, a palace, or a side view area
(`Location_GetArea`), and how Link enters it (`Location_SetupEntry`,
`Location_CheckFallEntry` for a hole that he falls into).  Returning,
`Mode_ReturnToOverworld` puts him back beside the location he left.
