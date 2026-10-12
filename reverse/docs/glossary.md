# Glossary

Terms used in these docs and in the disassembly's names.

## The Game World

- **Overworld**: the top-down map Link walks between places
  (`GAME_MODE_OVERWORLD`; [The overworld](world/overworld.md)).
- **Region** (`currentRegion`): one of the overworld maps: 0 West Hyrule,
  1 Death Mountain or Maze Island (told apart by `prevRegion`), 2 East
  Hyrule.
- **Location**: a place on the overworld map that leads somewhere (a cave,
  town, palace, bridge, ...), an entry in `sramLocY`/`sramLocX`/...;
  `areaLocation` is the one entered.
- **Encounter**: the blobs that chase Link on the overworld; touching one
  enters a side view **encounter area** for the terrain.
- **Side view**: the side-scrolling play of areas
  ([The side view frame](world/side-view.md)).
- **World** (`currentWorld`): the kind of side view areas and the bank that
  holds them: 0 overworld areas (caves, forests, ... of the region), 1-2
  towns (West, East), 3-4 palaces, 5 the Great Palace.  The ending uses
  world 6.
- **Area** (`areaId`): one side view map of up to 4 **pages** (screens) of
  16 x 13 **tiles**, built from a compact description
  ([Side view areas](world/areas.md)).  Also called a room.
- **Area object**: an entry of an area's description that draws part of
  it (a wall, a bridge, a window, ...): small objects, large objects (with
  a size) and items.
- **Metatile**: the 16x16-pixel tile of the area tile map, made of four
  CHR tiles; its top two bits are its palette
  ([PPU](hardware/ppu.md)).
- **Town code**, **palace code** (`townCode`, `palaceCode`): which town
  (0-7) or palace is being visited, from the location.

## Objects and Enemies

- **Object slot**: an entry of the per-object arrays (`objX`, `objY`,
  ...): 0 Link, 1-6 enemies, 7-12 enemy projectiles, 13-14 Link's
  projectiles ([Object slots](memory/objects.md)).
- **Enemy type** (`enemyType`): an index into the current enemy group's
  definition tables; the same number means different enemies in different
  banks ([Enemies](entities/README.md)).
- **Enemy group**: the definition tables of an area bank (hit points,
  experience, damage, routines, ...), copied to SRAM (`sramEnemyHp` ...);
  bank 4 has two (for palaces 1, 2 and 5, and for 3, 4 and 6).
- **Generator**: an invisible enemy that keeps spawning others (bubbles,
  Moblins, Townspeople).
- **Projectile**: an object fired by an enemy or by Link, in slots 7-14,
  with its own type.

## Game State

- **Game state** (`gameState`), **game mode** (`gameMode`), **step**
  (`gameStep`, `gameSubStep`): the nested state machines of the main loop
  ([Main loop and NMI](architecture/main-loop-and-nmi.md)).
- **Pause state** (`pauseState`): the state machine of menus and dialogs
  ([Menus](systems/menus.md)).
- **PPU string**: a run of bytes the NMI writes to the PPU (address,
  length, data), queued in `ppuBuffer0` or chosen by `ppuStringSelect`.

## Project Terms

- **Bank**: a 16K PRG ROM bank, 0-7; bank 7 is always mapped at $C000
  ([Bank layout](architecture/bank-layout.md)).
- **Area bank**: the bank holding the current world's areas (banks 1-5,
  `areaPrgBank`).
- **Interface**: routines and data that several banks provide at the same
  address (`AreaBank::`, `OverworldBank::`, `PalaceBank::`;
  [MMC1](hardware/mmc1.md)).
- **BankN suffix**: names ending in _Bank1 ... _Bank5 (and _Bank4B) are
  routines or tables that exist in several banks.
- **Mechanical name**: a placeholder made of a prefix (loc, sub, dat, ram,
  sram) and the address, kept until the item is understood.
- **Provisional**: a best-guess name or description, marked
  `Status: provisional` (or `(provisional)` in `ram.inc`).
- **QUIRK**, **BUG**: notes on odd behavior, collected in
  [Quirks](quirks.md).
- **The test movies**: the TAS movies (`private/`) whose runs are logged to
  tell executed code from data; "not executed in the test movies" marks
  code they never reach.
