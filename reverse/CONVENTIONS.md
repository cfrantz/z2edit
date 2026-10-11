# Disassembly Conventions

Status: **proposed**.  These conventions apply to everything under
`reverse/src`.  Names taken from prior disassemblies must be rewritten to
conform.

## Files

- `bankN.s`: one file per 16K PRG bank.  Banks 0-6 assemble at $8000;
  bank 7 assembles at $C000.
- `hardware.inc`: NES hardware and MMC1 registers.
- `ram.inc`: zero page, RAM ($0000-$07FF) and SRAM ($6000-$7FFF) variables.
- `constants.inc`: game-wide constants and enumerations.
- `interfaces.inc`: bank interfaces, i.e. routines and data that several
  switchable banks provide at a common address (see Bank Interfaces).
- `globals.inc`: `.global` declarations for every label referenced from
  another bank.  A link error for an undefined/unexported symbol means this
  file is out of date.

## Names

| Kind                     | Style              | Examples                                  |
|--------------------------|--------------------|-------------------------------------------|
| Routine                  | `Subsystem_VerbNoun` (PascalCase) | `Player_ApplyGravity`, `Hud_Update`, `Collision_TestBoxes` |
| Data table               | `PascalCase` noun  | `EnemyHitPoints`, `GameModeHandlers`      |
| RAM variable             | `camelCase`, subsystem prefix where useful | `gameMode`, `playerXVelocity`, `objHitPoints` |
| Constant / enum value    | `UPPER_SNAKE_CASE` | `GAME_MODE_OVERWORLD`, `OBJ_SLOT_COUNT`   |
| Hardware register        | NESdev-wiki names  | `PPUCTRL`, `OAMDMA`, `SND_CHN`, `MMC1_PRG`|
| Local label              | ca65 cheap local   | `@loop`, `@done`, `@skipDamage`           |

Rules:

- Spell words out; avoid abbreviations except well-established ones
  (`Hud`, `Ppu`, `Oam`, `Nmi`, `Rng`, `Xp`, `Obj`, `Ptr`, `Lo`, `Hi`).
- Per-object arrays are named for one element and indexed:
  `lda objXPosition,x`.
- 16-bit values split into separate bytes use `Lo`/`Hi` suffixes:
  `playerXPositionLo`, `playerXPositionHi`.  Pointer tables split into two
  byte tables likewise: `GameModeHandlersLo`, `GameModeHandlersHi`.
- Boolean variables and routines returning a flag read as predicates:
  `isPlayerCrouching`, `Collision_IsSolidTile`.
- Use game terminology from `docs/glossary.md` consistently (e.g. "area",
  "region", "world").
- Label names are unique across the whole project, not just within a bank.
  Names do not include a bank number, except as described in Bank
  Interfaces below; a routine's bank is recorded in its header.  The build
  fails on duplicate label names.

## Bank Interfaces

Some routines and data exist in several switchable banks at the same
address, so that bank 7 can switch banks and then call or read a fixed
address.  Such a common address is an *interface*: an API that every bank
of that kind must provide.

- Declare each interface once in `interfaces.inc`, in a scope named for the
  kind of bank that implements it (a `PascalCase` noun ending in `Bank`):

  ```
  .scope OverworldBank                ; implemented by banks 1 and 2
      DecompressMap   = $A123
      AreaPointers    = $8000
  .endscope
  ```

- Callers use the interface name: `jsr OverworldBank::DecompressMap`,
  `lda OverworldBank::AreaPointers,x`.
- Each implementation is named after the routine with a `_BankN` suffix and
  asserts that it is at the interface address:

  ```
  Overworld_DecompressMap_Bank1:
      .assert * = OverworldBank::DecompressMap, error, "interface address moved"
  ```

- The same `_BankN` suffix is used for any routine or table that exists in
  more than one bank, whether the copies are byte-identical or specialized,
  even when it is not part of an interface.
- Each implementation's header says how it differs from the others
  ("byte-identical to bank 2", "same as bank 2 except ...").
- Documentation refers to the interface name and lists the implementing
  banks.

## Unknowns

Anything not yet understood keeps a mechanical name that encodes its
address:

