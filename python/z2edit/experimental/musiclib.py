######################################################################
#
# A reasonably straight port of BGT's C++ music library.
#
######################################################################
import enum
import sys
from collections import defaultdict
try:
    import mido
except:
    mido = None

from z2edit import Address

totalm = 0
MIDI_PPQN = 120
OVERWORLD_SONG_TABLE = Address.prg(6, 0xa000)
TOWN_SONG_TABLE      = Address.prg(6, 0xa3ca)
PALACE_SONG_TABLE    = Address.prg(6, 0xa62f)
GP_SONG_TABLE        = Address.prg(6, 0xa936)

class Note(int):

    Sixteenth      = 0x00
    DottedQuarter  = 0x01
    DottedEighth   = 0x40
    Half           = 0x41
    Eighth         = 0x80
    EighthTriplet  = 0x81
    Quarter        = 0xc0
    QuarterTriplet = 0xc1

    Rest = 0x02
    Cs3  = 0x3e
    Db3  = 0x3e
    E3   = 0x04
    G3   = 0x06
    Gs3  = 0x08
    Ab3  = 0x08
    A3   = 0x0a
    As3  = 0x0c
    Bb3  = 0x0c
    B3   = 0x0e
    C4   = 0x10
    Cs4  = 0x12
    Db4  = 0x12
    D4   = 0x14
    Ds4  = 0x16
    Eb4  = 0x16
    E4   = 0x18
    F4   = 0x1a
    Fs4  = 0x1c
    Gb4  = 0x1c
    G4   = 0x1e
    Gs4  = 0x20
    Ab4  = 0x20
    A4   = 0x22
    As4  = 0x24
    Bb4  = 0x24
    B4   = 0x26
    C5   = 0x28
    Cs5  = 0x2a
    Db5  = 0x2a
    Ds5  = 0x2e
    Eb5  = 0x2e
    D5   = 0x2c
    E5   = 0x30
    F5   = 0x32
    Fs5  = 0x34
    Gb5  = 0x34
    G5   = 0x36
    A5   = 0x38
    As5  = 0x3a
    Bb5  = 0x3a
    B5   = 0x3c

    def pitch(self):
        return self & 0x3e

    def duration(self):
        return self & 0xc1

    def new_duration(self, duration):
        return Note((self & 0x3e) | duration)

    def length(self):
        d = self.duration()
        if d == Note.Sixteenth:
            return MIDI_PPQN // 4
        elif d == Note.DottedQuarter:
            return MIDI_PPQN * 3 // 2
        elif d == Note.DottedEighth:
            return MIDI_PPQN // 2 * 3 // 2
        elif d == Note.Half:
            return MIDI_PPQN * 2
        elif d == Note.Eighth:
            return MIDI_PPQN // 2
        elif d == Note.EighthTriplet:
            return MIDI_PPQN // 2 * 2 // 3
        elif d == Note.Quarter:
            return MIDI_PPQN
        elif d == Note.QuarterTriplet:
            return MIDI_PPQN * 2 // 3
        else:
            return MIDI_PPQN

    def __str__(self):
        p = self.pitch()
        if p == Note.Rest: return "---."
        elif p == Note.Cs3:  return "C#3."
        elif p == Note.E3:   return "E3.."
        elif p == Note.G3:   return "G3.."
        elif p == Note.Gs3:  return "G#3."
        elif p == Note.A3:   return "A3.."
        elif p == Note.As3:  return "A#3."
        elif p == Note.B3:   return "B3.."
        elif p == Note.C4:   return "C4.."
        elif p == Note.Cs4:  return "C#4."
        elif p == Note.D4:   return "D4.."
        elif p == Note.Ds4:  return "D#4."
        elif p == Note.E4:   return "E4.."
        elif p == Note.F4:   return "F4.."
        elif p == Note.Fs4:  return "F#4."
        elif p == Note.G4:   return "G4.."
        elif p == Note.Gs4:  return "G#4."
        elif p == Note.A4:   return "A4.."
        elif p == Note.As4:  return "A#4."
        elif p == Note.B4:   return "B4.."
        elif p == Note.C5:   return "C5.."
        elif p == Note.Cs5:  return "C#5."
        elif p == Note.D5:   return "D5.."
        elif p == Note.Ds5:  return "D#5."
        elif p == Note.E5:   return "E5.."
        elif p == Note.F5:   return "F5.."
        elif p == Note.Fs5:  return "F#5."
        elif p == Note.G5:   return "G5.."
        elif p == Note.A5:   return "A5.."
        elif p == Note.As5:  return "A#5."
        elif p == Note.B5:   return "B5.."
        else:
            print('Error: note value={}'.format(p), file=sys.stderr)
            return "????"

    def as_midi(self):
        p = self.pitch()
        if p == Note.Rest:   return 0
        elif p == Note.Cs3:  return 49
        elif p == Note.E3:   return 52
        elif p == Note.G3:   return 55
        elif p == Note.Gs3:  return 56
        elif p == Note.A3:   return 57
        elif p == Note.As3:  return 58
        elif p == Note.B3:   return 59
        elif p == Note.C4:   return 60
        elif p == Note.Cs4:  return 61
        elif p == Note.D4:   return 62
        elif p == Note.Ds4:  return 63
        elif p == Note.E4:   return 64
        elif p == Note.F4:   return 65
        elif p == Note.Fs4:  return 66
        elif p == Note.G4:   return 67
        elif p == Note.Gs4:  return 68
        elif p == Note.A4:   return 69
        elif p == Note.As4:  return 70
        elif p == Note.B4:   return 71
        elif p == Note.C5:   return 72
        elif p == Note.Cs5:  return 73
        elif p == Note.D5:   return 74
        elif p == Note.Ds5:  return 75
        elif p == Note.E5:   return 76
        elif p == Note.F5:   return 77
        elif p == Note.Fs5:  return 78
        elif p == Note.G5:   return 79
        elif p == Note.A5:   return 81
        elif p == Note.As5:  return 82
        elif p == Note.B5:   return 83
        else:
            return 0


