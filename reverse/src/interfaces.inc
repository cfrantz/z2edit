; Bank interfaces: routines and data that several switchable banks provide
; at a common address.  See "Bank Interfaces" in CONVENTIONS.md.
;
; Members keep mechanical names until understood.

.ifndef INTERFACES_INC
INTERFACES_INC = 1

; Area banks (1-5): hold the sideview areas of a world and their
; data.  Bank 7 selects one by world/region (see areaPrgBank).
.scope AreaBank
    dat_8000        = $8000     ; word table, indexed by x (all zero in bank 5)
    Palettes        = $800E     ; area palettes, copied to sramAreaPalettes
    dat_8500        = $8500     ; word table, indexed by y
    OverworldMapPtrs = $8508    ; compressed overworld map of each region (words)
    sub_850C        = $850C     ; routine; identical code in banks 1-5
    dat_851A        = $851A     ; 9 bytes compared individually ($851A-$8522)
    EnemyLists      = $88A0     ; enemies of each area (1024 bytes), copied to sramEnemyLists
.endscope

; Overworld banks (1: West Hyrule / Death Mountain, 2: East Hyrule /
; Maze Island).  Also area banks.
.scope OverworldBank
    sub_83A1        = $83A1     ; routine
    GetTerrain      = $83CF     ; terrain at overworld tile (temp0, temp1) -> temp2
    dat_8409        = $8409     ; indexed by y
    sub_879B        = $879B     ; routine
.endscope

; Palace banks (4: palaces, 5: Great Palace).  Also area banks.
.scope PalaceBank
    dat_84D8        = $84D8     ; indexed by y
.endscope

.endif
