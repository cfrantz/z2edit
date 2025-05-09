import json
import z2edit
from z2edit import Address
import copy as _copy

def register_types():
    _copy._deepcopy_dispatch[Address] = _copy._deepcopy_atomic

register_types()

class ConfigEncoder(json.JSONEncoder):
    def default(self, obj):
        if isinstance(obj, Address):
            return ObjectDict.from_address(obj)
        return json.JSONEncoder.default(self, obj)


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

    def _get(self, objpath, default=Exception):
        if objpath.startswith('/'):
            objpath = objpath[1:]
        try:
            obj = self
            for k in objpath.split('/'):
                if isinstance(obj, list):
                    obj = obj[int(k, 0)]
                else:
                    obj = obj[k]
            return obj
        except (KeyError, IndexError) as ex:
            if default is Exception:
                raise ex
            return default

    def get(self, objpath, default=None):
        return self._get(objpath, default)


    def query(self, objpath, **kwargs):
        """
        Query objects from an ObjectDict.

        objdict.query("bank/*/sideview/group/{name}/availability", name=lambda k,v: k != "foo")
        """
        if isinstance(objpath, str):
            if objpath.startswith('/'):
                objpath = objpath[1:]
            objpath = objpath.split('/')
        return dict(self._query(self, objpath, [], kwargs))

    @staticmethod
    def _query(item, objpath, qpath, qparam):
        if not objpath:
            yield ('/'.join(qpath), item)
        elif objpath[0] == '*':
            items = enumerate(item) if isinstance(item, list) else item.items()
            for k, v in items:
                yield from ObjectDict._query(v, objpath[1:], qpath+[k], qparam)
        elif objpath[0].startswith('{') and objpath[0].endswith('}'):
            qf = qparam[objpath[0][1:-1]]
            items = enumerate(item) if isinstance(item, list) else item.items()
            for k, v in items:
                if qf(k, v):
                    yield from ObjectDict._query(v, objpath[1:], qpath+[k], qparam)
        else:
            try:
                node = objpath[0]
                if isinstance(item, list):
                    node = int(node, 0)
                yield from ObjectDict._query(item[node], objpath[1:], qpath+[str(node)], qparam)
            except (KeyError, IndexError):
                pass

    def select(self, objpath, copy=False, **kwargs):
        obj = self._get(objpath)
        if not isinstance(obj, list):
            raise Exception("Object {} is not a list".format(objpath))
        if not kwargs:
            return obj

        result = []
        for o in obj:
            if all(v(o._get(k)) if callable(v) else v == o._get(k)
                   for (k, v) in kwargs.items()):
                result.append(o)
        if copy:
            result = _copy.deepcopy(result)
        return result

    def select_one(self, objpath, copy=False, **kwargs):
        result = self.select(objpath, copy, **kwargs)
        if len(result) != 1:
            raise Exception("Expected exactly 1 of {}, found {}".format(objpath, len(result)))
        return result[0]

    @staticmethod
    def from_json(text):
        return json.loads(text, object_hook=ObjectDict)

    @staticmethod
    def from_address(address):
        ty = repr(address)
        rest = []
        if ty != 'NullPtr':
            (ty, _) = ty.split('(')
            bank = address.bank()
            if bank is not None:
                rest.append(bank)
            rest.append(address.offset())
        return {ty: rest}

    def to_json(self):
        return json.dumps(self, indent=4, cls=ConfigEncoder)

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
    return Address.Chr(bank, char*16)

def chr_clear(rom, tile, with_id=False):
    if not isinstance(tile, Address.Chr):
        raise Exception('tile address not in "chr" segment')

    char = tile.offset() >> 4
    for y in range(8):
        if with_id:
            val = _CHRXDIGITS[char>>4][y] << 4 | _CHRXDIGITS[char&0xF][y]
        else:
            val = 0
        rom.write(tile + y, val)
        rom.write(tile + y + 8, val)

def chr_copy(rom, dst_tile, src_tile):
    if not isinstance(dst_tile, Address.Chr):
        raise Exception('dst_tile address not in "chr" segment')
    if not isinstance(src_tile, Address.Chr):
        raise Exception('src_tile address not in "chr" segment')
    rom.write_bytes(dst_tile, rom.read_bytes(src_tile, 16))

def chr_swap(rom, a_tile, b_tile):
    if not isinstance(a_tile, Address.Chr):
        raise Exception('a_tile address not in "chr" segment')
    if not isinstance(b_tile, Address.Chr):
        raise Exception('b_tile address not in "chr" segment')
    a = rom.read_bytes(a_tile, 16)
    b = rom.read_bytes(b_tile, 16)
    rom.write_bytes(a_tile, b)
    rom.write_bytes(b_tile, a)

#def version_tuple(version=z2edit.version):
#    (ver, *_) = version.split('-')
#    return tuple(map(int, ver.split('.')))
#
#def check_version(ver):
#    editor_version = version_tuple()
#    if callable(ver):
#        return ver(editor_version)
#    if isinstance(ver, str):
#        ver = version_tuple(ver)
#    if isinstance(ver, tuple):
#        return editor_version >= ver
#    print('Cannot compare editor version %r with %r' % (editor_version, ver))
#    return False
#
#def check_version_or_die(ver):
#    if not check_version(ver):
#        raise Exception('Require editor version >= %r' % ver, z2edit.version, ver)
