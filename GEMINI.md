# Gemini context for z2edit

This projct is `z2edit`, an editor and romhacking tool for the 8-bit Nintendo Entertainment System game "Zelda II: The Adventure of Link" (aka "Zelda 2" or "Zelda2").

This project is a multi-lingual project using Rust, Python and C++.  There are `ABOUT.md` files in many subdirectories of this codebase that contain important context about the code in those subdirectories.

The files ere in the root of the project are:
- Cargo.lock: lock file for the Cargo build system.
- Cargo.toml: configuration file for the Cargo build system.
- config: subdirectory with z2edit configuration data.
- emulator: A symlink to z2edit's built-in NES emulator.
- private: A subdirectory of work-in-progress files and other non-project resources useful to the author during development.
- pyproject.toml: Python project configuration.
- python: A subdirectory with python source code.
- python-requirements.txt: Python PyPI dependencies.
- README.md: The github user-visible README file.
- rust: A subdirectory with rust source code.
- rust-toolchain.toml: Rust toolchain configuration.
- target: Cargo buildsystem output.
- util: Utilities that may be of use during build (currently out of date).
- z2edit: A symlink to the python startup script for the application.