# Apparently, the Z2 music engine actually has the pulse channels reversed.
# This is not currently represented by the `Channel` enum except in the
# `as_midi` helper function.
class Channel(enum.IntEnum):
    Pulse1 = 1
    Pulse2 = 2
    Triangle = 3
    Noise = 4

    def __str__(self):
        if self == Channel.Pulse1:
            return "Pulse1"
        elif self == Channel.Pulse2:
            return "Pulse2"
        elif self == Channel.Triangle:
            return "Triangle"
        elif self == Channel.Noise:
            return "Noise"

    def as_famistudio(self):
        if self == Channel.Pulse1:
            return "Square1"
        elif self == Channel.Pulse2:
            return "Square2"
        elif self == Channel.Triangle:
            return "Triangle"
        elif self == Channel.Noise:
            return "Noise"

    def as_midi(self):
        if self == Channel.Pulse1:
            return 1
        elif self == Channel.Pulse2:
            return 0
        elif self == Channel.Triangle:
            return 2
        elif self == Channel.Noise:
            return 9

    def as_midi_instrument(self):
        if self == Channel.Pulse1:
            return 0
        elif self == Channel.Pulse2:
            return 0
        elif self == Channel.Triangle:
            return 34
        elif self == Channel.Noise:
            return 0


class Pattern(object):

    @staticmethod
    def from_rom(edit, address):
        tempo = edit.read(address + 0)
        data = edit.read_pointer(address + 1)
        tri = edit.read(address + 3)
        pw2 = edit.read(address + 4)
        noi = edit.read(address + 5)

        ret = Pattern(tempo)
        ret.read_notes(edit, Channel.Pulse1, data)
        if tri: ret.read_notes(edit, Channel.Triangle, data+tri)
        if pw2: ret.read_notes(edit, Channel.Pulse2, data+pw2)
        if noi: ret.read_notes(edit, Channel.Noise, data+noi)
        return ret

    def __init__(self, tempo, pw1=None, pw2=None, tri=None, noi=None):
        self.tempo = tempo
        self.notes = {
            # Convert int objects into `Note` objects.
            Channel.Pulse1: list(map(Note, pw1 if pw1 else [])),
            Channel.Pulse2: list(map(Note, pw2 if pw2 else [])),
            Channel.Triangle: list(map(Note, tri if tri else [])),
            Channel.Noise: list(map(Note, noi if noi else [])),
        }

    def to_string(self, channel):
        notes = self.notes[channel]
        s = []
        for n in notes:
            s.append(str(n))
            s.append('.' * (n.length()//4 - 4))
        return ''.join(s)

    def clear(self):
        self.notes = {
            Channel.Pulse1: [],
            Channel.Pulse2: [],
            Channel.Triangle: [],
            Channel.Noise: [],
        }

    def length(self, channel):
        return sum(map(lambda note: note.length(), self.notes[channel]))

    def total_length(self):
        return self.length(Channel.Pulse1)

    def add_notes(self, channel, notes):
        self.notes[channel].extend(notes)

    def pad_note_data(self, channel):
        if channel == Channel.Pulse1:
            return True
        else:
            x = self.length(channel)
            return x > 0 and x < self.total_length()

    def note_data(self, channel=None):
        if channel is not None:
            data = self.notes[channel][:]
            if self.pad_note_data(channel):
                data.append(0)
        else:
            data = []
            data.extend(self.note_data(Channel.Pulse1))
            data.extend(self.note_data(Channel.Pulse2))
            data.extend(self.note_data(Channel.Triangle))
            data.extend(self.note_data(Channel.Noise))
        return bytes(data)

    def meta_data(self, address):
        pw1 = len(self.note_data(Channel.Pulse1))
        pw2 = len(self.note_data(Channel.Pulse2))
        tri = len(self.note_data(Channel.Triangle))
        noi = len(self.note_data(Channel.Noise))

        address = address.addr()
        ret = bytearray()
        ret.append(self.tempo)
        ret.append(address & 255)
        ret.append(address >> 8)
        ret.append(0 if tri == 0 else pw1+pw2)
        ret.append(0 if pw2 == 0 else pw1)
        ret.append(0 if noi == 0 else pw1+pw2+tri)
        return bytes(ret)


    def read_notes(self, edit, channel, address):
        max_length = (MIDI_PPQN*64) if channel==Channel.Pulse1 else self.total_length()
        length = 0
        i = 0

        while length < max_length:
            note = Note(edit.read(address + i))
            i += 1
            if note == 0x00:
                break

            self.add_notes(channel, [note])
            # According to BGT, QuarterTriplet has special meaning when proceeded
            # by two EightTriplets and a special tempo flag is set.
            if len(self.notes[channel]) >= 3 and (
                self.notes[channel][-1].duration() == Note.QuarterTriplet and
                self.notes[channel][-2].duration() == Note.EighthTriplet and
                self.notes[channel][-3].duration() == Note.EighthTriplet):
                if self.tempo & 0x08:
                    self.notes[channel][-1] = self.notes[channel][-1].new_duration(Note.EighthTriplet)
                else:
                    self.notes[channel][-3] = self.notes[channel][-3].new_duration(Note.DottedEighth)
                    self.notes[channel][-2] = self.notes[channel][-2].new_duration(Note.DottedEighth)
                    self.notes[channel][-1] = self.notes[channel][-1].new_duration(Note.Eighth)

            length += self.notes[channel][-1].length()


class Song(object):
    @staticmethod
    def from_rom(edit, address, entry):
        table = edit.read_bytes(address, 8)
        offsets = {}
        sequence = []
        pattern = []
        i = 0
        n = 0
        while True:
            offset = edit.read(address + table[entry] + i)
            if offset == 0:
                break
            if offset not in offsets:
                offsets[offset] = n
                n += 1
                pattern.append(Pattern.from_rom(edit, address+offset))
            sequence.append(offsets[offset])
            i += 1

        return Song(pattern, sequence)

    @staticmethod
    def read_all_songs(edit):
        return {
            "OverworldIntro":  Song.from_rom(edit, OVERWORLD_SONG_TABLE, 0),
            "OverworldTheme":  Song.from_rom(edit, OVERWORLD_SONG_TABLE, 1),
            "BattleTheme":     Song.from_rom(edit, OVERWORLD_SONG_TABLE, 2),
            "CaveItemFanfare": Song.from_rom(edit, OVERWORLD_SONG_TABLE, 4),

            "TownIntro":       Song.from_rom(edit, TOWN_SONG_TABLE, 0),
            "TownTheme":       Song.from_rom(edit, TOWN_SONG_TABLE, 1),
            "HouseTheme":      Song.from_rom(edit, TOWN_SONG_TABLE, 2),
            "TownItemFanfare": Song.from_rom(edit, TOWN_SONG_TABLE, 4),

            "PalaceIntro":       Song.from_rom(edit, PALACE_SONG_TABLE, 0),
            "PalaceTheme":       Song.from_rom(edit, PALACE_SONG_TABLE, 1),
            "BossTheme":         Song.from_rom(edit, PALACE_SONG_TABLE, 3),
            "PalaceItemFanfare": Song.from_rom(edit, PALACE_SONG_TABLE, 4),
            "CrystalFanfare":    Song.from_rom(edit, PALACE_SONG_TABLE, 6),

            "GreatPalaceIntro":       Song.from_rom(edit, GP_SONG_TABLE, 0),
            "GreatPalaceTheme":       Song.from_rom(edit, GP_SONG_TABLE, 1),
            "ZeldaTheme":             Song.from_rom(edit, GP_SONG_TABLE, 2),
            "CreditsTheme":           Song.from_rom(edit, GP_SONG_TABLE, 3),
            "GreatPalaceItemFanfare": Song.from_rom(edit, GP_SONG_TABLE, 4),
            "TriforceFanfare":        Song.from_rom(edit, GP_SONG_TABLE, 5),
            "FinalBossTheme":         Song.from_rom(edit, GP_SONG_TABLE, 6),
        }

    @staticmethod
    def convert_all_to_famistudio(edit, filename='songs-fms.txt'):
        song = Song.read_all_songs(edit)
        with open(filename, 'wt') as f:
            print('Project Version="2.3.2" TempoMode="FamiStudio" Name="Z2 Extract" Author="Z2Edit-2.0" Copyright="N/A"', file=f)
            print(
# Some crap instruments.
"""	Instrument Name="Noise"
		Envelope Type="Volume" Length="5" Values="15,12,10,10,0"
	Instrument Name="Pulse1"
		Envelope Type="Volume" Length="1" Values="12"
	Instrument Name="Pulse2"
		Envelope Type="Volume" Length="1" Values="12"
	Instrument Name="Triangle"
		Envelope Type="Volume" Length="1" Values="15"
""", file=f)
            ov = song['OverworldTheme'].with_intro(song['OverworldIntro'])
            ov.as_famistudio('Overworld', fpqn=24, file=f)

            song['BattleTheme'].as_famistudio('Battle', fpqn=20, file=f)
            song['CaveItemFanfare'].as_famistudio('CaveItemFanfare', fpqn=24, file=f)

            town = song['TownTheme'].with_intro(song['TownIntro'])
            town.as_famistudio('Town', fpqn=28, file=f)
            song['HouseTheme'].as_famistudio('House', fpqn=28, file=f)
            song['TownItemFanfare'].as_famistudio('TownItemFanfare', fpqn=24, file=f)

            palace = song['PalaceTheme'].with_intro(song['PalaceIntro'])
            palace.as_famistudio('Palace', fpqn=24, file=f)
            song['BossTheme'].as_famistudio('Boss', fpqn=20, file=f)
            song['PalaceItemFanfare'].as_famistudio('PalaceItemFanfare', fpqn=24, file=f)
            song['CrystalFanfare'].as_famistudio('CrystalFanfare', fpqn=24, file=f)

            gp = song['GreatPalaceTheme'].with_intro(song['GreatPalaceIntro'])
            gp.as_famistudio('Great Palace', fpqn=24, file=f)
            song['GreatPalaceItemFanfare'].as_famistudio('GreatPalaceItemFanfare', fpqn=24, file=f)
            song['FinalBossTheme'].as_famistudio('FinalBoss', fpqn=20, file=f)
            song['TriforceFanfare'].as_famistudio('Triforce', fpqn=24, file=f)
            song['ZeldaTheme'].as_famistudio('Zelda', fpqn=24, file=f)
            song['CreditsTheme'].as_famistudio('Credits', fpqn=28, file=f)

    @staticmethod
    def convert_all_to_midi(edit):
        song = Song.read_all_songs(edit)
        song['OverworldIntro'].as_midi('overworld_intro', tempo=150)
        song['OverworldTheme'].as_midi('overworld', tempo=150)
        song['BattleTheme'].as_midi('battle', tempo=180)
        song['CaveItemFanfare'].as_midi('cave_item_fanfare', tempo=150)

        song['TownIntro'].as_midi('town_intro', tempo=128)
        song['TownTheme'].as_midi('town', tempo=128)
        song['HouseTheme'].as_midi('house', tempo=128)
        song['TownItemFanfare'].as_midi('town_item_fanfare', tempo=150)

        song['PalaceIntro'].as_midi('palace_intro', tempo=150)
        song['PalaceTheme'].as_midi('palace', tempo=150)
        song['BossTheme'].as_midi('boss', tempo=180)
        song['PalaceItemFanfare'].as_midi('palace_item_fanfare', tempo=150)
        song['CrystalFanfare'].as_midi('crystal_fanfare', tempo=150)

        song['GreatPalaceIntro'].as_midi('gp_intro', tempo=150)
        song['GreatPalaceTheme'].as_midi('gp', tempo=150)
        song['GreatPalaceItemFanfare'].as_midi('gp_item_fanfare', tempo=150)
        song['FinalBossTheme'].as_midi('final_boss', tempo=180)
        song['TriforceFanfare'].as_midi('triforce', tempo=150)
        song['ZeldaTheme'].as_midi('zelda', tempo=150)
        song['CreditsTheme'].as_midi('credits', tempo=128)

    @staticmethod
    def commit(edit, address, songs):
        if address == OVERWORLD_SONG_TABLE:
            table = bytes([0, 1, 2, 2, 3, 4, 4, 4])
        elif address == TOWN_SONG_TABLE:
            table = bytes([0, 1, 2, 2, 3, 4, 4, 4])
        elif address == PALACE_SONG_TABLE:
            table = bytes([0, 1, 1, 2, 3, 5, 4, 5])
        elif address == GP_SONG_TABLE:
            table = bytes([0, 1, 2, 3, 4, 5, 6, 7])
        else:
            raise Exception("Can't commit songs", address)

        if len(songs) != table[-1]:
            raise Exception("Expected exact number of songs", table[-1])

        # Calculate offsets table
        offset = 8
        offsets = bytearray()
        for s in songs:
            print("Offset for next song: %02x" % offset)
            offsets.append(offset)
            offset += len(s.sequence) + 1
        offsets.append(offset)

        for i in range(8):
            edit.write(address+i, offsets[table[i]])

        first_pattern = offset + 1
        seq_offset = 8
        pat_offset = first_pattern

        for s in songs:
            data = s.sequence_data(pat_offset)
            print("Writing seq at %02x with pat at %02x:" %(seq_offset, pat_offset), data.hex())
            edit.write_bytes(address + seq_offset, data)
            pat_offset += 6 * len(s.pattern)
            seq_offset += len(data)
        edit.write(address+seq_offset, 0)

        note_address = address + pat_offset
        pat_offset = first_pattern
        print("Note data to start at ", note_address)
        for s in songs:
            for p in s.pattern:
                data = p.note_data()
                print("Pattern at %s, notes at %s" % (address+pat_offset, note_address))
                edit.write_bytes(address+pat_offset, p.meta_data(note_address))
                edit.write_bytes(note_address, data)
                pat_offset += 6
                note_address += len(data)


    def __init__(self, pattern=[], sequence=[]):
        self.pattern = pattern
        self.sequence = sequence
        self.loop_point = 0

    def sequence_data(self, first):
        ret = bytearray()
        for n in self.sequence:
            ret.append(first + n*6)
        ret.append(0)
        return bytes(ret)

    def metadata_length(self):
        return len(self.sequence) + 1 + 6*len(self.pattern)

    def dump(self):
        for p in self.sequence:
            pattern = self.pattern[p]
            print(pattern.to_string(Channel.Pulse1))
            print(pattern.to_string(Channel.Pulse2))
            print(pattern.to_string(Channel.Triangle))
            print(pattern.to_string(Channel.Noise))
            print()


    def most_common_length(self):
        length = defaultdict(int)
        for p in self.pattern:
            length[p.total_length()] += 1
        lengths = sorted(length.items(), key=lambda x: x[1], reverse=True)
        return lengths[0][0]

    def pattern_as_famistudio(self, n, channel, length, fpqn, beat_len, file=None):
        print('\t\t\tPattern Name="{}-{}"'.format(channel, n), file=file)
        time = 0
        total = self.pattern[n].total_length()
        while time < total:
            for i, note in enumerate(self.pattern[n].notes[channel]):
                name = str(note).replace('.', '')
                if channel != Channel.Noise:
                    name = name.replace('3', '2').replace('4', '3').replace('5', '4')

                endtime = time + note.length() * fpqn // MIDI_PPQN
                if name == '---' or name == '????':
                    print('\t\t\t\tNote Time="{}" Value="Stop"'.format(time, name), file=file)
                else:
                    print('\t\t\t\tNote Time="{}" Value="{}" Instrument="{}"'.format(time, name, channel), file=file)
                    if i+1 < len(self.pattern[n].notes[channel]):
                        nextnote = self.pattern[n].notes[channel][i+1]
                        if channel != Channel.Noise and nextnote.pitch() == note.pitch():
                            print('\t\t\t\tNote Time="{}" Value="Stop"'.format(endtime-1, name), file=file)
                time = endtime
            if channel != Channel.Noise or time == 0:
                # The noise channel needs to loop, the others do not.
                break

    def pattern_as_midi(self, n, channel, track):
        time = 0
        total = self.pattern[n].total_length()
        ch = channel.as_midi()
        delay = 0
        midinote = 0
        prevnote = 0
        while time < total:
            for i, note in enumerate(self.pattern[n].notes[channel]):
                endtime = time + note.length()
                if channel == Channel.Noise:
                    midinote = 38 # Snare drum
                else:
                    midinote = note.as_midi()
                if prevnote:
                    track.append(mido.Message('note_off', channel=ch, note=prevnote, time=delay))
                    delay = 0
                if midinote:
                    if channel == Channel.Triangle:
                        # Fuck yeah I think it should be two octaves down.
                        midinote -= 24
                    track.append(mido.Message('note_on', channel=ch, note=midinote, time=delay))
                    delay = note.length()
                else:
                    midinote = 1
                    # Musescore interprets this as a rest.
                    track.append(mido.Message('note_off', channel=ch, note=1, velocity=0, time=delay))
                    delay = note.length()
                prevnote = midinote
                time = endtime
            if channel != Channel.Noise or time == 0:
                # The noise channel needs to loop, the others do not.
                break
        if prevnote:
            track.append(mido.Message('note_off', channel=ch, note=prevnote, time=delay))
            delay = 0

    def channel_as_famistudio(self, channel, length, fpqn, beat_len, file=None):
        print('\t\tChannel Type="{}"'.format(channel.as_famistudio()), file=file)
        for i, n in enumerate(self.sequence):
            p = self.pattern[n]
            plen = p.total_length() * beat_len // MIDI_PPQN
            if plen != length:
                print('\t\tPatternCustomSettings Time="{}" Length="{}" NoteLength="{}" BeatLength="{}"'.format(i, plen, fpqn//beat_len, beat_len), file=file)

        for n in range(len(self.pattern)):
            self.pattern_as_famistudio(n, channel, length, fpqn, beat_len, file=file)
        for i, n in enumerate(self.sequence):
            print('\t\t\tPatternInstance Time="{}" Pattern="{}-{}"'.format(i, channel, n), file=file)

    def channel_as_midi(self, channel, track):
        track.append(mido.Message('program_change', channel=channel.as_midi(), program=channel.as_midi_instrument()))
        for n in self.sequence:
            self.pattern_as_midi(n, channel, track)

    def as_famistudio(self, name, fpqn=32, beat_len=4, file=None):
        mcl = self.most_common_length() * beat_len // MIDI_PPQN

        print('\tSong Name="{}" Length="{}" LoopPoint="{}" PatternLength="{}" BeatLength="{}" NoteLength="{}"'.format(
            name, len(self.sequence), self.loop_point, mcl, beat_len, fpqn // beat_len), file=file)

        self.channel_as_famistudio(Channel.Pulse1, mcl, fpqn, beat_len, file=file)
        self.channel_as_famistudio(Channel.Pulse2, mcl, fpqn, beat_len, file=file)
        self.channel_as_famistudio(Channel.Triangle, mcl, fpqn, beat_len, file=file)
        self.channel_as_famistudio(Channel.Noise, mcl, fpqn, beat_len, file=file)

    def as_midi(self, name, tempo=120):
        if not mido:
            raise Exception("Need `mido` library to convert to MIDI")
        midi = mido.MidiFile(type=1, ticks_per_beat=MIDI_PPQN)
        # Flip channels 1 and 2 because of how Z2 encodes the pulse channels.
        for c in (2,1,3,4):
            channel = Channel(c)
            track = midi.add_track()
            track.append(mido.MetaMessage('track_name', name=str(channel)))
            track.append(mido.MetaMessage('set_tempo', tempo=mido.bpm2tempo(tempo)))
            self.channel_as_midi(channel, track)

        #midi.print_tracks()
        name = "{}.mid".format(name)
        midi.save(name)

    def with_intro(self, intro):
        song = Song(intro.pattern[:], intro.sequence[:])
        song.loop_point = len(intro.sequence)
        song.pattern.extend(self.pattern)
        song.sequence.extend(x+song.loop_point for x in self.sequence)
        return song
