from z2edit.app import Application
from z2edit import Address, Text

_last = Address.Prg(-1, 0xC000)

def _get_rom(rom):
    if rom is None:
        rom = Application.get().project().rom
    return rom

def _convert(address, b=None):
    global _last
    if isinstance(address, Address):
        return address
    if address is None:
        return _last
    if b is not None:
        return Address.Prg(b, address)
    return Address.prg(_last.bank(), address)


def _chr(val):
    val = Text.from_zelda2(bytes([val]))
    return val


def db(address=None, length=64, b=None, rom=None):
    global _last
    _last = _convert(address, b)
    buf = []
    rom = _get_rom(rom)

    for i in range(length):
        val = rom.read(_last)
        if i % 16 == 0:
            if i == 0:
                print("%04x: " % _last.offset(), end="")
            else:
                print("  %s\n%04x: " % ("".join(buf), _last.offset()), end="")
            buf = []
        print(" %02x" % val, end="")
        buf.append(_chr(val))
        _last += 1

    i = 16 if length % 16 == 0 else length % 16
    i = 48 - 3 * i
    print("%*s  %s" % (i, "", "".join(buf)))


def dw(address=None, length=32, b=None, rom=None):
    global _last
    _last = _convert(address, b)
    buf = []
    rom = _get_rom(rom)

    for i in range(length):
        val1 = rom.read(_last)
        val2 = rom.read(_last + 1)
        if i % 8 == 0:
            if i == 0:
                print("%04x: " % _last.offset(), end="")
            else:
                print("  %s\n%04x: " % ("".join(buf), _last.offset()), end="")
            buf = []
        print(" %02x%02x" % (val2, val1), end="")
        buf.append(_chr(val2))
        buf.append(_chr(val1))
        _last += 2

    i = 8 if length % 8 == 0 else length % 8
    i = 40 - 5 * i
    print("%*s  %s" % (i, "", "".join(buf)))
