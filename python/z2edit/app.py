import argparse
import IPython
from threading import Thread
import logging
import sys
import os.path

import z2edit
from z2edit import gui

LOG_LEVELS = {
    "TRACE": 5,
}


class Application(object):
    def __init__(self, args, dirs):
        self.running = True
        self.inner = None
        self.show_style_editor = False
        self.show_demo_window = False
        self.windows = []
        self.dirs = dirs 
        self.preferences_file = os.path.join(self.dirs.config_dir, args.preferences)
        self.preferences = z2edit.AppPreferences()
        self.preferences.load(self.preferences_file)
        self.preferences_gui = None;
        self.wizard = None
        if args.new:
            self.wizard = z2edit.ProjectWizardGui()
            self.wizard.done = True

    def new_project(self):
        project = z2edit.Project(self.wizard.name, self.wizard.rom, self.wizard.config, self.wizard.fix)
        self.windows.append(z2edit.ProjectGui(project))
        self.wizard = None

    def load_project(self, filename):
        if filename is None:
            dlg = z2edit.FileDialog()
            dlg.add_filter("Z2 Project", ["z2prj"])
            dlg.add_filter("All", ["*"])
            filename = dlg.pick_file()
        if filename is not None:
            project = z2edit.Project.load(filename)
            self.windows.append(z2edit.ProjectGui(project))

    def demo_window(self):
        if not self.show_demo_window:
            return
        self.show_demo_window = gui.show_demo_window(self.show_demo_window)

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
            gui.end_menu()

        gui.end_main_menu_bar()
        pass

    def run(self):
        self.inner = gui.Framework("Z2Edit", 1280, 720)
        self.inner.set_scale(0.0)
        self.inner.background = self.preferences.background
        self.inner.style = self.preferences.imgui_style
        self.preferences_gui = z2edit.AppPreferencesGui(self.preferences_file)

        while self.running:
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
            else:
                self.running = False
        self.inner = None

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

    args = p.parse_args()
    log = args.log.upper()
    logging.getLogger().setLevel(LOG_LEVELS.get(log, log))

    instdir = os.path.dirname(sys.argv[0])
    if instdir.endswith("python/z2edit"):
        instdir = os.path.normpath(os.path.join(instdir, "../.."))
    z2edit.Directories.init(instdir)
    dirs = z2edit.Directories.get()
    z2edit.Config.load(os.path.join(dirs.install, "config/vanilla/vanilla.json5"))

    a = Application(args, dirs)
    if args.interactive:
        # We start the interpreter on another thread because GUI resources
        # always should be created/destroyed on the main thread.
        Thread(target=a.interact).start()
    a.run()


if __name__ == "__main__":
    main()
