######################################################################
# Symbol table plugin.
#
# Loads a symbol table produced by reverse/tools/symbols.py so that
# addresses can be referred to by name, e.g. from the interactive shell:
#
#   ./emulator -i -p z2edit.emulator.symbols:reverse/build/zelda2.sym.json ROM
#   >>> sym = emulator.plugins[0]
#   >>> sym.addr("loc_C07B")             # Address.Prg(-1, 0xC07B)
#   >>> sym.name(emulator.nes.cpu.pc)    # "loc_C07B+3"
#   >>> sym.break_at("loc_C07B", lambda cpu: print(cpu.cpustate()) or cpu)
#   >>> sym.watch_write("ram_0736", lambda cpu, a, v: print(sym.name(cpu.pc)) or v)
######################################################################
import bisect
import json
import logging

from z2edit import Address
from z2edit.emulator import plugin

logger = logging.getLogger(__name__)


class Symbols(plugin.Plugin):
    def __init__(self, emulator, args):
        super().__init__(emulator)
        self.labels = {}
        self.equates = {}
        self._by_bank = {}
        if args:
            self.load(args[0])

    def load(self, filename):
        with open(filename) as f:
            table = json.load(f)
        self.labels = {s["name"]: (s["bank"], s["addr"]) for s in table["labels"]}
        self.equates = {s["name"]: s["value"] for s in table["equates"]}
        self._by_bank = {}
        for name, (bank, addr) in self.labels.items():
            self._by_bank.setdefault(bank, []).append((addr, name))
        for v in self._by_bank.values():
            v.sort()
        logger.info("Loaded %d labels and %d equates", len(self.labels), len(self.equates))

    def _fixed_bank(self):
        return self.emulator.nes.rom_prg_banks - 1

    def addr(self, name):
        """Return the Address of a ROM label, or the value of an equate."""
        if name in self.labels:
            bank, addr = self.labels[name]
            if bank == self._fixed_bank():
                bank = -1
            return Address.Prg(bank, addr)
        return self.equates[name]

    def name(self, where, bank=None):
        """Return 'label+offset' for a ROM location.

        `where` may be an Address or a CPU address; for a CPU address in the
        switchable window, the currently mapped bank is used unless `bank`
        is given.
        """
        if isinstance(where, int):
            if where < 0x8000:
                return f"${where:04X}"
            if bank is None:
                if where >= 0xC000:
                    bank = self._fixed_bank()
                else:
                    bank = self.emulator.nes.cpu_to_address(where).bank()
            addr = where
        else:
            bank, addr = where.bank(), where.offset()
        if bank is None:
            return f"${addr:04X}"
        if bank < 0:
            bank = self._fixed_bank()
        syms = self._by_bank.get(bank, [])
        i = bisect.bisect_right(syms, (addr, "￿")) - 1
        if i < 0:
            return f"{bank}:${addr:04X}"
        a, n = syms[i]
        return n if a == addr else f"{n}+{addr - a}"

    def break_at(self, name, callback):
        """Call `callback(cpu) -> cpu` before executing the labeled instruction."""
        self.emulator.nes.set_exec_callback(self.addr(name), callback)

    def watch_read(self, name, callback):
        """Call `callback(cpu, address, value) -> value` on reads of a RAM symbol."""
        self.emulator.nes.set_read_callback(self.addr(name), callback)

    def watch_write(self, name, callback):
        """Call `callback(cpu, address, value) -> value` on writes to a RAM symbol."""
        self.emulator.nes.set_write_callback(self.addr(name), callback)


def create(emulator, args):
    return Symbols(emulator, args)
