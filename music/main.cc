#include "music.h"

#include <iostream>

void dump_notes(std::vector<z2music::Note> notes) {
  for (z2music::Note n : notes) {
    std::cerr << n.pitch_string();
    size_t left = n.length() / 4 - 4;
    std::cerr << std::string(left, '.');
  }
  std::cerr << std::endl;
}

void dump_song(const z2music::Song& song) {
  std::cerr << "Song length: " << song.sequence_length() << " phrases" << std::endl;

  for (size_t i = 0; i < song.sequence_length(); ++i) {
    const z2music::Pattern* p = song.at(i);
    dump_notes(p->notes(z2music::Pattern::Channel::Pulse1));
    dump_notes(p->notes(z2music::Pattern::Channel::Pulse2));
    dump_notes(p->notes(z2music::Pattern::Channel::Triangle));
    dump_notes(p->notes(z2music::Pattern::Channel::Noise));
    std::cerr << std::endl;
  }
}

int main(int argc, char** argv) {
  if (argc != 2) {
    std::cerr << "Usage: " << argv[0] << " z2_rom" << std::endl;
    return 1;
  }

  const std::string file = std::string(argv[1]);

  z2music::Rom rom(file);
  z2music::Song* battle = rom.song(z2music::Rom::SongTitle::BattleTheme);
  dump_song(*battle);

  rom.save("/tmp/output.nes");

  return 0;
}
