# About `rust/z2edit/src/zelda2`

This subdirectory contains the data model and ROM pack/unpack routines for the z2edit editor.

- banks.rs: Models the game PRG banks in the ROM.
- chr.rs: Handles the CHR graphics banks in the ROM.
- config.rs: Constructs the entire configuration data structure from the other items in this subdir.
- connectivity.rs: Models the sideview connectivity relationships in the game.
- drops.rs: Handles the 6-count drop table and the palace dripper enemies and what items they drop.
- edit.rs: Models a single edit in the editor and provides a pyo3 binding.
- encounters.rs: Handles the overworld encounters table.
- enemies.rs: Handles the per-bank enemy properties.
- experience.rs: Handles the experience tables for Life, Magic and Attack.
- items.rs: Handles the collectable item properties.
- metatile.rs: Handles the metatile tables that construct 16x16 background objects from 8x8 tiles.
- misc_hacks.rs: Handles the miscellaneous hacks provided by the editor.
- mod.rs: The root of the module.
- object.rs: Represents background objects used by the sideview editor.
- overworld.rs: Handles overworld maps and the overworld connections table.
- palette.rs: Handles palette tables in the ROM.
- project.rs: Models a romhack project by representing the complete collection of edits made by the user.
- rom.rs: Models the initial starting ROM for a romhack project.
- sideview.rs: Handles the sideview areas in the game.  Includes sideview maps, enemy and item placement, townspeople text selection and sideview connectivity and doorway connectivity.
- start.rs: Handles the initial start values of the game.
- text_encoding.rs: Translates text between ASCII encoding and the proprietary Zelda2 encoding.
- text_table.rs: Models the text table for text content spoken by NPCs in the game.
- vchr.rs: Models the Virtual CHR table, which is a proprietary CHR expansion mechanism that may be employed by romhacks.
