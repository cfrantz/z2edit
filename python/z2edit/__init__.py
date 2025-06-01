import logging

FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.INFO)

from ._z2edit import *
# TODO(cfrantz): This is a legacy import.  Should migrate everyone to
# using something more like:
#    from z2edit.nes import Address
import _z2edit.nes
from _z2edit.nes import (Address, AddressRange, Alloc)

__doc__ = _z2edit.__doc__
if hasattr(_z2edit, "__all__"):
    __all__ = _z2edit.__all__
