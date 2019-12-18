#include "music.h"

#include <fstream>
#include <algorithm>

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

Note::Pitch Note::pitch() const {
  return static_cast<Pitch>(value_ & 0x3e);
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

void Pattern::set_tempo(uint8_t tempo) {
  tempo_ = tempo;
}

uint8_t Pattern::tempo() const {
  return tempo_;
}

bool Pattern::validate() const {
  return true;
}

void Pattern::write_notes(Rom* rom, size_t offset) const {
  // TODO write note data to rom
}

void Pattern::write_meta(Rom* rom, size_t offset, size_t notes) const {
  // TODO write metadata to rom
}

void Pattern::read_notes(Pattern::Channel ch, const Rom& rom, size_t address) {
  size_t max_length = ch == Channel::Pulse1 ? 16 * 96 : length();
  size_t length = 0;

  while (length < max_length) {
    Note n = Note(rom.getc(address++));
    // Note data can terminate early on 00 byte
    if (n.encode() == 0x00) break;
    length += n.length();
    add_notes(ch, {n});
  }
}

Song::Song() {}

Song::Song(const Rom& rom, size_t address, size_t entry) {
  if (entry > 7) return;

  uint8_t table[8];
  rom.read(table, address, 8);

  std::unordered_map<uint8_t, Pattern> patterns;
  std::unordered_map<uint8_t, size_t> map;
  std::vector<size_t> seq;
  size_t n = 0;
  for (size_t i = 0; true; ++i) {
    uint8_t offset = rom.getc(table[entry] + i);
    if (offset == 0) break;
    if (patterns.find(offset) == patterns.end()) {
      map[offset] = n++;
      patterns[offset] = Pattern(rom, address + offset);
      seq.push_back(map[offset]);
    }
  }

  for (const auto& pattern : patterns) {
    add_pattern(pattern.second);
  }
  /* set_sequence(seq); */
}

void Song::add_pattern(const Pattern& pattern) {
  patterns_.push_back(pattern);
}

void Song::set_sequence(std::initializer_list<size_t> seq) {
  sequence_.clear();
  for (size_t i : seq) {
    sequence_.push_back(i);
  }
}

void Song::write_sequnce(Rom* rom, size_t offset) const {
  // TODO write sequence to rom
}

size_t Song::sequence_length() const {
  return sequence_.size();
}

Pattern* Song::at(size_t i) {
  if (i < 0 || i >= sequence_.size()) return nullptr;
  return &(patterns_.at(sequence_.at(i)));
}

Rom::Rom(const std::string& filename) {
  std::ifstream file(filename, std::ios::binary);
  if (file.is_open()) {
    file.seekg(0x01a010);
    file.read(reinterpret_cast<char *>(&data_[0]), 0x2000 * sizeof(data_[0]));
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

void Rom::write(const std::string& filename) {
  // TODO write all the shit back to the ROM
}

} // namespace z2music
