# Hacks related to magic spells
from z2edit import Address
from z2edit.assembler import Asm
import logging

logger = logging.getLogger(__name__)

def flexible_bits(rom):
    """Allow Spells to activate multiple effects based on set bits.

    The XP/Spells editor will set bits in the table at Prg(0, $8dbb).
    We create a constant bit_table elsewhere to test against to see which
    spell(s) to activate.
    """
    logger.info("Applying flexible_bits");
    length = 8
    freespace = edit.alloc(Address.Prg(0, 0xbfe0), length)
    logger.info("Using freespace at %s", freespace)
    freespace = freespace.offset()
    asm = Asm(rom)
    asm(f"""
        .bank 0
        .org $8e2a
            LDA bit_table,y

        .org {freespace}
        bit_table:
            .db $01,$02,$04,$08,$10,$20,$40,$80
        .assert_org {freespace+length}
    """)


def fast_casting(rom):
    """Allow spells to be cast multiple times without menuing in-between casts."""

    logger.info("Applying fast_casting");
    asm = Asm(rom)
    asm(f"""
        .bank 0
        .org $8dd4
            NOP
            NOP
    """)

def no_restrictions(rom):
    """Don't restrict ability to learn spells to number of magic containers."""

    logger.info("Applying no_restrictions");
    asm = Asm(rom)
    asm(f"""
        .bank 3
        .org $b529
            CMP #0
    """)
