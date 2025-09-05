######################################################################
# Envelope implementation
######################################################################

import logging

from .datatypes import InstrumentKind, EnvelopeState, MissingValue
from .frequency import frequency, timer_value

logger = logging.getLogger(__name__)


class Envelope(object):

    def __init__(
        self, points={}, loop=-1, release=-1, missing=MissingValue.LAST, config=None
    ):
        self.frame = 0
        self.loop = loop
        self.release = release
        self.points = sorted(points.items())
        self.missing_value = missing
        if config:
            self.loop = -1 if config.loop is None else config.loop
            self.release = -1 if config.release is None else config.release
            self.missing_value = config.missing_value
            self.points = sorted(config.points.items())
        self.state = EnvelopeState.OFF

    def note_on(self):
        self.frame = 0
        self.state = EnvelopeState.ON

    def note_off(self):
        self.state = EnvelopeState.RELEASE

    def tick(self):
        if self.state == EnvelopeState.OFF:
            pass
        elif self.state == EnvelopeState.ON:
            self.frame += 1
            if self.frame == self.release:
                # We've reached the release point, but aren't released yet.
                # Go to the loop point.
                self.frame = self.loop
            else:
                frame, value = self.points[-1]
                # We've reached the end; go to the loop point or last element
                # of the envelope until we get released.
                if self.frame > frame:
                    if self.loop >= 0 and self.release < 0:
                        self.frame = self.loop
                    else:
                        self.frame = frame
        elif self.state == EnvelopeState.RELEASE:
            self.frame += 1
        else:
            logger.error("Invalid state: %d", self.state)

    @property
    def active(self):
        return self.state != EnvelopeState.OFF

    @property
    def value(self):
        if self.state == EnvelopeState.OFF:
            return 0

        prev = (0, 0)
        frame = None
        for fr, val in self.points:
            if fr >= self.frame:
                frame = (fr, val)
                break
            prev = (fr, val)

        if frame is None:
            self.state = EnvelopeState.OFF
            return 0
        if frame[0] == self.frame:
            return frame[1]

        if self.missing_value == MissingValue.LAST:
            return prev[1]
        elif self.missing_value == MissingValue.INTERPOLATE:
            t = (self.frame - prev[0]) / (frame[0] - prev[0])
            return int((1 - t) * prev[1] + t * frame[1])
        else:
            logger.error("Invalid missing_value: %d", self.missing_value)


class Instrument(object):
    def __init__(self, config=None, timer=None):
        self.kind = InstrumentKind.NES2A03
        self.volume = Envelope(points={0: 15, 1: 0}, loop=0, release=1)
        self.arpeggio = Envelope(points={0: 0})
        self.pitch = Envelope(points={0: 0})
        self.duty = Envelope(points={0: 2})
        self.timer = timer
        if config:
            self.kind = config.kind
            if config.volume:
                self.volume = Envelope(config=config.volume)
            if config.arpeggio:
                self.arpeggio = Envelope(config=config.arpeggio)
            if config.pitch:
                self.pitch = Envelope(config=config.pitch)
            if config.duty:
                self.duty = Envelope(config=config.duty)
        self.note = 0
        self.velocity = 0
        self.seq = 0

    def tick(self):
        self.volume.tick()
        self.arpeggio.tick()
        self.pitch.tick()
        self.duty.tick()

    def note_on(self, note, velocity=127, seq=0):
        self.note = note
        self.velocity = velocity / 127
        self.seq = seq
        self.volume.note_on()
        self.arpeggio.note_on()
        self.pitch.note_on()
        self.duty.note_on()

    def note_off(self):
        self.volume.note_off()
        self.arpeggio.note_off()
        self.pitch.note_off()
        self.duty.note_off()

    @property
    def state(self):
        return self.volume.state

    @property
    def active(self):
        return (
            self.volume.active
            or self.arpeggio.active
            or self.pitch.active
            or self.duty.active
        )

    @property
    def value(self):
        if self.timer is None:
            f = frequency(self.note + self.arpeggio.value)
            timer = timer_value(f) + self.pitch.value
        else:
            timer = self.timer + self.pitch.value
        return {
            "timer": timer,
            "volume": int(self.volume.value * self.velocity),
            "duty": self.duty.value,
        }
