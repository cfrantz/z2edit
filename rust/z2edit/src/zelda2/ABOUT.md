# About `rust/z2edit/src/zelda2`

This module implements the data model and ROM manipulation routines for "Zelda II: The Adventure of Link". It acts as the "source of truth" for the editor, managing how game data is interpreted from the ROM and how edits are tracked.

### Core Architecture
- **`Project` (`project.rs`)**: Represents the complete collection of user edits. Instead of modifying the base ROM directly, the editor tracks discrete `Edit` objects, allowing for non-destructive modification and easy undo/redo support.
- **`Edit` (`edit.rs`)**: A serializable representation of a single change to the game data, exposed to Python via `pyo3`.
- **`Config` (`config.rs`)**: A central registry that maps the game's physical ROM layout (offsets, sizes, banks) to the high-level data structures used by the editor.

### Subsystems
- **Map Editing**: `overworld.rs` (Overworld) and `sideview.rs` (Levels, Towns, Palaces).
- **Game Objects**: `enemies.rs`, `items.rs`, and `object.rs`.
- **Stat Tables**: `experience.rs`, `drops.rs`, and `encounters.rs`.
- **Visuals**: `palette.rs`, `chr.rs`, and `metatile.rs`.
- **Text**: `text_table.rs` and `text_encoding.rs` for proprietary Zelda 2 strings.
- **ROM Management**: `rom.rs` (base ROM loading) and `vchr.rs` (Virtual CHR expansion).
