# Hacks related to magic spells
from z2edit import PyAddress

def flexible_bits(edit, asm):
    """Allow Spells to activate multiple effects based on set bits.

    The XP/Spells editor will set bits in the table at Prg(0, $8dbb).
    We create a constant bit_table elsewhere to test against to see which
    spell(s) to activate.
    """
    length = 8
    freespace = edit.alloc_near(PyAddress.prg(0, 0xbfe0), length)
    print("Using freespace at {}".format(freespace))
    freespace = freespace.addr()
    asm(f"""
        .bank 0
        .org $8e2a
            LDA bit_table,y

        .org {freespace}
        bit_table:
            .db $01,$02,$04,$08,$10,$20,$40,$80
        .assert_org {freespace+length}
    """)


def fast_casting(edit, asm):
    """Allow spells to be cast multiple times without menuing in-between casts."""

    asm(f"""
        .bank 0
        .org $8dd4
            NOP
            NOP
    """)

def no_restrictions(edit, asm):
    """Don't restrict ability to learn spells to number of magic containers."""

    asm(f"""
        .bank 3
        .org $b529
            CMP #0
    """)
