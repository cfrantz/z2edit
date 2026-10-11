# Save Files

Zelda II keeps three save files in battery-backed SRAM.  The save code is
in bank 5 (`Save_*` routines); the symbols are in `src/ram.inc` and the
sizes and status values in `src/constants.inc`.

## What Is Saved

A save file is a copy of two blocks of RAM:

| Block | RAM | Size | Contents |
|-------|-----|------|----------|
| stats | `statLevel`-`playerName` ($0777-$07A8) | `SAVE_STATS_SIZE` = 50 | levels, spells, containers, items, keys, crystals remaining, name, ... |
| world | `saveWorld` ($0600-$06DF) | `SAVE_WORLD_SIZE` = 224 | per-area flags (see below) |

Everything else (current area, position, life and magic meters) is not
saved.  The stats block starts with these fields:

| Offset | Variable | Default |
|--------|----------|---------|
| 0-2 | `statLevel` (attack, magic, life) | 1, 1, 1 |
| 4-11 | `spellOwned` | 0 |
| 12-13 | `statContainers` (magic, heart) | 4, 4 |
| 14-21 | `itemOwned` | 0 |
| 28 | `keyCount` | 0 |
| 29 | `crystalsRemaining` | 6 |
| 42-49 | `playerName` | $F4 (blank) x 8 |

The defaults are `SaveDefaultStats` and `SaveDefaultWorld` in bank 5.

The world block holds 4 flags for each area, two areas per byte (even
areas in the high nibble), in seven blocks of 32 bytes (64 areas):

| Offset | Areas of |
|--------|----------|
| $00 | West Hyrule |
| $20 | Death Mountain and Maze Island (region 1) |
| $40 | East Hyrule |
| $60 | towns |
| $80 | palaces (world 3) |
| $A0 | palaces (world 4) |
| $C0 | Great Palace |

A set flag means the thing it stands for (an item, a key, a locked door)
is still there; `Area_ClearFlag` clears it when it is taken, and
`Area_TestFlag` tests it.  A new game starts with all flags set.

## SRAM Layout

| Address | Contents |
|---------|----------|
| $6002-$6097 | backup stats for files 0-2 (`sramBackupStats`, 3 x 50) |
| $6098-$6337 | backup world state for files 0-2 (`sramBackupWorld`, 3 x 224) |
| $7400 | file 0 status (`sramSaveStatus0`) |
| $7402-$7497 | stats for files 0-2 (`sramSaveStats`, 3 x 50) |
| $7498 | file 1 status (`sramSaveStatus1`) |
| $749A-$7739 | world state for files 0-2 (`sramSaveWorld`, 3 x 224) |
| $773A | file 2 status (`sramSaveStatus2`) |

The backup copies share SRAM with the current area's tile map
($6000-$633F), so saving overwrites the tile map.  Saving is reached from
bank 7 (`jsr Save_Write` at $CF26); how the tile map is restored afterwards
has not been traced yet.

## Crash-Safe Saving

Each file has a status byte, so that a save interrupted by a reset or power
loss can be recovered:

| Status | Constant | Meaning |
|--------|----------|---------|
| $A5 | `SAVE_STATUS_VALID` | the file is valid |
| $5A | `SAVE_STATUS_WRITING_BACKUP` | the backup copy is being written; the file is intact |
| $69 | `SAVE_STATUS_WRITING_FILE` | the backup copy is complete; the file is being written from it |

`Save_Write` saves RAM to file `saveSlot`:

1. Set the status to `$5A`, and copy RAM to the backup.
2. Set the status to `$69`, and copy the backup to the file.
3. Set the status to `$A5`.

At power-on, `Save_CheckFiles` examines each file's status:

- `$A5`: nothing to do.
- `$5A`: the file was never touched; mark it valid.
- `$69`: copy the backup to the file again, and mark it valid.
- anything else (e.g. new battery RAM): fill the file with the defaults.

The file screens use the backup copies as a scratch area:
`Save_CopyToBackup` loads a file for display or editing, and
`Save_RestoreFromBackup` writes it back with the same two-phase status.

## Copy Loops

The copy loops do not count bytes.  `Save_SetPointers` sets `temp6` to
`SaveDefaultWorld`, and every loop advances it with the other pointers,
stopping when it reaches `SaveDefaultWorldEnd`.  The world block is
therefore exactly as long as `SaveDefaultWorld` (224 bytes).
