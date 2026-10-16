#!/usr/bin/env python3
"""Decode the enemy definition tables of each enemy group.

    enemy_table.py [GROUP ...]

Each world's area bank holds a block of per-type tables at $9400 ($A900
for the second palace group) that Mode_LoadWorld copies to SRAM $6D00
(sramProjectilePtrs and following).  For each group this prints, per
enemy type: the name from z2edit's config, hit points, experience,
damage class, item drop group, palette, flags, hitbox, and the init
(sramEnemyInitPtrs), AI (sramEnemyAiPtrs) and draw (sramEnemyDrawPtrs)
routines by name; and the projectile routines and flags.

GROUP is one of: west east towns palace125 palace346 great (default all).
"""
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROM = os.path.join(HERE, "../build/zelda2.nes")
SYM = os.path.join(HERE, "../build/zelda2.sym.json")
CONFIG = os.path.join(HERE, "../../config/vanilla")
SRAM_BASE = 0x6D00

GROUPS = {
    "west": (1, 0x9400, "bank1.json5", "West Hyrule"),
    "east": (2, 0x9400, "bank2.json5", "East Hyrule"),
    "towns": (3, 0x9400, "bank3.json5", "Towns"),
    "palace125": (4, 0x9400, "bank4.json5", "Palaces 1/2/5"),
    "palace346": (4, 0xA900, "bank4.json5", "Palaces 3/4/6"),
    "great": (5, 0x9400, "bank5.json5", "Great Palace"),
}

# Offsets into the block (SRAM address - $6D00).
PROJ_PTRS = 0x00     # 11 words
PROJ_FLAGS = 0x17    # 10 bytes
HP = 0x21
INIT_PTRS = 0x45
AI_PTRS = 0x8D
ATTR = 0xD5
DAMAGE = 0xF9
HITBOX = 0x11D
FLAGS = 0x141
DRAW_PTRS = 0x165
COUNT = 36

# Experience points by class (Enemy_AwardExperience: dat_DDC0 low bytes,
# dat_DDDC high bytes).
EXP_LO = 0xDDC0
EXP_HI = 0xDDDC


# Enemy pages (docs/entities/enemies/NAME.md), by AI routine: --doc links
# the enemy's name to its page.
ENEMY_PAGES = {
    "Fairy_Ai": "fairy", "Item_Fall": "item", "LockedDoor_Ai": "locked-door", "Elevator_Ai": "elevator",
    "Myu_Ai": "myu", "Bot_Ai": "bot", "Bit_Ai": "bit", "Lowder_Ai": "lowder",
    "OctorokHopping_Ai": "octorok", "OctorokWalking_Ai": "octorok",
    "Moa_Ai": "moa", "Ache_Ai": "ache", "Deeler_Ai": "deeler",
    "BagoBagoGenerator_Ai": "bago-bago", "BagoBago_Ai": "bago-bago",
    "BubbleGenerator_Ai": "bubble-generator", "RockGenerator_Ai": "rock-generator",
    "Megmet_Ai": "megmet", "MobyGenerator_Ai": "moby", "Moby_Ai": "moby", "Geldarm_Ai": "geldarm",
    "DumbMoblinGenerator_Ai": "walking-moblin", "DumbMoblin_Ai": "walking-moblin",
    "Daira_Ai": "daira",
    "Tektite_Ai": "tektite", "Eye_Ai": "eye", "Leever_Ai": "leever", "Boon_Ai": "boon",
    "Basilisk_Ai": "basilisk", "Scorpion_Ai": "scorpion",
    "Lizalfos_Ai": "lizalfos", "OrangeLizalfos_Ai": "lizalfos", "RockThrower_Ai": "rock-thrower",
    "Bubble_Ai": "bubble", "FallingBlockGenerator_Ai": "falling-block", "FallingBlock_Ai": "falling-block",
    "CrystalSpot_Ai": "crystal", "Crystal_Ai": "crystal", "EnergyBallShooter_Ai": "energy-ball-shooter",
    "WolfHeadGenerator_Ai": "wolf-head", "WolfHead_Ai": "wolf-head",
}


def link_name(enemy, ai):
    page = ENEMY_PAGES.get(ai)
    if ai == "MoblinGoriya_Ai":  # shared by two pages
        page = "goriya" if "Gor" in enemy else "moblin"
    return f"[{enemy}](enemies/{page}.md)" if page else enemy


