# About `reverse`

This directory represents a sub-project to reverse engineer the Zelda2
ROM and document its features, data structures and code.

## Prior Work

People have spent considerable effort already to reverse engineer Zelda 2.

- In `../private/lemmy.nes` is Lemmy's diassembly.  This code represents
  the cumulative effort of several people including "Trax" and "Lemmy".
  This disassembly is a reasonable attempt, but does contain several
  errors, mis-spellings or comments that should be considered provisional
  or are true but not instructive (e.g. there are comments that simply
  document what a given 6502 opcode does rather than what it means to
  the code flow of the game).

  Example errors:
    - `bank7_idem__maybe` Seems like a mis-spelling of "item".
       I believe this routine is actually an axis-aligned boudning-box
       collision detection routine.
    - `Hub_Update_Routine` is a mis-spelling of "HUD Update Routine".
      I believe this routine updates the in-game heads-up display (HUD) or
      top-of-screen status bar.

- The [FriendsOfTheElements Z2 Disassembly](https://github.com/FiendsOfTheElements/z2disassembly)
  is an updated disassembly.  As far as I can tell, it is based on Lemmy's
  disassembly with some enhancements.  It suffers from some of the same
  issues as Lemmy's.

- The [datacrystal](https://datacrystal.tcrf.net/wiki/Zelda_II:_The_Adventure_of_Link)
  website documents several ROM data structures and RAM locations used
  by the game.  This information is fairly accurate, but there are some
  mistakes and repetitions.

- This git repo contains `z2edit` which is a Zelda2 editor.  It contains
  source code and data definitions to edit data structures in the game.
  The JSON5 files in `config/vanilla` represent the current best machine
  readable information about the structure of the Zelda2 ROM.  There are
  rust serde data structures in z2edit that represent the schema of
  these JSON5 files.

## Tools and Resources in this repo

- The file `zelda2.nes` is a dump of the ROM in iNES format.

- The `z2edit` editor uses the configuration schema to understand the
  locations of data structures in the game and can unpack (and re-pack)
  those data structures into convenient rust data structures to enable
  editing the data.

- The NES emulator in this repo can emulate the game, play back
  tool-assisted speed runs (via fm2 movie files) and has python emulator
  hooks that allow the user to set hooks and watches on the game execution.

- There are two fm2 movie files in the `private` subdir:
  - `Zelda2_100AK_schmidtty.fm2`: A run of the game in the "100% All Keys"
     speedrun category.
  - `Zelda2_NoGlitch_2014_Final.fm2`: A run of the game that is believed
    to complete as quickly as possible without exploiting any glitches or
    defects of the game.  are two runs through the game
