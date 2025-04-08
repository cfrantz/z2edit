import json
import z2edit
import logging

from z2edit import Address
from z2edit import AddressRange
from z2edit import Alloc
from z2edit.util import ObjectDict

logger = logging.getLogger(__name__)

class KeepoutFixup(object):
    """Find maps and code in the keepout regions and move them out."""
    
    def __init__(self, project, config):
        self.project = project
        self.rom = project.rom
        self.config = config
        self.moved = {}

    def bank3_code_move(self):
        # Bank3 has a fragment of code located in the keepout region.  There is
        # table of object construction routines starting at $9bdd, and the 
        # entry at $9bf9 points to the code in the keepout region.  Read it to
        # check if its in the keepout region (or maybe has already beend moved),
        # and then move it if needed.
        ptr = Address.Prg(3, 0x9bf9)
        code = self.rom.read_pointer(ptr)
        if keepout := self.address_in_keepout(code):
            logger.info(f'Code in bank3 at {code} in keepout range at {keepout}.')
            # The code go anywhere, but 0x84da is optimial because its nearly
            # an exact match on size.
            dst = self.rom.alloc(Address.Prg(3, 0x84da), 35, Alloc.Near)
            logger.info(f'{code} moved to {dst}.')
            self.copy_and_clear(dst, code, 35)
            self.rom.write_pointer(ptr, dst)
            self.rom.write_pointer(ptr+2, dst)
            # Note: actually in the previous table, but still points to code
            # we moved.
            self.rom.write_pointer(ptr-60, dst+7)

    def address_in_keepout(self, addr):
        bank = addr.bank()
        for keepout in self.config.get(f'bank/{bank}/freespace/keepout'):
            keepout = AddressRange(keepout)
            if keepout.contains_addr(addr):
                return keepout
        return None

    def copy_and_clear(self, dst, src, length):
        data = self.rom.read_bytes(src, length)
        self.rom.write_bytes(dst, data)
        self.rom.write_bytes(src, b'\xff' * length)

    def move_map(self, mapaddr):
        length = self.rom.read(mapaddr)
        newaddr = self.rom.alloc(mapaddr, length)
        self.copy_and_clear(newaddr, mapaddr, length)
        logging.info(f'... moved {length} bytes to {newaddr}')
        return newaddr

    def check_bank(self, bank):
        path = f'bank/{bank}/sideview'
        sideview = self.config.get(path)
        for (name, group) in sideview.group.items():
            grpaddr = Address(group.address)
            for i in range(0, group.length):
                mapaddr = self.rom.read_pointer(grpaddr + i*2)
                if keepout := self.address_in_keepout(mapaddr):
                    logging.info(f'Map {path}/{name}/{i} address {mapaddr} in keepout range {keepout}')
                    if mapaddr in self.moved:
                        logging.info(f'... already moved to {self.moved[mapaddr]}')
                    else:
                        newaddr = self.move_map(mapaddr)
                        self.moved[mapaddr] = newaddr
                    self.rom.write_pointer(grpaddr + i*2, self.moved[mapaddr])
        sideview.enemy_length = 1024 if bank != 5 else 432

    def check(self):
        self.bank3_code_move()
        for bank in range(1, 6):
            self.check_bank(bank)
        logging.info('Freespace after moves: %s', self.rom.report())

def fix_all(project):
    config = ObjectDict.from_json(project.config)
    KeepoutFixup(project, config).check()
    project.config = config.to_json()
