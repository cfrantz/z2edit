######################################################################
# Implement a game state monitor for Zelda 2
######################################################################
from z2edit import gui
from z2edit import Address
import collections


class GameState(object):

    STATE_VARIABLES = {
        "Game State": 0x736,
        "Bank": 0x769,
        "World": 0x707,
        "Region": 0x706,
        "Previous Region": 0x70A,
        "Area": 0x561,
        "Connection": 0x748,
        "Page": 0x75C,
        "Town code": 0x56B,
        "Palace code": 0x56C,
        "Overworld Terrain": 0x563,
        "Overworld Direction": 0x562,
    }

    def __init__(self, emulator):
        self.emulator = emulator
        self._visible = False

    @property
    def visible(self):
        return self._visible

    @visible.setter
    def visible(self, value):
        if value != self._visible:
            self._visible = value

    def update(self):
        if not self.visible:
            return
        self.bank.clear()
        self.cpuaddr.clear()
        self.total = 1
        banks = self.emulator.nes.rom_prg_banks
        for addr, cycles in self.emulator.nes.tracebuf.items():
            bank = addr.bank() % banks
            if addr in self.KNOWN_IDLE:
                bank = -1
            else:
                self.cpuaddr[addr] += cycles
            self.bank[bank] += cycles
            self.total += cycles
        self.emulator.nes.tracebuf = {}

    def draw(self):
        if not self.visible:
            return

        (_, self.visible) = gui.begin("Zelda2 Game State", self.visible)

        self.draw_gamestate_table()

        gui.text("\nLink:")
        self.draw_link_table()

        gui.text("\nEnemy:")
        self.draw_enemy_table()
        gui.end()

    def draw_gamestate_table(self):
        nes = self.emulator.nes
        if gui.begin_table(
            "gamestate", 2, gui.TableFlags.ROW_BG | gui.TableFlags.BORDERS
        ):
            gui.table_setup_column("Property", gui.TableColumnFlags.WIDTH_FIXED, 300.0)
            gui.table_setup_column("Value", gui.TableColumnFlags.WIDTH_STRETCH)
            gui.table_headers_row()

            for name, location in self.STATE_VARIABLES.items():
                gui.table_next_row()
                gui.table_next_column()
                gui.text(f"{name} (0x{location:04x})")
                gui.table_next_column()
                value = nes[location]
                gui.text(f"0x{value:02x} ({value} dec)")

            gui.end_table()

    def draw_link_table(self):
        nes = self.emulator.nes
        if gui.begin_table("link", 2, gui.TableFlags.ROW_BG | gui.TableFlags.BORDERS):
            gui.table_setup_column("Property", gui.TableColumnFlags.WIDTH_FIXED, 300.0)
            gui.table_setup_column("Value", gui.TableColumnFlags.WIDTH_STRETCH)
            gui.table_headers_row()

            gui.table_next_row()
            gui.table_next_column()
            gui.text("X position / Speed")
            gui.table_next_column()
            xpos = nes.read_u16(0x4D, 0x3B)
            xspeed = nes[0x70]
            subpix = nes[0x3D6]
            gui.text(f"{xpos:04x}.{subpix:02x} / {xspeed:02x}")

            gui.table_next_row()
            gui.table_next_column()
            gui.text("Y position / Speed")
            gui.table_next_column()
            ypos = nes[0x29]
            yspeed = nes[0x57D]
            gui.text(f"{ypos:02x} / {yspeed:02x}")

            gui.table_next_row()
            gui.table_next_column()
            gui.text("Standing")
            gui.table_next_column()
            gui.text(f"{nes[0x17]:02x}")

            gui.table_next_row()
            gui.table_next_column()
            gui.text("Facing")
            gui.table_next_column()
            gui.text(f"{nes[0x9f]:02x}")

            gui.table_next_row()
            gui.table_next_column()
            gui.text("Control")
            gui.table_next_column()
            gui.text(f"{nes[0x80]:02x}")

            gui.table_next_row()
            gui.table_next_column()
            gui.text("Sword X")
            gui.table_next_column()
            gui.text(f"{nes[0x47e]:02x}")

            gui.table_next_row()
            gui.table_next_column()
            gui.text("Sword Y")
            gui.table_next_column()
            gui.text(f"{nes[0x480]:02x}")

            gui.table_next_row()
            gui.table_next_column()
            gui.text("Overworld X")
            gui.table_next_column()
            gui.text(f"{nes[0x74]:02x}")

            gui.table_next_row()
            gui.table_next_column()
            gui.text("Overworld Y")
            gui.table_next_column()
            gui.text(f"{nes[0x73]:02x}")

            gui.table_next_row()
            gui.table_next_column()
            gui.text("Terrain")
            gui.table_next_column()
            gui.text(f"{nes[0x563]:02x}")
            gui.end_table()

    def draw_enemy_table(self):
        nes = self.emulator.nes
        if gui.begin_table("enemy", 7, gui.TableFlags.ROW_BG | gui.TableFlags.BORDERS):
            gui.table_setup_column("Property", gui.TableColumnFlags.WIDTH_FIXED, 300.0)
            for i in range(6):
                gui.table_setup_column(f"{i+1}", gui.TableColumnFlags.WIDTH_STRETCH)
            gui.table_headers_row()

            gui.table_next_row()
            gui.table_next_column()
            gui.text("Enemy ID")
            for i in range(6):
                gui.table_next_column()
                gui.text(f"{nes[0xa1+i]:02x}")

            gui.table_next_row()
            gui.table_next_column()
            gui.text("Enemy Exists")
            for i in range(6):
                gui.table_next_column()
                gui.text(f"{nes[0xb6+i]:02x}")

            gui.table_next_row()
            gui.table_next_column()
            gui.text("Enemy HP")
            for i in range(6):
                gui.table_next_column()
                gui.text(f"{nes[0xc2+i]:02x}")

            gui.table_next_row()
            gui.table_next_column()
            gui.text("X position/Speed")
            for i in range(6):
                gui.table_next_column()
                xpos = nes.read_u16(0x4E + i, 0x3C + i)
                xspeed = nes[0x71 + i]
                subpix = nes[0x3D7 + i]
                gui.text(f"{xpos:04x}.{subpix:02x} / {xspeed:02x}")

            gui.table_next_row()
            gui.table_next_column()
            gui.text("Y position / Speed")
            for i in range(6):
                gui.table_next_column()
                ypos = nes[0x2A + i]
                yspeed = nes[0x57E + i]
                gui.text(f"{ypos:02x} / {yspeed:02x}")

            gui.table_next_row()
            gui.table_next_column()
            gui.text("Facing")
            for i in range(6):
                gui.table_next_column()
                gui.text(f"{nes[0x60+i]:02x}")

            gui.table_next_row()
            gui.table_next_column()
            gui.text("Stun timer")
            for i in range(6):
                gui.table_next_column()
                gui.text(f"{nes[0x40e+i]:02x}")

            gui.table_next_row()
            gui.table_next_column()
            gui.text("Future item ID")
            for i in range(6):
                gui.table_next_column()
                gui.text(f"{nes[0x48e+i]:02x}")

            gui.end_table()
