#!/usr/bin/env python3
import argparse
import IPython
from threading import Thread
import logging
import sys
import os.path

from emu import Emulator
import z2edit
from z2edit import gui

LOG_LEVELS = {
    "TRACE": 5,
}
logger = logging.getLogger(__name__)


class EmulatorApp(object):
    _instance = None

    def __init__(self, args, dirs):
        self.running = True
        self.inner = None
        self.show_style_editor = False
        self.show_demo_window = False
        self.show_metrics_window = False
        self.windows = []
        self.args = args
        self.dirs = dirs
        self.preferences_gui = None
        self.emulator = Emulator(None, True)
        if args.rom:
            self.emulator.load_rom(args.rom)
        for p in args.plugin:
            self.emulator.load_plugin(p)

        if not EmulatorApp._instance:
            EmulatorApp._instance = self
        else:
            logger.error("An application instance already exists")

    @classmethod
    def get(cls):
        return cls._instance

    def run(self):
        self.inner = gui.Framework("NES Emulator", 1900, 900)
        self.inner.audio_init(48000, 1, 1024)
        self.inner.open_controller()

        self.inner.set_scale(self.args.dpi)
        self.inner.background = [0.3, 0.3, 0.3]

        while self.running and self.emulator.running:
            if ui := self.inner.prepare_frame():
                self.emulator.draw(ui)
                for window in self.windows:
                    window.draw(ui)

                self.inner.render_frame()
                self.windows = [w for w in self.windows if not w.wants_dispose]
            else:
                self.running = False
        self.inner = None

    def interact(self):
        emulator = self.emulator
        IPython.embed()
        self.running = False


def main():
    p = argparse.ArgumentParser(prog="Z2Edit NES Emulator", description="NES Emulator")
    p.add_argument(
        "--interactive",
        "-i",
        action="store_true",
        help="Start an interactive Python shell",
    )
    p.add_argument(
        "--plugin",
        "-p",
        type=str,
        action="append",
        help="Plugin to load",
    )
    p.add_argument(
        "--dpi",
        type=float,
        default=0.0,
        help="Set the DPI scaling factor",
    )
    p.add_argument(
        "--log",
        type=str,
        default="info",
        help="Logging level",
    )
    p.add_argument(
        "rom",
        metavar="ROM",
        type=str,
        nargs="?",
        help="NES ROM file",
    )

    args = p.parse_args()
    log = args.log.upper()
    logging.getLogger().setLevel(LOG_LEVELS.get(log, log))

    instdir = os.path.dirname(sys.argv[0])
    if instdir.endswith("python/z2edit/emulator"):
        instdir = os.path.normpath(os.path.join(instdir, "../../.."))
    z2edit.Directories.init(instdir)
    dirs = z2edit.Directories.get()

    a = EmulatorApp(args, dirs)
    if args.interactive:
        # We start the interpreter on another thread because GUI resources
        # always should be created/destroyed on the main thread.
        Thread(target=a.interact).start()
    a.run()
