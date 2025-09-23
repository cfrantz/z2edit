######################################################################
# NES-as-midi-instrument plugin
#
######################################################################

import logging
import random
from pprint import pprint
from z2edit import gui

from .datatypes import *
from .util import DragHelper

logger = logging.getLogger(__name__)


def clamp(v, mn, mx):
    if v < mn:
        v = mn
    if v > mx:
        v = mx
    return v


class EnvelopeGraph(object):
    SOLID = 0xFF000000
    TRANS = 0x88000000
    BLACK = 0x00000000
    WHITE = 0x00FFFFFF
    GREEN = 0x0000FF00
    GREY = 0x00606060

    def __init__(
        self,
        name,
        env,
        dimensions,
        invert=False,
        fixed_scale=gui.Vec2(1, 16),
        zoom=4,
        y_scale_factor=0,
    ):
        self.name = name
        self.env = env
        self.data = list(env.points.items())
        self.dimensions = dimensions
        self.size = dimensions[1] - dimensions[0]  # + gui.Vec2(1, 1)
        self.invert = invert
        self._fixed_scale = fixed_scale
        self._zoom = zoom
        self._y_scale_factor = y_scale_factor
        self.drag_helper = DragHelper()

    @property
    def zoom(self):
        # Really only expecting that _zoom[1] could be None
        if not self._y_scale_factor:
            z = gui.Vec2(self._zoom, 1.0)
        else:
            z = gui.Vec2(self._zoom, self._zoom * self._y_scale_factor)
        return z * self._fixed_scale

    @zoom.setter
    def zoom(self, val):
        self._zoom = val

    def _draw_background(self, dl, origin, cursor):
        size = self.size * self.zoom
        dl.add_rect_filled(
            origin,
            origin + size,
            self.SOLID | self.BLACK,
        )
        zoom = self.zoom
        prev = None
        for y in range(int(self.dimensions[0].y), int(self.dimensions[1].y)):
            yp = (y - self.dimensions[0].y) * zoom.y
            if prev is None or yp - prev >= 32:
                prev = yp
                if not self.invert:
                    yp = size.y - yp
                dl.add_line(
                    origin + gui.Vec2(0, yp),
                    origin + gui.Vec2(size.x, yp),
                    self.SOLID | self.GREY,
                )
                dl.add_line(
                    origin + gui.Vec2(0, yp),
                    origin + gui.Vec2(8, yp),
                    self.SOLID | self.WHITE,
                    3.0,
                )
                dl.add_line(
                    origin + gui.Vec2(size.x - 8, yp),
                    origin + gui.Vec2(size.x, yp),
                    self.SOLID | self.WHITE,
                    3.0,
                )
                dl.add_text(
                    origin + gui.Vec2(0, yp - 16), self.SOLID | self.WHITE, f"{y:3.0f}"
                )

        prev = None
        for x in range(int(self.dimensions[0].x), int(self.dimensions[1].x)):
            xp = (x - self.dimensions[0].x) * zoom.x
            if prev is None or xp - prev >= 48:
                prev = xp
                dl.add_line(
                    origin + gui.Vec2(xp, 0),
                    origin + gui.Vec2(xp, size.y),
                    self.SOLID | self.GREY,
                )
                dl.add_line(
                    origin + gui.Vec2(xp, 0),
                    origin + gui.Vec2(xp, 8),
                    self.SOLID | self.WHITE,
                    3.0,
                )
                dl.add_line(
                    origin + gui.Vec2(xp, size.y - 8),
                    origin + gui.Vec2(xp, size.y),
                    self.SOLID | self.WHITE,
                    3.0,
                )
                dl.add_text(
                    origin + gui.Vec2(xp - 24, 0), self.SOLID | self.WHITE, f"{x:3.0f}"
                )

        dl.add_rect(
            origin,
            origin + size,
            self.SOLID | self.WHITE,
        )

    def _draw_envelope(self, dl, origin, cursor):
        prev = gui.Vec2(0, 0)
        rnge = self.dimensions[1] - self.dimensions[0]
        xr = rnge.x
        yr = rnge.y
        size = self.size * self.zoom

        data = []
        changed = False
        to_delete = None

        for i, (x, y) in enumerate(self.data):
            xp = size.x * (x - self.dimensions[0].x) / xr
            yp = size.y * (y - self.dimensions[0].y) / yr
            if not self.invert:
                # The graph origin is in the bottom left corner.
                # Tranlate to screen coordinates
                yp = size.y - yp

            data.append((xp, yp))
            point = gui.Vec2(xp, yp)
            sz = gui.Vec2(8, 8)
            dl.add_rect(
                origin + point - sz, origin + point + sz, self.SOLID | self.GREEN
            )
            gui.set_cursor_pos(cursor + point - sz)
            gui.invisible_button(str(i), sz * 2.0)
            if gui.begin_popup_context_item(str(i)):
                if gui.menu_item("Delete"):
                    to_delete = i
                gui.end_popup()

            if gui.is_item_hovered():
                gui.set_tooltip(f"Frame: {x}\nTime: {x/60:.03f}\nValue: {y}")

            if gui.is_item_active():
                if gui.is_mouse_dragging(gui.MouseButton.LEFT):
                    mp = gui.get_mouse_pos() - origin
                    self.drag_helper.start(i)
                    self.drag_helper.position(i, mp)
                    dl.add_rect(
                        origin + mp - sz, origin + mp + sz, self.TRANS | self.WHITE
                    )
                    if not self.invert:
                        mp.y = size.y - mp.y
                    mp = (mp / self.zoom) + self.dimensions[0]
                    x = clamp(mp.x + 0.5, self.dimensions[0].x, self.dimensions[1].x)
                    y = clamp(mp.y + 0.5, self.dimensions[0].y, self.dimensions[1].y)
                    self.data[i] = (int(x), int(y))
            else:
                if self.drag_helper.finalize(i) is not None:
                    changed = True

        for i, (x, y) in enumerate(sorted(data)):
            point = gui.Vec2(x, y)
            if i != 0:
                if self.env.missing_value == MissingValue.INTERPOLATE:
                    dl.add_line(
                        origin + prev, origin + point, self.SOLID | self.GREEN, 3.0
                    )
                else:
                    nxt = gui.Vec2(x, prev.y)
                    dl.add_line(
                        origin + prev, origin + nxt, self.SOLID | self.GREEN, 3.0
                    )
                    dl.add_line(
                        origin + nxt, origin + point, self.SOLID | self.GREEN, 3.0
                    )
            prev = point

        if to_delete is not None:
            self.data.pop(to_delete)
            changed = True

        if gui.is_window_focused() and gui.is_mouse_double_clicked(
            gui.MouseButton.LEFT
        ):
            mp = gui.get_mouse_pos() - origin
            if not self.invert:
                mp.y = size.y - mp.y
            mp = (mp / self.zoom) + self.dimensions[0]
            x = clamp(mp.x + 0.5, self.dimensions[0].x, self.dimensions[1].x)
            y = clamp(mp.y + 0.5, self.dimensions[0].y, self.dimensions[1].y)
            self.data.append((int(x), int(y)))
            changed = True

        if changed:
            self.data = sorted(self.data)
            self.env.points.clear()
            self.env.points.update(self.data)

    def _draw_active(self, dl, origin, cursor, active_values):
        if values := active_values.get(self.name):
            state, frame, value = values
            if state == EnvelopeState.OFF:
                return

            rnge = self.dimensions[1] - self.dimensions[0]
            xr = rnge.x
            yr = rnge.y
            size = self.size * self.zoom
            xp = size.x * (frame - self.dimensions[0].x) / xr
            yp = size.y * (value - self.dimensions[0].y) / yr
            if not self.invert:
                yp = size.y - yp

            point = gui.Vec2(xp, yp)
            sz = gui.Vec2(8, 8)
            dl.add_rect_filled(
                origin + point - sz, origin + point + sz, self.SOLID | self.WHITE
            )

    def draw_properties(self):
        loop = self.env.loop if self.env.loop is not None else -1
        (changed, loop) = gui.input_int("Loop", loop, 1, 1)
        if changed:
            self.env.loop = loop

        release = self.env.release if self.env.release is not None else -1
        (changed, release) = gui.input_int("Release", release, 1, 1)
        if changed:
            self.env.release = release

        values = list(MissingValue.__members__.values())
        index = values.index(self.env.missing_value)
        (changed, mv) = gui.combo("Missing", index, values)
        if changed:
            self.env.missing_value = values[mv]

        (changed, zoom) = gui.input_float("Zoom", self._zoom, 1.0, 4.0, "%.02f")
        if changed:
            self.zoom = clamp(zoom, 1.0, 100.0)

    def draw(self, active_values):
        gui.set_next_window_content_size(self.size * self.zoom)
        gui.begin_child(
            self.name,
            gui.Vec2(0, 290),
            gui.ChildFlags.NONE,
            gui.WindowFlags.ALWAYS_VERTICAL_SCROLLBAR
            | gui.WindowFlags.ALWAYS_HORIZONTAL_SCROLLBAR
            | 0,
        )  # gui.NO_SCROLL_WITH_MOUSE)

        origin = gui.get_cursor_screen_pos()
        cursor = gui.get_cursor_pos()
        dl = gui.get_window_draw_list()

        self._draw_background(dl, origin, cursor)
        self._draw_envelope(dl, origin, cursor)
        self._draw_active(dl, origin, cursor, active_values)
        gui.end_child()


