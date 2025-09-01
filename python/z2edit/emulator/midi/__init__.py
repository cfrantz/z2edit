######################################################################
# NES-as-midi-instrument plugin
#
######################################################################

import logging
import mido

from z2edit import gui
from z2edit.emulator import plugin

from .channel import MidiChannel
from .apu import Apu
from .apu import VoiceType


logger = logging.getLogger(__name__)


class InputSelector(object):

    def __init__(self, midi):
        self.midi = midi
        self._visible = False

    @property
    def visible(self):
        return self._visible

    @visible.setter
    def visible(self, value):
        if value != self._visible:
            self._visible = value

    def draw(self):
        if not self.visible:
            return

        (_, self.visible) = gui.begin("Midi Input", self.visible)
        (changed, self.midi.port_index) = gui.combo(
            "Input Port", self.midi.port_index, self.midi.inputs, 8
        )
        if changed:
            self.midi.open_input(self.midi.port_index)
        gui.end()


class Midi(plugin.Plugin):

    def __init__(self, emulator, args):
        super().__init__(emulator)
        self.args = args
        self.port = None
        self.port_index = 0
        self.refresh_inputs()
        if name := self.args.get("port"):
            for i, input in enumerate(self.inputs):
                if name in input:
                    self.port_index = i
                    break

        self.open_input(self.port_index)
        self.input_selector = InputSelector(self)
        self.apu = Apu(emulator)
        self.channel = [
            MidiChannel(
                [
                    VoiceType.PULSE0,
                    VoiceType.PULSE1,
                    VoiceType.MMC5_PULSE0,
                    VoiceType.MMC5_PULSE1,
                ]
            )
        ]

    def menu_bar(self):
        """Hook into the menubar and some menus."""
        if gui.begin_menu("Midi"):
            if gui.menu_item("Input", ""):
                self.input_selector.visible = True
            gui.end_menu()

    def refresh_inputs(self):
        inputs = ["NES Emulator"]
        try:
            inputs.extend(mido.backend.get_input_names())
        except Exception as e:
            logger.error("Error scanning midi ports: %s", e)
        self.inputs = inputs

    def open_input(self, index):
        if self.port:
            self.port.close()
            self.port = None
        try:
            logging.info("Opening port [%d: %s]", index, self.inputs[index])
            v = index == 0
            self.port = mido.backend.open_input(self.inputs[index], virtual=v)
        except Exception as e:
            logger.error(
                "Error opening port [%d: %s]: %s", index, self.inputs[index], e
            )

    def run_pre_frame(self):
        if self.port is None:
            return

        for message in self.port.iter_pending():
            logger.info("Midi message: %s", message)
            self.channel[message.channel].process(message)

        for channel in self.channel:
            channel.tick(self.apu)

    def draw(self):
        self.input_selector.draw()


def create(emulator, args):
    arg = {}
    for a in args:
        k, v = a.split("=", 1)
        arg[k] = v
    return Midi(emulator, arg)
