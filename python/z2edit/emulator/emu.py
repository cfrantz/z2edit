#!/usr/bin/env python3
import logging
import importlib
import os

import z2edit
from z2edit.emulator.plugin import Plugin
from z2edit import gui
from z2edit import nes

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
        self.volume = 0.25
        self.preferences = Preferences(self)
        self.running = True
        self.plugins = []

    @property
    def nes(self):
        return self._emulator.nes

    def load_plugin(self, name):
        plugin = importlib.import_module(name)
        p = plugin.create(self)
        if isinstance(p, Plugin):
            self.plugins.append(p)
        else:
            logger.error("Requested plugin %s isn't a Plugin", name)

    def _load_rom(self, rom):
        if isinstance(rom, str):
            self._emulator = nes.EmulatorGui.from_file(rom)
            self.nes.name = os.path.basename(rom)
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
        for p in self.plugins:
            p.menu_bar()

    def file_menu(self):
        if gui.menu_item("Open"):
            self.load_rom(None)
        for p in self.plugins:
            p.menu("File")
        gui.separator()
        if gui.menu_item("Quit"):
            self.running = False

    def edit_menu(self):
        if gui.menu_item("Preferences", "", self.preferences.visible):
            self.preferences.visible = not self.preferences.visible
        for p in self.plugins:
            p.menu("Edit")

    def view_menu(self):
        if gui.menu_item("Audio", "", self._emulator.apu_debug):
            self._emulator.apu_debug = not self._emulator.apu_debug
        if gui.menu_item("CHR Viewer", "", self._emulator.chr_debug):
            self._emulator.chr_debug = not self._emulator.chr_debug
        if gui.menu_item("Controllers", "", self._emulator.controller_debug):
            self._emulator.controller_debug = not self._emulator.controller_debug
        if gui.menu_item("Memory", "", self._emulator.memory_debug):
            self._emulator.memory_debug = not self._emulator.memory_debug
        if gui.menu_item("VRAM Viewer", "", self._emulator.vram_debug):
            self._emulator.vram_debug = not self._emulator.vram_debug
        for p in self.plugins:
            p.menu("View")

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
                for p in self.plugins:
                    p.run_per_frame()
                origin = gui.get_cursor_screen_pos()
                self._emulator.draw_image(ui, self.scale, self.aspect)
                for p in self.plugins:
                    p.draw_image(origin)
        self.preferences.draw()
        for p in self.plugins:
            p.draw()
        gui.end()
        if self._emulator:
            self._emulator.draw_debug_windows(ui)
        if self.on_root:
            gui.pop_style_var(3)
