######################################################################
#
######################################################################

import base64
import enum
import struct
import logging
from typing import Optional
from dataclasses import dataclass, field
from dataclasses_json import DataClassJsonMixin, config
from marshmallow import fields

logger = logging.getLogger(__name__)


def encode_bytes(data: bytes) -> str:
    """Encodes bytes into a Base64 string."""
    return base64.b64encode(data).decode("utf-8")


def decode_bytes(data: str) -> bytes:
    """Decodes a Base64 string back into bytes."""
    return base64.b64decode(data.encode("utf-8"))


class VoiceKind(enum.StrEnum):
    APU_PULSE0 = enum.auto()
    APU_PULSE1 = enum.auto()
    APU_TRIANGLE = enum.auto()
    APU_NOISE = enum.auto()
    APU_DMC = enum.auto()
    MMC5_PULSE0 = enum.auto()
    MMC5_PULSE1 = enum.auto()


class EnvelopeKind(enum.StrEnum):
    UNKNOWN = enum.auto()
    VOLUME = enum.auto()
    ARPEGGIO = enum.auto()
    PITCH = enum.auto()
    HIPITCH = enum.auto()
    DUTY = enum.auto()


class InstrumentKind(enum.StrEnum):
    UNKNOWN = enum.auto()
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
    """
    Represents an envelope for sound playback (e.g. volume, apreggio, pitch
    or duty cycle).

    Attributes:
        points: A dict mapping frame number to value.
        loop: The frame to return to upon reaching the release frame while the
              note is still in the "on" state.
        release: The frame that represents the release phase of the envelope.
                 Prior to note-off, upon reaching this frame, the envelope will
                 return to the loop point.
        missing_value: The policy for supplying frame values not explicitly listed
                       in the points mapping.  Either "interpolate" or "last".
    """

    points: dict[int, int]
    loop: Optional[int] = None
    release: Optional[int] = None
    missing_value: MissingValue = MissingValue.LAST

    @staticmethod
    def from_fti(buffer, offset):
        count, loop, release, setting = struct.unpack_from("<IiiI", buffer, offset)
        offset += 16
        sequence = struct.unpack_from(f"<{count}b", buffer, offset)
        offset += count

        # Fix up bounds for loop and release
        if loop > count:
            loop = count - 1
        if release > count:
            release = count - 1
        if release != -1:
            if loop == -1:
                loop = release
            release += 1

        return (
            Envelope(
                points={i: v for i, v in enumerate(sequence)},
                loop=loop,
                release=release,
            ),
            offset,
        )


@dataclass
class DpcmAssignment(DataClassJsonMixin):
    """
    A DPCM note assignment.

    Attributes:
        note: The MIDI note number.
        sample: The sample to play for `note`.
        pitch: The DMC pitch/frequency value (0-15).
        loop: Whether this DPCM sample should loop.
    """

    note: int
    sample: int
    pitch: int
    loop: bool

    @staticmethod
    def from_fti(buffer, offset):
        note, sample, pitch, delta = struct.unpack_from("<BBBB", buffer, offset)
        offset += 4
        return (
            DpcmAssignment(
                note,
                sample - 1,
                pitch & 0x7F,
                pitch & 0x80 != 0,
            ),
            offset,
        )


@dataclass
class DpcmSample(DataClassJsonMixin):
    """
    A DPCM sample.

    Attributes:
        name: The name of the sample.
        size: The length of the sample.
        data: The DPCM encoded sample data.
    """

    name: str
    size: int
    data: bytes = field(
        metadata=config(
            encoder=encode_bytes,
            decoder=decode_bytes,
            mm_field=fields.String(),
        )
    )

    @staticmethod
    def from_fti(buffer, offset):
        (namelen,) = struct.unpack_from("<I", buffer, offset)
        offset += 4
        (name,) = struct.unpack_from(f"{namelen}s", buffer, offset)
        offset += namelen
        (size,) = struct.unpack_from("<I", buffer, offset)
        offset += 4
        data = struct.unpack_from(f"{size}B", buffer, offset)
        offset += size
        return (
            DpcmSample(
                name.decode("utf-8", errors="replace"),
                size,
                bytes(data),
            ),
            offset,
        )


