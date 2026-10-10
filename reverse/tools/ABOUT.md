# About `reverse/tools`

Tools supporting the reverse engineering project.  Run them from `reverse/`
with the repo's virtual environment active (`source ../.venv/bin/activate`).
Tools that drive the emulator need the Python extension built with
`maturin develop` (see the top-level README).

- `headless.py`: Helpers for running the emulator without a GUI (loading
  ROMs, FM2 movies and symbol tables).
- `cdl_movie.py`: Plays FM2 movies headlessly with the emulator's Code/Data
  Logger enabled and writes `build/<name>.cdl`, `.jumps.json`, `.refs.json`
  and `.progress.tsv`.  `--merge` combines logs from several runs.
- `bootstrap.py`: Generates the initial ca65 disassembly in `src/` from the
  ROM and a CDL.  Run once; it refuses to overwrite existing bank files.
- `symbols.py`: Converts the ld65 debug file into `build/zelda2.sym.json`
  for the emulator's `symbols` plugin and other tools.  Run by `make`.
- `trace.py`: Writes a per-instruction trace for a range of frames of a
  movie, annotated with symbol names.
- `status.py`: Updates the generated section of `STATUS.md`.  Run by
  `make status`.
