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
| `Deeler_Ai` | The ends of a Deeler's thread are single variables set by the last Deeler spawned, so one spawned at another height drops to the other's bottom and climbs back to the other's top, where it then hangs. |
| `BagoBagoGenerator_Ai` | Every Bago Bago gets 3 hit points, whatever its group's table says (8 in East Hyrule and the palaces). |
| `Fighter_Move` | Moblins, Goriyas and Dairas judge their distance from Link by the low byte of the X difference only, so one 128 or more pixels away can take him for near: a Goriya 210 pixels from Link stands and throws boomerangs as if he were 46 pixels away. |
| `Ironknuckle_Ai` | Half the time an Iron Knuckle picks its attack height by Link's stance with the Moblins' rule, but its poses are the other way round, so it strikes high at a standing Link and low at a crouching one: straight at his shield, where a Moblin aims away from it. |
| `Stalfos_Ai` | Stalfos walk faster to the left ($F2, 0.875 pixel per frame) than to the right ($0A, 0.625): `StalfosSpeeds` isn't symmetric. |
| `Stalfos_Ai` | While Link does the upward thrust, each Stalfos sets `swordY` to $F8, taking his sword out of the hit tests of every enemy handled after it in that frame. |
| `Wizrobe_Ai` | After its first appearance a Wizzrobe of palaces 3, 4 and 6 always reappears on page 1 (`Wizrobe_Restart` sets `objXHi` to 1), so one placed on another page of its room leaves that page for good. |
| `Area_IsSolidTile_Bank2` | Tiles of palette group 0 ($00-$3F) look up a byte 255 bytes past the threshold table; in bank 2 (East Hyrule's areas) that byte is $00, so every group 0 tile is solid there.  In banks 1, 3, 4 and 5 the byte is $D2 or more, so none are. |
| `Dialog_TechniqueMan` | The type $0D handler gives the upward thrust in any town but Rauru, Saria and Mido, including Ruto; it only matters where a type $0D person stands (the knight of Darunia). |

## Without Visible Effect

| Where | What happens |
|-------|--------------|
| `Boss_UpdateIntro` | When a boss of palaces 1, 2 or 5 dies, the other enemies are hit at attack level $0A, past the end of `SwordDamage`; the byte read there ($E0, after `ThunderDamage`) kills them all, as intended.  The code that would have spared off-screen and SPELL-immune enemies never runs (`drawScratch` is always $0A). |
| `KingBot_Ai` | A King Bot's burst writes Bots into enemy slots 5-1 but never slot 0, so one in slot 0 would burst again every frame, pinning its Bots in place, and could no longer be hit.  It can't happen in the game: the King Bots are alone in their rooms and spawn into slot 5. |
| `Thunderbird_Ai` | Its intro and death paths drop a return address with `pla`/`pla`, copied from `Boss_UpdateIntro`, where it skips the boss's AI; inline in the AI routine it skips the rest of `Enemies_Update` instead, so the enemy slots after the Thunderbird's and every projectile slot go un-updated and undrawn in those frames.  Its room has no other enemies. |
| `Dialog_SetWalkerFlag` | Talking to a walker of type $1D-$20 sets bit 7, 6, 5 or 4 of the town's `townFlags`.  Bits 6-4 are also the quest bits of Mido (the water of life), Darunia (the child) and Ruto (the trophy), so a type $1E walker in Mido would stand in for the water of life (tested: the sick girl's mother then leads Link to her door).  No town's streets hold a walker whose bit that town checks. |
| `Dialog_Error` | Outside Rauru, Ruto and Mido, a type $0E person's second text needs the cross, but those towns' two texts are the same. |
| `Thunderbird_Ai` | A test of `frameCounter` that branches to the next instruction either way. |
| `Bank5_Nmi` | Unlike the game NMI it does not save `a`; harmless, since it returns with a >= $80, which `Main_Loop` never takes for a load request. |
| `Nmi` | A branch on `nmiFlag` whose two outcomes are the next instruction. |
| `Sword_CheckBlocksInBank` | Checks of the thrust poses (objState 8, 9) whose branches both go to the next instruction. |
| `Lizalfos_ChooseGuard` | A quarter of the time it sets the attack count to $C0, as `Fighter_Move` sets $E0 for a quicker attack; but $C0 is a multiple of either Lizalfos period, so the next attack comes on time anyway. |
| `Raft_Wait` | Meant to act every 16th frame, but both branches return, so it does nothing. |
| `Barba_DrawAndCheck` | A test of `frameCounter` that branches to the next instruction either way. |
| `Area_Load` | Loads of `temp0` that are overwritten unused. |
| `Ppu_WriteAttrColumn` | Loads `a` with $FF but stores `y`. |
| `Daira_DrawWeapon` (BUG) | Writes to ROM at $95E6, which on the MMC1 is a write to its control register; probably meant for RAM $05E6.  Harmless: the value has bit 7 set, which only resets the MMC1 shift register and re-selects PRG mode 3, already in use. |
| `Palace_GetObjectTiles_Bank4`, `Palace_GetObjectTiles_Bank5` | If bit 7 of an area header's second byte were set, the object tiles would be read from code; no palace area sets it. |
| `Area_DrawLargeObject_Bank1` | Large object $1x (off row $F) has a routine address of $0000, so an area using it would crash; no area does (bank 2 draws a wall of pillars there, `Area_DrawPillarWall_Bank2`). |
| Bank 5 enemy types $1B, $1C | Their AI pointers point into data (`FokkeruSpeeds`); the types are not used. |
| `RomHeader` | The PRG ROM checksum at $FFF0 does not match the PRG ROM (the CHR checksum does); presumably stale. |
