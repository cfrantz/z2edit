# About `python/z2edit/emulator`

This directory contains the Python components of the NES emulator and its plugin system. The emulator is designed as a hybrid Rust/Python application, where the high-performance core (Rust) is extended via a flexible Python plugin architecture.

### Core Components
- `app.py`: The entry point for the standalone emulator application.
- `emu.py`: Python-side bindings and high-level management of the Rust emulator instance.
- `plugin.py`: Defines the base interface and discovery mechanism for emulator plugins.

### Plugins
The emulator supports runtime extensibility through plugins located here:
- `zelda2/`: A game-specific plugin providing deep state inspection, memory mapping, and specialized debug views for "Zelda II: The Adventure of Link".
- `midi/`: A plugin that transforms the NES emulator into a MIDI-controlled synthesizer, utilizing the APU's audio channels.
- `fm2.py`: Provides support for loading and playing FCEUX movie (`.fm2`) files for input recording and playback.
