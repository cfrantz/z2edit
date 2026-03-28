# About `rust/nes/src`

This crate implements a general-purpose Nintendo Entertainment System (NES) emulation core designed for broad compatibility with a wide range of games. It serves as the high-performance engine for a hybrid Rust/Python emulator application.

### Key Features
- **General Compatibility**: Implements core NES hardware and numerous mappers to support a vast library of titles.
- **Hybrid Architecture**: Leverages `pyo3` to expose the emulation core to Python, enabling high-level logic and plugins to be written in a more flexible language.
- **Extensible Plugin System**: The core is designed to be extended via Python-based plugins (found in `python/z2edit/emulator`) for tasks like specialized debugging, MIDI synthesis, and input recording.

### Core Components
- `system.rs`: The central emulator coordinator.
- `cpu/`: Instruction-accurate 6502 CPU emulation.
- `ppu.rs`: Cycle-accurate Picture Processing Unit emulation.
- `apu/`: Audio Processing Unit emulation for all standard NES channels.
- `mapper/`: Support for various NES memory mappers (MMC1, MMC3, MMC5, VRC7, etc.).
- `nesfile.rs`: Utilities for parsing and managing `.nes` ROM images.
- `address.rs`: Unified representation of NES memory and file offsets.
- `gui/`: Debug and visualization interfaces built on the `python_gui` framework.

Other files provide essential hardware support, including `controller.rs`, `ram.rs`, `stall.rs` (DMA handling), and `freespace.rs` for ROM modification tasks.
