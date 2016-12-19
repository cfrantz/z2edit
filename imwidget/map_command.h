#ifndef Z2UTIL_IMWIDGET_MAP_COMMAND_H
#define Z2UTIL_IMWIDGET_MAP_COMMAND_H
#include <cstdint>
#include <string>
#include <memory>
#include <vector>

#include "proto/rominfo.pb.h"
#include "nes/mapper.h"

namespace z2util {

class MapHolder;
class MapCommand {
  public:
    MapCommand(const MapHolder* holder, uint8_t position, uint8_t object,
               uint8_t extra);
    MapCommand(const MapHolder* holder, int x0, uint8_t position,
               uint8_t object, uint8_t extra);

    bool Draw(bool abscoord=false);
    std::vector<uint8_t> Command();
    // areas: overword sideviews, towns, palaces, great palace
    const static int NR_AREAS = 4;
    // sets: small objects, object set 0, object set 1, extra objects
    const static int NR_SETS = 4;
    static const DecompressInfo* info_[NR_AREAS][NR_SETS][16];
    static const char* object_names_[NR_AREAS][NR_SETS][16];
    inline int absx() const { return data_.absx; }
    inline int absy() const { return data_.y; }
    inline void set_relx(int x) { data_.x = x; }
    inline uint8_t object() const { return object_; }
  private:
    int id_;
    const MapHolder* holder_;
    uint8_t position_;
    uint8_t object_;
    uint8_t extra_;
    struct {
        int absx;
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
    void Save();
    void Parse(const Map& map);
    std::vector<uint8_t> MapData();
    std::vector<uint8_t> MapDataAbs();
    inline void set_mapper(Mapper* m) { mapper_ = m; }
    inline uint8_t flags() const { return flags_; };
    inline uint8_t ground() const { return ground_; };
    inline uint8_t back() const { return back_; };
    inline const Map& map() const { return map_; }
  private:
    std::vector<uint8_t> MapDataWorker(std::vector<MapCommand>& cmd);
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

class MapConnection {
  public:
    MapConnection();
    inline void set_mapper(Mapper* m) { mapper_ = m; }

    void Draw();
    void Parse(const Map& map);
    void Save();
  private:
    Mapper* mapper_;
    Address connector_;
    int world_;

    struct Unpacked {
        int destination;
        int start;
    };
    Unpacked data_[4];
};

class MapEnemyList {
  public:
    MapEnemyList();
    inline void set_mapper(Mapper* m) { mapper_ = m; }

    void Draw();
    void Parse(const Map& map);
    void Save();
  private:
    Mapper* mapper_;
    Address pointer_;
    int world_;
    int length_;

    struct Unpacked {
        int enemy;
        int x, y;
    };
    Unpacked data_[128];
};

}  // namespace z2util
#endif // Z2UTIL_IMWIDGET_MAP_COMMAND_H
