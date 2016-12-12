#ifndef Z2UTIL_IMWIDGET_MAP_COMMAND_H
#define Z2UTIL_IMWIDGET_MAP_COMMAND_H
#include <cstdint>
#include <string>
#include <vector>

#include "proto/rominfo.pb.h"
#include "nes/mapper.h"

namespace z2util {

class MapHolder;
class MapCommand {
  public:
    MapCommand(const MapHolder& holder, uint8_t position, uint8_t object, uint8_t extra);

    bool Draw();
    std::vector<uint8_t> Command();
    // areas: overword sideviews, towns, palaces, great palace
    const static int NR_AREAS = 4;
    // sets: small objects, object set 0, object set 1, extra objects
    const static int NR_SETS = 4;
    static const DecompressInfo* info_[NR_AREAS][NR_SETS][16];
    static const char* object_names_[NR_AREAS][NR_SETS][16];
  private:
    int id_;
    const MapHolder& holder_;
    uint8_t position_;
    uint8_t object_;
    uint8_t extra_;
    struct {
        int x, y;
        int object;
        int param;
        int extra;
        int extra_param;
    } data_;
    char obuf_[4];
    char ebuf_[4];
    const char *names_[64];

    static void Init();
    static int newid();
};


class MapHolder {
  public:
    MapHolder();
    bool Draw();
    void Parse(const Map& map);
    std::vector<uint8_t> MapData();
    inline void set_mapper(Mapper* m) { mapper_ = m; }
    inline uint8_t flags() const { return flags_; };
    inline uint8_t ground() const { return ground_; };
    inline uint8_t back() const { return back_; };
    inline const Map& map() const { return map_; }
  private:
    void Unpack();
    void Pack();

    uint8_t length_;
    uint8_t flags_;
    uint8_t ground_;
    uint8_t back_;
    std::vector<MapCommand> command_;
    Map map_;
    Mapper* mapper_;

    struct Unpacked {
        int objset;
        int width;
        bool grass;
        bool bushes;
        bool ceiling;
        int ground;
        int floor;
        int spal;
        int bpal;
        int bmap;
    };
    Unpacked data_;
};

}  // namespace z2util
#endif // Z2UTIL_IMWIDGET_MAP_COMMAND_H
