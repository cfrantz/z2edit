######################################################################
# Hitboxes for Link, Enemies and Projectiles in Zelda 2
######################################################################
from z2edit import gui
from z2edit import Address


class EnemyHitbox(object):
    SIZETABLE = 0xE8FA
    HITBOX = 0x60FF0000
    SOLID = 0xFF000000
    WHITE = 0xFFFFFFFF
    TRANSPARENT = gui.Vec4(0, 0, 0, 0)
    ENEMY_LIST_SIZE = 36
    ENEMY_LIST_RAM = 0x6D00

    def enemy_list_size(self, enemies=36, total_size=0x2A1):
        offset = 0
        table = {}
        # fmt: off
        table['projectile_init']    = (offset, 12*2);      offset += 12*2
        table['projectile_table1']  = (offset, 9);         offset += 9
        table['enemy_hp']           = (offset, enemies);   offset += enemies
        table['enemy_init']         = (offset, enemies*2); offset += enemies * 2
        table['enemy_ai']           = (offset, enemies*2); offset += enemies * 2
        table['enemy_xp']           = (offset, enemies);   offset += enemies
        table['enemy_vuln']         = (offset, enemies);   offset += enemies
        table['enemy_size']         = (offset, enemies);   offset += enemies
        table['enemy_misc']         = (offset, enemies);   offset += enemies
        table['enemy_display']      = (offset, enemies*2); offset += enemies * 2
        table['projectile_table2']  = (offset, 9);         offset += 9
        table['projectile_table3']  = (offset, 12);        offset += 12
        table['projectile_display'] = (offset, 9*2);       offset += 9*2
        table['sprite_table']       = (offset, 0)
        # fmt: on
        self.sizecodes = self.ENEMY_LIST_RAM + table["enemy_size"][0]

    def __init__(self, emulator, index):
        self.emulator = emulator
        self.index = index
        self.name = "enemy%d" % index
        self.hb = gui.Vec2(0, 0)
        self.hbsz = gui.Vec2(0, 0)
        self.dragging = False
        self.locked = False
        self.exists = False
        self.emulator.nes.set_read_callback(0x2A + self.index, self.mem_cb)
        self.emulator.nes.set_read_callback(0x4E + self.index, self.mem_cb)
        self.emulator.nes.set_read_callback(0x3C + self.index, self.mem_cb)
        list_size = self.emulator.nes.read(0xFFF8)
        if list_size == 1:
            list_size = self.ENEMY_LIST_SIZE
        self.enemy_list_size(list_size)

    def mem_cb(self, cpu, addr, val):
        """Trap CPU memory reads so we can place enemies arbitrarily."""
        if self.exists and (self.dragging or self.locked):
            if addr == 0x2A + self.index:
                val = int(self.ypos)
            elif addr == 0x4E + self.index:
                val = int(self.xpos) & 0xFF
            elif addr == 0x3C + self.index:
                val = int(self.xpos) >> 8
        return val

    def update(self):
        """Read enemy or projectile data out of NES memory."""
        nes = self.emulator.nes
        # Game State 0x0b is sideview mode.  If we aren't in sideview mode,
        # just exit.
        if nes[0x736] != 0x0B:
            self.exists = False
            self.dragging = False
            self.locked = False
            return

        # Screen scroll position.
        scroll = nes.read_u16(0x72C, 0x72A)
        if self.dragging or self.locked:
            # If we're moving an enemy, write the desired position
            nes[0x2A + self.index] = int(self.ypos)
            nes.write_u16(0x4E + self.index, 0x3C + self.index, int(self.xpos))
        else:
            # Otherwise read where the game says the enemy is.
            self.ypos = nes[0x2A + self.index]
            self.xpos = nes.read_u16(0x4E + self.index, 0x3C + self.index)

        self.xscr = self.xpos - scroll
        if self.xscr < 0 or self.xscr > 255:
            self.xscr = -1
        # Get the enemy id or projectile id
        if self.index < 6:
            self.exists = nes[0xB6 + self.index]
            self.enemyid = nes[0xA1 + self.index]
        else:
            self.enemyid = nes[0x87 + self.index - 6]
            self.exists = nes[0x87 + self.index - 6]
            if self.exists > 15:
                self.exists = 0
        self.entype = nes[self.sizecodes + self.enemyid]

        # Compute the hitbox.
        ofs = (self.entype * 4) & 0xFF
        self.hb.x = self.xscr + nes.read_i8(self.SIZETABLE + ofs + 0)
        self.hb.y = self.ypos + nes.read_i8(self.SIZETABLE + ofs + 2)
        self.hbsz.x = nes[self.SIZETABLE + ofs + 1]
        self.hbsz.y = nes[self.SIZETABLE + ofs + 3]

    def draw_image(self, origin):
        if self.exists and self.xscr != -1:
            scale = gui.Vec2(
                self.emulator.scale * self.emulator.aspect, self.emulator.scale
            )
            dl = gui.get_window_draw_list()
            dl.add_rect_filled(
                origin + self.hb * scale,
                origin + (self.hb + self.hbsz) * scale,
                self.HITBOX,
            )
            dl.add_rect(
                origin + self.hb * scale,
                origin + (self.hb + self.hbsz) * scale,
                self.SOLID | self.HITBOX,
            )
            dl.add_text(origin + self.hb * scale, self.WHITE, str(self.index + 1))

            gui.set_cursor_screen_pos(origin + self.hb * scale)
            gui.invisible_button(self.name, self.hbsz * scale)
            if gui.is_item_clicked(gui.MouseButton.RIGHT):
                self.locked = not self.locked
            if gui.is_item_active():
                self.dragging = True
                if gui.is_mouse_dragging(gui.MouseButton.LEFT):
                    delta = gui.get_io().mouse_delta
                    delta /= scale
                    self.xpos += delta.x
                    self.ypos += delta.y
            else:
                self.dragging = False


