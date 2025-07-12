######################################################################
# Simple cheats script for Zelda 2.
#
######################################################################

from z2edit import gui
from z2edit.emulator import plugin


class Zelda2(plugin.Plugin):

    def __init__(self, emulator):
        super().__init__(emulator)

    def menu_bar(self):
        """Hook into the menubar and some menus."""
        if gui.begin_menu("Goto"):
            if gui.menu_item("Palace 1", ""):
                self.z2goto(4, 0, 3, 4, 0, 52, 0)
            elif gui.menu_item("Palace 2", ""):
                self.z2goto(4, 0, 3, 4, 1, 53, 0xE)
            elif gui.menu_item("Palace 3", ""):
                self.z2goto(4, 0, 4, 5, 2, 54, 0)
            elif gui.menu_item("Palace 4", ""):
                self.z2goto(4, 1, 4, 8, 0, 52, 0xF)
            elif gui.menu_item("Palace 5", ""):
                self.z2goto(4, 2, 3, 8, 0, 52, 0x23)
            elif gui.menu_item("Palace 6", ""):
                self.z2goto(4, 2, 4, 8, 1, 53, 0x24)
            elif gui.menu_item("Palace 7", ""):
                self.z2goto(5, 2, 5, 9, 2, 54, 0)
            elif gui.menu_item("Rauru", ""):
                self.z2goto(3, 0, 1, 0, 0xF8, 45, 2)
            elif gui.menu_item("Ruto", ""):
                self.z2goto(3, 0, 1, 1, 0xF8, 47, 5)
            elif gui.menu_item("Saria", ""):
                self.z2goto(3, 0, 1, 2, 0xF8, 49, 8)
            elif gui.menu_item("Mido", ""):
                self.z2goto(3, 0, 1, 3, 0xF8, 51, 11)
            elif gui.menu_item("Nabooru", ""):
                self.z2goto(3, 2, 2, 4, 0xF8, 45, 14)
            elif gui.menu_item("Darunia", ""):
                self.z2goto(3, 2, 2, 5, 0xF8, 47, 17)
            elif gui.menu_item("New Kasuto", ""):
                self.z2goto(3, 2, 2, 6, 0xF8, 49, 20)
            elif gui.menu_item("Old Kasuto", ""):
                self.z2goto(3, 2, 2, 7, 0xF8, 51, 23)
            elif gui.menu_item("T-Bird Fight", ""):
                self.z2goto(5, 2, 5, 9, 2, 54, 53)
            elif gui.menu_item("Dark Link Fight", ""):
                self.z2goto(5, 2, 5, 9, 2, 54, 54)
            gui.end_menu()

    def z2goto(self, a, b, c, d, e, f, g):
        """Go to a sideview area in Zelda 2"""
        mem = self.emulator.nes
        mem[0x769] = a
        mem[0x706] = b
        mem[0x707] = c
        mem[0x56B] = d
        mem[0x56C] = e
        mem[0x748] = f
        mem[0x561] = g
        mem[0x736] = 0


def create(emulator):
    return Zelda2(emulator)
