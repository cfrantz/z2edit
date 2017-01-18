# Understanding Overworld Encounters

During gameplay, when Link walks on the overworld, random spawns
try to chase him down for side-scroll enemy encounters.  The type
of side scroll area Link enters is based on the type of tile he
was standing on at the time of the encounter.

In addition to the type of tile, Overworld maps have a Northern and Southern
region (you have no doubt noticed the Grass encounter outside of North Palce
is just Bots vs. Bots and Megmets south of Jump Cave).

### Tile types

Recall that there are 16 types of overworld tiles:

| 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | A | B | C | D | E | F
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---
|Town|Cave|Palace|Bridge|Desert|Grass|Frest|Swamp|Graveyard|Road|Lava|Mountain|Water|Water(walk)|Rock|Spider

Of these, you can only (legally) take an encounter on `4` - `A`.

## Side-view selection routine

When Link gets an encounter on the overworld, this is the relevant bit of
code which selects the sideview encounter.

* First, using the current *world* (`$706`) and Links overworld *Y-position*
(`$73`), we determine whether we are in the Northern or Southern side of the
world, with a 3-byte table at `$CB32` (in bank 7) which has the Y coordinate
which determines the north/south dividing line.

```
cc0f: ac0607        LDY $0706
cc12: a573          LDA $73
cc14: d932cb        CMP $cb32,Y
```

* Using the result of the compare, we clear the `A` register, rotate the
carry flag into `A` and temporarily store the result at address `$00`.

```
cc17: a900          LDA #$00
cc19: 2a            ROL A
cc1a: 8500          STA $00
```

* We load the current tile-type where Link is standing into `A`, subtract `4`,
multiply by `2` (shift left) and add the prevously computed north/south result.
We then transfer the result to the `Y` register.


```
cc1c: ad6305        LDA $0563
cc1f: 38            SEC
cc20: e904          SBC #$04
cc22: 0a            ASL A
cc23: 6500          ADC $00
cc25: a8            TAY
```

* Now we use the `Y` register to load the sideview area ID from a table at
`$8409`, and store the value in offset `$3e` of the RAM-copy of the
encounter table (`$6a7e`).

```
cc26: b90984        LDA $8409,Y
cc29: a03e          LDY #$3e
cc2b: 997e6a        STA $6a7e,Y
```

## TL;DR

There is a 14-byte table in the overworld PRG banks at `$8409`, which determines
the sideview area for encounters:

| Offset | Meaning |
|--------|---------|
| 0 | Desert (North) |
| 1 | Desert (South) |
| 2 | Grass (North) |
| 3 | Grass (South) |
| 4 | Forest (North) |
| 5 | Forest (South) |
| 6 | Swamp (North) |
| 7 | Swamp (South) |
| 8 | Graveyard (North) |
| 9 | Graveyard (South) |
| A | Road (North) |
| B | Road (South) |
| C | Lava (North) |
| D | Lava (South) |

## Trivia

If you have hacked your ROM with the *walk-anywhere* hack, you can get an
encounter on Mountain, Water, the Boulders or the Spider.  This is because
the game will happily index past the end of the 14-byte table.

Immediately after the overworld encounter table, at offset `$8417` is the
object table for side-scroll areas.  In the case of taking an encounter on
a Mountain tile, the game is mis-interpeting data from another table as an
encounter destination ID.


