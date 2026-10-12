# Graphics

## CHR Banks

The cartridge has 128K of CHR ROM, used in 8K mode: `Mmc1_SetChrBank`
writes a 4K bank number whose low bit is ignored (see [MMC1](../hardware/mmc1.md)),
so the values below are even.

| Value | Used for | Set by |
|-------|----------|--------|
| $00 | title and file screens | `Title_Init` |
| $02 | falling into an area; the death screen | `Fall_FillNametable`, `Death_ClearScreen` |
| $06 | continue screen | `Death_DrawContinueScreen` |
| $0E | the ending | `GameState_StartEnding` (`areaChrBank`) |
| $10 | overworld; Link dying | `Mode_OverworldSetup`, `Death_Animate` |
| `areaChrBank` | side view areas | `Mode_StartSideView`, `Lives_Draw` |

For side view areas `Mode_LoadWorld` sets `areaChrBank` to twice the
area's PRG bank (West Hyrule $02, East Hyrule $04, towns $06), and in
palaces to twice the palace's entry in `PalaceChrBanks` (`palaceChrBank`;
palaces 1-6 $08, $0A, $12, $14, $16, $18, the Great Palace $0C; see
[Palaces](palaces.md)).

## Palettes

Palettes reach the PPU as PPU strings (`Ppu_WriteStrings`), queued in a
RAM buffer or selected from a table (`PpuStringPtrs`, by
`ppuStringSelect`).

- **Side view areas**: `AreaBank::Palettes` is copied to
  `sramAreaPalettes` when the world loads (in palaces, from
  `PalaceOutdoorPalettes` and `PalaceIndoorPalettes` instead).  Header
  byte 3 of an area (`areaPaletteHeader`) selects its background palette
  (bits 5-3) and sprite palette (bits 7-6), queued by
  `DrawArea_QueuePalettes`; the backdrop color is kept in `backdropColor`.
- **Link** (sprite palette 0) comes from four SRAM tables
  (`sramLinkPalette0`-`sramLinkPalette3`, filled from `NewGameSramData`
  at a new game): entry 4 normally, entry 5 in a dark cave without the
  candle, entries 0-3 in turn while the screen flashes for a spell
  (`spellFlash`, `Link_UpdatePalette`).  The shield spell turns his
  tunic red ($16 for $2A, `Spell_ShieldEffect`).
- **Overworld**: `OverworldPalette`; each map square's palette comes from
  its terrain (`OverworldTerrainPalettes`).
- Flashes: the crystal (`CrystalSpot_FlashPalette`), Dark Link's room
  (`DarkLink_FlashPalette`), the dying boss (`BossDeathPalette`), the
  death screen (`Death_QueueRedPalette`).

## Metatiles

The side view tile map holds metatile numbers.  Bits 7-6 of a number are
its palette and select one of four metatile tables
(`AreaBank::MetatilePtrs`, `Area_GetMetatilePtr`); the low six bits pick
a metatile of four CHR tiles in it.  The palette bits are also what the
attribute bytes are built from (`Menu_ComputeAttrs_Bank0`), and what
decides solidity (`AreaBank::IsSolidTile`; see [Areas](areas.md)).  The
overworld uses its own 2x2 metatiles (`OverworldTileMetatiles`).

## Sprites

The game draws into `oamBuffer`, which the NMI copies to the PPU by OAM
DMA.  Sprites are 8x16 (PPUCTRL $B0 from power on: 8x16 sprites,
background tiles at $1000).  The sprite slots are fixed by role:

| oamBuffer offset | Sprites |
|------------------|---------|
| $00 | sprite 0, for the split under the status bar |
| $08 | Link |
| $20, $28, $30, $E0, $E8, $F0 | enemy projectiles (`ProjectileOamOffsets`) |
| $38, $40 | Link's projectiles (`LinkProjectileOamOffsets`) |
| $50-$C8, $18 apart | the six enemies (`EnemyOamOffsets`) |

Each object's slot is in `objOamOffset`.  The enemies' and their
projectiles' slots are handed out in a rotating order (`oamRotation`,
`Enemies_Update`), so that when too many sprites share a scanline a
different one disappears each frame.

Most objects are drawn as pairs of 8x16 sprites side by side
(`Sprite_DrawEnemyPair`, `Sprite_DrawPairFromTable`): the tiles come from
`sramEnemyTiles` (the enemy group's `EnemyTiles_Bank1` ...) or a ROM
table, mirrored for the facing, with the type's palette
(`sramEnemyAttr` bits 7-6).  Projectiles use one sprite, with their
type's tile and attributes and a small draw routine for flicker, flips
or spinning (`sramProjectileTiles`, `sramProjectileAttrs`,
`sramProjectileDrawPtrs`).
