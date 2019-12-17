#include "music.h"

#include <fstream>
#include <cstdio>

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

uint8_t Note::encode() const {
  return value_;
}

Pattern::Pattern() : tempo_(0x18) {}

Pattern::Pattern(const Rom& rom, size_t address) {
  uint8_t header[6];
  rom.read(header, address, 6);

  tempo_ = header[0];

  size_t note_base = header[2] << 8 | header[1];

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
  // TODO Implement validator
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

Song::Song(const Rom& rom, size_t address) {
  // TODO parse song from rom
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
  // TODO memcpy
}

void Rom::write(const std::string& filename) {
  // TODO write all the shit back to the ROM
}

} // namespace z2music
