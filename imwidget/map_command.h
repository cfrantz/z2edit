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
    // sets: small objects, object set 0, object set 1,
    // extra small objects, extra objects
    const static int NR_SETS = 5;
    static const DecompressInfo* info_[NR_AREAS][NR_SETS][16];
    static const char* object_names_[NR_AREAS][NR_SETS][16];
    inline int absx() const { return data_.absx; }
    inline int absy() const { return data_.y; }
    inline void set_relx(int x) { data_.x = x; }
    inline int relx() const { return data_.x; }
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
    const char *names_[100];

    static void Init();
    static int newid();
};


class MapHolder {
  public:
    MapHolder();
    bool Draw();
    void Save();
    void Parse(const Map& map, uint16_t altaddr=0);
    std::vector<uint8_t> MapData();
    std::vector<uint8_t> MapDataAbs();
    inline void set_mapper(Mapper* m) { mapper_ = m; }
    inline uint8_t flags() const { return flags_; };
    inline uint8_t ground() const { return ground_; };
    inline uint8_t back() const { return back_; };
    inline const Map& map() const { return map_; }
    inline bool cursor_moves_left() {
        return cursor_moves_left_;
    }
    inline void set_cursor_moves_left(bool v) {
        cursor_moves_left_ = v;
    }
  private:
    std::vector<uint8_t> MapDataWorker(std::vector<MapCommand>& cmd);
    void Unpack();
    void Pack();

    uint8_t length_;
    uint8_t flags_;
    uint8_t ground_;
    uint8_t back_;
    bool cursor_moves_left_;
    std::vector<MapCommand> command_;
    Map map_;
    Mapper* mapper_;
    bool data_changed_;
    bool addr_changed_;
    uint16_t map_addr_;

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
    struct Unpacked {
        int destination;
        int start;
    };
    inline Unpacked left() { return data_[0]; }
    inline Unpacked down() { return data_[1]; }
    inline Unpacked up() { return data_[2]; }
    inline Unpacked right() { return data_[3]; }
  private:
    Mapper* mapper_;
    Address connector_;
    int world_;
    int overworld_;
    int subworld_;

    Unpacked data_[4];
};

class MapEnemyList {
  public:
    struct Unpacked {
        Unpacked(int e_, int x_, int y_) : enemy(e_), x(x_), y(y_) {}
        int enemy;
        int x, y;
    };
    MapEnemyList();
    MapEnemyList(Mapper* m);
    inline void set_mapper(Mapper* m) { mapper_ = m; }

    void Draw();
    void Parse(const Map& map);
    std::vector<uint8_t> Pack();
    void Save();
    const std::vector<Unpacked>& data();
  private:
    Mapper* mapper_;
    bool is_large_;
    bool is_encounter_;
    Address pointer_;
    int world_;
    int overworld_;
    int area_;
    int display_;

    std::vector<Unpacked> data_;
    std::unique_ptr<MapEnemyList> large_;
};

}  // namespace z2util
#endif // Z2UTIL_IMWIDGET_MAP_COMMAND_H
