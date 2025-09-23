######################################################################
# Some basic configurations
######################################################################
import os
import logging

from .datatypes import *
from z2edit import gui

logger = logging.getLogger(__name__)


def basic_config(name, mapper):
    cfg = MidiConfig(
        name="builtin",
        channel={
            10: ChannelConfig(
                voice=[VoiceKind.APU_NOISE],
                pad={
                    36: Trigger(instrument="hat", timer=0xE),  # Kick
                    38: Trigger(instrument="snare", timer=0xA),  # Snare
                    41: Trigger(instrument="snare", timer=0x6),  # Low Tom
                    45: Trigger(instrument="snare", timer=0x6),  # Mid Tom
                    47: Trigger(instrument="snare", timer=0x6),  # Mid Tom 2
                    48: Trigger(instrument="snare", timer=0x6),  # High Tom
                    50: Trigger(instrument="snare", timer=0x6),  # High Tom 2
                    49: Trigger(instrument="snare", timer=0x6),  # Crash
                    57: Trigger(instrument="snare", timer=0x6),  # Crash 2
                    42: Trigger(instrument="snare", timer=0x6),  # Closed HH
                    -1: Trigger(instrument="__skip__", timer=0),  # Default: no sound
                },
            ),
        },
        instrument=[
            Instrument(
                name="pulse",
                kind=InstrumentKind.NES2A03,
                volume=Envelope(
                    points={0: 15, 1: 15, 8: 10, 9: 10, 20: 0},
                    loop=8,
                    release=9,
                    missing_value=MissingValue.INTERPOLATE,
                ),
                arpeggio=Envelope({}),
                pitch=Envelope({}),
                hipitch=None,
                duty=Envelope(
                    points={0: 2, 1: 2, 20: 2},
                    loop=0,
                    release=1,
                    missing_value=MissingValue.LAST,
                ),
            ),
            Instrument(
                name="triangle",
                kind=InstrumentKind.NES2A03,
                volume=Envelope(
                    points={0: 0, 1: 15},
                    loop=-1,
                    release=-1,
                    missing_value=MissingValue.LAST,
                ),
            ),
            Instrument(
                name="snare",
                kind=InstrumentKind.NES2A03,
                volume=Envelope(
                    points={
                        0: 15,
                        1: 12,
                        2: 9,
                        3: 8,
                        4: 7,
                        5: 6,
                        6: 5,
                        7: 4,
                        9: 3,
                        12: 2,
                        15: 1,
                        18: 0,
                    },
                    missing_value=MissingValue.LAST,
                ),
            ),
            Instrument(
                name="hat",
                kind=InstrumentKind.NES2A03,
                volume=Envelope(
                    points={
                        0: 10,
                        1: 6,
                        2: 4,
                        3: 3,
                        4: 2,
                        5: 2,
                        6: 1,
                        7: 1,
                        9: 0,
                    },
                    missing_value=MissingValue.LAST,
                ),
            ),
        ],
    )

    if name == "builtin":
        cfg.channel[1] = ChannelConfig(voice=[VoiceKind.APU_PULSE0], instrument="pulse")
        cfg.channel[2] = ChannelConfig(voice=[VoiceKind.APU_PULSE1], instrument="pulse")
        cfg.channel[3] = ChannelConfig(
            voice=[VoiceKind.APU_TRIANGLE], note_offset=24, instrument="triangle"
        )
        if mapper == 5:
            cfg.channel[4] = ChannelConfig(
                voice=[VoiceKind.MMC5_PULSE0], instrument="pulse"
            )
            cfg.channel[5] = ChannelConfig(
                voice=[VoiceKind.MMC5_PULSE1], instrument="pulse"
            )
    elif name == "builtin-poly":
        voices = [VoiceKind.APU_PULSE0, VoiceKind.APU_PULSE1]
        if mapper == 5:
            voices.extend([VoiceKind.MMC5_PULSE0, VoiceKind.MMC5_PULSE1])
        cfg.channel[1] = ChannelConfig(voice=voices, instrument="pulse")
        cfg.channel[2] = ChannelConfig(
            voice=[VoiceKind.APU_TRIANGLE], note_offset=12, instrument="default"
        )
    else:
        raise Exception(f"No builtin config named {name!r}")
    return cfg


def _load_config(name):
    cfgdir = os.path.dirname(name)
    cfg = open(name, "rt").read()
    cfg = MidiConfig.from_json(cfg)
    for file in cfg.load_instruments:
        path = os.path.join(cfgdir, file)
        (_, ext) = os.path.splitext(path)
        logger.info("Loading instrument %r", path)
        data = open(path, "rb").read()
        if ext == ".fti":
            cfg.instrument.append(Instrument.parse_fti(data))
        else:
            cfg.instrument.append(Instrument.from_json(data))
    return cfg


def load_config(name, mapper):
    dirs = gui.Directories.get()
    dirs = [
        os.getcwd(),
        os.path.join(dirs.install, "config/emulator/midi"),
    ]
    for d in dirs:
        path = os.path.join(d, name)
        if os.path.exists(path):
            logger.info("Loading config %r", path)
            return _load_config(path)
    else:
        logger.info("Using built-in config %r", name)
        return basic_config(name, mapper)
