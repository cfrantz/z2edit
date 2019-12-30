#include "music.h"

#include <array>
#include <cassert>
#include <fstream>
#include <map>

namespace z2music {

template <typename E>
constexpr uint8_t to_byte(E e) noexcept {
  return static_cast<std::uint8_t>(e);
}

Note::Note(uint8_t value) : value_(value) {}

Note::Note(Duration d, Pitch p) : value_(to_byte(d) | to_byte(p)) {}

Note::Duration Note::duration() const {
  return static_cast<Duration>(value_ & 0xc1);
}

void Note::duration(Note::Duration d) {
  value_ = to_byte(d) | to_byte(pitch());
}

Note::Pitch Note::pitch() const {
  return static_cast<Pitch>(value_ & 0x3e);
}

void Note::pitch(Note::Pitch p) {
  value_ = to_byte(duration()) | to_byte(p);
}

size_t Note::length() const {
  // MIDI generally uses 96 ppqn, so we'll use the same
  switch (duration()) {
    case Duration::Sixteenth:       return 96 / 4;
    case Duration::DottedQuarter:   return 96 * 3/2;
    case Duration::DottedEighth:    return 96 / 2 * 3/2;
    case Duration::Half:            return 96 * 2;
    case Duration::Eighth:          return 96 / 2;
    case Duration::EighthTriplet:   return 96 / 2 * 2/3;
    case Duration::Quarter:         return 96;
    case Duration::QuarterTriplet:  return 96 * 2/3;
  }

  return 0;
}

std::string Note::pitch_string() const {
  switch (pitch()) {
    case Pitch::Rest: return "---.";
    case Pitch::E3:   return "E3..";
    case Pitch::G3:   return "G3..";
    case Pitch::Gs3:  return "G#3.";
    case Pitch::A3:   return "A3..";
    case Pitch::As3:  return "A#3.";
    case Pitch::B3:   return "B3..";
    case Pitch::C4:   return "C4..";
    case Pitch::Cs4:  return "C#4.";
    case Pitch::D4:   return "D4..";
    case Pitch::Ds4:  return "D#4.";
    case Pitch::E4:   return "E4..";
    case Pitch::F4:   return "F4..";
    case Pitch::Fs4:  return "F#4.";
    case Pitch::G4:   return "G4..";
    case Pitch::Gs4:  return "G#4.";
    case Pitch::A4:   return "A4..";
    case Pitch::As4:  return "A#4.";
    case Pitch::B4:   return "B4..";
    case Pitch::C5:   return "C5..";
    case Pitch::Cs5:  return "C#5.";
    case Pitch::D5:   return "D5..";
    case Pitch::Ds5:  return "D#5.";
    case Pitch::E5:   return "E5..";
    case Pitch::F5:   return "F5..";
    case Pitch::Fs5:  return "F#5.";
    case Pitch::G5:   return "G5..";
    case Pitch::A5:   return "A5..";
    case Pitch::As5:  return "A#5.";
    case Pitch::B5:   return "B5..";
    case Pitch::C6:   return "C6..";
  }

  return "???.";
}

Note::operator uint8_t() const {
  return value_;
}

Pattern::Pattern() : tempo_(0x18) {
  clear();
}

Pattern::Pattern(const Rom& rom, size_t address) {
  clear();

  uint8_t header[6];
  rom.read(header, address, 6);

  tempo_ = header[0];

  size_t note_base = (header[2] << 8) + header[1] + 0x10000;

  read_notes(Channel::Pulse1, rom, note_base);

  if (header[3] > 0) read_notes(Channel::Triangle, rom, note_base + header[3]);
  if (header[4] > 0) read_notes(Channel::Pulse2, rom, note_base + header[4]);
  if (header[5] > 0) read_notes(Channel::Noise, rom, note_base + header[5]);
}

Pattern::Pattern(uint8_t tempo,
    std::initializer_list<Note> pw1,
    std::initializer_list<Note> pw2,
    std::initializer_list<Note> triangle,
    std::initializer_list<Note> noise):
tempo_(tempo) {
  clear();
  add_notes(Channel::Pulse1, pw1);
  add_notes(Channel::Pulse2, pw2);
  add_notes(Channel::Triangle, triangle);
  add_notes(Channel::Noise, noise);
}

size_t Pattern::length() const {
  return length(Channel::Pulse1);
}

void Pattern::add_notes(Pattern::Channel ch, std::initializer_list<Note> notes) {
  for (auto n : notes) {
    notes_[ch].push_back(n);
  }
}

void Pattern::clear() {
  notes_[Channel::Pulse1].clear();
  notes_[Channel::Pulse2].clear();
  notes_[Channel::Triangle].clear();
  notes_[Channel::Noise].clear();
}

std::vector<Note> Pattern::notes(Pattern::Channel ch) const {
  return notes_.at(ch);
}

void Pattern::tempo(uint8_t tempo) {
  tempo_ = tempo;
}

uint8_t Pattern::tempo() const {
  return tempo_;
}

bool Pattern::validate() const {
  // TODO validate pattern

  // check that pw1 length is <= 16 quarter notes
  // check that other lengths are equal or valid partial lengths
  // TODO does the game handle unusual lengths?  maybe this isn't really needed

  return true;
}

std::vector<uint8_t> Pattern::note_data() const {
  std::vector<uint8_t> b;

  const std::array<Channel, 4> channels = {
    Channel::Pulse1,
    Channel::Pulse2,
    Channel::Triangle,
    Channel::Noise,
  };

  for (auto ch : channels) {
    const std::vector<uint8_t> c = note_data(ch);
    b.insert(b.end(), c.begin(), c.end());
  }

  return b;
}

std::vector<uint8_t> Pattern::meta_data(size_t notes) const {
  // FIXME calculate which channels need extra bytes :(
  const size_t pw1 = note_data_length(Channel::Pulse1);
  const size_t pw2 = note_data_length(Channel::Pulse2);
  const size_t tri = note_data_length(Channel::Triangle);
  const size_t noi = note_data_length(Channel::Noise);

  std::vector<uint8_t> b;
  b.reserve(6);

  b.push_back(tempo_);
  b.push_back(notes % 256);
  b.push_back(notes >> 8);
  b.push_back(tri == 0 ? 0 : pw1 + pw2);
  b.push_back(pw2 == 0 ? 0 : pw1);
  b.push_back(noi == 0 ? 0 : pw1 + pw2 + tri);

  return b;
}

size_t Pattern::length(Pattern::Channel ch) const {
  size_t length = 0;
  for (auto n : notes_.at(ch)) {
    length += n.length();
  }
  return length;
}

bool Pattern::pad_note_data(Pattern::Channel ch) const {
  if (ch == Channel::Pulse1) return true;

  const size_t l = length(ch);
  return l > 0 && l < length();
}

std::vector<uint8_t> Pattern::note_data(Pattern::Channel ch) const {
  std::vector<uint8_t> b;
  b.reserve(notes_.at(ch).size() + 1);

  for (auto n : notes_.at(ch)) {
    b.push_back(n);
  }
  if (pad_note_data(ch)) b.push_back(0);

  return b;
}

size_t Pattern::note_data_length(Pattern::Channel ch) const {
  return notes_.at(ch).size() + (pad_note_data(ch) ? 1 : 0);
}

void Pattern::read_notes(Pattern::Channel ch, const Rom& rom, size_t address) {
  const size_t max_length = ch == Channel::Pulse1 ? 64 * 96 : length();
  size_t length = 0;

  fprintf(stderr, "Reading note data at %06lx\n", address);

  while (length < max_length) {
    Note n = Note(rom.getc(address++));
    // Note data can terminate early on 00 byte
    if (n == 0x00) {
      fprintf(stderr, "Terminating early on 00 note\n");
      break;
    }

    length += n.length();
    add_notes(ch, {n});

    // The QuarterTriplet duration has special meaning when preceeded by
    // two EighthTriplets, which differs based on a tempo flag.
    if (n.duration() == Note::Duration::QuarterTriplet) {
      const size_t i = notes_[ch].size() - 3;
      if (notes_[ch][i + 0].duration() == Note::Duration::EighthTriplet &&
          notes_[ch][i + 1].duration() == Note::Duration::EighthTriplet) {
        if (tempo_ & 0x08) {
          // If flag 0x08 is set, just count 0xc1 as a third EighthTriplet
          notes_[ch][i + 2].duration(Note::Duration::EighthTriplet);
        } else {
          // If flag 0x08 is not set, rewrite the whole sequence
          notes_[ch][i + 0].duration(Note::Duration::DottedEighth);
          notes_[ch][i + 1].duration(Note::Duration::DottedEighth);
          notes_[ch][i + 2].duration(Note::Duration::Eighth);
        }
      }
    }
  }
}

Song::Song() {}

Song::Song(const Rom& rom, size_t address, size_t entry) {
  if (entry > 7) return;

  uint8_t table[8];
  rom.read(table, address, 8);

  std::unordered_map<uint8_t, size_t> offset_map;
  std::vector<size_t> seq;
  size_t n = 0;

  for (size_t i = 0; true; ++i) {
    uint8_t offset = rom.getc(address + table[entry] + i);

    if (offset == 0) break;
    if (offset_map.find(offset) == offset_map.end()) {
      offset_map[offset] = n++;
      add_pattern(Pattern(rom, address + offset));
    }
    append_sequence(offset_map.at(offset));
  }
}

void Song::add_pattern(const Pattern& pattern) {
  patterns_.push_back(pattern);
}

void Song::set_sequence(const std::vector<size_t>& seq) {
  sequence_ = seq;
}

void Song::append_sequence(size_t n) {
  sequence_.push_back(n);
}

std::vector<uint8_t> Song::sequence_data(uint8_t first) const {
  std::vector<uint8_t> b;
  b.reserve(sequence_.size());

  for (size_t n : sequence_) {
    b.push_back(first + n * 6);
  }

  b.push_back(0);

  return b;
}

size_t Song::sequence_length() const {
  return sequence_.size();
}

size_t Song::pattern_count() const {
  return patterns_.size();
}

size_t Song::metadata_length() const {
  return sequence_length() + 1 + 6 * pattern_count();
}

void Song::clear() {
  patterns_.clear();
  sequence_.clear();
}

std::vector<Pattern> Song::patterns() {
  return patterns_;
}

Pattern* Song::at(size_t i) {
  if (i < 0 || i >= sequence_.size()) return nullptr;
  return &(patterns_.at(sequence_.at(i)));
}

const Pattern* Song::at(size_t i) const {
  if (i < 0 || i >= sequence_.size()) return nullptr;
  return &(patterns_.at(sequence_.at(i)));
}

Rom::Rom(const std::string& filename) {
  std::ifstream file(filename, std::ios::binary);
  if (file.is_open()) {
    file.read(reinterpret_cast<char *>(&header_[0]), kHeaderSize);
    file.read(reinterpret_cast<char *>(&data_[0]), kRomSize);

    songs_[SongTitle::OverworldIntro] = Song(*this, kOverworldSongTable, 0);
    songs_[SongTitle::OverworldTheme] = Song(*this, kOverworldSongTable, 1);
    songs_[SongTitle::BattleTheme] = Song(*this, kOverworldSongTable, 2);
    songs_[SongTitle::CaveItemFanfare] = Song(*this, kOverworldSongTable, 4);

    songs_[SongTitle::TownIntro] = Song(*this, kTownSongTable, 0);
    songs_[SongTitle::TownTheme] = Song(*this, kTownSongTable, 1);
    songs_[SongTitle::HouseTheme] = Song(*this, kTownSongTable, 2);
    songs_[SongTitle::TownItemFanfare] = Song(*this, kTownSongTable, 4);

    songs_[SongTitle::PalaceIntro] = Song(*this, kPalaceSongTable, 0);
    songs_[SongTitle::PalaceTheme] = Song(*this, kPalaceSongTable, 1);
    songs_[SongTitle::BossTheme] = Song(*this, kPalaceSongTable, 3);
    songs_[SongTitle::PalaceItemFanfare] = Song(*this, kPalaceSongTable, 4);
    songs_[SongTitle::CrystalFanfare] = Song(*this, kPalaceSongTable, 6);

    songs_[SongTitle::GreatPalaceIntro] = Song(*this, kGreatPalaceSongTable, 0);
    songs_[SongTitle::GreatPalaceTheme] = Song(*this, kGreatPalaceSongTable, 1);
    songs_[SongTitle::ZeldaTheme] = Song(*this, kGreatPalaceSongTable, 2);
    songs_[SongTitle::CreditsTheme] = Song(*this, kGreatPalaceSongTable, 3);
    songs_[SongTitle::GreatPalaceItemFanfare] = Song(*this, kGreatPalaceSongTable, 4);
    songs_[SongTitle::TriforceFanfare] = Song(*this, kGreatPalaceSongTable, 5);
    songs_[SongTitle::FinalBossTheme] = Song(*this, kGreatPalaceSongTable, 6);
  }
}

uint8_t Rom::getc(size_t address) const {
  if (address > kRomSize) return 0xff;
  return data_[address];
}

void Rom::read(uint8_t* buffer, size_t address, size_t length) const {
  fprintf(stderr, "Reading %02lx bytes at %06lx\n", length, address);
  // Could use std::copy or std::memcpy but this handles out of range addresses
  for (size_t i = 0; i < length; ++i) {
    buffer[i] = getc(address + i);
  }
}

void Rom::putc(size_t address, uint8_t data) {
  if (address > kRomSize) return;
  data_[address] = data;
}

void Rom::write(size_t address, std::vector<uint8_t> data) {
  /* fprintf(stderr, "Write %lu bytes at %06lx\n", data.size(), address); */
  for (size_t i = 0; i < data.size(); ++i) {
    putc(address + i, data[i]);
  }
}

bool Rom::commit() {
  // TODO check if committing worked

  commit(kOverworldSongTable, {
      SongTitle::OverworldIntro,
      SongTitle::OverworldTheme,
      SongTitle::BattleTheme,
      SongTitle::CaveItemFanfare});

  commit(kTownSongTable, {
      SongTitle::TownIntro,
      SongTitle::TownTheme,
      SongTitle::HouseTheme,
      SongTitle::TownItemFanfare});

  commit(kPalaceSongTable, {
      SongTitle::PalaceIntro,
      SongTitle::PalaceTheme,
      SongTitle::BossTheme,
      SongTitle::PalaceItemFanfare,
      SongTitle::CrystalFanfare});

  commit(kGreatPalaceSongTable, {
      SongTitle::GreatPalaceIntro,
      SongTitle::GreatPalaceTheme,
      SongTitle::ZeldaTheme,
      SongTitle::CreditsTheme,
      SongTitle::GreatPalaceItemFanfare,
      SongTitle::TriforceFanfare,
      SongTitle::FinalBossTheme});

  return true;
}

void Rom::save(const std::string& filename) {
  if (commit()) {
    std::ofstream file(filename, std::ios::binary);
    if (file.is_open()) {
      file.write(reinterpret_cast<char *>(&header_[0]), kHeaderSize);
      file.write(reinterpret_cast<char *>(&data_[0]), kRomSize);
    }
  }
}

Song* Rom::song(Rom::SongTitle title) {
  return &songs_[title];
}

void Rom::commit(size_t address, std::initializer_list<Rom::SongTitle> songs) {
  // TODO commit song data to PRG ROM
  std::array<uint8_t, 8> table;

  // TODO make these changeable.
  // This will require rearchitecting things so that there is a Score object
  // which is a list of 8 (possibly duplicate) songs.  For now, it's just 
  // hardcode which songs are where in each table.
  switch (address) {
    case kOverworldSongTable:
    case kTownSongTable:
      table = {0, 1, 2, 2, 3, 4, 4, 4 };
      break;

    case kPalaceSongTable:
      table = { 0, 1, 1, 2, 3, 5, 4, 5 };
      break;

    case kGreatPalaceSongTable:
      table = { 0, 1, 2, 3, 4, 5, 6, 7 };
      break;

    default:
      // Other values are nonsense
      return;
  }

  /**************
   * SONG TABLE *
   **************/

  uint8_t offset = 8;
  std::vector<uint8_t> offsets;
  offsets.reserve(8);

  // Calculate song offset table
  for (auto s : songs) {
    offsets.push_back(offset);
    fprintf(stderr, "Offset for next song: %02x\n", offset);
    offset += songs_.at(s).sequence_length() + 1;
  }

  // One extra offset for the "empty" song at the end
  // We could save a whole byte by pointing this at the end of some other
  // sequence but it's kind of nice to see the double 00 to mean the end of the
  // sequence data.
  offsets.push_back(offset);

  // Write song table to ROM
  for (size_t i = 0; i < 8; ++i) {
    putc(address + i, offsets[table[i]]);
  }

  /******************
   * SEQUENCE TABLE *
   ******************/

  const uint8_t first_pattern = offset + 1;
  uint8_t seq_offset = 8;
  uint8_t pat_offset = first_pattern;

  for (auto s : songs) {
    fprintf(stderr, "Writing seq at %02x with pat at %02x: ", seq_offset, pat_offset);
    const std::vector<uint8_t> seq = songs_.at(s).sequence_data(pat_offset);
    write(address + seq_offset, seq);

    for (auto b : seq) fprintf(stderr, "%02x ", b);
    fprintf(stderr, "\n");

    pat_offset += 6 * songs_.at(s).pattern_count();
    seq_offset += seq.size();
  }

  // Write an empty sequence for the empty song
  putc(address + seq_offset, 0);

  /*******************************
   * PATTERN TABLE AND NOTE DATA *
   *******************************/

  size_t note_address = pat_offset + address;
  pat_offset = first_pattern;

  fprintf(stderr, "Note data to start at %06lx\n", note_address);

  for (auto s : songs) {
    for (auto p : songs_.at(s).patterns()) {
      const std::vector<uint8_t> note_data = p.note_data();

      fprintf(stderr, "Pattern at %06lx, notes at %06lx\n", address + pat_offset, note_address);

      write(address + pat_offset, p.meta_data(note_address));
      write(note_address, note_data);

      pat_offset += 6;
      note_address += note_data.size();
    }
  }
}

} // namespace z2music
