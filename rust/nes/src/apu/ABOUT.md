# About `rust/nes/apu`

The files in this subdirectory emulate the various components of the NES Audio Processing Unit.
- apu_dmc.rs: Emulates the delta modulation channel (DMC).
- apu_noise.rs: Emulates the noise channel.
- apu_pulse.rs: Emulates the pulse wave channels.
- apu_triangle.rs: Emulates the triangle wave.
- mod.rs:  Emulates the whole APU by binding the other channels together into a single struct.
