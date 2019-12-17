#include "music.h"

#include <iomanip>
#include <iostream>

void dump_notes(std::vector<z2music::Note> notes) {
  for (z2music::Note n : notes) {
    std::cerr << '[' << std::hex << std::setw(2) << n.encode() << ']';
    size_t left = n.length() / 4 - 4;
    std::cerr << std::string(left, '.');
  }
  std::cerr << std::endl;
}

int main(void) {
  z2music::Rom rom("/home/alan/source/z2-music/z2.nes");
  z2music::Pattern pattern(rom, 0x01a970);

  std::cerr << "Pattern length: " << (pattern.length() / 96.0) << std::endl;

  std::cerr << "PW1 : ";
  dump_notes(pattern.notes(z2music::Pattern::Channel::Pulse1));
  std::cerr << "PW2 : ";
  dump_notes(pattern.notes(z2music::Pattern::Channel::Pulse2));
  std::cerr << "TRI : ";
  dump_notes(pattern.notes(z2music::Pattern::Channel::Triangle));
  std::cerr << "NSE : ";
  dump_notes(pattern.notes(z2music::Pattern::Channel::Noise));

  return 0;
}
