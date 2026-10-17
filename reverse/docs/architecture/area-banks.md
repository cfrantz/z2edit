# The Area Banks' Shared Code

Banks 1-5 are the area banks.  Bank 7 maps one of them for the current
world (`WorldPrgBanks`, `areaPrgBank`):

| Bank | Areas | Groups (`AreaPtrs`, `AreaPtrs2`) |
|------|-------|----------------------------------|
| 1 | West Hyrule (and its overworld) | 1: West Hyrule; 2: Death Mountain |
| 2 | East Hyrule (and its overworld) | 1: East Hyrule; 2: Maze Island |
| 3 | the towns | 1 only |
| 4 | palaces 1-6 | 1: palaces 1, 2, 5; 2: palaces 3, 4, 6 |
| 5 | the Great Palace | 1 only |

Each bank carries its own copy of the code its areas need: the area
object routines, the enemy group's spawn, AI and drawing routines and
tables, and the projectile routines.  Bank 4 has two enemy groups, one
for each palace group, so most of its enemy and projectile code exists
twice.  The second copies have the suffix _Bank4B; the first group's
often have none (`SkullHead_Ai`, `SkullHead_Ai_Bank4B`).

This page lists what the banks share, which copies are identical, and
where the copies differ.  `tools/bank_compare.py` makes the comparison
(see [Checking](#checking)).

## The Interface

Bank 7 reaches the area bank through fixed addresses, the same in every
area bank (`src/interfaces.inc`; each bank asserts them):

- **`AreaBank` (banks 1-5):** `BackgroundAreas`, `Palettes`,
  `MetatilePtrs`, `OverworldMapPtrs`, `IsSolidTile`, `SpecialTiles`,
  `AreaPtrs`, `EnemyListPtrs`, `Connections`, `Doors`, `EnemyLists`,
  `AreaPtrs2`, `EnemyListPtrs2`, `Connections2`, `DrawLargeObject` and
  `DrawSmallObject`.
- **`OverworldBank` (banks 1, 2):** the overworld's map strips, terrain,
  encounters, palace stones, floor tiles, the flute and the location
  tables.
- **`PalaceBank` (banks 4, 5):** `PaletteRows`.

The enemy group's tables, from `ProjectilePtrs_BankN` at $9400 to
`EnemyTiles_BankN` at $95D4 (`EnemyHp_BankN`, `EnemyInitPtrs_BankN`,
`EnemyAiPtrs_BankN`, `EnemyDrawPtrs_BankN`, ...), sit at the same
addresses in every bank.  Bank 4's second group has its own at the same
offsets from $A900.  Bank 7 copies the current group's into SRAM
(`sramEnemyHp`, ...) when the world loads.  It also copies the enemy
lists, at $88A0.

Every bank fills the whole interface, even where its areas don't use a
part of it:

- **`Doors`** ($8817) matters only in the towns (`TownDoors`).  Banks 1, 2
  and 4 hold rows of $FF there (`Doors_Bank1`, `Doors_Bank2`,
  `Doors_Bank4`).  In bank 5 the address falls inside area data.
- **`AreaPtrs2` and the second group's tables** exist only in banks 1, 2
  and 4.  Banks 3 and 5 have one group, and other code sits at those
  addresses.
- **`DrawLargeObject` and `DrawSmallObject`** exist only in banks 1, 2, 4
  and 5.  For the towns, bank 7's world tables call bank 3's
  `Town_DrawLargeObject` and `Town_DrawSmallObject` instead.
- **The `OverworldBank` addresses** are area data in banks 4 and 5
  (`Locations` at $861F and `Locations2` at $A0FC are each inside an
  area's data).

## Identical in Every Bank

Five routines are the same in all five banks:

| Routine | |
|---------|-|
| `Area_IsSolidTile_BankN` | solid if the tile is at least its group's threshold (the thresholds differ) |
| `Area_PlaceElevator_BankN` | the elevator object |
| `Area_PlaceLockedDoor_BankN` | the locked door object |
| `Generator_Init_BankN` | the enemy generators' spawn routine |
| `Enemy_InitNone_BankN` | the empty spawn routine (`rts`) |

The two object dispatchers, `Area_DrawLargeObject_BankN` and
`Area_DrawSmallObject_BankN` (banks 1, 2, 4, 5), have the same code.  Only
their inline jump tables differ, since each bank has its own objects.

## Banks 1 and 2: the Overworld Banks

Banks 1 and 2 share all their area drawing:
- **Object routines:** identical copies (`Area_DrawCactus_BankN`,
  `Area_DrawCloud_BankN`, `Area_DrawLava_BankN`, `Area_DrawPit_BankN`,
  `Area_DrawStonehenge_BankN`, `Area_DrawSleepingZelda_BankN`,
  `Area_FillObjectRow_BankN`, ...).
- **Tables:** the floor records and the object tile tables.
- **Most of the overworld code:** `Overworld_GetTerrain_BankN`,
  `Overworld_GetRowPtr_BankN`, `Overworld_QueueMapStrip_BankN` and
  `Overworld_SetFloorTiles_BankN`.
- **Also:** `Deeler_Init_BankN`, and the projectile routines
  `Projectile_Bubble_BankN`, `Projectile_DrawBob_BankN` and
  `Projectile_DrawFlip_BankN`.

Where they differ:

| Routine | Bank 1 | Bank 2 |
|---------|--------|--------|
| `Overworld_UpdatePalaceStones_BankN` | palaces 1-3 (entries 2-0) | East Hyrule: palaces 5 and 6 (entries 1-0); Maze Island: palace 4 (entry 3) |
| `Overworld_UseFlute_BankN` | the same code as bank 2's.  It runs whenever the flute plays in West Hyrule or on Death Mountain, and returns at once (its first test is `currentRegion` = 2) | the river devil and palace 6 (see `Overworld_CheckSpecialTerrain`) |
| `RedJar_Init_BankN` | kind $11 | kind $11, and 3 pixels right |
| `Projectile_Draw_BankN` | also leaves Y, X, attributes and facing in temp0-temp3 | as banks 4 and 5 |
| `Projectile_Fireball_BankN` | sound by `Projectile_PlaySound_Bank1`, then straight | the same sound set inline, then straight |
| `Projectile_Straight_BankN` | Y velocity -2/16, then falls into the gravity routine's +2/16: level (as bank 5) | Y velocity 0, then `Projectile_MoveAndCheck_Bank2`: level |
| `Projectile_Rock_BankN` | `Projectile_FallIfDeflected_Bank1`, then straight | the deflection test inline: deflected, it falls (`Projectile_RockGravity_Bank2`) |
| `Projectile_MoveAndCheck_BankN` | off screen: removed; else drawn and tested | the same, through `Projectile_DrawAndCheck_Bank2` |

Bank 1's `Fighter_KnockBack` (Moblins, Goriyas, Dairas) and bank 2's
`Lizalfos_KnockBack` are the same routine except at its end without
knockback.  The first jumps to `Enemy_MoveXUnlessWall_Bank1`; the second
falls into its walk.

## Banks 4 and 5: the Palace Banks

The palace object routines are shared, most of them identical
(`Palace_DrawCloud_BankN`, `Palace_DrawObjectPillar_BankN`,
`Palace_FillObjectRow_BankN`, `Palace_GetObjectTiles_BankN`,
`Palace_SetFloorTiles_BankN`, ...).  The fill routines are also the same
as banks 1 and 2's (`Area_FillObjectRow_BankN`,
`Area_FillObjectColumn_BankN`).  Bank 5's `Palace_DrawElevatorShaft` is
bank 1's `Area_DrawPit_Bank1` under another name.  Where they differ:

| Routine | Bank 4 | Bank 5 |
|---------|--------|--------|
| `Palace_DrawLava_BankN` | lava from row $B | from row $A, one higher |
| `Palace_DrawRatHead_BankN`, `Palace_DrawUnicornHead_BankN` | decorations 1 and 2 | 2 and 1: the heads swapped (`PalaceDecorationTiles_BankN`) |
| `Palace_DrawObjectTwoRows_BankN` | gets the tiles and fills a row | through `Palace_DrawObjectRow_Bank5` (the same) |
| `Boss_DrawHpBar` (bank 4), `Boss_DrawHpBar_Bank5` | the same code; the Iron Knuckle's tables are read out of its bytes (`IronknuckleGuardMasks` and the next two) | the same |
| `Boss_SpawnDeathSpark_BankN` | the spark's X low byte only | the high byte too |
| `HiddenJar_Ai` (bank 4), `HiddenJar_Ai_Bank5` | always a red jar (palaces 1, 2, 5) | half the time a red Fokka (bank 4's second group has the Iron Knuckle variant, `HiddenJarOrIronknuckle_Ai`, 90% alike) |
| `Projectile_DrawFlame_BankN` | flickers faster as its timer runs out, alternating its flip every 4 frames | drawn as a flash, nothing at speed 0, flickers below timer $20, tile $52 |
| `Projectile_DeathSpark_BankN` | removed by `Projectile_Remove_Bank4` | by `Projectile_Kill_Bank5` |

Bank 5's Great Palace enemies include copies of bank 4's under other
names.  `GpBubble_Draw` is `Bubble_Draw`.  `GpSkullHead_Ai`,
`GpSkullHead_Init` and `GpSkullHead_Draw` are 80% alike to bank 4's
second-group Skull Head.

## Bank 4's Two Groups

The second palace group's copies (suffix _Bank4B) differ from the first's:

| Routine | First group (palaces 1, 2, 5) | Second group (palaces 3, 4, 6) |
|---------|-------------------------------|--------------------------------|
| `Boss_UpdateIntro`, `Boss_UpdateIntro_Bank4B` | when the boss dies, every other enemy takes a 224-point hit | sets `TIMER_BOSS` to 255 when the fight starts; the other enemies are left alone |
| `SkullHead_Init`, `SkullHead_Init_Bank4B` | spawns 32 above Link | 36 above |
| `Projectile_Fireball_BankN` | sound, then `Projectile_Straight_Bank4` (a spark below Y $C5) | sound; on the background it becomes a spark |
| `Projectile_DrawFlash_BankN` | keeps the vertical flip ($83) | the same, through `Projectile_SetAttr_Bank4B` |
| `Projectile_DrawFlicker_BankN` | ORs the palette into the attributes | flips it by its X direction |
| `Projectile_DrawSpin_BankN` | as banks 1, 2 | the spin table `SpinProjectileAttrs_Bank4B` |
| `Projectile_MoveAndCheck_BankN` | off a side, types 6 and 8 go on unseen | type 8 only |
| `Projectile_RockGravity_BankN` | falls through `Projectile_Fall_Bank4` | gravity 2/16 |

## Projectiles Across All Banks

Every bank has its own projectile code.  The shared pieces:

- **Identical:** `Projectile_DrawAndCheck_BankN` (banks 2, 4, 5),
  `Projectile_DrawSpin_BankN` (banks 1, 2, 4) and
  `Projectile_SetAttr_BankN` (banks 1, 2, 5).
- **Per bank:**
  - `Projectile_Straight_BankN`: level everywhere, but in bank 4 it turns
    into a spark below Y $C5.
  - `Projectile_DrawFlash_BankN`: keeps the horizontal flip ($43) in banks
    1 and 2, the vertical flip ($83) in bank 4, and both ($C3) in bank 5.
  - `Projectile_MoveAndCheck_BankN`: only the palace banks let some types
    fly on off a side of the screen (bank 5: type 7).
  - `Projectile_Kill_BankN`: in banks 2 and 4 it stores the caller's state
    (0 removes, bank 4's $F2 makes a spark); bank 5's always removes.

## Bank 3: the Towns

Bank 3 shares only the interface, the five routines above, and the enemy
tables' layout.  Its object drawing is its own (`Town_DrawWall`, ...),
except `Town_DrawCastleBricks`, a near copy of the fill row routine (86%
alike).  Its people's code has no copies elsewhere.

## Checking

```
tools/bank_compare.py                     # every shared name: which banks' copies match
tools/bank_compare.py --diff Projectile_Rock
tools/bank_compare.py --similar --min 5   # copies under other names
```

The comparison works on the disassembly's text, so it follows the routine
boundaries the labels give:
- **Dropped:** comments, bank suffixes, local label names, and data that
  trails a routine.
- **Treated as the same:** branches to any nearby `rts`.
- **Checked by eye:** a table label inside a routine splits it (bank 4's
  `Boss_DrawHpBar`).
