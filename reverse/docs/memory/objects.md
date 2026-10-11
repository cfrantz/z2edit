# Object Slots

Side-view objects (Link, enemies and projectiles) are stored as parallel
arrays in zero page and RAM, indexed by *object slot*.  The constants are
`OBJ_SLOT_*` in `src/constants.inc`; the arrays are the `obj*` and `enemy*`
variables in `src/ram.inc`.

| Slot | Object | Iterated by |
|------|--------|-------------|
| 0 | Link | bank 0 player code, `sub_E079` |
| 1-6 | enemies | `sub_D5DA`, `sub_DE40`, `sub_DEB8`, `sub_DEC8` |
| 7-12 | enemy projectiles | `sub_D56D`, `sub_DED4` |
| 13-14 | Link's projectiles | `sub_0_97FA`, `sub_0_98A6` |
| 15 | unused | |
| 16 | sword tip probe, for breaking blocks | `sub_E1E6` |

Each array is only as long as the last slot that uses it:

| Array | Address | Slots |
|-------|---------|-------|
| `objState` | $0019 | 0-14 |
| `objY` | $0029 | 0-16 |
| `objXHi` | $003B | 0-16 |
| `objX` | $004D | 0-16 |
| `objFacing` | $005F | 0-12 |
| `objXVelocity` | $0070 | 0-14 |
| `objCollision` | $00A7 | 0-6 |
| `objXSubpixel` | $03D6 | 0-14 |
| `objYVelocityFrac` | $03E6 | 0-12 |
| `objYVelocity` | $057D | 0-12 |
| `enemyType`, `enemyActive`, `enemySramIndex`, `enemyHp` | $00A1, $00B6, $00BC, $00C2 | 1-6 |

## Indexing

Routines shared by all objects take the slot in `x` (e.g. `sub_EAE8`,
which reads the background tile at an object's position, and `sub_F27D`).
Enemy and projectile code instead indexes by its own number, 0-5, from the
group's first slot: enemy code reads `objY+1,x`, projectile code
`objY+7,x`.  The `enemy*` arrays hold only the six enemy slots and are
always indexed by enemy number.

The bank 5 title screens reuse `objY+15` to `objY+17` ($0038-$003A) for
their own purposes.

## Evidence

- `tools/ram_usage.py --arrays` shows the indexed base addresses and their
  extents.  The bases fall 1, 7 and 13 bytes after each array's start.
- `tools/callers.py` over the first 60000 frames of the no-glitch movie
  shows the slot ranges each routine is called with (the table above).
