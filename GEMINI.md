# Gemini Context: z2edit

`z2edit` is a comprehensive ROM editor and development environment for the NES game "Zelda II: The Adventure of Link". It is built as a modular, hybrid application using Rust, Python, and C++.

## Core Architecture: Hybrid Rust/Python

The project is designed with a high-performance Rust core (for ROM manipulation, emulation, and heavy GUI rendering) and a flexible Python shell (for application logic, extensibility, and scripting). 

### Key Frameworks
- **`rust/nes`**: A general-purpose NES emulation core designed for broad compatibility.
- **`rust/python_gui`**: A standalone, high-performance framework for building hybrid Rust/Python applications using the `Dear ImGui` toolkit.
- **`rust/z2edit`**: The game-specific implementation that consumes the other crates to provide the ROM editor's logic.

## Communication & Clarification
- If any instruction or architectural direction is ambiguous, you **must** ask the user for clarification before proceeding with the implementation. Never make assumptions that lead to implementing logic with uncertainty (e.g., leaving questions in code comments).
- **Commit Approval:** Do not perform git commits unless specifically approved by the user or if the user temporarily countermands this instruction.
- **Commit Format:** When committing, you **must** use the `--signoff` (or `-s`) flag. Additionally, override the commit author to include `+gemini` in the email address (e.g., `git commit --author="Chris Frantz <frantzcj+gemini@gmail.com>" -s ...`).

## Directory Navigation & `ABOUT.md` Context

This project uses `ABOUT.md` files in nearly every subdirectory to provide localized architectural and technical context. **Before investigating a specific subdirectory, always check for an `ABOUT.md` file.**

### Root Directory Overview
- `config/`: JSON5-based game schemas and emulator plugin configurations. (See `config/ABOUT.md`).
- `python/`: The Python shell, extension modules, and the emulator's plugin system. (See `python/z2edit/ABOUT.md`).
- `rust/`: The high-performance core logic, divided into several crates. (See `rust/ABOUT.md`).
- `.venv/`: The local Python virtual environment. It contains the configuration and installed modules listed in `python-requirements.txt`. This directory is **not under source control** because virtual environments are system-dependent (e.g., based on the local Python version).
- `z2edit`: A symlink to the Python entry point for the application.
- `emulator`: A symlink to the standalone emulator application.
- `Cargo.toml`/`Cargo.lock`: Configuration for the Rust build system.
- `pyproject.toml`/`python-requirements.txt`: Configuration for the Python environment and its dependencies.

## Extension & Extensibility
The NES emulator includes a Python-based plugin system (located in `python/z2edit/emulator/`) for tasks like specialized debugging, MIDI synthesis, and movie playback. (See `python/z2edit/emulator/ABOUT.md`).

## Developer Resources
- `private/`: Workspace for in-progress files and internal documentation (ignored by source control).
- `util/`: Build-related utilities (may be out-of-date).
- `README.md`: High-level project information for users.
