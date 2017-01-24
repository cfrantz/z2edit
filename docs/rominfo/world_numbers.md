# Understanding World Numbers

In Zelda2, overworld map tranfer table can place link into sideview
areas which are in different "worlds" (e.g. palaces and towns).

| Destination | World | ROM Bank |
|:------------|------:|---------:|
| Western Hyrule Side-Scroll Areas | 0 | 1 |
| Death Mountain Side-Scroll Areas | 1 | 1 |
| Eastern Hyrule Side-Scroll Areas | 0 | 2 |
| Maze IslandSide-Scroll Areas     | 1 | 2 |
| Western Hyrule Towns             | 4 | 3 |
| Eastern Hyrule Towns             | 10 | 3 |
| Palace 1                         | 12 | 4 |
| Palace 2                         | 12 | 4 |
| Palace 5                         | 14 | 4 |
| Palace 3                         | 16 | 4 |
| Palace 4                         | 17 | 4 |
| Palace 6                         | 18 | 4 |
| Palace 7                         | 22 | 5 |

At first glance, this list of words seems haphazard, however there is structure
when the world number is examined as a bitfield.

### The meaning of the world number

Let `n` be the normalized world number and `o` be the overworld number (where
`0` is West Hyrule, `1` is Death Mountain/Maze Island, and `2` is East Hyrule).
The destination world in the tranfer table can be thought of as a byte
arranged as `nnnnnnoo`.


| Destination | Normalized World | Overworld |
|:------------|------:|---------:|
| Western Hyrule Side-Scroll Areas | 0 | 0 |
| Death Mountain Side-Scroll Areas | 0 | 1 |
| Eastern Hyrule Side-Scroll Areas | 0 | 2 |
| Maze IslandSide-Scroll Areas     | 0 | 1 `*` |
| Western Hyrule Towns             | 1 | 0 |
| Eastern Hyrule Towns             | 2 | 2 |
| Palace 1                         | 3 | 0 |
| Palace 2                         | 3 | 0 |
| Palace 5                         | 3 | 2 |
| Palace 3                         | 4 | 0 |
| Palace 4                         | 4 | 1 `*` |
| Palace 6                         | 4 | 2 |
| Palace 7                         | 5 | 2 |

`*` In the FDS version of the game, Death Mountain and Maze Island shared a
bank.  In the US version of the game, they are in separate banks, but they
are the exact same maps.  There is a special case in the code to handle
switching to Death Moutain and Maze Island.

### Converting World number to Bank Number

In order to convert from the world number to a bank number, the game
first converts the world number to the Overworld Number and
Normalized World Number and stores these values at `$706` and `$707`
respectively.  It then calls a subroutine at `$cd40` (in bank 7) to
determine the ROM bank number.

1. Using the normalized world number, look up the bank number in a
   lookup table located at `$cb47`.

   ```
   cd40: ac0707        LDY $0707
   cd43: b9b7c4        LDA $c4b7,Y  # cb47: 01 03 03 04 04 05 00 00 00 ...
   ```

1. If the normalized world number is not `1` (not one of West Hyrule, East
   Hyrule, DM or MZ), then jump to `$cd59`.

   ```
   cd46: c901          CMP #$01
   cd48: d00f          BNE $0f
   ```

1. Load the value of the overworld number.  If it is `0` (West Hyrule), then
   jump to `$cd59`.

   ```
   cd4a: ac0607        LDY $0706
   cd4d: f00a          BEQ $0a
   ```

1. Decrement the overworld number.  If it is not `0` (e.g. was not `1`, DM),
   then it must be overworld `2`.  Jump to `$cd57`.

   ```
   cd4f: 88            DEY
   cd50: d005          BNE $05
   ```

1. Check the previous overworld number.  If it was `0` (Western Hyrule),
   then we want bank `1`, which is the current value in the `A` register.

   ```
   cd52: ac0a07        LDY $070a  # previous overworld believed to be stored here.
   cd55: f002          BEQ $02
   ```

1. A jump to `$cd57` means we want bank `2`.  The next instruction stores
   whichever value is in the `A` register.

   ```
   cd57: a902          LDA #$02
   cd59: 8d6907        STA $0769
   ```

### TL;DR

The world number is a bitfield which combines the overworld number and
the normalized world number.  Apart from special handling for Death Moutain
and Maze island, the ROM bank to load is stored in a table in bank 7 at
`$cb47`.
