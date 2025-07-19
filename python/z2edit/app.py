import argparse
import IPython
from threading import Thread
import logging
import sys
import os.path

import z2edit
from z2edit import gui
from z2edit import nes
from z2edit.emulator.emu import Emulator

LOG_LEVELS = {
    "TRACE": 5,
}
logger = logging.getLogger(__name__)


# This parser is used when spawning the internal emulator from
# the main gui.
emulator_parser = argparse.ArgumentParser(
    prog="emulator", description="Z2Edit Emulator"
)
emulator_parser.add_argument(
    "--plugin",
    "-p",
    type=str,
    default=[],
    action="append",
    help="Plugin to load",
)
emulator_parser.add_argument(
    "rom",
    metavar="ROM",
    type=str,
    nargs="?",
    help="NES ROM file",
)


class Application(object):
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
        self.preferences_file = os.path.join(self.dirs.config_dir, args.preferences)
        self.preferences = z2edit.AppPreferences()
        self.preferences.load(self.preferences_file)
        self.preferences_gui = None
        self.wizard = None
        self.emulator_args = emulator_parser.parse_args([])
        if args.new:
            self.wizard = z2edit.ProjectWizardGui()
            self.wizard.done = True

        if args.project:
            if args.project.endswith(".nes"):
                self.wizard = z2edit.ProjectWizardGui(args.project)
            else:
                self.load_project(args.project)

        if not Application._instance:
            Application._instance = self
        else:
            logger.error("An application instance already exists")

    @classmethod
    def get(cls):
        return cls._instance

    def project(self, name=None):
        for window in self.windows:
            if isinstance(window, z2edit.ProjectGui):
                if name is None or name == window.name:
                    return window.project
        return None

    def new_project(self):
        project = z2edit.Project(
            self.wizard.name, self.wizard.rom, self.wizard.config, self.wizard.fix
        )
        self.windows.append(z2edit.ProjectGui(project))
        self.wizard = None

    def load_project(self, filename):
        if filename is None:
            dlg = z2edit.FileDialog()
            dlg.add_filter("Z2 Project", ["z2e3"])
            dlg.add_filter("All", ["*"])
            filename = dlg.pick_file()
        if filename is not None:
            project = z2edit.Project.load(filename)
            gui = z2edit.ProjectGui(project)
            gui.filename = filename
            self.windows.append(gui)

    def demo_window(self):
        if self.show_demo_window:
            self.show_demo_window = gui.show_demo_window(self.show_demo_window)
        if self.show_metrics_window:
            self.show_metrics_window = gui.show_metrics_window(self.show_metrics_window)

    def menu_bar(self):
        gui.begin_main_menu_bar()

        if gui.begin_menu("File"):
            if gui.menu_item("New"):
                self.wizard = z2edit.ProjectWizardGui()
            if gui.menu_item("Open"):
                self.load_project(None)
            gui.separator()
            if gui.menu_item("Quit"):
                self.running = False
            gui.end_menu()

        if gui.begin_menu("Edit"):
            gui.menu_item("Cut")
            gui.menu_item("Copy")
            gui.menu_item("Paste")
            if gui.menu_item("Preferences"):
                self.preferences_gui.show()
            gui.end_menu()

        if gui.begin_menu("View"):
            if gui.menu_item("Demo Window"):
                self.show_demo_window = True
            if gui.menu_item("Metrics Window"):
                self.show_metrics_window = True
            gui.end_menu()

        gui.end_main_menu_bar()
        pass

    def run(self):
        self.inner = gui.Framework("Z2Edit", 1900, 900)
        self.inner.audio_init(48000, 1, 1024)
        self.inner.open_controller()

        self.inner.set_scale(self.args.dpi)
        self.inner.background = self.preferences.background
        self.inner.style = self.preferences.imgui_style
        self.preferences_gui = z2edit.AppPreferencesGui(self.preferences_file)

        while self.running:
            self._emulator()
            if ui := self.inner.prepare_frame():
                self.menu_bar()
                self.preferences_gui.draw(ui)
                self.demo_window()
                if self.wizard:
                    if self.wizard.draw(ui):
                        self.new_project()

                for window in self.windows:
                    window.draw(ui)

                self.inner.render_frame()
                self.windows = [w for w in self.windows if not w.wants_dispose]
            else:
                self.running = False
        self.inner = None

    def emulate(self, args, rom=None):
        # We parse args and stash in an instance variable so we can spawn
        # the emulator from the interactive commandline thread as well as
        # from the gui.
        self.emulator_args = emulator_parser.parse_args(args)
        if rom:
            if self.emulator_args.rom:
                logger.error("Overriding %s with %s", self.emulator_args.rom, rom)
            self.emulator_args.rom = rom

    def _emulator(self):
        if rom := self.emulator_args.rom:
            try:
                self.emulator_args.rom = None
                if isinstance(rom, str):
                    rom = nes.NesFile.load(rom)
                emu = Emulator(rom)
                for p in self.emulator_args.plugin:
                    emu.load_plugin(p)
                self.windows.append(emu)
            except Exception as e:
                logger.error("Error creating emulator: %s", e)

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
        "--new",
        action="store_true",
        help="Start a new project",
    )

    p.add_argument(
        "--log",
        type=str,
        default="info",
        help="Logging level",
    )
    p.add_argument(
        "--preferences",
        type=str,
        default="preferences.json",
        help="Preferences file (relative to $XDG_CONFIG_HOME/z2edit)",
    )
    p.add_argument(
        "project",
        metavar="PROJECT",
        type=str,
        nargs="?",
        help="Project file",
    )

    args = p.parse_args()
    log = args.log.upper()
    logging.getLogger().setLevel(LOG_LEVELS.get(log, log))

    instdir = os.path.dirname(sys.argv[0])
    if instdir.endswith("python/z2edit"):
        instdir = os.path.normpath(os.path.join(instdir, "../.."))
    gui.Directories.init("org", "CF207", "Z2Edit", instdir)
    dirs = gui.Directories.get()
    z2edit.Config.load(os.path.join(dirs.install, "config/vanilla/vanilla.json5"))

    a = Application(args, dirs)
    if args.interactive:
        # We start the interpreter on another thread because GUI resources
        # always should be created/destroyed on the main thread.
        Thread(target=a.interact).start()
    a.run()


if __name__ == "__main__":
    main()
