# Apparently Unused RAM

RAM and battery-backed SRAM that the game doesn't appear to use, with a
confidence for each range.  "Unused" here means the game never reads a
value there.  A free range may still be written by the RAM clearing
loops at power-on, at a new game or on loading a world.  Those writes
don't matter to anything else, but a hack using the space has to expect
them.

## How It Was Found

`tools/ram_unused.py` combines two kinds of evidence:

- **The test movies.** `cdl_movie.py` logs every RAM and SRAM access the
  CPU makes in the 100AK and no-glitch movies and the scripted movies
  (`build/zelda2.ram.json`, all merged).  Each access is a *use*, a
  *clear* (the RAM clearing loops) or a *copy* (block copies: tables
  copied into SRAM, the save routines).  An address only ever cleared or
  copied is a candidate.
- **The source.** An address that an instruction names directly, even in
  code the movies never ran, is not a candidate.  Nor is an untouched
  element of a declared array, or an entry of a table copied into SRAM
  whose other entries are used (an enemy type the movies never met).

Things the log can't see were checked separately:
- **Stack pushes:** measured in the emulator (below).
- **OAM DMA's reads of the $0200 page:** that page counts as used.
- **Area flags in `saveWorld`:** worked out from the area data.

**Confidence:**
- **High:** only clears or copies ever touch it, and no instruction names
  it.  Any indexed array ending just below it is small and indexed within
  its size (`statMeterPending`, `columnIndex`, `jarFillAmounts`,
  `playerName`, ...), so no index can reach it.
- **Medium:** the same, but an indexed array ends just below it, and an
  index the movies never used could reach it.
- **Write-only** ranges are listed separately: code writes them, but no
  code ever reads the value.  Using them means removing those writes.

## Internal RAM ($0000-$07FF)

