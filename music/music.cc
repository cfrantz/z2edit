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
  // TODO parse pattern
}

size_t Pattern::length() const {
  size_t length = 0;
  for (auto n : notes_.at(Channel::Pulse1)){
    length += n.length();
  }
  return length;
}

void Pattern::add_notes(Pattern::Channel ch, std::initializer_list<Note> notes) {
  for (auto n : notes) {
    notes_[ch].push_back(n);
  }
}

void Pattern::write_notes(Rom* rom, size_t offset) const {
  // TODO write note data to rom
}

void Pattern::write_meta(Rom* rom, size_t offset, size_t notes) const {
  // TODO write metadata to rom
}

Song::Song() {}

Song::Song(const Rom& rom, size_t address) {
  // TODO parse song from rom
}

size_t Song::add_pattern(const Pattern& pattern) {
  patterns_.push_back(pattern);
  size_t index = patterns_.size() - 1;
  sequence_.push_back(index);
  return index;
}

void Song::repeat_pattern(size_t index) {
  sequence_.push_back(index);
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
