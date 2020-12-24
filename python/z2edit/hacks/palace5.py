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