| Range | Bytes | Between | Touched by | Confidence |
|-------|------:|---------|------------|------------|
| $00F0-$00F4 | 5 | `sfxRequest` (4 bytes) and `joypadPressed` | the power-on clear of $E0-$FF (`Game_PowerOnInit`) | High |
| $00F9-$00FB | 3 | `joypadHeld` (2 bytes) and `ppuScrollY` | the same | High |
| $0101-$0109 | 9 | `nmiMode` and `areaBuildColumn` | nothing (see the stack page) | High |
| $0110-$0111 | 2 | `floorRowCount` and `objectTile` | nothing | High |
| $0113-$01C6 | 180 | `objectTile` and the stack, with 32 bytes to spare below its deepest push | nothing | High |
| $01C7-$01E6 | 32 | the margin above: the stack's deepest push in the movies was $01E7 | nothing | Medium |
| $03E5 | 1 | the end of `objXSubpixel` (15 bytes; this would be object 15) and `objYFrac` | `Ram_ClearWork`, `SideView_Start` | Medium |
| $03F3-$03FF | 13 | the end of `objYFrac` (13 bytes) and the page end | the same | Medium |
| $0523 | 1 | the end of `rng` (9 bytes) and `pauseState` | `Ram_ClearExceptSaved` | Medium |
| $058A-$058C | 3 | the end of `objYVelocity` (13 bytes; indices 13-15 never used in the movies) and `townspersonDoorPtr` | the same | Medium |
| $05EA-$05FF | 22 | `overworldEntrySfx` and `saveWorld` | the same | High |
| $06E0-$06FF | 32 | the end of `saveWorld` (224 bytes, not saved beyond) and `lives` | `Ram_ClearAll` | High |
| $0702-$0703 | 2 | `areaEntrySide` and `linkFallingIn` | `Ram_ClearAll` | High |
| $0711-$0716 | 6 | `reflectSpellActive` and `areaBuildPage` | the same | High |
| $0718-$071A | 3 | `areaBuildPage` and `columnLastPage` | the same | High |
| $0723-$0724 | 2 | `drawAreaColumnsLeft` and `ppuStringSelect` | the same | High |
| $073A | 1 | `gameStepPrev` and `fileStep` | the same | High |
| $0745 | 1 | `joypadHeldPrev` and `playfieldNametable` | the same | High |
| $074D | 1 | `dialogType` and `levelUpCursor` | the same | High |
| $075E | 1 | `unused075D` and `areaMusic` | the same | High |
| $076A | 1 | `areaPrgBank` and `unused076B` | the same | High |
| $07B1-$07D9 | 41 | `saveBlinkTimer` and `laughSample` (the sound engine's RAM starts at $07DA) | `Ram_ClearAll`; $07C0 up also the continue screen's clear of the sound RAM | High |

That is 312 bytes at high confidence and 50 at medium.

### Write-Only

Code writes these bytes but never reads them, so their values change
nothing.  The ones named unused followed by their address were found and named earlier; the
others have names from what writes them:

| Address | Name | Written by | Confidence |
|---------|------|------------|------------|
| $0016 | `swordBaseX+1` | `Link_DrawBody`, for Dark Link; never read in the movies, Dark Link's fight included, though `Sword_UpdatePosition` reads `swordBaseX,x` | Medium |
| $0708 | `unused0708` | `Game_StartAtNorthCastle` ($FF) | High |
| $070B | `palaceChrBank` | `Mode_LoadWorld`; `areaChrBank` is computed from the same value without reading it back | High |
| $071C | `unused071C` | `Area_Load` (0) | High |
| $0727 | `nmiFlag` | side view frames, `Title_Show`, `SideView_Start`, `Game_RunState`; read only by a branch in the NMI whose two outcomes are the same | High |
| $073C | `unused073C` | `Game_CheckStateChange` (0) | High |
| $073F | `unused073F` | `Game_StartAtNorthCastle` (0) | High |
| $075D | `unused075D` | `Game_StartAtNorthCastle` (1) | High |
| $0760 | `unused0760` | `Game_ResetLives` (counts it up) and `Lives_Blank` (0) | High |
| $0765 | `unused0765` | `Credits_Run` (0 every frame) | High |
| $076B | `unused076B` | `Game_PowerOnInit` ($0F, with SND_CHN) | High |

### Unused Area Flags in `saveWorld`

`saveWorld` ($0600-$06DF) holds 4 flags per area, two areas per byte, in
seven blocks of 32 bytes (by `AreaFlagsPtrs`).  An area's flags are used
only if something tests or clears them:
- a placed item or a locked door;
- a red jar, or a hidden jar's red jar;
- a boss (flag 1) or a crystal statue (flag 3) whose spawn routine tests
  its flag.

These bytes cover two areas with none of these.  Each is saved with the
game but never meaningful.  No movie ever really used one of them, which
agrees.

| Block | Free bytes |
|-------|------------|
| West Hyrule ($0600) | $0600-$0605, $0609-$060F, $0611-$0612, $0614, $0618, $061A-$061B, $061E-$061F (21) |
| Death Mountain and Maze Island ($0620, shared) | $0620-$0623, $0625, $0627, $062B-$062C, $062E-$0631, $0633-$0638, $063A-$063F (24) |
| East Hyrule ($0640) | $0640-$0642, $0644, $0647-$064A, $064C-$064F, $0651-$0652, $0654-$0655, $0658, $065A-$065B, $065E (20) |
| Towns ($0660) | $0660-$0676, $0678-$067F (31) |
| Palaces 1, 2, 5 ($0680) | $0698 (1) |
| Palaces 3, 4, 6 ($06A0) | $06B9, $06BC (2) |
| Great Palace ($06C0) | $06C0-$06C4, $06C6-$06C7, $06C9-$06CA, $06CC-$06CF, $06D2-$06D3, $06D5, $06D7-$06D9, $06DB-$06DF (24) |

That is 123 bytes, at medium-high confidence.  It rests on decoding
every area's objects and enemy lists (`tools/area_objects.py` and the
enemy list layout), and on the list of flag users above.  A byte's flags
are free only while no hack gives one of its two areas an item, door,
jar, boss or statue.

