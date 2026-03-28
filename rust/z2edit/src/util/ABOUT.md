# About `rust/z2edit/src/util`

This directory contains shared utilities for the `z2edit` crate.

### Performance & Caching
- **`tile_cache.rs`**: A critical performance component for the editor. It manages an image cache for tiles, sprites, and metatiles. By pre-rendering game graphics based on the current CHR bank and palette, it allows the GUI to render maps and sprites with minimal overhead.

### Helpers
- **`undo.rs`**: Implements a generic undo/redo stack that tracks discrete `Edit` objects, enabling seamless history management within the editor.
- **`time.rs`**: Specialized helpers for handling time-related data within the game's constraints.
- **`mod.rs`**: The module root and common interface.