| Kind                         | Name           |
|------------------------------|----------------|
| Subroutine (JSR target)      | `sub_C123` (bank 7), `sub_3_A123` (bank 3) |
| Other code location          | `loc_C123`, `loc_3_A123` |
| Data                         | `dat_C123`, `dat_3_A123` |
| RAM / zero page              | `ram_0736`, `ram_0012`   |
| SRAM                         | `sram_6A00`              |

A name is replaced only when the item is understood.  A best-guess name is
allowed, but its header comment must say `Status: provisional`.

## Routine Headers

Every named routine has a header comment:

```
; ---------------------------------------------------------------------------
; Player_ApplyGravity
;
; Accelerate the player downward, clamping to terminal velocity.
;
; In:      x = object slot (always 0 for the player)
; Out:     playerYVelocity updated
; Clobbers: a
; Bank:    any (bank 7)
; Status:  confirmed
; ---------------------------------------------------------------------------
```

`In`, `Out`, `Clobbers` and `Bank` may be omitted when trivially empty.
`Status` is `confirmed` (behavior verified by reading and in the emulator)
or `provisional` (best guess).

## RAM Declarations

`ram.inc` declares variables in address order, grouped under section
comments.  Each named variable has one line:

```
; ---------------------------------------------------------------------------
; Object slots: arrays indexed by object slot (0 = Link, 1-6 = enemies, ...)
; ---------------------------------------------------------------------------
objY                    = $0029     ; [17] Y position in side view
objXPage                = $003B     ; [17] high byte of X position
gameMode                = $0736     ; current mode; see GAME_MODE_* (provisional)
```

- `[n]` gives the number of elements of an array; it is omitted for a
  single byte.  A 16-bit value names its low byte and says `(16-bit)`.
- `(provisional)` marks a best-guess name or meaning; unmarked entries are
  confirmed (by reading the code and observing the value in the emulator).
- Enumerated values get `UPPER_SNAKE` constants in `constants.inc`.
- An array has a single name; code refers to its elements as `objY+3`, not
  by separate names.  Mechanical `ram_XXXX` names inside a named array are
  removed.
- Temporaries reused by many routines (e.g. `$0000-$000F`) are named
  `temp0`, `temp1`, ... unless a routine-local alias in the source is
  clearer: `@count := temp2`.

## Comments

- Explain intent and game meaning, not the opcode:
  `cmp #$18 ; at max speed?` rather than `; compare A with $18`.
- Document non-obvious tricks: `bit` used to skip an instruction, `rts`
  dispatch, flags carried across `jsr`, intentional fall-through into the
  next routine.
- Note known bugs and quirks with `; BUG:` or `; QUIRK:` so they can be
  collected for `docs/quirks.md`.
- Each instruction line may carry a trailing address comment (`; $A123`)
  emitted by the bootstrap; keep it, as it links the source to emulator
  addresses.

## Formatting

- Lowercase mnemonics, indented 4 spaces; operands start at column 12.
- Labels in column 0 on their own line.
- Hex constants use `$`, binary uses `%` where the value is a bit mask.
- Data: `.byte` / `.word` with at most 16 values per line, split at labels
  and at logical record boundaries once the structure is known.

## Byte-Exactness Idioms

The build must reproduce the original ROM exactly.  Be aware of:

- **Absolute addressing of zero-page operands.**  ca65 uses zero-page
  addressing when it can.  Where the original used absolute addressing for
  an address below $100, force it: `lda a:ram_0012,y`.
- **Labels inside instructions.**  When code jumps into or reads from the
  middle of an instruction, define the label relative to the instruction:
  `loc_C123 := * + 1` on the line before it.
- **Unofficial opcodes and unknown bytes** are emitted with `.byte`.
- **Inline jump tables.**  The game dispatches by calling a routine that
  pulls the return address and indexes a table of `.word` addresses placed
  directly after the `jsr`.  The `jsr` does not return; the table follows
  it as `.word` entries.  The bootstrap marks entries it could not confirm
  with `(not observed)`; verify these by reading the code that computes the
  index.
- **Do not let ca65 optimize.**  No `.smart`, no macros that pick different
  encodings based on operand values.
