import json
import z2edit
from z2edit import Address
from z2edit.util import ObjectDict

class KeepoutFixup(object):
    """Find maps and code in the keepout regions and move them out."""
    
    def __init__(self, edit, config):
        self.edit = edit
        self.config = config
        self.moved = {}

    def report(self):
        for bank in range(8):
            (chunks, total) = self.edit.report(Address.prg(bank, 0))
            print("Bank {}: {} bytes in {} chunks".format(bank, total, chunks))

    def bank3_code_move(self):
        # Bank3 has a fragment of code located in the keepout region.  There is
        # table of object construction routines starting at $9bdd, and the 
        # entry at $9bf9 points to the code in the keepout region.  Read it to
        # check if its in the keepout region (or maybe has already beend moved),
        # and then move it if needed.
        ptr = Address.prg(3, 0x9bf9)
        code = self.edit.read_pointer(ptr)
        koaddr = self.address_in_keepout(code)
        if koaddr is not None:
            print('Code in bank3 at {} in keepout range at {}... '.format(code, koaddr), end='')
            # The code go anywhere, but 0x84da is optimial because its nearly
            # an exact match on size.
            dst = self.edit.alloc_near(Address.prg(3, 0x84da), 35)
            print('moved to {}.'.format(dst))
            self.copy_and_clear(dst, code, 35)
            self.edit.write_pointer(ptr, dst)
            self.edit.write_pointer(ptr+2, dst)
            # Note: actually in the previous table, but still points to code
            # we moved.
            self.edit.write_pointer(ptr-60, dst+7)

    def address_in_keepout(self, addr):
        for (koaddr, kolen) in self.config.misc.freespace.keepout:
            koaddr = Address(koaddr)
            if addr.in_range(koaddr, kolen):
                return koaddr
        return None

    def copy_and_clear(self, dst, src, length):
        data = self.edit.read_bytes(src, length)
        self.edit.write_bytes(dst, data)
        self.edit.write_bytes(src, b'\xff' * length)

    def move_map(self, mapaddr):
        length = self.edit.read(mapaddr)
        newaddr = self.edit.alloc(mapaddr, length)
        self.copy_and_clear(newaddr, mapaddr, length)
        return newaddr

    def check_group(self, group):
        for i in range(0, group.length):
            grpaddr = Address(group.address)
            mapaddr = self.edit.read_pointer(grpaddr + i*2)
            koaddr = self.address_in_keepout(mapaddr)
            if koaddr is not None:
                print('Map {}/{} address {} in keepout range at {}... '.format(group.id, i, mapaddr, koaddr), end='')
                if mapaddr in self.moved:
                    print('already moved to {}.'.format(self.moved[mapaddr]))
                else:
                    newaddr = self.move_map(mapaddr)
                    self.moved[mapaddr] = newaddr
                    print('moved to {}.'.format(newaddr))
                self.edit.write_pointer(grpaddr + i*2, self.moved[mapaddr])

    def check(self):
        self.report()
        self.bank3_code_move()
        for group in self.config.sideview.group:
            self.check_group(group)
        self.report()

def fix_all(edit):
    meta = edit.meta
    extra = meta['extra']
    if extra.get('fix') != "true":
        return

    config = ObjectDict.from_json(z2edit.config[meta['config']])
    KeepoutFixup(edit, config).check()

    for el in config.sideview.enemy_list:
        el.length = 1024 if el.bank !=5 else 432

    name = 'config-{}'.format(extra.get('project'))
    z2edit.config[name] = config.to_json()
    extra['next_config'] = name
    edit.meta = meta
