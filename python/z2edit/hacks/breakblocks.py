# Break-blocks for overworlds
from z2edit import Address
import logging

logger = logging.getLogger(__name__)

def classic(rom):
    logger.info("Applying overworld breakblocks")
    block = bytes.fromhex('BA BB BC BD')
    rom.write_bytes(Address.Prg(1, 0x849b), block)
    rom.write_bytes(Address.Prg(2, 0x849b), block)
