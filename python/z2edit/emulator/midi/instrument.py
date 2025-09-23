######################################################################
# Envelope implementation
######################################################################

import logging

from .datatypes import InstrumentKind, EnvelopeState, MissingValue
from .frequency import frequency, timer_value, DMC_FREQ
from . import datatypes

logger = logging.getLogger(__name__)


class Envelope(object):

    def __init__(
        self, points={}, loop=-1, release=-1, missing=MissingValue.LAST, config=None
    ):
        self.frame = 0
        if not config:
            config = datatypes.Envelope(points, loop, release, missing)
        self.config = config
        self.state = EnvelopeState.OFF

    @property
    def loop(self):
        v = self.config.loop
        return v if v is not None else -1

    @property
    def release(self):
        v = self.config.release
        return v if v is not None else -1

    @property
    def missing_value(self):
        return self.config.missing_value

    @property
    def points(self):
        if not self.config.points:
            return [(0, 0)]
        return sorted(self.config.points.items())

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
        prev = (0, 0)
        frame = None
        for fr, val in self.points:
            if fr >= self.frame:
                frame = (fr, val)
                break
            prev = (fr, val)

        if self.state == EnvelopeState.OFF or frame is None:
            # TODO: what should we do when the envelope is off?
            # If we return the last point, the instrument can
            # keep processing all envelopes until they all reach
            # the OFF state.
            self.state = EnvelopeState.OFF
            return prev[1]
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
        self.name = "unknown"
        self.volume = Envelope(points={0: 15, 1: 0}, loop=0, release=1)
        self.arpeggio = Envelope(points={0: 0})
        self.pitch = Envelope(points={0: 0})
        self.duty = Envelope(points={0: 2})
        self.timer = timer
        self.dpcm_map = {}
        self.sample = {}
        if config:
            self.kind = config.kind
            self.name = config.name
            if config.volume:
                self.volume = Envelope(config=config.volume)
            if config.arpeggio:
                self.arpeggio = Envelope(config=config.arpeggio)
            if config.pitch:
                self.pitch = Envelope(config=config.pitch)
            if config.duty:
                self.duty = Envelope(config=config.duty)
            if config.dpcm:
                self.dpcm_map = {i: v for i, v in enumerate(config.dpcm)}
            if config.sample:
                self.sample = config.sample
        self.note = 0
        self.velocity = 0
        self.seq = 0
        self.dpcm = None
        self.dpcm_sample = bytes()
        self.dpcm_length = 0
        self.dpcm_per_frame = 0

    def tick(self):
        self.volume.tick()
        self.arpeggio.tick()
        self.pitch.tick()
        self.duty.tick()
        if self.dpcm_length > 0:
            self.dpcm_length -= self.dpcm_per_frame

    def note_on(self, note, velocity=127, seq=0):
        self.note = note
        self.velocity = velocity / 127
        self.seq = seq
        self.volume.note_on()
        self.arpeggio.note_on()
        self.pitch.note_on()
        self.duty.note_on()
        if (dpcm := self.dpcm_map.get(note)) is not None:
            if (sample := self.sample.get(dpcm.sample)) is not None:
                self.dpcm = dpcm
                self.dpcm_sample = sample.data
                self.dpcm_length = sample.size
                self.dpcm_per_frame = int(DMC_FREQ[dpcm.pitch] / 60.0988)
            else:
                logger.error("No DPCM sample found for %r", dpcm)

    def note_off(self):
        self.volume.note_off()
        self.arpeggio.note_off()
        self.pitch.note_off()
        self.duty.note_off()

    def update_active_values(self, active_values):
        values = {}
        for e in ("volume", "arpeggio", "pitch", "duty"):
            env = getattr(self, e)
            values[e] = (env.state, env.frame, env.value)
        active_values[self.name] = values

    @property
    def state(self):
        if self.dpcm_length > 0:
            return EnvelopeState.ON

        # Sample each envelope and only return OFF when they all
        # reach the OFF state.
        states = [
            self.volume.state,
            self.arpeggio.state,
            self.pitch.state,
            self.duty.state,
        ]
        on = 0
        release = 0
        off = 0
        for s in states:
            if s == EnvelopeState.ON:
                on += 1
            if s == EnvelopeState.RELEASE:
                release += 1
            if s == EnvelopeState.OFF:
                off += 1

        if off == len(states):
            return EnvelopeState.OFF
        if on:
            return EnvelopeState.ON
        return EnvelopeState.RELEASE

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
        if self.dpcm_map:
            if dpcm := self.dpcm:
                self.dpcm = None
                return {
                    "frequency": dpcm.frequency,
                    "sample": self.dpcm_sample,
                    "size": self.dpcm_length,
                }
            else:
                return {}

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
