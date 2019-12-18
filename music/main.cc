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

int main(void) {
  z2music::Rom rom("/home/alan/source/z2-music/z2.nes");
  z2music::Song song(rom, 0x01a946, 5);

  std::cerr << "Song length: " << song.sequence_length() << " phrases" << std::endl;

  for (size_t i = 0; i < song.sequence_length(); ++i) {
    z2music::Pattern* p = song.at(i);
    std::cerr << "PW1 : "; dump_notes(p->notes(z2music::Pattern::Channel::Pulse1));
  }

  return 0;
}
