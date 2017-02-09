# TODO List

### Issues found in alpha release
1. Fix sideview editor crash (fixed?)
2. Reminder dialogs for "Commit to Rom" (and flag to disable)
3. Feature to swap sideview areas around (because of the 28 limitation)
4. Button on sideview connection edtior to open up the destination area. (done)
5. Add "did you remember to save all of your files?" dialog when you quit
   with unsaved changes
6. Enemy level editor wanted
7. Move palaces between contitnents

### Add config which describes how to render towns
Done, but still need:
  * Inventory of all the different townspeople
  * Townspeople text editor
  * Building entrances to which rooms? (understood, no widget for editing yet).

### Overworld Editing
  * Fix raft spot.
  * Fix hidden places in East Hyrule (New Kasuto, P6)
  * Figure out if there is anything to do for boulders or river devil.

### Enemy editor to change enemy HP, XP etc.

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

### Create a target for Windows (build for Windows from Linux)
  1. One-time setup:
     ```
     ./tools/downloader.py --nowin32 compiler SDL2
     ```

  2. Build:
     ```
     bazel build --crosstool_top=//tools/windows:toolchain --cpu=win64 :main
     ```

  3. Package:
     ```
     ./tools/windows/zip4win.py --mxe tools/mxe --out z2edit.zip \
         bazel-bin/main zelda2.textpb content/*
     ```


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

### Figure out how overworld map size is determined
  * Done: hardcoded constant in ROM.
  * Need documentation on how to change.

### Overworld enemy encounter editor
  * Which sideview areas to load (done)

### Sideview editing
  * Add/Delete enemies from a sideview area (done)
  * Sideview modifications should allocate new memory regions (done)
  * Large/small encounter enemy lists (done)
  * Initial item availability editor (doors & items) (done)
