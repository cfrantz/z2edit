######################################################################
# Midi Channel implementation
######################################################################

import logging
from pprint import pprint

from .instrument import Instrument
from .datatypes import EnvelopeState, MidiConfig


logger = logging.getLogger(__name__)


class MidiChannel(object):

    def __init__(self, midi, voices=[], config=None, channel=None):
        self.midi = midi
        self.note_offset = 0
        self.instrument = None
        self.n_pressed = 0
        self.pad = {}
        if voices:
            self.voices = {v: [] for v in voices}
        if channel:
            self.note_offset = channel.note_offset
            self.voices = {v: [] for v in channel.voice}
            self.instrument = channel.instrument
            self.pad = channel.pad
        self.config = config if config else MidiConfig()
        self.keys = [
            Instrument(config=self.config.instrument.get(self.instrument))
            for _ in range(128)
        ]

    def choose_voice(self, inst):
        # If the voice is already playing, we don't have to do anything.
        for voice, playing in self.voices.items():
            if inst in playing:
                return

        rel_cand = None
        rel_time = self.n_pressed
        on_cand = None
        on_time = self.n_pressed

        for voice, playing in self.voices.items():
            if not playing:
                # If there is a free channel, assign that voice
                playing.append(inst)
                return
            # If there is no free channel, find the oldest relasing or playing
            # voice (in that order).
            playing = playing[-1]
            if playing.state == EnvelopeState.RELEASE and playing.seq < rel_time:
                rel_cand = voice
                rel_time = playing.seq
            if playing.state == EnvelopeState.ON and playing.seq < on_time:
                on_cand = voice
                on_time = playing.seq
        candidate = rel_cand or on_cand
        self.voices[candidate].append(inst)

    def note_on(self, msg):
        if msg.velocity == 0:
            return self.note_off(msg)

        self.n_pressed += 1
        note = msg.note + self.note_offset
        if trigger := self.pad.get(note, self.pad.get(0)):
            if trigger.instrument == "__skip__":
                return
            self.keys[note] = Instrument(
                config=self.config.instrument.get(trigger.instrument),
                timer=trigger.timer,
            )

        inst = self.keys[note]
        inst.note_on(note, msg.velocity, seq=self.n_pressed)
        voice = self.choose_voice(inst)

    def note_off(self, msg):
        note = msg.note + self.note_offset
        self.keys[note].note_off()

    def process(self, msg):
        if handler := getattr(self, msg.type, None):
            handler(msg)
        else:
            logger.info("Unhandled MIDI message: %s", msg)

    def tick(self, apu):
        for voice, inst in self.voices.items():
            if inst:
                apu.write(voice, **inst[-1].value)
                inst[-1].update_active_values(self.midi.active_values)
                state = EnvelopeState.RELEASE
                for i in inst:
                    i.tick()
                    if i.state >= state:
                        state = i.state
                    else:
                        inst.remove(i)
            else:
                apu.write(voice, volume=0)