@dataclass
class Instrument(DataClassJsonMixin):
    """
    A NES Instrument.

    Attributes:
        name: The name of the instrument.
        kind: The kind of instrument (e.g. 2A03, VRC6, VRC7, etc).
        volume: A volume envelope.
        arpeggio: An arpeggio envelope.
        pitch: A pitch envelope.
        hipitch: A hipitch envelope (unused).
        duty: A duty cycle envelope.
        dpcm: A list of DPCM assignments.
        sample: A mapping of DPCM samples (integer index to sample).
    """

    name: str
    kind: InstrumentKind = InstrumentKind.NES2A03
    volume: Optional[Envelope] = None
    arpeggio: Optional[Envelope] = None
    pitch: Optional[Envelope] = None
    hipitch: Optional[Envelope] = None
    duty: Optional[Envelope] = None
    dpcm: list[DpcmAssignment] = field(default_factory=list)
    sample: dict[int, DpcmSample] = field(default_factory=dict)

    def _parse_basic(self, buffer, offset):
        (count,) = struct.unpack_from("<B", buffer, offset)
        offset += 1
        if count != 5:
            raise ValueError(f"Unexpected number of envelopes: {count}")

        for i in range(count):
            (present,) = struct.unpack_from("?", buffer, offset)
            offset += 1
            print(f"Parsing env {i} {present=}")
            if present:
                print(f"Parsing env {i} at {offset}")
                env, offset = Envelope.from_fti(buffer, offset)
                if i == 0:
                    self.volume = env
                if i == 1:
                    self.arpeggio = env
                if i == 2:
                    self.pitch = env
                if i == 3:
                    self.hipitch = env
                if i == 4:
                    self.duty = env
        return offset

    def _parse_dpcm(self, buffer, offset):
        (assigned,) = struct.unpack_from("<I", buffer, offset)
        offset += 4
        for i in range(assigned):
            dpcm, offset = DpcmAssignment.from_fti(buffer, offset)
            self.dpcm.append(dpcm)

        (count,) = struct.unpack_from("<I", buffer, offset)
        offset += 4
        for i in range(count):
            (index,) = struct.unpack_from("<I", buffer, offset)
            offset += 4
            (sample, offset) = DpcmSample.from_fti(buffer, offset)
            self.sample[index] = sample
        return offset

    @staticmethod
    def _from_fti(buffer, offset):
        if buffer[offset : offset + 3] != b"FTI":
            raise ValueError("Bad FTI signature")
        offset += 3
        if buffer[offset : offset + 3] != b"2.4":
            raise ValueError("Bad FTI version")
        offset += 3

        kind, namelen = struct.unpack_from("<BI", buffer, offset)
        kind = list(InstrumentKind)[kind]
        offset += 5
        (name,) = struct.unpack_from(f"{namelen}s", buffer, offset)
        offset += namelen

        inst = Instrument(name.decode("utf-8", errors="replace"), kind)
        if kind == InstrumentKind.NES2A03 or kind == InstrumentKind.VRC6:
            offset = inst._parse_basic(buffer, offset)
        else:
            raise NotImplementedError(f"parse {kind}")

        if kind == InstrumentKind.NES2A03:
            offset = inst._parse_dpcm(buffer, offset)
        return inst, offset

    @staticmethod
    def parse_fti(file):
        if isinstance(file, str):
            file = open(file, "rb").read()
        if not isinstance(file, bytes):
            raise ValueError("expected bytes buffer")

        inst, _ = Instrument._from_fti(file, 0)
        return inst


@dataclass
class Trigger(DataClassJsonMixin):
    """
    A "pad" trigger.  Used to trigger noise or DPCM samples.

    Attributes:
        instrument: The instrument (envelope) to use on this channel.
        timer: The timer value to use (on this channel)
        remap: A mapping of channel to note to trigger notes on other channels.
    """

    instrument: Optional[str] = None
    timer: int = 0
    remap: dict[int, int] = field(default_factory=dict)


@dataclass
class ChannelConfig(DataClassJsonMixin):
    """
    A channel configuration.

    Attributes:
        voice: A set of NES voices to use (e.g. APU_PULSE0, APU_TRIANGLE,
               MMC5_PULSE0).
        note_offset: Number of semitones added to the note before playing.
        instrument: The instrument to play on this channel.
        pad: A set of "pads" which allow triggering other effects or notes when
             a particular note is played.
    """

    voice: list[VoiceKind]
    note_offset: int = 0
    instrument: str = "default"
    pad: dict[int, Trigger] = field(default_factory=dict)


@dataclass
class MidiConfig(DataClassJsonMixin):
    """
    A MIDI configuration for the emulator.

    Attributes:
        name: The name of this configuration.
        channel: A mapping of MIDI channel numbers (1-based) to channel
                 configurations.
        instrument: A list of instruments known to this configuration.
        load_instruments: A list of filenames of instruments to load into
                          this configuration.
    """

    name: str = "empty"
    channel: dict[int, ChannelConfig] = field(default_factory=dict)
    instrument: list[Instrument] = field(default_factory=list)
    load_instruments: list[str] = field(default_factory=list)
    midi_program: dict[int, str] = field(default_factory=dict)

    def get_instrument(self, name):
        if isinstance(name, str):
            for i, inst in enumerate(self.instrument):
                if name == inst.name:
                    name = i
                    break
            else:
                name = 0
        return self.instrument[name]
