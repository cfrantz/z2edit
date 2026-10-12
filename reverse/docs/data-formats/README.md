# Data Formats

The ROM's data structures, where they are documented, and how they line
up with z2edit's schema (`config/vanilla/*.json5`).  The full list of
config addresses with the disassembly symbol at each is generated in
[config-xref.md](config-xref.md) (`tools/config_xref.py`).

## Structures

| Data | Where | Format described in |
|------|-------|---------------------|
| Side view areas: header and objects | `AreaBank::AreaPtrs` (and `AreaPtrs2`), banks 1-5 | [Side view areas](../world/areas.md) |
| Area connections (page edges) | `AreaBank::Connections` (and `Connections2`) | [Side view areas](../world/areas.md#moving-between-areas) |
| Town doors | `TownDoors` (`AreaBank::Doors`, bank 3) | [Towns](../world/towns.md) |
| Enemy lists of areas | `AreaBank::EnemyListPtrs` into `AreaBank::EnemyLists` | [Enemies](../entities/README.md) |
| Enemy definitions (hit points, damage, routines, ...) | `ProjectilePtrs_Bank1` ... at $9400 of each area bank | [Enemies](../entities/README.md) |
| Projectile drawing (tiles, attributes, routines) | `ProjectileTiles_Bank1` ... | [PPU](../hardware/ppu.md#sprites) |
| Overworld maps | `AreaBank::OverworldMapPtrs` | [The overworld](../world/overworld.md) |
| Overworld locations | `OverworldBank::Locations` (and `Locations2`), copied to `sramLocY` ... | [The overworld](../world/overworld.md) |
| Encounters | `OverworldBank::Encounters`, `EncounterTerrains` ... | [The overworld](../world/overworld.md#encounters) |
| Area palettes | `AreaBank::Palettes`; palaces `PalaceOutdoorPalettes`, `PalaceIndoorPalettes` | [PPU](../hardware/ppu.md#palettes) |
| Metatiles | `AreaBank::MetatilePtrs`, `OverworldTileMetatiles` | [PPU](../hardware/ppu.md#metatiles) |
| Town dialog text | `TownText`, `WestTextIds` ..., `TownTextTables` | [Towns](../world/towns.md#text-format) |
| Music | `OverworldMusic`, `TownMusic`, `PalaceMusic`, `GreatPalaceMusic`; `TitleMusic` | [Sound](../audio/engines.md) |
| Level up and spell costs | `LevelUpCostHi`/`Lo`, `SpellCosts` | [Link](../systems/link.md) |
| Damage to Link | `LinkDamage` | [Link](../systems/link.md#taking-damage) |
| Save files and new game defaults | `SaveDefaultStats`, `SaveDefaultWorld` | [Save files](../memory/save-files.md) |
| Story, credits, ending text | `StoryText`, `CreditsText`, `EndingMessage` | [Title, file screens and ending](../systems/title-and-files.md) |

## Notes on the Config

Where z2edit's schema and the code differ, or the config uses another
name:

- **Connections**: banks 1 and 2 have 29 areas of connections per group
  (`max_connectable_index: 28`), not 63; the 116 bytes end where
  `PalaceStoneMapPtrs_Bank1` begins.  Banks 3-5 have 63.
- **Doors**: every sideview group lists `doors` at $8817, but only the
  towns (bank 3, `max_door_index: 28`) use them; in banks 1 and 2 that
  address is inside other data, which is harmless with `max_door_index: 0`.
- **availability** (`Prg: [5, 0xbb15]` ...) is the default area flags of
  each world, the slices of `SaveDefaultWorld` that a new game starts
  with.
- **xp** of an enemy group (`$94D5` ...) is `EnemyAttr_Bank1` ...: the
  experience class is in its low bits, with the palette and other flags.
- **Spell costs** appear twice: the costs used (`SpellCosts`, bank 0
  $8D7B) and the digits shown in the pause menu (inside
  `MenuTextTemplate`, $9C2A ...); an editor has to keep both in step.
- **Patch points**: many config entries are operands inside instructions
  (lives, item effects, sword beam timing, hidden places); the
  cross-reference shows them as `Routine+k`.
