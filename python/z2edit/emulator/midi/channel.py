######################################################################
# Midi Channel implementation
######################################################################

import logging

from .apu import VoiceType
from .frequency import frequency


logger = logging.getLogger(__name__)


class MidiChannel(object):

    def __init__(self, voices):
        self.voices = voices
        self.keys = [None for _ in range(128)]
        self.n_pressed = 0

    def note_on(self, msg):
        self.n_pressed += 1
        self.keys[msg.note] = (self.n_pressed, msg.note, msg.velocity)

    def note_off(self, msg):
        self.keys[msg.note] = None

    def process(self, msg):
        if handler := getattr(self, msg.type, None):
            handler(msg)
        else:
            logger.info("Unhandled MIDI message: %s", msg)

    def tick(self, apu):
        keys = sorted((k for k in self.keys if k is not None), reverse=True)[
            : len(self.voices)
        ]
        keys.reverse()
        while len(keys) < len(self.voices):
            keys.append((0, 0, 0))
        for voice, (_, note, velocity) in zip(self.voices, keys):
            apu.write(voice, frequency=frequency(note), volume=(velocity >> 3), duty=2)
