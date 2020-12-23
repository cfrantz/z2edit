import z2edit
from z2edit import PyAddress

_last = PyAddress.prg(-1, 0xc000)

def _convert(address, b=None):
    global _last
    if isinstance(address, PyAddress):
        return address
    if address is None:
        return _last
    if b is not None:
        return PyAddress.prg(b, address)
    return PyAddress.prg(_last.bank()[1], address)


def _get_rom(rom):
    if rom is None:
        rom = z2edit.app[0][-1]
    return rom


def db(address=None, length=64, b=None, rom=None):
    global _last
    _last = _convert(address, b)
    rom = _get_rom(rom)
    buf = []

    for i in range(length):
        val = rom.read_byte(_last)
        if i % 16 == 0:
            if i != 0:
                print('  %s\n%04x: ' % (''.join(buf), _last.addr()), end='')
            else:
                print('%04x: ' % _last.addr(), end='')
            buf = []

        print(' %02x' % val, end='')
        buf.append(chr(val) if val >= 32 and val < 127 else '.')
        _last += 1

    i = 16 if length % 16 == 0 else length % 16
    i = 48 - 3 * i
    print('%*s  %s' % (i, '', ''.join(buf)))


def dw(address=None, length=32, b=None, rom=None):
    global _last
    _last = _convert(address, b)
    rom = _get_rom(rom)
    buf = []

    for i in range(0, length):
        val1 = rom.read_byte(_last)
        val2 = rom.read_byte(_last + 1)
        if i % 8 == 0:
            if i != 0:
                print('  %s\n%04x: ' % (''.join(buf), _last.addr()), end='')
            else:
                print('%04x: ' % _last.addr(), end='')
            buf = []

        print(' %02x%02x' % (val2, val1), end='')
        buf.append(chr(val2) if val2 >= 32 and val2 < 127 else '.')
        buf.append(chr(val1) if val1 >= 32 and val1 < 127 else '.')
        _last += 2

    i = 16 if length % 8 == 0 else length % 8
    i = 48 - 3 * i
    print('%*s  %s' % (i, '', ''.join(buf)))


__all__ = ['db', 'dw']
