# Memory Map

An overview of RAM and SRAM.  The variables themselves are declared, with
one-line descriptions, in `src/ram.inc`.

## Internal RAM ($0000-$07FF)

| Range | Contents |
|-------|----------|
| $0000-$000F | `temp0`-`temp15`: scratch, used freely by many routines |
| $0010-$00DF | game variables; the object slot arrays (`objY`, `objX`, ... see [objects](objects.md)) |
| $00E0-$00EF | sound |
| $00F5-$00F8 | controllers (`joypadPressed`, `joypadHeld`) |
| $00FC-$00FF | PPU register shadows |
| $0100 | `nmiMode` |
| $0100-$01FF | stack |
| $0200-$02FF | `oamBuffer`: sprites, copied by OAM DMA every NMI |
| $0301-$03D5 | PPU update buffers (`ppuBuffer0`-`ppuBuffer2`, each preceded by its length) |
| $03D6-$05FF | more object arrays and game variables |
| $0600-$06DF | `saveWorld`: world state saved with the game |
| $0700-$07FF | game state, Link's stats and inventory, sound |

## Battery-Backed SRAM ($6000-$7FFF)

| Range | Contents |
|-------|----------|
| $6000-$633F | `sramAreaTiles`: the current side view area's tile map |
| $6002-$6337 | save file backups, used while saving and on the file screens (overlapping the tile map) |
| $6A00-$6C57 | overworld locations, area connections and doors (from the area bank) |
| $6D00-$6FA0 | `sramEnemyData`: enemy definitions (from the area bank) |
| $7000-$73FF | `sramEnemyLists`: the enemies of each area (from the area bank) |
| $7400-$773A | save files; see [save files](save-files.md) |
| $7919-$79F8 | `sramAreaPalettes` (from the area bank) |
| $7C00-$7F7F | `sramOverworldMap`: the region's compressed overworld map (from the area bank) |

When a world is loaded (`gameMode` 0, `Mode_LoadWorld`), the world's tables are
copied from its area bank into SRAM, so that area loading and the game
can read them whichever bank is mapped.  The game state therefore depends
on SRAM contents beyond the save files.

## How the Map Was Built

- `tools/cdl_movie.py` records which instructions read and write each RAM
  address while playing the test movies; `tools/ram_usage.py` reports it
  and finds arrays from indexed accesses.
- `tools/callers.py` and `tools/watch.py` play a movie with breakpoints
  and watchpoints, to see which slots a routine is called with and how a
  variable changes over a game.
- Names and meanings were checked against Lemmy's disassembly,
  datacrystal and z2edit's configuration (for the SRAM world tables), and
  confirmed with RAM snapshots from the no-glitch movie where possible.
