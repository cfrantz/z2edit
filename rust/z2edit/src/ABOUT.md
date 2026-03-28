# About `rust/z2edit/src`

This crate implements the core logic for the "Zelda II: The Adventure of Link" ROM editor. It leverages the general-purpose `nes` and `python_gui` crates to provide a feature-rich, hybrid Rust/Python editing environment.

### Core Modules
- `zelda2/`: The primary data model for the game, including ROM packing/unpacking and edit tracking.
- `gui/`: Implements the user interface for the editor using the `python_gui` framework.
- `util/`: Shared utilities, including a high-performance image and tile cache for the GUI.

### Key Files
- `lib.rs`: The crate root and `pyo3` binding entry point consumed by the Python shell.
- `app_preferences.rs`: Management of user-configurable editor settings.
- `error.rs`: Centralized error handling for the `z2edit` workspace.
