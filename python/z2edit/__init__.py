import logging

FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.INFO)

from ._z2edit import *
__doc__ = _z2edit.__doc__
if hasattr(_z2edit, "__all__"):
    __all__ = _z2edit.__all__
