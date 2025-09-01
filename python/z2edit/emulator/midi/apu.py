######################################################################
# Control the NES API
######################################################################

import enum
import logging

logger = logging.getLogger(__name__)

F_CPU = 1789773


class VoiceType(enum.StrEnum):
    PULSE0 = "PULSE0"
    PULSE1 = "PULSE1"
    TRIANGLE = "TRIANGLE"
    NOISE = "NOISE"
    DMC = "DMC"
    MMC5_PULSE0 = "MMC5_PULSE0"
    MMC5_PULSE1 = "MMC5_PULSE1"


class Pulse(object):

    def __init__(self, emulator, address):
        self.emulator = emulator
        self.address = address
        self._duty = 0
        self._volume = 0
        self._timer = 0

    def write(self, **kwargs):
        needs_update = False
        if (v := kwargs.get("volume")) is not None:
            if v != self._volume:
                needs_update = True
            self._volume = v
        if (v := kwargs.get("duty")) is not None:
            if v != self._duty:
                needs_update = True
            self._duty = v
        if (v := kwargs.get("timer")) is not None:
            if v != self._timer:
                needs_update = True
            self._timer = v
        if (v := kwargs.get("frequency")) is not None:
            v = int(F_CPU / (16 * v)) - 1
            if v != self._timer:
                needs_update = True
            self._timer = v

        if needs_update:
            v = 0x30 | ((self._duty & 0x03) << 6) | (self._volume & 0x0F)
            self.emulator.nes[self.address + 0] = v
            self.emulator.nes[self.address + 2] = self._timer & 0xFF
            self.emulator.nes[self.address + 3] = (self._timer >> 8) & 0x07


class Triangle(object):

    def __init__(self, emulator, address):
        self.emulator = emulator
        self.address = address
        self._timer = 0
        self._volume = 0

    def write(self, **kwargs):
        needs_update = False
        if (v := kwargs.get("volume")) is not None:
            if v != self._volume:
                needs_update = True
            self._volume = v
        if (v := kwargs.get("timer")) is not None:
            if v != self._timer:
                needs_update = True
            self._timer = v
        if (v := kwargs.get("frequency")) is not None:
            v = int(F_CPU / (32 * v)) - 1
            if v != self._timer:
                needs_update = True
            self._timer = v

        if needs_update:
            self.emulator.nes[self.address + 0] = 0xFF if self._volume else 0x80
            self.emulator.nes[self.address + 2] = self._timer & 0xFF
            self.emulator.nes[self.address + 3] = (self._timer >> 8) & 0x07


class Noise(object):

    def __init__(self, emulator, address):
        self.emulator = emulator
        self.address = address
        self._volume = 0
        self._period = 0

    def write(self, **kwargs):
        needs_update = False
        if (v := kwargs.get("volume")) is not None:
            if v != self._volume:
                needs_update = True
            self._volume = v
        if (v := kwargs.get("period")) is not None:
            if v != self._period:
                needs_update = True
            self._period = v

        if needs_update:
            v = 0x10 | (self._volume & 0x0F)
            self.emulator.nes[self.address + 0] = v
            self.emulator.nes[self.address + 2] = self._period


class Apu(object):

    def __init__(self, emulator):
        self.emulator = emulator
        self.voice = {
            VoiceType.PULSE0: Pulse(emulator, 0x4000),
            VoiceType.PULSE1: Pulse(emulator, 0x4004),
            VoiceType.TRIANGLE: Triangle(emulator, 0x4008),
            VoiceType.NOISE: Noise(emulator, 0x400C),
        }
        self.emulator.nes[0x4015] = 0x0F
        mapper = emulator.nes.rom_mapper

        if mapper == 5:
            logging.info("MMC5: Adding additional pulse channels")
            self.voice.update(
                {
                    VoiceType.MMC5_PULSE0: Pulse(emulator, 0x5000),
                    VoiceType.MMC5_PULSE1: Pulse(emulator, 0x5004),
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
