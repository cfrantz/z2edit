# About `reverse/tools`

Tools supporting the reverse engineering project.  Run them from `reverse/`
with the repo's virtual environment active (`source ../.venv/bin/activate`).
Tools that drive the emulator need the Python extension built with
`maturin develop` (see the top-level README).

- `headless.py`: Helpers for running the emulator without a GUI (loading
  ROMs, FM2 movies and symbol tables).
- `cdl_movie.py`: Plays FM2 movies headlessly with the emulator's Code/Data
  Logger enabled and writes `build/<name>.cdl`, `.jumps.json`, `.refs.json`
  `.ram.json` (RAM accesses by instruction) and `.progress.tsv`.  `--merge`
  combines logs from several runs.
- `bootstrap.py`: Generates the initial ca65 disassembly in `src/` from the
  ROM and a CDL.  Run once; it refuses to overwrite existing bank files.
  Known limitation: an inline jump table followed by data that is read
  indirectly (e.g. a PPU string) was extended over that data; the cases
  found so far have been fixed by hand in `src/`.
- `symbols.py`: Converts the ld65 debug file into `build/zelda2.sym.json`
  for the emulator's `symbols` plugin and other tools.  Run by `make`.
- `trace.py`: Writes a per-instruction trace for a range of frames of a
  movie, annotated with symbol names.
- `status.py`: Updates the generated section of `STATUS.md`.  Run by
  `make status`.
- `rename.py`: Renames symbols across `src/` and `docs/` (whole words), keeping the
  address comment column aligned.  Refuses unknown or already-used names.
  `--array NAME BASE COUNT` names an array and turns the mechanical names
  of its elements into `NAME+k`.
- `localize.py`: Turns `loc_` labels referenced only within their own
  routine into cheap local labels (`@done`, `@loop`, or `@loc_XXXX`); `--relabel-loops`
  re-checks existing `@loop` labels.
- `table.py`: Turns a run of `.byte` data into a named `.byte` or `.word`
  table (`--word` writes known addresses by name), turning labels inside
  it into `NAME+k`.
- `header.py`: Adds or replaces routine header comments from a file of
  `=== Name` blocks; `--status confirmed NAME...` sets their status.
- `label.py`: Finds or creates a label at a bank/address, splitting `.byte`
  lines or using `name := * + k` as needed.
- `bank_usage.py`: Plays a movie and reports which PRG bank executes in
  each game state and world, with the main thread's idle loop counted
  separately.  Slow: about 35 minutes for the 100AK movie; shows progress
  on stderr (`--quiet` to suppress).
- `ram_usage.py`: Reports, for each RAM/SRAM address, the routines that
  read, write or use it as a pointer, directly or through an index, from
  the RAM access log (`build/zelda2.ram.json`, written by `cdl_movie.py`).
  `--arrays` lists indexed base addresses with their observed extents.
- `callers.py`: Plays a movie with breakpoints on routines and tallies the
  JSR sites that call them, with a register's value on entry.
- `watch.py`: Plays a movie and logs writes to RAM variables (by symbol
  or address), with the frame number and the writing routine.
- `snap.py`: Saves screenshots (and prints memory) at chosen frames of a
  movie, or where variables take given values (`--when gameMode=0x11`).
- `play.py`: Plays a movie until given variables hold given values, then
  takes over with scripted input (and memory pokes), logging changes to
  watched variables and taking screenshots; for exercising code the
  movies never reach.
- `routine_report.py`: For each routine with a header, its status, whether
  the movies ran it, and the variables it was seen to read and write;
  `--provisional` lists only provisional routines, for checking headers
  against the traces.
- `check_docs.py`: Reports symbol names in `docs/` that no longer exist in
  the source.  Run by `make verify`.
