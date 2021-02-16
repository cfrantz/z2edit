import json
import z2edit
from z2edit import Address

class ObjectDict(dict):
    "ObjectDict is a `dict` with attribute access to the dictionary contents."

    def __getattr__(self, name):
        if name in self:
            return self[name]
        else:
            raise AttributeError('No such attribute: ' + name)

    def __setattr__(self, name, value):
        self[name] = value

    def __delattr__(self, name):
        if name in self:
            del self[name]
        else:
            raise AttributeError('No such attribute: ' + name)

    @staticmethod
    def from_json(text):
        return json.loads(text, object_hook=ObjectDict)

    @staticmethod
    def from_address(pyaddr):
        (segment, bank) = pyaddr.bank()
        address = pyaddr.addr()
        return {segment.capitalize(): [bank, address]}

    def to_json(self):
        return json.dumps(self, indent=4)


class Config(object):
    "Config is a convenience proxy for configuration objects."

    @staticmethod
    def get(name):
        return ObjectDict.from_json(z2edit.config[name])

    @staticmethod
    def put(name, config):
        z2edit.config[name] = config.to_json()

    @staticmethod
    def keys():
        return z2edit.config.keys()


# A 4x8 bitmap for writing a hexcode into a CHR tile.
_CHRXDIGITS = [
    [ 2, 5, 5, 5, 5, 5, 2, 0 ],    # 0
    [ 2, 6, 2, 2, 2, 2, 7, 0 ],    # 1
    [ 2, 5, 1, 2, 4, 4, 7, 0 ],    # 2
    [ 2, 5, 1, 3, 1, 5, 2, 0 ],    # 3
    [ 5, 5, 5, 7, 1, 1, 1, 0 ],    # 4
    [ 7, 4, 7, 1, 1, 5, 2, 0 ],    # 5
    [ 3, 4, 4, 4, 7, 5, 7, 0 ],    # 6
    [ 7, 1, 1, 2, 2, 4, 4, 0 ],    # 7
    [ 2, 5, 5, 2, 5, 5, 2, 0 ],    # 8
    [ 7, 5, 7, 1, 1, 1, 1, 0 ],    # 9
    [ 2, 5, 5, 7, 5, 5, 5, 0 ],    # A
    [ 6, 5, 5, 6, 5, 5, 6, 0 ],    # B
    [ 2, 5, 4, 4, 4, 5, 2, 0 ],    # C
    [ 6, 5, 5, 5, 5, 5, 6, 0 ],    # D
    [ 7, 4, 4, 7, 4, 4, 7, 0 ],    # E
    [ 7, 4, 4, 7, 4, 4, 4, 0 ],    # F
]

def Tile(bank, char):
    return Address.chr(bank, char*16)

def chr_clear(edit, tile, with_id=False):
    (seg, _) = tile.bank()
    if seg != "chr":
        raise Exception('tile address not in "chr" segment')

    char = tile.addr() >> 4
    for y in range(8):
        if with_id:
            val = _CHRXDIGITS[char>>4][y] << 4 | _CHRXDIGITS[char&0xF][y]
        else:
            val = 0
        edit.write(tile + y, val)
        edit.write(tile + y + 8, val)

def chr_copy(edit, dst_tile, src_tile):
    (seg, _) = dst_tile.bank()
    if seg != "chr":
        raise Exception('dst_tile address not in "chr" segment')
    (seg, _) = src_tile.bank()
    if seg != "chr":
        raise Exception('src_tile address not in "chr" segment')
    edit.write_bytes(dst_tile, edit.read_bytes(src_tile, 16))

def chr_swap(edit, a_tile, b_tile):
    (seg, _) = a_tile.bank()
    if seg != "chr":
        raise Exception('a_tile address not in "chr" segment')
    (seg, _) = b_tile.bank()
    if seg != "chr":
        raise Exception('b_tile address not in "chr" segment')
    a = edit.read_bytes(a_tile, 16)
    b = edit.read_bytes(b_tile, 16)
    edit.write_bytes(a_tile, b)
    edit.write_bytes(b_tile, a)

def version_tuple(version=z2edit.version):
    (ver, *_) = version.split('-')
    return tuple(map(int, ver.split('.')))

def check_version(ver):
    editor_version = version_tuple()
    if callable(ver):
        return ver(editor_version)
    if isinstance(ver, str):
        ver = version_tuple(ver)
    if isinstance(ver, tuple):
        return editor_version >= ver
    print('Cannot compare editor version %r with %r' % (editor_version, ver))
    return False

def check_version_or_die(ver):
    if not check_version(ver):
        raise Exception('Require editor version >= %r' % ver, z2edit.version, ver)
