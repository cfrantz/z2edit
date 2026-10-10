; iNES header and CHR ROM.

.segment "HEADER"
    .byte   "NES", $1A
    .byte   8                   ; 8 x 16K PRG ROM banks
    .byte   16                  ; 16 x 8K CHR ROM banks
    .byte   $12                 ; mapper 1 (MMC1), battery-backed SRAM
    .byte   $00                 ; mapper high nibble 0
    .res    8, $00

; The CHR data is extracted from the reference ROM at build time.
.segment "CHR"
    .incbin "chr.bin"