## The Stack Page ($0100-$01FF)

The game keeps a few variables at the bottom of the page: `nmiMode`
($0100) and the area building variables at $010A-$010F and $0112.  The
stack grows down from $01FF.  Pushes aren't in the access log, so the
stack's depth was measured in the emulator, by watching writes that land
at the stack pointer, over both full movies:

- **Depth:** the deepest push was at $01E7, in both movies within a byte
  (the 100AK movie on frame 105814, inside `Music_PlayNotes`: the sound
  engine run from the NMI on top of the main thread's calls).  So the
  stack used at most 25 bytes, $01E7-$01FF.
- **Other writes:** none landed between $0100 and $01E9, except at the
  variables above (`Game_PowerOnInit`, `Area_Load`,
  `Area_DrawFloors`, `Area_DrawFloorColumn`, ...).
- **Reads:** the only reads that reach the rest of the page come from
  `Mode_LoadWorld`.  In worlds without an overworld map, its map copy's
  source pointer lands on this page, and it copies the page as junk into
  `sramOverworldMap`.

So $0113-$01E6 is free.  The table above gives high confidence to all of
it but the 32 bytes nearest the stack, since a deeper call chain the
movies never reached could take the stack further.

```
tools/stack_depth.py ../private/Zelda2_100AK_schmidtty.fm2 260000 --below 1EA
tools/stack_depth.py ../private/Zelda2_NoGlitch_2014_Final.fm2:0 166000 --below 1EA
```

(`--below 1EA` watches only the part below the depth an unrestricted run
had already reached; each takes about 30 minutes.)

## SRAM ($6000-$7FFF)

| Range | Bytes | Between | Confidence |
|-------|------:|---------|------------|
| $6340-$6956 | 1559 | the end of `sramAreaTiles` (4 pages of $D0) and the tables from $6957 | High |
| $6957 | 1 | the base of the table `Game_StartAtNorthCastle` fills (`sram_6957,y` with y from $88 down to 1, so $6958-$69DF) | High |
| $69E0-$69FF | 32 | the end of `sramLinkPalette3` (6 bytes) and `sramLocY` | High |
| $6BF8-$6C57 | 96 | `sramDoors`: copied from the area bank at every world load, but never read; the towns' doors are read from bank 3's ROM (`TownDoors`) | High |
| $6C58-$6CFF | 168 | the end of the doors and `sramProjectilePtrs` | High |
| $6FA1-$6FFF | 95 | the end of the enemy and projectile tables and `sramEnemyLists` | High |
| $7401 | 1 | `sramSaveStatus0` and file 0's stats | High |
| $7499 | 1 | `sramSaveStatus1` and file 1's world state | High |
| $773B-$77FF | 197 | `sramSaveStatus2` (the last save byte) and `sramMenuText` | High |
| $7918 | 1 | the end of `sramMenuText` (280 bytes) and `sramAreaPalettes` | High |
| $79F9-$7BFF | 519 | the end of `sramAreaPalettes` (224 bytes) and `sramOverworldMap` | High |
| $7FA6-$7FFF | 90 | the end of `sramFileScreenText` (166 bytes from $7F00) and the end of SRAM | High |

SRAM is battery-backed, so these bytes keep their values across power
cycles.  Nothing initializes or checks them; a hack using them for saved
data would need its own validity check.

## Caveats

- **Coverage:** the movies don't reach everything.  For example, nothing
  plays the second quest, and the movies rarely reach the continue
  screen or a game over.  Code they skip shows up only through the static
  checks, which see direct references and declared arrays, not every
  computed index.
- **Medium entries:** each one depends on what an array's index can be.
- **Rerunning:** `tools/ram_unused.py` lists the candidates; `--all`
  lists every range with its status, and `tools/ram_usage.py ADDR` shows
  who touches an address.