class LinkHitbox(object):
    SIZETABLE = 0xE8FA
    HITBOX = 0x60FF0000
    SHIELD = 0x6000FF00
    SWORD = 0x600000FF
    SOLID = 0xFF000000

    def __init__(self, emulator):
        self.emulator = emulator
        self.name = "link"
        self.hb = gui.Vec2(0, 0)
        self.hbsz = gui.Vec2(0, 0)
        self.sh = gui.Vec2(0, 0)
        self.shsz = gui.Vec2(0, 0)
        self.sw = gui.Vec2(0, 0)
        self.swsz = gui.Vec2(0, 0)
        self.dragging = False
        self.locked = False
        self.exists = False

    def update(self):
        nes = self.emulator.nes
        if nes[0x736] != 0x0B:
            self.exists = False
            self.dragging = False
            self.locked = False
            return

        self.exists = True
        scroll = nes.read_u16(0x72C, 0x72A)
        if self.dragging or self.locked:
            nes[0x29] = int(self.ypos)
            nes.write_u16(0x4D, 0x3B, int(self.xpos))
        else:
            self.ypos = nes[0x29]
            self.xpos = nes.read_u16(0x4D, 0x3B)

        self.xscr = self.xpos - scroll
        if self.xscr < 0 or self.xscr > 255:
            self.xscr = -1
        self.controller = nes[0x80]
        self.standing = nes[0x17]
        self.facing = nes[0x9F] - 1
        self.swordx = nes[0x47E]
        self.swordy = nes[0x480]

        # Link's hitbox
        self.hb.x = self.xscr + 9
        self.hb.y = self.ypos + nes[0xE971 + self.standing]
        self.hbsz.x = 13
        self.hbsz.y = nes[0xE973 + self.standing]

        # Compute shield defense box
        self.sh.x = self.xscr + ((8 + 14) if self.facing == 0 else (8 - 1))
        self.sh.y = self.ypos + (2 if self.standing else 17)
        self.shsz.x = 5
        self.shsz.y = 12

        # Compute sword attack box
        self.sw.x = self.swordx + (-8 if self.swordx > self.xscr else 2)
        self.sw.y = self.swordy + (0 if self.controller == 9 else 7)
        self.swsz.x = 14
        self.swsz.y = 3

    def draw_image(self, origin):
        if not self.exists:
            return

        scale = gui.Vec2(
            self.emulator.scale * self.emulator.aspect, self.emulator.scale
        )
        dl = gui.get_window_draw_list()
        dl.add_rect_filled(
            origin + self.hb * scale,
            origin + (self.hb + self.hbsz) * scale,
            self.HITBOX,
        )
        dl.add_rect(
            origin + self.hb * scale,
            origin + (self.hb + self.hbsz) * scale,
            self.SOLID | self.HITBOX,
        )
        dl.add_rect_filled(
            origin + self.sh * scale,
            origin + (self.sh + self.shsz) * scale,
            self.SHIELD,
        )
        dl.add_rect(
            origin + self.sh * scale,
            origin + (self.sh + self.shsz) * scale,
            self.SOLID | self.SHIELD,
        )
        if self.swordy != 0xF8:
            dl.add_rect_filled(
                origin + self.sw * scale,
                origin + (self.sw + self.swsz) * scale,
                self.SWORD,
            )
            dl.add_rect(
                origin + self.sw * scale,
                origin + (self.sw + self.swsz) * scale,
                self.SOLID | self.SWORD,
            )

        gui.set_cursor_screen_pos(origin + self.hb * scale)
        if gui.invisible_button(self.name, self.hbsz * scale):
            # We don't want to allow locking link
            # self.locked = not self.locked
            pass
        if gui.is_item_active():
            self.dragging = True
            if gui.is_mouse_dragging(gui.MouseButton.LEFT):
                delta = gui.get_io().mouse_delta / scale
                self.xpos += delta.x
                self.ypos += delta.y
        else:
            self.dragging = False
