"""Helpers for driving the emulator without a GUI."""
import os
import sys
from types import SimpleNamespace

# The z2edit package expects both `python` and `python/z2edit` on the path.
_PYTHON = os.path.join(os.path.dirname(os.path.abspath(__file__)), "../../python")
sys.path[:0] = [_PYTHON, os.path.join(_PYTHON, "z2edit")]

from z2edit import nes  # noqa: E402
from z2edit.emulator.fm2 import Fm2Movie  # noqa: E402
from z2edit.emulator.symbols import Symbols  # noqa: E402


def emulator(rom):
    """Create a powered-on emulator for `rom`."""
    return nes.Nes(nes.NesFile.load(rom))


def movie(emu, filename):
    """Load an FM2 movie for `emu`.  Call `.run_pre_frame()` before each
    frame (e.g. from the `pre_frame` callback of `run_frames`).

    `filename` may be suffixed with `:N` to set the movie's input frame
    offset (see Fm2Movie.offset).
    """
    offset = None
    name, sep, suffix = filename.rpartition(":")
    if sep and suffix.lstrip("-").isdigit():
        filename, offset = name, int(suffix)
    m = Fm2Movie(SimpleNamespace(nes=emu), [filename])
    if offset is not None:
        m.offset = offset
    return m


def symbols(emu, filename):
    """Load a symbol table produced by tools/symbols.py."""
    return Symbols(SimpleNamespace(nes=emu), [filename])


class Checkpoint:
    """An emulator state, restorable with a movie still in sync.

    The emulator's frame counter (which a movie uses to pick its input) is
    not part of the saved state, so restoring shifts the movie's offset by
    the difference.  States live in memory only (NesState is not exposed
    for saving to disk).
    """

    def __init__(self, emu, mv=None):
        self.emu, self.mv = emu, mv
        self.state = emu.save_state()
        self.frame = emu.frame
        self.offset = mv.offset if mv else None

    def restore(self):
        self.emu.restore_state(self.state)
        if self.mv:
            self.mv.offset = self.offset + (self.frame - self.emu.frame)
