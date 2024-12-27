import argparse
import IPython
from threading import Thread

import z2edit
from z2edit import gui


class Application(object):
    def __init__(self):
        self.running = True
        self.inner = None
        self.show_style_editor = False
        self.show_demo_window = False

    def extra_window(self, ui):
        gui.begin("Python Window")
        gui.text("Hello from Python!")
        gui.end()

    def style_editor(self):
        if not self.show_style_editor:
            return
        (window, self.show_style_editor) = gui.begin(
            "Style Editor", self.show_style_editor
        )
        if window:
            gui.show_style_editor(gui.get_style())
        gui.end()

    def demo_window(self):
        if not self.show_demo_window:
            return
        self.show_demo_window = gui.show_demo_window(self.show_demo_window)

    def menu_bar(self):
        gui.begin_main_menu_bar()

        if gui.begin_menu("File"):
            gui.menu_item("New")
            gui.menu_item("Open")
            gui.menu_item("Save")
            gui.end_menu()

        if gui.begin_menu("Edit"):
            gui.menu_item("Cut")
            gui.menu_item("Copy")
            gui.menu_item("Paste")
            if gui.menu_item("Style Preferences"):
                self.show_style_editor = True
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
        while self.running:
            if ui := self.inner.prepare_frame():
                self.menu_bar()
                self.style_editor()
                self.demo_window()
                self.extra_window(ui)
                self.inner.render_frame()
            else:
                self.running = False
        self.inner = None


def main():
    p = argparse.ArgumentParser(prog="z2edit", description="Zelda2 Editor")
    p.add_argument(
        "--interactive",
        "-i",
        action="store_true",
        help="Start an interactive Python shell",
    )
    args = p.parse_args()

    if args.interactive:
        a = Application()
        thread = Thread(target=a.run)
        thread.start()
        IPython.embed()
        a.running = False
    else:
        a = Application()
        a.run()


if __name__ == "__main__":
    main()
