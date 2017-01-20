#ifndef Z2UTIL_IMWIDGET_OVERWORLD_ENCOUNTERS_H
#define Z2UTIL_IMWIDGET_OVERWORLD_ENCOUNTERS_H
#include <vector>

#include "proto/rominfo.pb.h"
class Mapper;
namespace z2util {

class OverworldEncounters {
  public:
    OverworldEncounters() : visible_(true) {}
    void Unpack();
    void Draw();
    void Save();

    inline void set_mapper(Mapper* m) { mapper_ = m; }
    inline void set_map(const Map& map) { map_ = map; }
  private:
    bool visible_;
    Mapper* mapper_;
    Map map_;

    struct Unpacked {
        int area;
        int screen;
    };
    std::vector<Unpacked> data_;
    int south_side_;
};

}  // namespace z2util

#endif // Z2UTIL_IMWIDGET_OVERWORLD_ENCOUNTERS_H
