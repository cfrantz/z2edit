######################################################################
# FM2 Movie Player
#
######################################################################
import re
import logging

import z2edit
from z2edit import gui
from z2edit.emulator import plugin

logger = logging.getLogger(__name__)


class Fm2Movie(plugin.Plugin):
    CONTROLLER = re.compile(r"\|.\|((?:[^|]+\|)+)\|")

    def __init__(self, emulator, args):
        super().__init__(emulator)
        self.frames = {}
        if args:
            self.load_movie(args[0])

    def menu(self, name):
        if name == "File":
            if gui.menu_item("Load FM2 Movie"):
                self.load_movie(None)

    def run_pre_frame(self):
        fnum = self.emulator.nes.frame + 2
        if frame := self.frames.get(fnum):
            # logger.info("Frame %d = %r", fnum, frame)
            for i, buttons in enumerate(frame.get("buttons", [])):
                self.emulator.nes.controller_value(i, buttons)

    def load_movie(self, filename):
        if filename is None:
            dlg = z2edit.FileDialog()
            dlg.add_filter("FM2 Movie", ["fm2"])
            dlg.add_filter("All", ["*"])
            filename = dlg.pick_file()
        if filename is not None:
            self._load_movie(filename)

    def _load_movie(self, filename):
        with open(filename, "rt") as movie:
            self.frames = {}
            for line in movie:
                line = line.strip()
                if line.startswith("|"):
                    self.parse(line)
                else:
                    logger.info("TAS: %s", line)
        logger.info("Parsed %d frame records", len(self.frames))
        self.emulator.nes.reset()
        self.emulator.nes.name = None

    def parse(self, line):
        if m := self.CONTROLLER.match(line):
            buttons = []
            controller = m.group(1)[:-1].split("|")
            for button in controller:
                val = 0
                for b in button:
                    val <<= 1
                    val |= 1 if b != "." else 0
                buttons.append(val)
            n = len(self.frames)
            self.frames[n] = {"buttons": buttons}


def create(emulator, args):
    return Fm2Movie(emulator, args)
