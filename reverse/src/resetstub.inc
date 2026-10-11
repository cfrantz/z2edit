; Reset stub present at $BF70-$BFFF in every switchable bank (0-6).
;
; At power-on the MMC1 may have any bank mapped at $8000 (and, before it is
; configured, may even map a switchable bank at $C000-$FFFF).  Every bank
; therefore carries a copy of the reset code from bank 7 ($FF70-$FFDF) and
; its own interrupt vectors, so that reset works whichever bank is mapped.
; The copies are identical to bank 7's code apart from addresses.  See
; Reset in bank7.s for the commented original.
;
; Banks 0 and 5 also call their own copies of the MMC1 helpers.

.ifndef RESETSTUB_INC
RESETSTUB_INC = 1

.macro ResetStub bank
    .assert * = $BF70, error, "reset stub must be at $BF70"

.ident(.sprintf("Reset_Bank%d", bank)):
    sei
    cld
    ldx     #$00
    stx     PPUCTRL
    inx
@waitVblank:
    lda     PPUSTATUS
    bpl     @waitVblank
    dex
    beq     @waitVblank
    txs
    stx     MMC1_CONTROL
    stx     MMC1_CHR0
    stx     MMC1_CHR1
    stx     MMC1_PRG
    lda     #MMC1_MIRROR_HORIZONTAL | MMC1_PRG_FIX_LAST | MMC1_CHR_8K
    jsr     .ident(.sprintf("Mmc1_SetControl_Bank%d", bank))
    jsr     .ident(.sprintf("Mmc1_SetChrBank_Bank%d", bank))
    lda     #$07
    jsr     .ident(.sprintf("Mmc1_SetPrgBank_Bank%d", bank))
    jmp     Main

.ident(.sprintf("Mmc1_SetControl_Bank%d", bank)):
    .repeat 4
    sta     MMC1_CONTROL
    lsr     a
    .endrepeat
    sta     MMC1_CONTROL
    rts

.ident(.sprintf("Mmc1_SetChrBank_Bank%d", bank)):
    .repeat 4
    sta     MMC1_CHR0
    lsr     a
    .endrepeat
    sta     MMC1_CHR0
    rts

.ident(.sprintf("Mmc1_SetPrgBank0_Bank%d", bank)):
    lda     #$00
    beq     .ident(.sprintf("Mmc1_SetPrgBank_Bank%d", bank))

.ident(.sprintf("Mmc1_SetPrgBankArea_Bank%d", bank)):
    lda     areaPrgBank

.ident(.sprintf("Mmc1_SetPrgBank_Bank%d", bank)):
    .repeat 4
    sta     MMC1_PRG
    lsr     a
    .endrepeat
    sta     MMC1_PRG
    rts

    .assert * = $BFE0, error, "reset stub size changed"
    .res    $1A, $FF

    ; Interrupt vectors.
    .word   Nmi
    .word   .ident(.sprintf("Reset_Bank%d", bank))
    .word   RomHeaderChecksums      ; IRQ (never enabled)
.endmacro

.endif
