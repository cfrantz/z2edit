# Reverse Engineering Zelda 2: Project Plan

## Goals

The goal of this project is to reverse engineer the Zelda 2 ROM and gain a
complete understanding of how the game works.  Success is judged by two
deliverables:

1. **Documentation.**  A well-structured hierarchy of documents describing the
   game's behaviors, data structures and code, detailed enough that a skilled
   engineer could write an accurate independent implementation of the game in
   a modern programming language on different hardware.  "Accurate" means
   exact: constants, sub-pixel velocities, RNG behavior, frame ordering and
   timing quirks are all part of the specification.

2. **Annotated disassembly.**  A human-comprehensible disassembly in `ca65`
   syntax that assembles and links (via `ld65`) back to a ROM that is
   **byte-identical** to the original.

### Non-goals (for now)

- A "shiftable" disassembly (one that still works after code is inserted or
  moved).  We will avoid gratuitously breaking shiftability (e.g. prefer
  symbolic operands over raw addresses), but we won't block on it.
- Fixing bugs in the original game.

## Ground Rules

- **The ROM is never committed.**  `private/zelda2.nes` is the reference
  image.  The build extracts CHR data (and the iNES header) from it at build
  time; the repo contains only the source and tooling.
- **Hand-maintained source.**  After an initial machine-generated bootstrap,
  the `.s` files are the source of truth for code labels and comments.
  Machine-readable symbol data (for the emulator, docs and tools) is
  *extracted from* the build (`ld65 --dbgfile` / label output), never edited
  separately.
