# About `rust/z2edit/src/gui/zelda2`

This subdirectory contains the specialized GUI editors for the "Zelda II: The Adventure of Link" ROM components. Each module is built to provide an intuitive interface for complex ROM data structures.

### Map & World Editors
- **`overworld.rs`**: Editor for overworld maps and connection tables.
- **`sideview.rs`**: The primary level editor for sideview areas, enemy/item placement, and connectivity.
- **`multimap.rs`**: A visualization tool for entire connected sideview areas (e.g., palaces, towns).

### Graphics & Visuals
- **`chr.rs` & `vchr.rs`**: Editors for standard and virtual (extended) CHR banks.
- **`metatile.rs`**: Editor for background 16x16 metatiles.
- **`palette.rs`**: A specialized color palette editor for game visuals.

### Game Mechanics & Stats
- **`enemies.rs`**: Editor for global enemy properties.
- **`drops.rs`**: Controls the enemy drop table and dripper properties.
- **`encounters.rs`**: Editor for overworld encounter triggers.
- **`experience.rs`**: Adjusts the level-up tables for Life, Magic, and Attack.
- **`items.rs`**: Editor for collectable item availability and conditions.
- **`start.rs`**: Editor for initial starting properties (stats, items, position).

### Data & Misc
- **`banks.rs`**: Visual representation and navigation of PRG ROM banks.
- **`text_table.rs`**: Editor for NPC dialogue and town text.
- **`metadata.rs`**: Editor for project-level information and notes.
- **`misc_hacks.rs`**: Selection and adjustment of built-in ROM patches.
- **`config.rs`**: Core GUI initialization logic driven by the vanilla configuration.
