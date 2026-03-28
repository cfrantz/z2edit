# About `config/`

This directory contains the various configuration files used by the `z2edit` application. These configurations define the behavior of the ROM editor, the base game's schema, and the built-in NES emulator's extensions.

### Subdirectories
- **`vanilla/`**: The most critical configuration. These JSON5 files define the **ROM schema** for the original "Zelda II: The Adventure of Link", including all physical offsets, bank layouts, and data structures. This is the source of truth for the Rust-side data model.
- **`emulator/`**: Contains configurations for the emulator's Python-based plugin system.
  - **`midi/`**: Instrument and channel definitions for using the NES emulator as a MIDI synthesizer.

### Configuration Strategy
The project uses a data-driven approach where the core logic (Rust) is decoupled from the game-specific layout (JSON5). This allows for easier adaptation to different ROM versions or revisions by simply providing a new configuration set in this directory.
