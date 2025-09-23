######################################################################
#
######################################################################

import enum
from typing import Optional
from dataclasses import dataclass, field
from dataclasses_json import DataClassJsonMixin


class VoiceKind(enum.StrEnum):
    PULSE0 = "PULSE0"
    PULSE1 = "PULSE1"
    TRIANGLE = "TRIANGLE"
    NOISE = "NOISE"
    DMC = "DMC"
    MMC5_PULSE0 = "MMC5_PULSE0"
    MMC5_PULSE1 = "MMC5_PULSE1"


class EnvelopeKind(enum.StrEnum):
    UNKNOWN = enum.auto()
    VOLUME = enum.auto()
    ARPEGGIO = enum.auto()
    PITCH = enum.auto()
    HIPITCH = enum.auto()
    DUTY = enum.auto()


class InstrumentKind(enum.StrEnum):
    NES2A03 = enum.auto()
    VRC6 = enum.auto()
    VRC7 = enum.auto()
    FDS = enum.auto()
    N163 = enum.auto()
    S5B = enum.auto()


class EnvelopeState(enum.IntEnum):
    OFF = 0
    RELEASE = 1
    ON = 2


class MissingValue(enum.StrEnum):
    LAST = enum.auto()
    INTERPOLATE = enum.auto()


@dataclass
class Envelope(DataClassJsonMixin):
    kind: EnvelopeKind
    points: dict[int, int]
    loop: Optional[int] = None
    release: Optional[int] = None
    missing_value: MissingValue = MissingValue.LAST


@dataclass
class Instrument(DataClassJsonMixin):
    name: str
    kind: InstrumentKind = InstrumentKind.NES2A03
    volume: Optional[Envelope] = None
    arpeggio: Optional[Envelope] = None
    pitch: Optional[Envelope] = None
    hipitch: Optional[Envelope] = None
    duty: Optional[Envelope] = None


@dataclass
class Trigger(DataClassJsonMixin):
    instrument: str
    timer: int


@dataclass
class ChannelConfig(DataClassJsonMixin):
    voice: list[VoiceKind]
    note_offset: int = 0
    instrument: str = "default"
    pad: dict[int, Trigger] = field(default_factory=dict)


@dataclass
class MidiConfig(DataClassJsonMixin):
    name: str = "empty"
    channel: dict[int, ChannelConfig] = field(default_factory=dict)
    instrument: dict[str, Instrument] = field(default_factory=dict)
