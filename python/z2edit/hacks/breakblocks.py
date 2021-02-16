# Break-blocks for overworlds
from z2edit import Address

def classic(edit):
    block = bytes.fromhex('BA BB BC BD')
    edit.write_bytes(Address.prg(1, 0x849b), block)
    edit.write_bytes(Address.prg(2, 0x849b), block)
