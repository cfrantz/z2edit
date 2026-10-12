# Zelda II Documentation

Notes on how the game works, written alongside the disassembly in `src/`.
Symbol names refer to `src/`; `make verify` checks that they still exist.

- [Glossary](glossary.md): the game's and the project's terms.
- [Quirks](quirks.md): bugs and odd behaviors.

## Architecture

- [Bank layout](architecture/bank-layout.md): what each PRG bank holds.
- [Main loop and NMI](architecture/main-loop-and-nmi.md): the frame, and
  the game state machine.
- [MMC1](hardware/mmc1.md): bank switching and the bank interfaces.
- [PPU](hardware/ppu.md): CHR banks, palettes, metatiles and sprites.

## Memory

- [Memory map](memory/memory-map.md): RAM and SRAM.
- [Object slots](memory/objects.md): Link, enemies and projectiles, and
  collision.
- [Save files](memory/save-files.md): what is saved and how.

## Enemies

- [Enemies and objects](entities/README.md): the definition tables,
  damage, experience, drops; a page per enemy group.

## Data

- [Data formats](data-formats/README.md): where each data structure is
  described, and how it matches z2edit's config.

## World

- [The overworld](world/overworld.md): moving, drawing the map,
  encounters, entering locations, palaces as stone, the raft.
- [Side view areas](world/areas.md): the area format, objects, building
  the tile map, scrolling and moving between areas.
- [The side view frame](world/side-view.md): what runs each frame.
- [Towns](world/towns.md): town codes, buildings, conversations and the
  dialog text.
- [Palaces](world/palaces.md): palace codes, CHR and palettes, area
  objects, bosses and the crystal.

## Systems

- [Link](systems/link.md): movement, jumping, damage, experience, the
  sword, spells, holding up items.
- [Enemies](systems/enemies.md): how enemies spawn, move, are hit and die.
- [Collision and combat](systems/collision-and-combat.md): background
  probes, hitboxes, the shield, damage.
- [Random numbers](systems/rng.md): the 72-bit shift register.
- [Items and drops](systems/items.md): item kinds, collecting, enemy drops.
- [Death and continuing](systems/death.md): losing a life, game over, the continue screen.
- [Menus and dialogs](systems/menus.md): the pause menu, level up window,
  flute and continue screen.
- [Title, file screens and ending](systems/title-and-files.md): the
  title, the story scroll, file select, name registration and
  elimination, the ending and credits, and the second quest.

## Audio

- [Sound engines](audio/engines.md): the two sound engines in bank 6,
  sound requests and effects, and the music data formats.
