#ifndef Z2UTIL_ALG_PALACE_GEN_H
#define Z2UTIL_ALG_PALACE_GEN_H

#include <cstdint>
#include <random>
#include <functional>
#include <memory>
#include <vector>

#include "nes/mapper.h"
#include "proto/generator.pb.h"
#include "proto/rominfo.pb.h"

namespace z2util {

class PalaceGenerator {
  public:
    struct Room {
        int room;
        uint8_t up;
        uint8_t down;
        uint8_t left;
        uint8_t right;
        bool has_up, has_down, has_left, has_right;
        bool visited;
    };
    PalaceGenerator(PalaceGeneratorOptions& opt);

    void Generate();
    void SimplePrint();
    inline void set_mapper(Mapper* m) { mapper_ = m; }
  private:
    void InitPalaceMaps();
    void GenerateMaze();
    void VisitRooms(int x, int y);
    void MapToRooms();
    void PrepareRoom(int r);

    inline double real() { return real_(rng_); }
    inline bool bit(double prob=0.5) { return real() < 0.5; }
    inline int integer(int n) { return real() * double(n); }

    PalaceGeneratorOptions opt_;
    std::vector<std::vector<Room>> map_;
    std::vector<Room> rooms_;
    std::mt19937 rng_;
    std::uniform_real_distribution<double> real_;
    int room_;
    Mapper* mapper_;
    Map palace_maps_[64];

};

}  // namespace
#endif // Z2UTIL_ALG_PALACE_GEN_H
