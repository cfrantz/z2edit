#!/usr/bin/env python3
import argparse
import IPython
from threading import Thread
import logging
import sys
import os.path

import z2edit
from z2edit import gui
from z2edit import nes

LOG_LEVELS = {
    "TRACE": 5,
}
logger = logging.getLogger(__name__)


class Preferences(object):

    def __init__(self, emulator):
        self.visible = False
        self.emulator = emulator

    def draw(self):
        if not self.visible:
            return
        gui.set_next_window_size(gui.Vec2(600.0, 250.0), gui.FIRST_USE_EVER)
        w, self.visible = gui.begin("Preferences", self.visible)
        if w:
            _, self.emulator.scale = gui.drag_float(
                "Scale", self.emulator.scale, 0.125, 0.25, 8.0, "%.03f"
            )
            _, self.emulator.aspect = gui.drag_float(
                "Aspect", self.emulator.aspect, 0.01, 0.5, 2.0, "%.03f"
            )
            _, self.emulator.volume = gui.drag_float(
                "Volume", self.emulator.volume, 0.01, 0.01, 1.0, "%.02f"
            )
        gui.end()


class Emulator(object):

    def __init__(self, rom=None, on_root=False):
        self._emulator = None
        if rom:
            self._load_rom(rom)
        self.on_root = on_root
        self.scale = 4.0
        self.aspect = 1.333
        self.volume = 1.0
        self.preferences = Preferences(self)
        self.running = True

    @property
    def nes(self):
        return self._emulator.nes

    def _load_rom(self, rom):
        if isinstance(rom, str):
            self._emulator = nes.EmulatorGui.from_file(rom)
        else:
            self._emulator = nes.EmulatorGui(rom)

    def load_rom(self, filename):
        if filename is None:
            dlg = z2edit.FileDialog()
            dlg.add_filter("NES ROM", ["nes"])
            dlg.add_filter("All", ["*"])
            filename = dlg.pick_file()
        if filename is not None:
            self._load_rom(filename)

    def menu_bar(self):
        if gui.begin_menu("File"):
            self.file_menu()
            gui.end_menu()
        if gui.begin_menu("Edit"):
            self.edit_menu()
            gui.end_menu()
        if gui.begin_menu("View"):
            self.view_menu()
            gui.end_menu()

    def file_menu(self):
        if gui.menu_item("Open"):
            self.load_rom(None)
        gui.separator()
        if gui.menu_item("Quit"):
            self.running = False

    def edit_menu(self):
        if gui.menu_item("Preferences", "", self.preferences.visible):
            self.preferences.visible = not self.preferences.visible

    def view_menu(self):
        if gui.menu_item("Audio", "", self._emulator.apu_debug):
            self._emulator.apu_debug = not self._emulator.apu_debug
        if gui.menu_item("CHR Viewer", "", self._emulator.chr_debug):
            self._emulator.chr_debug = not self._emulator.chr_debug
        if gui.menu_item("Controllers", "", self._emulator.controller_debug):
            self._emulator.controller_debug = not self._emulator.controller_debug
        if gui.menu_item("VRAM Viewer", "", self._emulator.vram_debug):
            self._emulator.vram_debug = not self._emulator.vram_debug

    def draw(self, ui):
        if self.on_root:
            gui.begin_main_menu_bar()
            self.menu_bar()
            gui.end_main_menu_bar()

            gui.push_style_var(gui.StyleVar.WINDOW_PADDING, gui.Vec2(0.0, 0.0))
            gui.push_style_var(gui.StyleVar.WINDOW_ROUNDING, 0.0)
            gui.push_style_var(gui.StyleVar.WINDOW_BORDER_SIZE, 0.0)
            gui.set_next_window_pos(gui.Vec2(0.0, 20.0), gui.ALWAYS)
            root_flags = (
                gui.WindowFlags.NO_TITLE_BAR
                | gui.WindowFlags.ALWAYS_AUTO_RESIZE
                | gui.WindowFlags.NO_MOVE
                | gui.WindowFlags.NO_SCROLLBAR
                | gui.WindowFlags.NO_BRING_TO_FRONT_ON_FOCUS
                | gui.WindowFlags.NO_RESIZE
            )
        else:
            root_flags = gui.WindowFlags.MENU_BAR

        (w, _) = gui.begin("emulator", flags=root_flags)
        if w:
            if not self.on_root:
                gui.begin_menu_bar()
                self.menu_bar()
                gui.end_menu_bar()

            if self._emulator:
                self._emulator.nes.volume = self.volume
                self._emulator.handle_input(ui)
                self._emulator.emulate_frame(ui)
                self._emulator.draw_image(ui, self.scale, self.aspect)
        self.preferences.draw()
        gui.end()
        if self._emulator:
            self._emulator.draw_debug_windows(ui)
        if self.on_root:
            gui.pop_style_var(3)


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
        self.rom_to_emulate = None
        self.emulator = Emulator(None, True)
        if args.rom:
            self.emulator.load_rom(args.rom)

        if not EmulatorApp._instance:
            EmulatorApp._instance = self
        else:
            logger.error("An application instance already exists")

    @classmethod
    def get(cls):
        return cls._instance

    def run(self):
        self.inner = gui.Framework("Z2Edit", 1900, 900)
        self.inner.audio_init(48000, 1, 1024)
        self.inner.open_controller()

        self.inner.set_scale(self.args.dpi)
        self.inner.background = [0.3, 0.3, 0.3]

        while self.running and self.emulator.running:
            self._emulator()
            if ui := self.inner.prepare_frame():
                self.emulator.draw(ui)
                for window in self.windows:
                    window.draw(ui)

                self.inner.render_frame()
                self.windows = [w for w in self.windows if not w.wants_dispose]
            else:
                self.running = False
        self.inner = None

    def _emulator(self):
        if not self.rom_to_emulate:
            return
        rom = self.rom_to_emulate
        self.rom_to_emulate = None
        if isinstance(rom, str):
            rom = nes.NesFile.load(rom)
        self.emulator = Emulator(rom, True)

    def interact(self):
        a = self
        IPython.embed()
        self.running = False


def main():
    p = argparse.ArgumentParser(prog="z2edit", description="Zelda2 Editor")
    p.add_argument(
        "--interactive",
        "-i",
        action="store_true",
        help="Start an interactive Python shell",
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
    if instdir.endswith("python/z2edit"):
        instdir = os.path.normpath(os.path.join(instdir, "../.."))
    z2edit.Directories.init(instdir)
    dirs = z2edit.Directories.get()

    a = EmulatorApp(args, dirs)
    if args.interactive:
        # We start the interpreter on another thread because GUI resources
        # always should be created/destroyed on the main thread.
        Thread(target=a.interact).start()
    a.run()


if __name__ == "__main__":
    main()
