### Add config which describes how to render towns
Done, but still need:
  * Inventory of all the different townspeople
  * Townspeople text editor
  * Building entrances to which rooms?

### Add config which describes how to render GP.
Done

### Better modification of enemy lists (e.g. add/del enemies from an area).

### Sideview modifications should allocate new memory regions.

### Overworld enemy encounter editor
  * Which sideview areas to load
  * Large/small encounter enemy lists

### Enemy editor to change enemy HP, XP etc.

### Figure out how overworld map size is determined
  * Possible to not need to be copied to RAM?

### Better visualization of side-view areas
Done: Multimap editor
  * Figure out room reuse in Palaces 1,2,5
  * Figure out drop rooms in GP.
  * How to render when multmap puts two rooms in the same place?

### Curate list of known hacks to optionally apply to a ROM:
  * walk anywhere (done)
  * text speed
  * pick-up-item delay

### Startup conditions editor
  * starting lives
  * starting levels

### Sprite and tiles editor
  * List of sprite equivalents between banks

### HiDPI solution.
  * Done: --hidpi <scale> command line flag.

### Documentation:
  * Sideview editing flow
  * Enemy editing
  * Known restrictions:
    * Overworld map must be < 1K
    * Enemy lists must be < 1K