- **Consistent naming.**  Prior disassemblies (Lemmy's, FiendsOfTheElements)
  and datacrystal may be *consulted* for names and comments, but every name
  we emit must follow `reverse/CONVENTIONS.md`.  Nothing is bulk-imported.
- **Name only what we understand.**  A routine keeps its auto-generated name
  (e.g. `sub_C123`) until we understand it.  A name that is a best guess
  carries a provisional marker in its header comment (see Conventions).
- **The round-trip check must always pass.**  Every change to the
  disassembly is verified by rebuilding and comparing against the original
  ROM.

## Repository Layout

```
reverse/
  ABOUT.md          Context and prior work.
  PLAN.md           This file.
  CONVENTIONS.md    Naming, commenting and formatting rules (Phase 0).
  STATUS.md         Coverage and progress tracking (generated + notes).
  Makefile          build / verify / symbols targets.
  src/
    zelda2.cfg      ld65 linker configuration (header, 8 PRG banks, CHR).
    header.s        iNES header.
    hardware.inc    PPU / APU / MMC1 / controller register definitions.
    ram.inc         Zero page, RAM ($0000-$07FF) and SRAM ($6000-$7FFF) map.
    constants.inc   Game-wide constants (enums, object IDs, game modes...).
    bank0.s ... bank7.s
  tools/            Bootstrap disassembler, verifier, symbol exporters.
  docs/             The documentation hierarchy (see Phase 5).
  build/            Build output (not under source control).
```

## Tooling Prerequisites

- `cc65` (provides `ca65` and `ld65`) must be installed and on `PATH`
  (Arch: `pacman -S cc65`).

## Phases

### Phase 0: Conventions and tooling

Goal: make the rest of the work fast and mechanical where it can be.

1. **Write `CONVENTIONS.md`.**  Proposed starting point:
   - Routines: `PascalCase`, prefixed by subsystem where useful
     (`Player_ApplyGravity`, `Hud_Update`, `Collision_TestAabb`).
   - RAM variables: `camelCase` with a short subsystem prefix
     (`playerXVel`, `gameMode`, `objHp,x`).  Arrays of per-object state are
     named for one element and indexed.
   - Constants and enums: `UPPER_SNAKE_CASE`.
   - Local labels: ca65 cheap locals (`@loop`, `@done`).
   - Data tables: `PascalCase` noun with a suffix describing shape
     (`EnemyHpTable`, `GameModeJumpTable`).
   - Unknowns: `sub_XXXX`, `loc_XXXX`, `dat_XXXX`, `ram_XXXX` (bank-qualified
     where needed, e.g. `sub_3_A123`).
   - Every named routine gets a header comment: purpose, inputs (registers,
     RAM), outputs, clobbers, bank requirements, and status
     (`; Status: confirmed` / `; Status: provisional`).
   - Comments explain *intent*, not opcodes ("clamp speed to max", not
     "compare A with #$18").
   - Byte-exactness idioms: forcing absolute addressing for zero-page
     operands (`a:`), and how to encode data embedded in code.

2. **Code/Data Logger (CDL) in the emulator.**  Record, for each PRG byte,
   whether it was executed as an opcode, read as an operand, or read as data,
   and which bank was mapped at the time.  Also record targets of indirect
   jumps (`JMP (ind)`, `RTS`-dispatch jump tables).  Export per-bank results.

3. **Collect coverage.**  Run both FM2 movies
   (`Zelda2_100AK_schmidtty.fm2`, `Zelda2_NoGlitch_2014_Final.fm2`) with CDL
   enabled and merge the results.  Supplement with manual play for paths the
   movies miss (deaths, game over, unused menus, etc.).

4. **Trace logger.**  Optional per-instruction trace (PC, bank, registers,
   symbol) for a window of frames, so a behavior can be studied offline.

5. **Bootstrap disassembler** (`tools/`).  Using the ROM plus CDL data,
   generate the initial `bank*.s` files: code as instructions, everything else
   as `.byte`, auto-labels for every branch/jump/data reference, and
   symbolic references to `hardware.inc` registers.  This runs **once**; its
   output is then hand-maintained.

6. **Build and verify** (`Makefile`).  `make` assembles and links;
   `make verify` compares the output with `private/zelda2.nes` and reports
   the first differing offsets on failure.

7. **Symbol export.**  Convert `ld65` debug output into (a) labels the
   emulator's Python plugin can load, so breakpoints, watchpoints and traces
   use our names, and (b) a JSON symbol table for documentation tooling.

8. **Coverage report.**  Script that fills in `STATUS.md`: per bank, bytes
   classified as code/data/unknown, and counts of named vs auto-named
   routines and RAM variables.

**Exit criteria:** `make verify` passes on the bootstrap output; the emulator
can display our symbols; `STATUS.md` reports a baseline.

### Phase 1: Skeleton and hardware layer

- Confirm (with CDL data) or correct the bank-content claims in "Initial
  high-level knowledge" below.
- Document the MMC1 usage: how and where banks are switched, the
  trampolines in bank 7 that call into switchable banks, and the
  serial-write protocol for MMC1 registers.
- Name the reset handler, NMI handler, IRQ handler, and the bank-switching
  helpers.
- Populate `hardware.inc` fully and ensure every hardware register access in
  the source is symbolic.

### Phase 2: Memory map

- Build `ram.inc` for zero page, RAM and SRAM, using datacrystal, the
  z2edit emulator plugin, and watchpoints during movie playback.
- Each variable gets a name, size/array length and a one-line description;
  confidence is tracked the same way as routines.
- Document the save-file layout in SRAM.

### Phase 3: Core engine (bank 7 and bank 0)

- Boot sequence and initialization.
- Main loop and the game-mode state machine (title, file select, overworld,
  sideview, town, palace, death/game-over, ending...).
- NMI: PPU update queue, scrolling, sprite DMA, controller reading, sound
  call.
- Shared services: RNG, math helpers, object/entity slot management,
  collision, physics, HUD.

### Phase 4: Subsystems

Each is a self-contained work area; order is flexible after Phase 3.

- Player: movement, physics, sword, shield, downstab/upstab, damage,
  knockback, experience and leveling, magic spells.
- Enemies and objects: shared AI helpers, then one routine family at a time
  (per-world enemy code in banks 1-5).
- Overworld (banks 1-2): map format, movement, random encounters, special
  tiles, raft/bridges/hidden areas.
- Sideview areas: room loading, scrolling, area transitions.
- Towns (bank 3): NPC behavior, dialog system, spell/skill teachers.
- Palaces and Great Palace (banks 4-5): palace mechanics, bosses,
  palace completion and crystal placement.
- Title, file select, name entry, ending (bank 5).
- Audio (bank 6): both music engines, sound effects, data formats.
- Graphics: CHR bank usage and switching, palettes, sprite/metatile formats.

### Phase 5: Documentation hierarchy

Written progressively alongside Phases 1-4, under `reverse/docs/`:

```
docs/
  README.md             Index and how to read these docs.
  glossary.md           Game and project terminology (worlds, areas, ...).
  hardware/             MMC1 usage, PPU usage, timing and frame budget.
  architecture/         Boot, main loop, game modes, NMI, bank layout.
  memory/               RAM map, SRAM / save format.
  systems/              Physics, collision, combat, RNG, leveling, magic,
                        items, death and continue.
  world/                Overworld, encounters, sideview areas, towns,
                        palaces.
  entities/             One page per enemy/object family.
  audio/                Music engines, sound effects, data formats.
  data-formats/         ROM data structures, cross-referenced with
                        config/vanilla/*.json5.
  quirks.md             Bugs and edge-case behaviors a faithful
                        reimplementation must reproduce.
```

Docs reference routines and variables by their disassembly names so the two
deliverables stay linked.  Where z2edit's `config/vanilla` schema disagrees
with what we learn, we record the discrepancy and fix the config separately.

## Unit of Work

Most work in Phases 1-4 is a repeatable loop over one routine or one small
subsystem:

1. Read the code; consult prior disassemblies and datacrystal for hints.
2. Observe it in the emulator (breakpoints, watchpoints, traces) to confirm
   behavior.
3. Rename routine, locals, RAM and data per `CONVENTIONS.md`; write the
   header comment and intent comments.
4. Update or add the relevant `docs/` page.
5. `make verify` passes.
6. Update `STATUS.md`; commit (with approval).

Units are sized so a single session (human or AI agent) can complete one
without needing to hold the whole game in mind.

## Validation

- **Byte-identical build** (always).
- **Behavioral replay:** both FM2 movies play to completion on the rebuilt
  ROM (redundant if byte-identical, but a useful sanity check of the tooling).
- **Reference traces for reimplementers:** per-frame RAM snapshots from
  movie playback, published as test vectors so an independent
  implementation can be checked against the original.

## Initial High-Level Knowledge (to be verified in Phase 1)

- The game is for the 8-bit Nintendo Entertainment System (NES), written in
  6502 assembly in the mid-1980s.  The code is not structured in a modern
  way; it's not terrible, but not great either.
- The ROM is 16-byte iNES header + 128 KiB PRG (8 x 16 KiB banks) + 128 KiB
  CHR.
- The game uses the Nintendo MMC1 mapper with a fixed bank at $C000-$FFFF
  and a switchable bank at $8000-$BFFF, plus 8 KiB of battery-backed RAM at
  $6000-$7FFF.
  - Bank 7 is always mapped at $C000-$FFFF.  It contains common code: the
    main idle loop, the NMI handler, display management, movement, game
    physics and common enemy routines.
  - Bank 0 contains per-frame common code and is typically mapped for a
    portion of each frame.
  - Bank 1 contains the West Hyrule and Death Mountain overworlds and the
    maps of their sideview battle areas.
  - Bank 2 contains the East Hyrule and Maze Island overworlds and their
    sideview battle areas.
  - Bank 3 contains towns, town NPC routines and NPC dialog (worlds 1 & 2 in
    game terminology).
  - Bank 4 contains the palaces and palace enemy routines (worlds 3 & 4).
  - Bank 5 contains the Great Palace and its enemy routines (world 5), plus
    the title screen, save-file selection and new-game initialization (e.g.
    the initial item-presence bitmaps).
  - Bank 6 contains music and sound effects, called once per frame.  It has
    two music players: one for the title screen and one for gameplay.