class InstrumentEditor(object):

    def __init__(self, midi):
        self.midi = midi
        self._visible = True
        self.selindex = 0
        self.instrument = self.midi.config.instrument[self.selindex]
        self.window_id = random.randint(0, 0xFFFFFFFF)
        self.nes2a03_setup()

    @property
    def visible(self):
        return self._visible

    @visible.setter
    def visible(self, value):
        if value != self._visible:
            self._visible = value

    def _instrument_selector(self):
        instruments = [i.name for i in self.midi.config.instrument]
        self.selindex = clamp(self.selindex, 0, len(instruments) - 1)
        (changed, index) = gui.combo("Instrument", self.selindex, instruments)
        if changed:
            self.selindex = index
            self.instrument = self.midi.config.instrument[self.selindex]
            self.nes2a03_setup()
        return changed

    def nes2a03_setup(self):
        instrument = self.instrument
        k = InstrumentKind.NES2A03
        if not instrument.volume:
            instrument.volume = Envelope(k, points={0: 15, 1: 0}, loop=0, release=1)
        if not instrument.arpeggio:
            instrument.arpeggio = Envelope(k, points={0: 0}, loop=-1, release=-1)
        if not instrument.pitch:
            instrument.pitch = Envelope(k, points={0: 0}, loop=-1, release=-1)
        if not instrument.duty:
            instrument.duty = Envelope(k, points={0: 2}, loop=-1, release=-1)

        self.envelopes = {
            "Volume": EnvelopeGraph(
                "volume",
                instrument.volume,
                dimensions=(gui.Vec2(0, 0), gui.Vec2(255, 15)),
                fixed_scale=gui.Vec2(1, 16),
            ),
            "Arpeggio": EnvelopeGraph(
                "arpeggio",
                instrument.arpeggio,
                dimensions=(gui.Vec2(0, -24), gui.Vec2(255, 24)),
                fixed_scale=gui.Vec2(1, 5),
                y_scale_factor=1 / 10,
            ),
            "Pitch": EnvelopeGraph(
                "pitch",
                instrument.pitch,
                dimensions=(gui.Vec2(0, -128), gui.Vec2(255, 127)),
                fixed_scale=gui.Vec2(1, 1),
                y_scale_factor=1 / 4,
            ),
            "Duty": EnvelopeGraph(
                "duty",
                instrument.duty,
                dimensions=(gui.Vec2(0, 0), gui.Vec2(255, 3)),
                fixed_scale=gui.Vec2(1, 64),
            ),
        }

    def nes2a03_instrument(self):
        instrument = self.instrument
        if gui.begin_table(
            "properties", 2, gui.TableFlags.ROW_BG | gui.TableFlags.BORDERS
        ):
            gui.table_setup_column("Property", gui.TableColumnFlags.WIDTH_FIXED, 300.0)
            gui.table_setup_column("Value", gui.TableColumnFlags.WIDTH_STRETCH)

            gui.table_next_row()
            gui.table_next_column()
            gui.text("Name")
            gui.table_next_column()
            gui.push_item_width(-1.0)
            (changed, name) = gui.input_text("##name", instrument.name, 80)
            gui.pop_item_width()
            if changed:
                # TODO: error if the new name already exists.
                instrument.name = name

            for name, env in self.envelopes.items():
                gui.push_id(name)
                gui.table_next_row()
                gui.table_next_column()
                gui.text(f"{name} Envelope")
                env.draw_properties()
                gui.table_next_column()
                env.draw(self.midi.active_values.get(instrument.name, {}))
                gui.pop_id()

            gui.end_table()

    def draw(self):
        if not self.visible:
            return

        (_, self.visible) = gui.begin(
            f"Instrument Editor##{self.window_id}", self.visible
        )
        self._instrument_selector()
        gui.separator()
        self.nes2a03_instrument()
        gui.end()
