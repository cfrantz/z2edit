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
    dat_800E        = $800E     ; indexed by y
    dat_8500        = $8500     ; word table, indexed by y
    dat_8508        = $8508     ; word table, indexed by x
    sub_850C        = $850C     ; routine; identical code in banks 1-5
    dat_851A        = $851A     ; 9 bytes compared individually ($851A-$8522)
    dat_88A0        = $88A0     ; indexed by y
    dat_89A0        = $89A0     ; indexed by y
    dat_8AA0        = $8AA0     ; indexed by y
    dat_8BA0        = $8BA0     ; indexed by y
.endscope

; Overworld banks (1: West Hyrule / Death Mountain, 2: East Hyrule /
; Maze Island).  Also area banks.
.scope OverworldBank
    sub_83A1        = $83A1     ; routine
    sub_83CF        = $83CF     ; routine
    dat_8409        = $8409     ; indexed by y
    sub_879B        = $879B     ; routine
.endscope

; Palace banks (4: palaces, 5: Great Palace).  Also area banks.
.scope PalaceBank
    dat_84D8        = $84D8     ; indexed by y
.endscope

.endif
