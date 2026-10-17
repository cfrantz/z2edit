# About `reverse/tools`

Tools supporting the reverse engineering project.  Run them from `reverse/`
with the repo's virtual environment active (`source ../.venv/bin/activate`).
Tools that drive the emulator need the Python extension built with
`maturin develop` (see the top-level README).

- `headless.py`: Helpers for running the emulator without a GUI (loading
  ROMs, FM2 movies and symbol tables), and `Checkpoint`, which saves the
  emulator's state and restores it with a movie still in sync (for
  re-running from a late frame without replaying the whole movie).
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
  movies never reach.  `--at VAR=VALUE,...` starts a new game at a chosen area
  (with `build/movies/file_select.fm2`, in about 12 seconds), and
  `--invincible` keeps Link's meters full.
- `routine_report.py`: For each routine with a header, its status, whether
  the movies ran it, and the variables it was seen to read and write;
  `--provisional` lists only provisional routines, for checking headers
  against the traces.
- `enemy_scenario.py`: Sets up an enemy experiment: reaches an area
  (a movie frame, or a new game `--at` an area in about 12 seconds),
  optionally walks Link right to a point (`--approach`), clears the
  area's enemies, places enemies of chosen types in chosen slots, adds
  solid floor (`--fill`), and holds Link still and invincible while
  logging each enemy's state every frame and hits on event addresses
  (`--projectiles` adds the enemy projectiles in flight).
  `--map` prints the area's solid tiles; `--dialog` prints each town
  conversation's text with `townFlags` and `techniques`.  The enemy pages under
  `docs/entities/enemies` give the commands they were verified with.
- `enemy_table.py`: Decodes each enemy group's definition tables (hit
  points, experience, damage, flags, and init/AI/draw routines by name).
- `area_objects.py`: Decodes every side view area's objects (and the
  background layers) in an area bank, naming the routine that draws each;
  `--by-routine` lists the areas that use each routine.
- `area_dump.py`: Loads areas in the emulator by `areaId` (`gameMode`
  $14) and prints the tile maps the game builds for them.
- `disasm.py`: Disassembles a run of `.byte` data into instructions (code
  the movies never ran, which the bootstrap left as data).
- `name_at.py`: Names the code or data at addresses, whatever label (or
  none, or a cheap local) is there now.
- `resymbolize.py`: Rewrites `.word` table entries with current names.
- `make_movie.py`: Writes an FM2 movie from power-on with scripted button
  presses (`K:BUTTONS[:LEN]`).  `make movies` writes the scripted runs
  (title screen left alone, file select menus) that were merged into
  `build/zelda2.cdl` with the two test movies.
- `mark_unexecuted.py`: Rewrites the "not executed in the test movies"
  markers of bank files from the CDL, and drops "(not observed)" from jump
  table entries the jump log has seen, after new runs are merged.
- `check_docs.py`: Reports symbol names in `docs/` that no longer exist in
  the source.  Run by `make verify`.
- `config_xref.py`: Writes `docs/data-formats/config-xref.md`, every ROM
  address in z2edit's `config/vanilla` with the disassembly symbol there.
