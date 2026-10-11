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
    sub_850C        = $850C     ; routine; identical code in banks 1-5
    dat_851A        = $851A     ; 9 bytes compared individually ($851A-$8522)
    EnemyLists      = $88A0     ; enemies of each area (1024 bytes), copied to sramEnemyLists
    sub_80EE        = $80EE     ; routine (banks 1-5)
    sub_812F        = $812F     ; routine (banks 1, 2, 4, 5; not bank 3)
.endscope

; Overworld banks (1: West Hyrule / Death Mountain, 2: East Hyrule /
; Maze Island).  Also area banks.
.scope OverworldBank
    sub_83A1        = $83A1     ; routine
    GetTerrain      = $83CF     ; terrain at overworld tile (temp0, temp1) -> temp2
    Encounters      = $8409     ; overworld encounter table, by terrain
    sub_879B        = $879B     ; routine
    sub_81A3        = $81A3     ; routine
.endscope

; Palace banks (4: palaces, 5: Great Palace).  Also area banks.
.scope PalaceBank
    dat_84D8        = $84D8     ; indexed by y
.endscope

.endif
