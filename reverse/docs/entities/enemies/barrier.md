# Barrier

The flashing wall across the Great Palace's entrance.  It hurts Link
while it stands, and dissolves once all six crystals are placed.

## Where It Appears

| Group | Type | Enemy | HP | Exp | Damage class | Drop group |
|-------|------|-------|---:|----:|-------------:|-----------:|
| [Great Palace](../great-palace.md) | $20 | Barrier | 0 | 3 | 6 | - |

It is placed in the entrance (room $00) at X $110, Y $30, the left edge
of the wall the area draws on page 1.  It has no spawn or drawing routine:
the wall is background tiles, and the enemy only animates and guards it.
Link's projectiles bounce off it, the sword passes through it, and
spells don't touch it.

## Behavior

`Barrier_Ai`, in state 0 (`enemyAiState`), while it is on screen:

- **Flashing.**  Every 4 frames it writes the next of the four
  `BarrierPaletteStrings` to background palette 2.  It alternates two
  color sets with a darker one between them, a 16-frame cycle.
- **Guarding.**  While Link is alive, each frame it clears his
  invulnerability timer, so contact hurts him every time it is tested
  (every other frame), not once per 80 frames.  The contact damage is
  class 6.  A fairy Link who touches it dies at once
  (`linkDeathPending`).

**Opening.**  When no crystals remain (`crystalsRemaining` 0), it is on
screen and Link reaches X $C0 on page 0, it:

- freezes the game (`gameFrozen`), with sound $40;
- puts Link in `objState` 3;
- goes to state 3, and writes the first palette string once more.

**Dissolving** (state 3).  The barrier keeps running while the game is
frozen.  It counts frames in its `objXVelocity`, and borrows enemy slot
3's `objX`, `objXHi` and `objState` for a name table address and a map
offset (nothing else is in slot 3 in that room).

- Every 4 frames it blanks a row of 12 tiles (tile $F4), starting at
  name table address $26A2 and moving up a row each time.
- Every 8 frames it also clears 7 blocks of the area map to $40
  (`sramAreaTiles`, page 1, columns 1-7), rows 8 up to 2, so the opening
  stays open when the screen scrolls back.
- After 14 rows, the last at $2502, it unfreezes the game (56 frames
  in all).  It stays in state 3, so it no longer flashes or hurts; each
  frame after, it shifts `gameFrozen` right again, harmless as nothing
  else freezes the game in that room.

## Routines

| Routine | Address | Role |
|---------|---------|------|
| `Barrier_Ai` | 5:$A234 | everything above |

## Verification

In the Great Palace's entrance (see [Bubble](bubble.md#great-palace) for
`$GP` and `$GF`).  The barrier spawns in slot 5.

- **Guarding.**  Link moved onto the step at ($E8, $80) and walked
  right.  At $FB it hurt him (event at `Link_HurtByEnemy`, 7:$E2EF)
  and knocked him back.  It hurt him again 2 frames later, though he had
  just been hurt.

  ```
  tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $GP --until currentWorld=5 --after 200 \
      $GF --approach B0 --link E8:80 --log 5 --press 0:RIGHT:60 --event Hurt=7:E2EF --frames 60
  ```

- **Fairy.**  The same with `--link E8:60 --poke linkFairy=8`: the
  first contact, at X $FB, set `linkDeathPending` (event at 5:$A279).

- **Opening.**  With `crystalsRemaining` 0 and Link moved to ($C8, $80):
  - It went to state 3 on the first frame.
  - Its frame count reached $38 (56 frames).
  - The screenshots show the wall vanishing from the bottom up, gone
    by frame 80.

  ```
  tools/enemy_scenario.py --movie build/movies/file_select.fm2 --at $GP,crystalsRemaining=0 \
      --until currentWorld=5 --after 200 $GF --approach B0 --link C8:80 --log 5 --frames 140 \
      --shot 1 --shot 40 --shot 80
  ```
