#include "music.h"

#include <iostream>

int main(int argc, char** argv) {
  if (argc != 2) {
    std::cerr << "Usage: " << argv[0] << " z2_rom" << std::endl;
    return 1;
  }

  const std::string file = std::string(argv[1]);
  z2music::Rom rom(file);
  const z2music::Credits *credits = rom.credits();

  std::cout << "====================" << std::endl;
  for (size_t i = 0; i < 9; ++i) {
    const auto text = credits->get(i);
    std::cout << text.title << std::endl;
    std::cout << "    " << text.name1 << std::endl;
    std::cout << "    " << text.name2 << std::endl;
    std::cout << "====================" << std::endl;
  }

  return 0;
}
