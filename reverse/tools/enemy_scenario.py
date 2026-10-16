#!/usr/bin/env python3
"""Place enemies in a side view area and log what they do, frame by frame.

    enemy_scenario.py --movie MOVIE --frame F [--clear] \\
        [--spawn SLOT:TYPE:X:Y[:FACING] ...] [--link X:Y] [--press K:BUTTONS[:LEN]] \\
        [--event NAME=BANK:ADDR ...] [--log SLOTS] [--frames N] [--shot K ...]

Plays MOVIE up to movie frame F (or, with --at, starts a new game at an
area as play.py does, waiting for --until), then takes over: Link gets no
input (except --press, counted from the takeover) and is invincible.  At
the takeover:

  --clear     removes the area's enemies and enemy projectiles (they stay
              marked as spawned in the area's list, so they don't come
              back) and stops new enemies spawning from the list.
  --spawn     puts an enemy of TYPE in enemy SLOT (0-5) at X (16-bit, in
              pixels from the area's left edge) and Y, set up as
              Enemy_Spawn would (hit points from the type, X velocity 1/2
              pixel per frame toward Link, other state cleared) but with
              no list entry and no spawn routine.  FACING (1 right, 2
              left) overrides the direction.
  --link      moves Link (the screen does not scroll to follow).
  --poke      writes VAR=VALUE (a symbol, NAME+k, or hex address).

Then it runs --frames frames, printing one line per frame for each enemy
slot in --log (default: the spawned slots) as seen at the start of that
frame's Enemy_RunAi (k = frames since the takeover), and a line for each
hit of an --event address.  --projectiles adds a line per enemy projectile
in flight (objState+7 = its type), at the start of each frame.  --shot K saves build/snap/scenario-K.png.

Values are hex unless noted.  Velocities are 4.4 fixed point.
"""
import argparse
import os
import sys

import headless
from PIL import Image
from z2edit import Address

HERE = os.path.dirname(os.path.abspath(__file__))
FIELDS = [
    # name, symbol, offset of enemy 0 within the array, width
    ("X", "objX", 1, 4), ("Y", "objY", 1, 2), ("xv", "objXVelocity", 1, 2),
    ("yv", "objYVelocity", 1, 2), ("face", "objFacing", 1, 1), ("st", "objState", 1, 2),
    ("ai", "enemyAiState", 0, 2), ("dir", "enemyAiDir", 0, 2), ("var", "enemyAiVar", 0, 2),
    ("tmr", "timerFast", 3, 2), ("col", "objCollision", 1, 2), ("stun", "enemyStunTimer", 0, 2),
]


LOCATION_VARS = ["areaPrgBank", "currentRegion", "currentWorld", "townCode", "palaceCode",
                 "areaLocation", "areaId", "areaStartPage", "areaEntrySide", "prevRegion"]


class Done(Exception):
    pass


def resolve(sym, spec):
    name, _, k = spec.partition("+")
    try:
        base = sym.equates[name]
    except KeyError:
        base = int(name, 16)
    return base + (int(k, 0) if k else 0)


