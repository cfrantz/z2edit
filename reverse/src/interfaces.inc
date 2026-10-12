; Bank interfaces: routines and data that several switchable banks provide
; at a common address.  See "Bank Interfaces" in CONVENTIONS.md.
;
; Members keep mechanical names until understood.

.ifndef INTERFACES_INC
INTERFACES_INC = 1

; Area banks (1-5): hold the sideview areas of a world and their
; data.  Bank 7 selects one by world/region (see areaPrgBank).
.scope AreaBank
    BackgroundAreas = $8000     ; background layer areas (words; all zero in bank 5)
    Palettes        = $800E     ; area palettes, copied to sramAreaPalettes
    MetatilePtrs    = $8500     ; side view metatile tables (words)
    OverworldMapPtrs = $8508    ; compressed overworld map of each region (words)
    IsSolidTile     = $850C     ; carry set if tile a is solid (identical in banks 1-5)
    SpecialTiles    = $851A     ; tiles with special behavior ($851A-$8522): +0-+2 doors, +3 chimney,
                                ; +4 breakable block, +5 crumbling floor, +6 lava (provisional)
    EnemyLists      = $88A0     ; enemies of each area (1024 bytes), copied to sramEnemyLists
    AreaPtrs        = $8523     ; area data of each area of the first group (words, by areaId)
    EnemyListPtrs   = $85A1     ; enemy list of each area (words into sramEnemyLists)
    Connections     = $871B     ; 4 per area: the area beyond each page edge (sramRoomConnections)
    Doors           = $8817     ; 4 per area: the area behind each door (towns only: TownDoors)
    AreaPtrs2       = $A000     ; area data of the second group (banks 1, 2: Death Mountain / Maze Island; 4: palaces 3, 4, 6)
    EnemyListPtrs2  = $A07E     ; enemy lists of the second group
    Connections2    = $A1F8     ; connections of the second group
    DrawLargeObject = $80EE     ; draw large area object areaObjectType (banks 1, 2, 4, 5; towns use Town_DrawLargeObject)
    DrawSmallObject = $812F     ; draw small area object (banks 1, 2, 4, 5)
.endscope

; Overworld banks (1: West Hyrule / Death Mountain, 2: East Hyrule /
; Maze Island).  Also area banks.
.scope OverworldBank
    QueueMapStrip   = $83A1     ; queue a strip of the overworld map in ppuBuffer0
    GetTerrain      = $83CF     ; terrain at overworld tile (temp0, temp1) -> temp2
    Encounters      = $8409     ; overworld encounter table, by terrain
    UpdatePalaceStones = $879B  ; show palaces as stone once their crystals are placed
    SetFloorTiles   = $81A3     ; set up the floor tiles for side view areas
    UseFlute        = $8368     ; effects of playing the flute at Link's position
    Locations       = $861F     ; overworld location tables (Y, X, area, destination; 63 each), copied to sramLocY ...
    Locations2      = $A0FC     ; location tables of the second region (Death Mountain / Maze Island)
.endscope

; Palace banks (4: palaces, 5: Great Palace).  Also area banks.
.scope PalaceBank
    PaletteRows     = $84D8     ; 4 colors per palace (the first $FF, keeping the backdrop), copied by bank 7 into sramAreaPalettes+172 and +188
.endscope

.endif
