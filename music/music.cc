#include "music.h"

#include <algorithm>
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

uint8_t Note::encode() const {
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

  size_t note_base = (header[2] << 8) + header[1] + 0x10010;

  read_notes(Channel::Pulse1, rom, note_base);

  if (header[3] > 0) read_notes(Channel::Triangle, rom, note_base + header[3]);
  if (header[4] > 0) read_notes(Channel::Pulse2, rom, note_base + header[4]);
  if (header[5] > 0) read_notes(Channel::Noise, rom, note_base + header[5]);
}

size_t Pattern::length() const {
  size_t length = 0;
  for (auto n : notes_.at(Channel::Pulse1)) {
    length += n.length();
  }
  return length;
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

void Pattern::write_notes(Rom& rom, size_t offset) const {
  const size_t pw1 = notes_.at(Channel::Pulse1).size();
  const size_t pw2 = notes_.at(Channel::Pulse2).size();
  const size_t tri = notes_.at(Channel::Triangle).size();

  write_channel(Channel::Pulse1, rom, offset);
  write_channel(Channel::Pulse2, rom, offset + pw1);
  write_channel(Channel::Triangle, rom, offset + pw1 + pw2);
  write_channel(Channel::Noise, rom, offset + pw1 + pw2 + tri);
}

void Pattern::write_meta(Rom& rom, size_t offset, size_t notes) const {
  const size_t pw1 = notes_.at(Channel::Pulse1).size();
  const size_t pw2 = notes_.at(Channel::Pulse2).size();
  const size_t tri = notes_.at(Channel::Triangle).size();

  rom.putc(offset + 0, tempo_);
  rom.putc(offset + 1, notes % 256);
  rom.putc(offset + 2, notes >> 8);
  rom.putc(offset + 3, notes_.at(Channel::Triangle).empty() ? 0 : pw1 + pw2);
  rom.putc(offset + 4, notes_.at(Channel::Pulse2).empty() ? 0 : pw1);
  rom.putc(offset + 5, notes_.at(Channel::Noise).empty() ? 0 : pw1 + pw2 + tri);
}

void Pattern::read_notes(Pattern::Channel ch, const Rom& rom, size_t address) {
  const size_t max_length = ch == Channel::Pulse1 ? 16 * 96 : length();
  size_t length = 0;

  while (length < max_length) {
    Note n = Note(rom.getc(address++));
    // Note data can terminate early on 00 byte
    if (n.encode() == 0x00) break;

    length += n.length();
    add_notes(ch, {n});

    // If a QuarterTriplet is preceeded by two EighthTriplets, there is special
    // meaning.  In this case, the three notes should really be DottedEighth
    // DottedEighth Eighth.  This doesn't change the overall length of the
    // pattern, but the duration bits need to be rewritten.
    if (n.duration() == Note::Duration::QuarterTriplet) {
      const size_t i = notes_[ch].size() - 3;
      if (notes_[ch][i + 0].duration() == Note::Duration::EighthTriplet &&
          notes_[ch][i + 1].duration() == Note::Duration::EighthTriplet) {

        notes_[ch][i + 0].duration(Note::Duration::DottedEighth);
        notes_[ch][i + 1].duration(Note::Duration::DottedEighth);
        notes_[ch][i + 2].duration(Note::Duration::Eighth);
      }
    }
  }
}

void Pattern::write_channel(Pattern::Channel ch, Rom& rom, size_t offset) const {
  for (size_t i = 0; i < notes_.at(ch).size(); ++i) {
    rom.putc(offset + i, notes_.at(ch).at(i).encode());
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

void Song::write_sequnce(Rom& rom, size_t offset) const {
  // TODO write sequence to rom
}

size_t Song::sequence_length() const {
  return sequence_.size();
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
    file.seekg(0x01a010);
    file.read(reinterpret_cast<char *>(&data_[0]), 0x2000 * sizeof(data_[0]));

    songs_[SongTitle::OverworldIntro] = Song(*this, 0x1a010, 0);
    songs_[SongTitle::OverworldTheme] = Song(*this, 0x1a010, 1);
    songs_[SongTitle::BattleTheme] = Song(*this, 0x1a010, 2);
    songs_[SongTitle::CaveItemFanfare] = Song(*this, 0x1a010, 4);

    songs_[SongTitle::TownIntro] = Song(*this, 0x1a3da, 0);
    songs_[SongTitle::TownTheme] = Song(*this, 0x1a3da, 1);
    songs_[SongTitle::HouseTheme] = Song(*this, 0x1a3da, 2);
    songs_[SongTitle::TownItemFanfare] = Song(*this, 0x1a3da, 4);

    songs_[SongTitle::PalaceIntro] = Song(*this, 0x1a63f, 0);
    songs_[SongTitle::PalaceTheme] = Song(*this, 0x1a63f, 1);
    songs_[SongTitle::BossTheme] = Song(*this, 0x1a63f, 3);
    songs_[SongTitle::PalaceItemFanfare] = Song(*this, 0x1a63f, 4);
    songs_[SongTitle::CrystalFanfare] = Song(*this, 0x1a63f, 6);

    songs_[SongTitle::GreatPalaceIntro] = Song(*this, 0x1a946, 0);
    songs_[SongTitle::GreatPalaceTheme] = Song(*this, 0x1a946, 1);
    songs_[SongTitle::ZeldaTheme] = Song(*this, 0x1a946, 2);
    songs_[SongTitle::CreditsTheme] = Song(*this, 0x1a946, 3);
    songs_[SongTitle::GreatPalaceItemFanfare] = Song(*this, 0x1a946, 4);
    songs_[SongTitle::TriforceFanfare] = Song(*this, 0x1a946, 5);
    songs_[SongTitle::FinalBossTheme] = Song(*this, 0x1a946, 6);
  }
}

uint8_t Rom::getc(size_t address) const {
  if (address < 0x1a010 || address > 0x1c00f) return 0xff;
  return data_[address - 0x1a010];
}

void Rom::read(uint8_t* buffer, size_t address, size_t length) const {
  // Could use std::copy or std::memcpy but this handles out of range addresses
  for (size_t i = 0; i < length; ++i) {
    buffer[i] = getc(address + i);
  }
}

void Rom::putc(size_t address, uint8_t data) {
  if (address < 0x1a010 || address > 0x1c00f) return;
  data_[address - 0x1a010] = data;
}

void Rom::write(size_t address, std::vector<uint8_t> data) {
  for (size_t i = 0; i < data.size(); ++i) {
    putc(address + i, data[i]);
  }
}

void Rom::save(const std::string& filename) {
  // TODO write all the shit back to the ROM
}

Song* Rom::song(Rom::SongTitle title) {
  return &songs_[title];
}

} // namespace z2music
