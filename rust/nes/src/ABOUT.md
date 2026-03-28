# About `rust/nes/src`

The files in this subdirectory implement useful utilities for dealing with NES ROM images (so-called `.nes` files) as well as a reasonably accurate NES emulator.  The utility structs/functions and the emulatore have bindings to python via the `pyo3` crate.

The following files exist in this subdirectory:
- address.rs: Rust representation of NES address of different types (Cartridge PRG and CHR types, CPU address and .nes file offsets).
- controller.rs: Emulates the NES controllers.
- error.rs: Errors local to this crate.
- freespace.rs: Utilities for managing free-space areas within a NES ROM.
- hwpalette.rs: The traditional NES hardware color palette expressed as RGB constants.
- lib.rs: The `nes` crate root.
- nesfile.rs: Structs for dealing with .nes files.
- peripheral.rs: A peripheral abstraction for the NES emulator.
- ppu.rs: Emulates the NES Picture Processing Unit (aka PPU).
- ram.rs: Emulates the RAM in the NES.
- stall.rs: Helps the emulator manage CPU stall conditions because of DMA.
- system.rs: The NES emulator.

The following additional subdirectories exist:
- apu: Emulates the NES Audio Processing Unit (aka APU).
- cpu: Emulates the NES 6502 CPU.
- gui: Builds the emulator debug GUIs.
- mapper: Contains emulations of several NES memory mapper chips.
