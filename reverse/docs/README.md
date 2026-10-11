# Zelda II Documentation

Notes on how the game works, written alongside the disassembly in `src/`.
Symbol names refer to `src/`; `make verify` checks that they still exist.

## Architecture

- [Bank layout](architecture/bank-layout.md): what each PRG bank holds.
- [Main loop and NMI](architecture/main-loop-and-nmi.md): the frame, and
  the game state machine.
- [MMC1](hardware/mmc1.md): bank switching and the bank interfaces.

## Memory

- [Memory map](memory/memory-map.md): RAM and SRAM.
- [Object slots](memory/objects.md): Link, enemies and projectiles, and
  collision.
- [Save files](memory/save-files.md): what is saved and how.

## Engine

- [The side view frame](engine/side-view.md)
- [Link](engine/link.md): movement, jumping, spells, damage, experience.
- [Enemies](engine/enemies.md)
- [Side view areas](engine/areas.md): the area format and how areas are
  built.
- [The overworld](engine/overworld.md): moving, drawing the map,
  encounters, entering locations.
- [Menus and dialogs](engine/menus.md): the pause menu, level up window,
  flute and continue screen.