def parse_press(spec):
    k, _, rest = spec.partition(":")
    buttons, _, n = rest.partition(":")
    bits = {"A": 0x01, "B": 0x02, "SELECT": 0x04, "START": 0x08,
            "UP": 0x10, "DOWN": 0x20, "LEFT": 0x40, "RIGHT": 0x80}
    mask = 0
    for b in buttons.split("+"):
        mask |= bits[b.upper()]
    return int(k), mask, int(n or 2)


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--rom", default=os.path.join(HERE, "../build/zelda2.nes"))
    p.add_argument("--symbols", default=os.path.join(HERE, "../build/zelda2.sym.json"))
    p.add_argument("--movie", required=True, help="FM2 movie (FILE or FILE:OFFSET)")
    p.add_argument("--frame", type=int, help="movie frame to take over at")
    p.add_argument("--at", help="start a new game at an area (see play.py)")
    p.add_argument("--until", help="with --at: take over once VAR=VALUE[,...] hold")
    p.add_argument("--after", type=int, default=0, help="frames to wait after --until holds")
    p.add_argument("--clear", action="store_true")
    p.add_argument("--spawn", action="append", default=[])
    p.add_argument("--link")
    p.add_argument("--approach", help="before the takeover, walk Link right (invincible, with --clear applied) until his X >= this")
    p.add_argument("--poke", action="append", default=[])
    p.add_argument("--fill", action="append", default=[],
                   help="X0:X1:ROW[:SRC]: copy map row SRC (default $C0, solid) to row ROW from X0 to X1, "
                        "e.g. a row of air to dig out a floor; not drawn")
    p.add_argument("--press", action="append", default=[])
    p.add_argument("--event", action="append", default=[])
    p.add_argument("--log", help="enemy slots to log, e.g. 0,2")
    p.add_argument("--map", action="store_true", help="print the area's solid tiles at the takeover")
    p.add_argument("--projectiles", action="store_true", help="also log the enemy projectile slots in use, each frame")
    p.add_argument("--frames", type=int, default=600)
    p.add_argument("--shot", action="append", type=int, default=[])
    p.add_argument("--out", default=os.path.join(HERE, "../build/snap"))
    a = p.parse_args()
    if (a.frame is None) == (a.until is None):
        p.error("give exactly one of --frame and --until")

    emu = headless.emulator(a.rom)
    mv = headless.movie(emu, a.movie)
    sym = headless.symbols(emu, a.symbols)
    eq = sym.equates
    rd, wr = emu.read, emu.write

    if a.at:
        code = []
        for c in a.at.split(","):
            var, _, value = c.partition("=")
            addr = resolve(sym, var)
            code += [0xA9, int(value, 0), 0x8D, addr & 0xFF, addr >> 8]
        code.append(0x60)
        for k, b in enumerate(code):
            emu[Address.Prg(0, 0xAA3F + k)] = b
    conds = []
    if a.until:
        for c in a.until.split(","):
            var, _, value = c.partition("=")
            conds.append((resolve(sym, var), int(value, 0)))

    spawns = []
    for s in a.spawn:
        parts = s.split(":")
        slot, typ, x, y = int(parts[0]), int(parts[1], 16), int(parts[2], 16), int(parts[3], 16)
        facing = int(parts[4]) if len(parts) > 4 else None
        spawns.append((slot, typ, x, y, facing))
    log_slots = [int(s) for s in a.log.split(",")] if a.log else [s[0] for s in spawns]
    presses = [parse_press(s) for s in a.press]
    state = {"start": None, "matched": None}

    def k():
        return emu.frame - state["start"] if state["start"] is not None else None

    def fields(x):
        out = []
        for name, symbol, off, width in FIELDS:
            v = rd(eq[symbol] + off + x)
            if name == "X":
                v |= rd(eq["objXHi"] + 1 + x) << 8
            out.append(f"{name}={v:0{width}X}" if width > 1 else f"{name}={v}")
        return " ".join(out)

    def link():
        return f"link={rd(eq['objX']) | rd(eq['objXHi']) << 8:04X},{rd(eq['objY']):02X} fc={rd(eq['frameCounter']):02X}"

    def on_ai(cpu):
        if state["start"] is not None and cpu.x in log_slots:
            t = rd(eq["enemyType"] + cpu.x)
            print(f"{k():5} e{cpu.x} t={t:02X} {fields(cpu.x)} {link()}")
        return cpu

    sym.break_at("Enemy_RunAi", on_ai)
    for e in a.event:
        name, where = e.split("=")
        bank, addr = where.split(":")
        bank = int(bank)

        def on_event(cpu, name=name):
            if state["start"] is not None:
                print(f"{k():5} e{cpu.x} EVENT {name} {fields(cpu.x)} {link()}")
            return cpu

        emu.set_exec_callback(Address.Prg(-1 if bank == 7 else bank, int(addr, 16)), on_event)

    rom = open(a.rom, "rb").read()

    def print_map():
        # The tile map: 4 pages of 13 rows x 16 columns at $6000, $60D0,
        # $61A0, $6270 (Obj_GetBgTile).  Solid as AreaBank::IsSolidTile:
        # tile >= threshold[group - 1], group = bits 7-6 (group 0 reads
        # the byte 255 past the table).
        bank = rd(eq["areaPrgBank"])
        _, table = sym.labels[f"SolidTileThresholds_Bank{bank}"]
        base = 16 + bank * 0x4000 + table - 0x8000
        print("# map: '#' solid, '.' not; row Y is the map Y (a floor at row Y holds enemies at Y+$10)")
        print("#      " + "".join(f"{x:<16X}" for x in range(0, 0x400, 0x100)))
        print("#      " + "".join(f"{(x >> 4) & 0xF:X}" for x in range(0, 0x400, 0x10)))
        for row in range(13):
            line = ""
            for page in range(4):
                for col in range(16):
                    tile = rd(0x6000 + page * 0xD0 + row * 16 + col)
                    y = ((tile >> 6) - 1) & 0xFF
                    line += "#" if tile >= rom[base + y] else "."
            print(f"# {row * 16:02X}   {line}")

    def prepare():
        meter, cont = eq["statMeter"], eq["statContainers"]
        full = lambda cpu, ad, v: 32 * rd(cont + ad - meter) - 1  # noqa: E731
        emu.set_read_callback(meter, full)
        emu.set_read_callback(meter + 1, full)
        clear()

    def clear():
        if a.clear:
            for x in range(6):
                wr(eq["enemyActive"] + x, 0)
                wr(eq["objState"] + 7 + x, 0)
            wr(eq["ram_0759"], 1)  # Enemy_TrySpawn: no spawning from the list

    def take_over():
        clear()
        for slot, typ, x, y, facing in spawns:
            if facing is None:
                linkx = rd(eq["objX"]) | rd(eq["objXHi"]) << 8
                facing = 1 if linkx + 8 > x else 2
            wr(eq["enemyActive"] + slot, 1)
            wr(eq["enemyType"] + slot, typ)
            wr(eq["enemySramIndex"] + slot, 0xFF)  # no list entry
            wr(eq["objX"] + 1 + slot, x & 0xFF)
            wr(eq["objXHi"] + 1 + slot, x >> 8)
            wr(eq["objY"] + 1 + slot, y)
            wr(eq["objYHi"] + 1 + slot, 1)
            wr(eq["objFacing"] + 1 + slot, facing)
            wr(eq["objXVelocity"] + 1 + slot, 0x08 if facing == 1 else 0xF8)
            wr(eq["enemyHp"] + slot, rd(eq["sramEnemyHp"] + typ))
            for name in ("enemyStunTimer", "enemyAiState", "enemyKnockBack", "enemyAiVar", "ram_04A0", "enemyAiDir"):
                wr(eq[name] + slot, 0)
            for name in ("objState", "objYVelocity", "objCollision"):
                wr(eq[name] + 1 + slot, 0)
            wr(eq["timerFast"] + 3 + slot, 0)
        if a.link:
            x, y = (int(v, 16) for v in a.link.split(":"))
            wr(eq["objX"], x & 0xFF)
            wr(eq["objXHi"], x >> 8)
            wr(eq["objY"], y)
        for f in a.fill:
            x0, x1, row, *src = (int(v, 16) for v in f.split(":"))
            src = src[0] if src else 0xC0
            for x in range(x0 & ~0xF, x1 + 1, 0x10):
                page = 0x6000 + (x >> 8) * 0xD0
                wr(page + row + ((x >> 4) & 0xF), rd(page + src + ((x >> 4) & 0xF)))
        for s in a.poke:
            var, _, value = s.partition("=")
            wr(resolve(sym, var), int(value, 0))
        print(f"# takeover at emulator frame {emu.frame}; scroll={rd(eq['scrollXHi']):02X}{rd(eq['scrollX']):02X} {link()}")
        where = ",".join(f"{v}=0x{rd(eq[v]):02X}" for v in LOCATION_VARS)
        print(f"# --at {where}")
        if a.map:
            print_map()

    def pre_frame(f):
        if state["start"] is None:
            ready = False
            if a.frame is not None:
                ready = f >= a.frame
            else:
                if state["matched"] is None and all(rd(ad) == v for ad, v in conds):
                    state["matched"] = f
                ready = state["matched"] is not None and f >= state["matched"] + a.after
            if not ready:
                mv.run_pre_frame()
                return
            if not state.get("prepared"):
                state["prepared"] = True
                prepare()
            if a.approach and (rd(eq["objX"]) | rd(eq["objXHi"]) << 8) < int(a.approach, 16):
                clear()
                emu.controller_value(0, 0x80)  # RIGHT
                emu.controller_value(1, 0)
                return
            state["start"] = emu.frame
            take_over()
        if a.projectiles:
            for i in range(6):
                st = rd(eq["objState"] + 7 + i)
                if st:
                    x = rd(eq["objX"] + 7 + i) | rd(eq["objXHi"] + 7 + i) << 8
                    print(f"{k():5} p{i} type={st:02X} X={x:04X} Y={rd(eq['objY'] + 7 + i):02X} "
                          f"xv={rd(eq['objXVelocity'] + 7 + i):02X} yv={rd(eq['objYVelocity'] + 7 + i):02X} "
                          f"face={rd(eq['objFacing'] + 7 + i)} tmr={rd(eq['projectileTimer'] + i):02X}")
        pad = 0
        for start, mask, n in presses:
            if start <= k() < start + n:
                pad |= mask
        emu.controller_value(0, pad)
        emu.controller_value(1, 0)
        if k() - 1 in a.shot:
            os.makedirs(a.out, exist_ok=True)
            path = os.path.join(a.out, f"scenario-{k() - 1}.png")
            bmp = path[:-4] + ".bmp"
            emu.screenshot(bmp)
            Image.open(bmp).convert("RGB").save(path)
            os.remove(bmp)
            print(f"{k() - 1:5} screenshot {path}")
        if k() > a.frames:
            raise Done()

    try:
        emu.run_frames(len(mv.frames) + 100000, pre_frame)
    except Done:
        return 0
    sys.exit("never took over")


if __name__ == "__main__":
    sys.exit(main())
