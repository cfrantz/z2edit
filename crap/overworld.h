#ifndef Z2UTIL_OVERWORLD_H
#define Z2UTIL_OVERWORLD_H
#include <cstdint>
#include "util/string.h"

class RomFile;

class Overworld {
  public:
    Overworld(RomFile* rom): rom_(rom) {}

    void Decompress(uint32_t start, uint32_t end);
    void Print();
    void PrintAreas(uint32_t start);

  private:
    RomFile* rom_;
    string map_;
};

#endif // Z2UTIL_OVERWORLD_H
