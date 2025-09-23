######################################################################
# Control the NES API
######################################################################

import enum
import logging

from .frequency import F_CPU
from .datatypes import VoiceKind

logger = logging.getLogger(__name__)


class Pulse(object):

    def __init__(self, emulator, address):
        self.emulator = emulator
        self.address = address
        self._duty = 0
        self._volume = 0
        self._timer = 0

    def write(self, **kwargs):
        update_v = False
        update_t0 = False
        update_t1 = False
        if (v := kwargs.get("volume")) is not None:
            if v != self._volume:
                update_v = True
            self._volume = v
        if (v := kwargs.get("duty")) is not None:
            if v != self._duty:
                update_v = True
            self._duty = v
        if (v := kwargs.get("timer")) is not None:
            if v & 0xFF != self._timer & 0xFF:
                update_t0 = True
            if v & 0xFF00 != self._timer & 0xFF00:
                update_t1 = True
            self._timer = v
        if (v := kwargs.get("frequency")) is not None:
            v = int(F_CPU / (16 * v)) - 1
            if v != self._timer:
                update_t0 = True
                update_t1 = True
            self._timer = v

        if update_v:
            v = 0x30 | ((self._duty & 0x03) << 6) | (self._volume & 0x0F)
            self.emulator.nes[self.address + 0] = v
        if update_t0:
            self.emulator.nes[self.address + 2] = self._timer & 0xFF
        if update_t1:
            self.emulator.nes[self.address + 3] = (self._timer >> 8) & 0x07


class Triangle(object):

    def __init__(self, emulator, address):
        self.emulator = emulator
        self.address = address
        self._timer = 0
        self._volume = 0

    def write(self, **kwargs):
        update_v = False
        update_t0 = False
        update_t1 = False
        if (v := kwargs.get("volume")) is not None:
            if v != self._volume:
                update_v = True
            self._volume = v
        if (v := kwargs.get("timer")) is not None:
            if v & 0xFF != self._timer & 0xFF:
                update_t0 = True
            if v & 0xFF00 != self._timer & 0xFF00:
                update_t1 = True
            self._timer = v
        if (v := kwargs.get("frequency")) is not None:
            v = int(F_CPU / (32 * v)) - 1
            if v != self._timer:
                update_t = True
            self._timer = v

        if update_v:
            self.emulator.nes[self.address + 0] = 0xFF if self._volume else 0x80
        if update_t0:
            self.emulator.nes[self.address + 2] = self._timer & 0xFF
        if update_t1:
            self.emulator.nes[self.address + 3] = (self._timer >> 8) & 0x07


class Noise(object):

    def __init__(self, emulator, address):
        self.emulator = emulator
        self.address = address
        self._volume = 0
        self._timer = 0

    def write(self, **kwargs):
        update_v = False
        update_t = False
        if (v := kwargs.get("volume")) is not None:
            if v != self._volume:
                update_v = True
            self._volume = v
        if (v := kwargs.get("timer")) is not None:
            if v != self._timer:
                update_t = True
            self._timer = v

        if update_v:
            v = 0x30 | (self._volume & 0x0F)
            self.emulator.nes[self.address + 0] = v
        if update_t:
            # Although this register is formally named "period", it is in
            # some sense the same as the timer for the other channels.
            self.emulator.nes[self.address + 2] = self._timer
            self.emulator.nes[self.address + 3] = 0xF8


class Dmc(object):

    def __init__(self, emulator, address):
        self.emulator = emulator
        self.address = address

    def write(self, **kwargs):

        active = self.emulator.nes.mem[0x4015] & 0x10

        if (f := kwargs.get("frequency")) is not None:
            loop = kwargs.get("loop", False)
            f |= 0x60 if loop else 0
            self.emulator.nes[self.address + 0] = f
            active = True

        size = None
        if (sample := kwargs.get("sample")) is not None:
            addr = (0xFF80 - len(sample)) & 0xFFC0
            self.emulator.nes.write(addr, sample)
            self.emulator.nes[self.address + 2] = addr
            size = len(sample)
            active = True

        if (size := kwargs.get("size", size)) is not None:
            self.emulator.nes[self.address + 3] = size
            active = True

        if active:
            self.emulator.nes[0x4015] = 0x1F


class Apu(object):

    def __init__(self, emulator):
        self.emulator = emulator
        self.voice = {
            VoiceKind.APU_PULSE0: Pulse(emulator, 0x4000),
            VoiceKind.APU_PULSE1: Pulse(emulator, 0x4004),
            VoiceKind.APU_TRIANGLE: Triangle(emulator, 0x4008),
            VoiceKind.APU_NOISE: Noise(emulator, 0x400C),
            VoiceKind.APU_DMC: Dmc(emulator, 0x4010),
        }
        self.emulator.nes[0x4015] = 0x0F
        mapper = emulator.nes.rom_mapper

        if mapper == 5:
            logging.info("MMC5: Adding additional pulse channels")
            self.voice.update(
                {
                    VoiceKind.MMC5_PULSE0: Pulse(emulator, 0x5000),
                    VoiceKind.MMC5_PULSE1: Pulse(emulator, 0x5004),
                }
            )
            self.emulator.nes[0x5015] = 0x03
        elif mapper in (24, 26):
            logger.error("TODO: VRC6 not implemented")
        elif mapper == 85:
            logger.error("TODO: VRC7 not implemented")
        elif mapper == 19:
            logger.error("TODO: Namco 163 not implemented")
        elif mapper == 19:
            logger.error("TODO: Sunsoft 5B not implemented")
        else:
            logging.info("Assuming no audio expansion for mapper %d", mapper)

    def write(self, voice, **kwargs):
        if v := self.voice.get(voice):
            v.write(**kwargs)
        else:
            logger.error("Voice %s does not exist", voice)
