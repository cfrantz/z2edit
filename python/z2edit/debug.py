import z2edit
from z2edit import Address, Text

_last = Address.prg(-1, 0xc000)
_z2text = False

def _convert(address, b=None):
    global _last
    if isinstance(address, Address):
        return address
    if address is None:
        return _last
    if b is not None:
        return Address.prg(b, address)
    return Address.prg(_last.bank()[1], address)

def _chr(val):
    global _z2text
    if _z2text:
        val = ord(Text.from_zelda2(bytes([val])))

    val = chr(val) if val >= 32 and val < 127 else '.'
    return val

def _get_rom(rom):
    if rom is None:
        rom = z2edit.app[0][-1]
    return rom

def set_text(text=None):
    global _z2text
    if text is not None:
        _z2text = bool(text)


def db(address=None, length=64, b=None, rom=None, text=None):
    global _last
    set_text(text)
    _last = _convert(address, b)
    rom = _get_rom(rom)
    buf = []

    for i in range(length):
        val = rom.read(_last)
        if i % 16 == 0:
            if i != 0:
                print('  %s\n%04x: ' % (''.join(buf), _last.addr()), end='')
            else:
                print('%04x: ' % _last.addr(), end='')
            buf = []

        print(' %02x' % val, end='')
        buf.append(_chr(val))
        _last += 1

    i = 16 if length % 16 == 0 else length % 16
    i = 48 - 3 * i
    print('%*s  %s' % (i, '', ''.join(buf)))


def dw(address=None, length=32, b=None, rom=None, text=None):
    global _last
    set_text(text)
    _last = _convert(address, b)
    rom = _get_rom(rom)
    buf = []

    for i in range(0, length):
        val1 = rom.read(_last)
        val2 = rom.read(_last + 1)
        if i % 8 == 0:
            if i != 0:
                print('  %s\n%04x: ' % (''.join(buf), _last.addr()), end='')
            else:
                print('%04x: ' % _last.addr(), end='')
            buf = []

        print(' %02x%02x' % (val2, val1), end='')
        buf.append(_chr(val2))
        buf.append(_chr(val1))
        _last += 2

    i = 16 if length % 8 == 0 else length % 8
    i = 48 - 3 * i
    print('%*s  %s' % (i, '', ''.join(buf)))


__all__ = ['db', 'dw']
