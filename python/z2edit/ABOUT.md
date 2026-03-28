# About `python/z2edit`

This directory contains the Python shell and extension modules for the `z2edit` editor. The application is built as a hybrid Rust/Python environment, where high-performance logic is handled in Rust and high-level application logic, extensibility, and scripting are handled in Python.

### Core Shell
- `app.py`: The main Python application that initializes the hybrid environment and starts the editor.
- `__main__.py`: Standard Python package entry point.
- `debug.py`: Interactive debugging utilities (e.g., memory inspection and hexdumps).

### Extension Subsystems
- **`emulator/`**: Hosts the Python-side components of the NES emulator and its powerful plugin system (e.g., Zelda II state tracking, MIDI synthesis).
- **`hacks/`**: Contains Python-based implementations of various ROM patches and functional hacks.
- **`experimental/`**: A staging area for new features and advanced ROM modification techniques.
- **`fix.py`**: Automated routines for restructuring Zelda II ROMs to a more editor-friendly layout.

### Shared Logic
- **`assembler.py`**: A 6502 assembler/disassembler integrated into the editor for developing custom ROM code in Python.
- **`util.py`**: General-purpose helper functions available for use in Python-based romhacking scripts.
