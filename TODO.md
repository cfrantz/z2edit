# TODO List

### Create a target for Windows
  * Try to use https://github.com/cfrantz/hello_windows so I can
    build from Linux.

### Add config which describes how to render towns
Done, but still need:
  * Inventory of all the different townspeople
  * Townspeople text editor
  * Building entrances to which rooms? (understood, no widget for editing yet).

### Sideview editing
  * Better modification of enemy lists (e.g. add/del enemies from an area).
  * Sideview modifications should allocate new memory regions (done).

### Overworld enemy encounter editor
  * Which sideview areas to load
  * Large/small encounter enemy lists

### Enemy editor to change enemy HP, XP etc.

### Figure out how overworld map size is determined
  * Possible to not need to be copied to RAM?

### Curate list of known hacks to optionally apply to a ROM:
  * walk anywhere (done)
  * text speed
  * pick-up-item delay

### Sprite and tiles editor
  * List of sprite equivalents between banks

# Documentation:
  * Sideview editing flow
  * Enemy editing
  * Known restrictions:
    * Overworld map must be < 1K
    * Enemy lists must be < 1K
  * Auto-generate documentation from `content/obj_*.textpb`?
  * Auto-generate documentation from `content/{enemies,items}.textpb`?
  * Document used/unused sidview areas in vanilla game.
    * Create a vanilla ROM which fixes all sideview exit oddities.

# Done
### Better visualization of side-view areas
Done: Multimap editor
  * Figure out drop rooms in GP (done-ish)
    * Elevators and side-exits use separate subroutines.
    * Dropping out the bottom uses the side-exit routine.
    * Suspect Link Y-pos overflow triggers up exit.
  * Figure out room reuse in Palaces 1,2,5 (fixed, config error).
  * How to render when multmap puts two rooms in the same place? (use
    force-directed-graph to visualize).

### Decompress Configs for all side-scroll areas
  * Overworld caves/outside areas (done)
  * Palaces (done)
  * Towns (done)
  * Great Palace (done)

### Startup conditions editor
  * starting levels (done)
  * starting lives (done)
  * starting itesm (done)
  * starting spells (done)
  * starting sword techs (done)

### HiDPI solution.
  * Done: --hidpi <scale> command line flag.
