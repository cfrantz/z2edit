######################################################################
# Implement an execution monitor for Zelda 2
######################################################################
from z2edit import gui
from z2edit import Address
import collections


class Execution(object):
    KNOWN_IDLE = {
        Address.Prg(0, 0x9D59),
        Address.Prg(0, 0x9DB0),
        Address.Prg(0, 0xA7B1),
        Address.Prg(0, 0xAB73),
        Address.Prg(0, 0xB08F),
        Address.Prg(-1, 0xD4B2),
        # Central dispatch loop.  Not really idle, but close enough.
        Address.Prg(-1, 0xC010),
        Address.Prg(-1, 0xC013),
        Address.Prg(-1, 0xC015),
        Address.Prg(-1, 0xC017),
        Address.Prg(-1, 0xC019),
    }

    def __init__(self, emulator):
        self.emulator = emulator
        self._visible = False
        self.total = 1
        self.bank = collections.defaultdict(int)
        self.cpuaddr = collections.defaultdict(int)

    @property
    def visible(self):
        return self._visible

    @visible.setter
    def visible(self, value):
        if value != self._visible:
            self._visible = value
        self.emulator.nes.trace = value

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

        (_, self.visible) = gui.begin("Zelda2 Execution Statistics", self.visible)
        for i in range(-1, self.emulator.nes.rom_prg_banks):
            count = self.bank[i]
            frac = count / self.total
            gui.progress_bar(frac, gui.Vec2(400, 40), "")
            gui.same_line()
            if i == -1:
                i = "Idle"
            gui.text("%s: %d (%.2f)" % (i, count, 100.0 * frac))

        pairs = sorted(self.cpuaddr.items(), key=lambda x: x[1], reverse=True)
        for addr, count in pairs[0:20]:
            gui.text("%r: %d (%.2f)" % (addr, count, count / self.total * 100))
        gui.end()


class Bank7Tracker(object):
    def __init__(self, emulator):
        self.emulator = emulator
        self.report = collections.defaultdict(lambda: collections.defaultdict(int))
        for pc in range(0xC000, 0xFFFF):
            self.emulator.nes.set_exec_callback(Address.Prg(-1, pc), self.callback)

    def callback(self, cpu):
        a = self.emulator.nes.cpu_to_address(0x8000)
        self.report[cpu.pc][a.bank()] += 1
        return cpu
