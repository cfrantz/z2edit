######################################################################
# Implement cheats for Zelda 2
######################################################################
from z2edit import gui
from z2edit import Address


class Cheats(object):
    LEVELS = [("AtkLvl", 0), ("MagLvl", 1), ("LifeLvl", 2)]
    HEALTH = [("Hearts", 13), ("Magic", 12), ("Crystals", 29)]
    SPELLS = [
        ("Shield", 4),
        ("Jump", 5),
        ("Life", 6),
        ("Fairy", 7),
        ("Fire", 8),
        ("Reflect", 9),
        ("Spell", 10),
        ("Thunder", 11),
    ]
    ITEMS = [
        ("Candle", 14),
        ("Glove", 15),
        ("Raft", 16),
        ("Boots", 17),
        ("Flute", 18),
        ("Cross", 19),
        ("Hammer", 20),
        ("Key", 21),
    ]

    def __init__(self, emulator):
        self.emulator = emulator
        self.visible = False
        for x, _ in self.LEVELS + self.HEALTH:
            setattr(self, x, 0)
        for x, _ in self.SPELLS + self.ITEMS:
            setattr(self, x, False)
        self.downstab = False
        self.upstab = False
        self.invincible = False
        self.walk_anywhere = False
        # Trigger on memory reads to 0x773 and 0x774, which hold the amount
        # of magic and health Link has.
        self.emulator.nes.set_read_callback(0x773, self.invincible_cb)
        self.emulator.nes.set_read_callback(0x774, self.invincible_cb)

    def invincible_cb(self, addr, val):
        """Implement invicibility by trapping CPU reads to health values."""
        # On each read of 773/774, val will hold the value read from
        # memory.  We can return a different value if we want.
        if self.invincible:
            if addr == 0x773:
                val = (32 * self.Magic) - 1
            if addr == 0x774:
                val = (32 * self.Hearts) - 1
        return val

    def unpack(self):
        """Read health, spells and inventory out of NES memory."""
        mem = self.emulator.nes
        addr = 0x777
        for x, offset in self.LEVELS + self.HEALTH:
            setattr(self, x, mem[addr + offset])
        for x, offset in self.SPELLS + self.ITEMS:
            setattr(self, x, mem[addr + offset] != 0)

        self.downstab = (mem[addr + 31] & 0x10) != 0
        self.upstab = (mem[addr + 31] & 0x04) != 0
        self.walk_anywhere = mem[Address.Prg(0, 0x871E)] == 0

    def pack(self):
        """Write health, spells and inventory into NES memory."""
        mem = self.emulator.nes
        addr = 0x777
        for x, offset in self.LEVELS + self.HEALTH:
            mem[addr + offset] = getattr(self, x)
        for x, offset in self.SPELLS + self.ITEMS:
            mem[addr + offset] = getattr(self, x)

        tech = mem[addr + 31] & ~0x14
        if self.downstab:
            tech |= 0x10
        if self.upstab:
            tech |= 0x04
        mem[addr + 31] = tech
        mem[Address.Prg(0, 0x871E)] = 0 if self.walk_anywhere else 2

    def draw(self):
        if not self.visible:
            return

        self.unpack()
        (w, self.visible) = gui.begin("Cheats", self.visible)
        gui.push_item_width(200)
        for i, (x, _) in enumerate(self.LEVELS):
            if i:
                gui.same_line()
            (_, v) = gui.input_int(x, getattr(self, x), 1, 1)
            setattr(self, x, v)

        for i, (x, _) in enumerate(self.HEALTH):
            if i:
                gui.same_line()
            (_, v) = gui.input_int(x, getattr(self, x), 1, 1)
            setattr(self, x, v)
        gui.pop_item_width()

        gui.separator()
        gui.columns(2, "items", True)
        for x, _ in self.SPELLS:
            (_, v) = gui.checkbox(x, getattr(self, x))
            setattr(self, x, v)

        gui.next_column()
        for x, _ in self.ITEMS:
            (_, v) = gui.checkbox(x, getattr(self, x))
            setattr(self, x, v)

        gui.columns(1)
        gui.separator()
        gui.checkbox("Downstab", self.downstab)
        gui.same_line()
        gui.checkbox("Upstab", self.upstab)
        gui.same_line()
        if gui.button("Everything"):
            for x, _ in self.LEVELS + [("Hearts", 0), ("Magic", 0)]:
                setattr(self, x, 8)
            for x, _ in self.SPELLS + self.ITEMS:
                setattr(self, x, True)
            self.upstab = True
            self.downstab = True

        (_, self.invincible) = gui.checkbox("Invincible", self.invincible)
        (_, self.walk_anywhere) = gui.checkbox(
            "Walk anywhere on overworld", self.walk_anywhere
        )

        gui.end()
        self.pack()