def config_names(fname, title):
    t = open(os.path.join(CONFIG, fname)).read()
    if title == "Palaces 3/4/6":
        # The second group in bank 4 follows the first.
        i = t.find('name: "Palaces 1/2/5"')
        i = t.find("offset: 35", i)
    else:
        i = t.find(f'name: "{title}"')
    names = {}
    for m in re.finditer(r'"(\d+)": \{ ?offset: (\d+), name: "([^"]+)"', t[i:]):
        k = int(m.group(1))
        if k in names:
            break
        names[k] = m.group(3)
        if k == 35:
            break
    return names


def main():
    rom = open(ROM, "rb").read()
    prg = rom[16:16 + 8 * 0x4000]
    labels = {}
    for s in json.load(open(SYM))["labels"]:
        if not s["name"].startswith("@"):
            labels.setdefault((s["bank"], s["addr"]), s["name"])

    def cpu(bank, addr, n=1):
        if addr >= 0xC000:
            off = 7 * 0x4000 + addr - 0xC000
        else:
            off = bank * 0x4000 + addr - 0x8000
        return prg[off:off + n]

    def name(bank, addr):
        b = 7 if addr >= 0xC000 else bank
        return labels.get((b, addr), f"${addr:04X}")

    doc = "--doc" in sys.argv
    which = [a for a in sys.argv[1:] if a != "--doc"] or list(GROUPS)
    for g in which:
        bank, base, fname, title = GROUPS[g]
        block = cpu(bank, base, 0x220)
        names = config_names(fname, title)
        word = lambda o: block[o] | block[o + 1] << 8
        if doc:
            print("| Type | Enemy | HP | Exp | Damage | Drop | Notes | AI |")
            print("|------|-------|----|-----|--------|------|-------|----|")
            for t in range(COUNT):
                ai = word(AI_PTRS + 2 * t)
                if ai == 0 and t not in names:
                    continue
                attr, dmg, fl = block[ATTR + t], block[DAMAGE + t], block[FLAGS + t]
                cls = attr & 0x0F
                exp = cpu(7, EXP_LO + cls)[0] | cpu(7, EXP_HI + cls)[0] << 8
                notes = []
                if attr & 0x10: notes.append("touch drains 20 exp" if t == 6 else "touch drains 10 exp")
                if attr & 0x20: notes.append("sword clinks, beam blocked")
                if dmg & 0x20: notes.append("blocks Link's projectiles")
                if dmg & 0x10: notes.append("immune to SPELL")
                if fl & 0x80: notes.append("immune to Thunder")
                elif fl & 0x20: notes.append("Thunder weak")
                if fl & 0x10: notes.append("sword passes through")
                if fl & 0x40: notes.append("comes back")
                drop = dmg >> 6
                print(f"| ${t:02X} | {link_name(names.get(t, '?'), name(bank, ai))} | {block[HP + t]} | {exp} | {dmg & 0x0F} |"
                      f" {drop or '-'} | {', '.join(notes)} | `{name(bank, ai)}` |")
            print()
            continue
        print(f"## {title} (bank {bank}, ${base:04X})\n")
        print("| Type | Name | HP | Exp | Attr | Damage | Flags | Box | Init | AI | Draw |")
        print("|------|------|----|-----|------|--------|-------|-----|------|----|------|")
        for t in range(COUNT):
            attr = block[ATTR + t]
            cls = attr & 0x0F
            exp = cpu(7, EXP_LO + cls)[0] | cpu(7, EXP_HI + cls)[0] << 8
            print(f"| ${t:02X} | {names.get(t, '?')} | {block[HP + t]} | {exp} | ${attr:02X} |"
                  f" ${block[DAMAGE + t]:02X} | ${block[FLAGS + t]:02X} | {block[HITBOX + t]} |"
                  f" {name(bank, word(INIT_PTRS + 2 * t))} | {name(bank, word(AI_PTRS + 2 * t))} |"
                  f" {name(bank, word(DRAW_PTRS + 2 * t))} |")
        print("\nProjectiles:\n")
        print("| Type | Routine | Flags |")
        print("|------|---------|-------|")
        for p in range(11):
            ptr = word(PROJ_PTRS + 2 * p)
            fl = block[PROJ_FLAGS + p + 1] if p < 10 else None  # indexed by type (p + 1)
            print(f"| {p + 1} | {name(bank, ptr)} | {'' if fl is None else f'${fl:02X}'} |")
        print()
    return 0


if __name__ == "__main__":
    sys.exit(main())
