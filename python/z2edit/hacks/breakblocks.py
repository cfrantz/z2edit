# Break-blocks for overworlds
from z2edit import PyAddress

def classic(edit):
    block = bytes.fromhex('BA BB BC BD')
    edit.write_bytes(PyAddress.prg(1, 0x849b), block)
    edit.write_bytes(PyAddress.prg(2, 0x849b), block)
