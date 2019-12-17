#include "music.h"

#include <iostream>

int main(void) {
  z2music::Rom rom("/home/alan/source/z2-music/z2.nes");
  z2music::Pattern pattern(rom, 0x01a970);

  std::cerr << "Pattern length: " << pattern.length() << std::endl;

  return 0;
}
