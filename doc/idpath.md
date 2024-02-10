# Id Path

The IdPath is a unique identifier of a resource in the Zelda2 ROM.  Resources
include overworld maps, sideview maps, palettes, dialogs, etc.

- `prg/$n/overworld/0`: West Hyrule, East Hyrule.
- `prg/$n/overworld/1`: Death Mountain, Maze Island.
- `prg/$n/sideview/bg/$x`: Sideview backgrounds (templates).
- `prg/$n/sideview/a/$x`: WestHy areas, EastHy areas, Palace125.
- `prg/$n/sideview/b/$x`: DM areas, MZ areas, Palace346.
- `prg/$n/text/a/$x`: Text for WestHy towns.
- `prg/$n/text/b/$x`: Text for EastHy towns.
- `prg/$n/palette/background/$x`: Sideview background palettes.
- `prg/$n/palette/outdoor/$x`: Outdoor palettes (for palaces).
- `prg/$n/palette/sprite/$x`: Sprite palettes.
- `prg/$n/metatile/group{0,1,2,3}/$x`: Metatiles for sideview areas.

- `chr/$n`: Chr graphics banks.

- `global/levels/attack`: Attack level-up experience table.
- `global/levels/magic`: Magic level-up experience table.
- `global/levels/life`: Life level-up experience table.
- `global/levels/spell`: Spell costs and names.
- `global/enemy/xp`: Enemy experience and xp-graphics.
- `global/palette/shield`: Link's tunic for shield spell.
- `global/palette/darklink`: Dark Link fight.
- `global/palette/post_darklink`: After Dark Link fight.
- `global/palette/title/$x`: After Dark Link fight.
- `global/palette/overworld/background`: Overworld background.
- `global/palette/overworld/sprite`: Overworld sprites.
- `global/metatile/overworld/$x`: Metatiles associated with the overworld.

## Configuration structs

- `prg[n].<resource>`: Description how to access per-bank resources.
  - `overworld`:
  - `sideview`:
  - `text`:
  - `palette`:
  - `metatile`:
- `prg[n].enemy[x]`: Enemy table for bank n.
- `global.<resource>`: Description of how to access global resources.
  - `levels.xxx`:
  - `enemy.xp``:
  - `palette.xxx`:
  - `metatile.overworld`:
- `global.item[x]`: Items table.
- `global.chr_scheme`: How to map Z2 CHR banks to physical CHR banks.
- `object[$x]`: Object table for sideview objects.
- `rom.layout`: NES ROM layout (prg/chr bank arrangement).
- `rom.freespace`: Freespace ranges in the ROM.


