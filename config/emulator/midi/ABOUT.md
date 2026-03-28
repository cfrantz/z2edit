# About `config/emulator/midi`

This directory contains instrument and channel configurations for the NES emulator's **MIDI Synthesis Plugin** (located in `python/z2edit/emulator/midi/`).

### Usage
These JSON files allow the emulator to function as a MIDI-controlled synthesizer. They map MIDI channels and program changes to specific NES APU configurations (Pulse, Triangle, Noise, and DMC). By modifying these files, users can customize how the NES hardware responds to MIDI input, enabling experimental sound design and music composition using the emulator's core.

The configurations specify parameters like duty cycles, envelope settings, and hardware sweep registers for the emulation of various instruments.
