# About `config/vanilla`

This directory contains the core configuration for the original, unmodified version of "Zelda II: The Adventure of Link" (the "vanilla" version). 

These JSON5 files serve as the **game's schema definition**, specifying the precise ROM offsets, bank locations, and data lengths for every editable component of the game. The Rust-side `serde` structures in `rust/z2edit/src/zelda2` use these files to interpret the raw bytes of the ROM into the high-level objects used by the editor.

Any romhacking project starts by loading this vanilla configuration to understand the baseline state of the target ROM.
