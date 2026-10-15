# Quirks

Bugs and odd behaviors found while reading the code.  The source marks
each with `QUIRK:` or `BUG:` in the routine's header or at the
instruction.  The first section lists the ones that change what the
player sees, which a faithful reimplementation must reproduce; the second
lists code oddities with no visible effect.

## Visible in Play

| Where | What happens |
|-------|--------------|
| `Spell_Update` | A spell costing one more than the magic left is still cast, and leaves the magic at 0. |
| `Enemy_FaceLink` | The comparison of Link's and the enemy's X does not set up the carry, so whether an enemy exactly level with Link turns toward him depends on the carry on entry (an off-by-one). |
| `Myu_Move` | Myus, Bots and Bits test for a floor only when their Y is $60 or more, so one standing on a floor higher up the screen would fall through it. |
| `Bit_Ai` | The Bit's fast periods depend on its enemy slot, which is ORed into the frame counter (an add would have staggered them evenly): Bits in enemy slots 0-1 are fast a quarter of the time, those in slots 2-5 half of it. |
| `Lowder_Ai` | A Lowder treats the first frame without a floor like a ledge, so one that spawns in mid-air turns around once as it starts to fall. |
| `Area_IsSolidTile_Bank2` | Tiles of palette group 0 ($00-$3F) look up a byte 255 bytes past the threshold table; in bank 2 (East Hyrule's areas) that byte is $00, so every group 0 tile is solid there.  In banks 1, 3, 4 and 5 the byte is $D2 or more, so none are. |
| `Dialog_Type1D` | Townspeople of types $1D-$20 are meant to set a flag in `townFlags` when talked to, but the index misses `TownFlagBits` and lands on zeros, so nothing is set. |
| `Dialog_TechniqueMan` | The type $0D handler gives the upward thrust in any town but Rauru, Saria and Mido, including Ruto; it only matters where a type $0D person stands (the knight of Darunia). |

## Without Visible Effect

| Where | What happens |
|-------|--------------|
| `Bank5_Nmi` | Unlike the game NMI it does not save `a`; harmless, since it returns with a >= $80, which `Main_Loop` never takes for a load request. |
| `Nmi` | A branch on `ram_0727` whose two outcomes are the next instruction. |
| `Sword_CheckBlocksInBank` | Checks of the thrust poses (objState 8, 9) whose branches both go to the next instruction. |
| `Raft_Wait` | Meant to act every 16th frame, but both branches return, so it does nothing. |
| `Barba_DrawAndCheck` | A test of `frameCounter` that branches to the next instruction either way. |
| `Area_Load` | Loads of `temp0` that are overwritten unused. |
| `Ppu_WriteAttrColumn` | Loads `a` with $FF but stores `y`. |
| `Daira_DrawWeapon` (BUG) | Writes to ROM at $95E6, which on the MMC1 is a write to its control register; probably meant for RAM $05E6.  Harmless: the value has bit 7 set, which only resets the MMC1 shift register and re-selects PRG mode 3, already in use. |
| `Palace_GetObjectTiles_Bank4`, `Palace_GetObjectTiles_Bank5` | If bit 7 of an area header's second byte were set, the object tiles would be read from code; no palace area sets it. |
| `Area_DrawLargeObject_Bank1` | Large object $10 has a routine address of $0000, so an area using it would crash (bank 2 draws a slope there). |
| Bank 5 enemy types $1B, $1C | Their AI pointers point into data (`FokkeruSpeeds`); the types are not used. |
| `RomHeader` | The PRG ROM checksum at $FFF0 does not match the PRG ROM (the CHR checksum does); presumably stale. |
