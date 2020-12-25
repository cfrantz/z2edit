# Hacks for how to detect P2 vs P5.
from z2edit import PyAddress

def palace2_must_be_connection52(edit, asm):
    """Palace 2 must be connection #52"""
    asm("""
        .bank 4
        .org $bac3
            LDA $056c
        .org $bad3
            LDA $056c
        .org $bc83
            LDA $056c
        .org $bd75
            LDA $056c
    """)

def palace2_even_palace5_odd(edit, asm):
    """P2 even / P5 odd"""
    length = 7
    freespace = edit.alloc_near(PyAddress.prg(4, 0xbfe0), length)
    print("Using freespace at {}".format(freespace))
    freespace = freespace.addr()
    asm(f"""
        .bank 4
        .org $bac3
            JSR check_palace_code
            BCC #3
        .org $bad3
            JSR check_palace_code
            BCC #2
        .org $bc83
            JSR check_palace_code
            BCC #1
        .org $bd75
            JSR check_palace_code
            BCC #3

        .org {freespace}
        check_palace_code:
            PHA
            LDA $056c
            LSR
            PLA
            RTS
        .assert_org {freespace+length}
    """)


def palace2_odd_palace5_even(edit, asm):
    """P2 odd / P5 even"""
    length = 7
    freespace = edit.alloc_near(PyAddress.prg(4, 0xbfe0), length)
    print("Using freespace at {}".format(freespace))
    freespace = freespace.addr()
    asm(f"""
        .bank 4
        .org $bac3
            JSR check_palace_code
            BCS #3
        .org $bad3
            JSR check_palace_code
            BCS #2
        .org $bc83
            JSR check_palace_code
            BCS #1
        .org $bd75
            JSR check_palace_code
            BCS #3

        .org {freespace}
        check_palace_code:
            PHA
            LDA $056c
            LSR
            PLA
            RTS
        .assert_org {freespace+length}
    """)

def continue_from_palaces(edit, asm):
    """Continue from any Palace"""
    length = 45
    # FIXME: should be able to use -1, but freespace allocator doesn't support it.
    bank = 7
    freespace = edit.alloc_near(PyAddress.prg(bank, 0xd39a), length)
    print("Using freespace at {}".format(freespace))
    freespace = freespace.addr()
    asm(f"""
        SAVE_VALID = $7b0
        SAVED_ROOM = $7b1
        .bank {bank} 
        .org {freespace}
        save_on_entry:
            STA $0707           ; World number is in A register, store it
            CMP #$03            ; World less than 3
            BMI done            ; yes, branch to end
            LDA SAVE_VALID      ; Has the room code already been stored
            BNE done            ; yes, branch to end
            LDA $0561           ; get the current room code
            STA SAVED_ROOM      ; store it
            INC SAVE_VALID      ; mark it as stored
        done:
            LDA $0707           ; End: reload the world number
            RTS                 ; Return

        load_continue:
            LDA SAVED_ROOM      ; Load the room code
            STA $0561           ; Store it in the room code location
            JSR $a057           ; something to do with levels and XP?
            LDA $0707           ; Load the world number
            RTS                 ; return

        clear_on_exit:
            STY $0707           ; Zero out the world number (return to overworld)
            STY SAVE_VALID      ; Zero out the room-code-stored marker
            RTS
        .assert_org {freespace+length}

        .org $cbaa
            JSR save_on_entry   ; Entering a new area, maybe store room number
        .org $cf92
            JSR clear_on_exit   ; Leaving an area, clear the room-code-stored-marker
        .org $cad0
            JSR load_continue   ; Refresh the room marker and load the world number
            CMP #$03            ; World 3 or greater (palaces)
            BCS #$07            ; jump to restore/continue routine
        .org $cae3
            NOP                 ; zap out the code which clears the room number
            NOP
            NOP
    """)


def palaces_to_stone_fixed(edit, asm):
    """A fixed version of palaces turning to stone.

    The vanilla has a coding error and it only works because the item numbers
    equal the palace numbers.
    """
    asm("""
        .bank 1
        .org  $87aa
            LDA $078D,X
        .bank 2
        .org  $87aa
            LDA $078D,X
    """)


def palaces_to_stone_never(edit, asm):
    """Completely disable palaces turning to stone."""
    asm("""
        .bank -1
        .org  $e01e
            NOP
            NOP
            NOP
    """)
