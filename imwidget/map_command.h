#ifndef Z2UTIL_IMWIDGET_MAP_COMMAND_H
#define Z2UTIL_IMWIDGET_MAP_COMMAND_H
#include <cstdint>
#include <string>
#include <memory>
#include <vector>

#include "proto/rominfo.pb.h"
#include "nes/mapper.h"
#include "nes/text_list.h"

namespace z2util {

class MapHolder;
class MapCommand {
  public:
    enum DrawResult {
        DR_NONE,
        DR_CHANGED,
        DR_COPY,
        DR_DELETE,
    };
    MapCommand(const MapHolder* holder, uint8_t position, uint8_t object,
               uint8_t extra);
    MapCommand(const MapHolder* holder, int x0, uint8_t position,
               uint8_t object, uint8_t extra);
    MapCommand(const MapCommand& other);

    MapCommand Copy();
    bool Draw(bool abscoord=false, bool popup=false);
    DrawResult DrawPopup(float scale);
    std::vector<uint8_t> Command();
    // areas: overword sideviews, towns, palaces, great palace
    const static int NR_AREAS = 4;
    // sets: small objects, object set 0, object set 1,
    // extra small objects, extra objects
    const static int NR_SETS = 5;
    const static int MAX_COLLECTABLE = 36;
    static const DecompressInfo* info_[NR_AREAS][NR_SETS][16];
    static const char* object_names_[NR_AREAS][NR_SETS][16];
    static const char *collectable_names_[MAX_COLLECTABLE];
    inline int absx() const { return data_.absx; }
    inline int absy() const { return data_.y; }
    inline void set_relx(int x) { data_.x = x; }
    inline int relx() const { return data_.x; }
    inline uint8_t object() const { return object_; }
    inline void set_show_origin(bool s) { show_origin_ = s; }
    static void Init();
  private:
    int id_;
    const MapHolder* holder_;
    bool show_origin_;
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
    const char *summary_;
    const char *names_[100];

    static int newid();
};


class MapHolder {
  public:
    enum DrawResult {
        DR_NONE,
        DR_CHANGED,
        DR_PALETTE_CHANGED,
    };
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

    MapHolder();
    MapHolder(Mapper* m);
    DrawResult Draw();
    bool DrawPopup(float scale);
    void Save() { Save([](){}, true); }
    void Save(std::function<void()> finish, bool force=false);
    void Parse(const Map& map, uint16_t altaddr=0);
    std::vector<uint8_t> MapData();
    std::vector<uint8_t> MapDataAbs();
    void Clear(const Unpacked& data) {
        command_.clear();
        data_ = data;
        data_changed_ = true;
        cursor_moves_left_ = false;
    }
    void Append(const MapCommand& cmd);
    void Extend(const std::vector<MapCommand>& cmds);
    inline std::vector<MapCommand>* mutable_command() { return &command_; }
    inline void set_mapper(Mapper* m) { mapper_ = m; }
    inline uint8_t flags() const { return flags_; };
    inline uint8_t ground() const { return ground_; };
    inline uint8_t back() const { return back_; };

    inline void set_objset(int val) { data_.objset = val; }
    inline void set_width(int val) { data_.width = val; }
    inline void set_grass(bool val) { data_.grass = val; }
    inline void set_bushes(bool val) { data_.bushes = val; }
    inline void set_ceiling(bool val) { data_.ceiling = val; }
    inline void set_ground(int val) { data_.ground = val; }
    inline void set_floor(int val) { data_.floor = val; }
    inline void set_spal(int val) { data_.spal = val; }
    inline void set_bpal(int val) { data_.bpal = val; }
    inline void set_bmap(int val) { data_.bmap = val; }

    inline const Map& map() const { return map_; }
    inline bool cursor_moves_left() {
        return cursor_moves_left_;
    }
    inline void set_cursor_moves_left(bool v) {
        cursor_moves_left_ = v;
    }
    inline void set_show_origin(bool s) {
        for(auto& c : command_) {
            c.set_show_origin(s);
        }
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

    Unpacked data_;
};

class MapConnection {
  public:
    MapConnection();
    MapConnection(Mapper* m);
    inline void set_mapper(Mapper* m) { mapper_ = m; }

    bool Draw();
    void Parse(const Map& map);
    void Save();
    struct Unpacked {
        int destination;
        int start;
    };
    inline Unpacked left() const { return data_[0]; }
    inline Unpacked down() const { return data_[1]; }
    inline Unpacked up() const { return data_[2]; }
    inline Unpacked right() const { return data_[3]; }
    inline Unpacked door(int d) const {
        if (doors_.address() && d >= 0 && d < 4) {
            return data_[4+d];
        } else {
            return Unpacked{63, 3};
        }
    }

    inline void set_left(int dest, int start) {
        data_[0].destination = dest; data_[0].start = start;
    }
    inline void set_down(int dest, int start) {
        data_[1].destination = dest; data_[1].start = start;
    }
    inline void set_up(int dest, int start) {
        data_[2].destination = dest; data_[2].start = start;
    }
    inline void set_right(int dest, int start) {
        data_[3].destination = dest; data_[3].start = start;
    }
  private:
    Mapper* mapper_;
    Address connector_;
    Address doors_;
    int world_;
    int overworld_;
    int subworld_;

    Unpacked data_[8];
};

class MapEnemyList {
  public:
    enum DrawResult {
        DR_NONE,
        DR_CHANGED,
        DR_COPY,
        DR_DELETE,
    };
    struct Unpacked {
        Unpacked(int e_, int x_, int y_)
            : enemy(e_), x(x_), y(y_), text{-1, -1} {}
        int enemy;
        int x, y;
        int text[2];
        int condition;
    };
    MapEnemyList();
    MapEnemyList(Mapper* m);
    void Init();
    inline void set_mapper(Mapper* m) { mapper_ = m; }

    bool Draw();
    bool DrawOne(Unpacked* item, bool popup);
    DrawResult DrawOnePopup(Unpacked* item, float scale);
    bool DrawPopup(float scale);
    void Parse(const Map& map);
    std::vector<uint8_t> Pack();
    void Save();
    std::vector<Unpacked>& data();
    inline void set_show_origin(bool s) { show_origin_ = s; }
  private:
    Mapper* mapper_;
    bool show_origin_;
    bool is_large_;
    bool is_encounter_;
    Address pointer_;
    Map map_;
    int world_;
    int overworld_;
    int subworld_;
    int area_;
    int display_;

    std::vector<Unpacked> data_;
    std::unique_ptr<MapEnemyList> large_;
    const char *names_[256];
    int max_names_;
    TextListPack text_;
};

class MapItemAvailable {
  public:
    struct Unpacked {
        bool avail[4];
    };
    MapItemAvailable() : MapItemAvailable(nullptr) {}
    MapItemAvailable(Mapper* m) : mapper_(m) {}
    inline void set_mapper(Mapper* m) { mapper_ = m; }

    bool Draw();
    void Parse(const Map& map);
    void Save();
    inline const Unpacked& data() { return data_; }
    inline bool get(int x) { return data_.avail[x/16]; }
    inline void set_show(bool s) { show_ = s; }
    inline bool show() { return show_; }
  private:
    Mapper* mapper_;
    bool show_;
    AvailableBitmap avail_;
    int area_;
    Unpacked data_;
};

class MapSwapper {
  public:
    MapSwapper() : MapSwapper(nullptr) {}
    MapSwapper(Mapper* m)
        : id_(888888),
        mapper_(m),
        srcarea_(0),
        dstarea_(0),
        swap_map_(true),
        swap_conn_(true),
        swap_enemies_(true),
        swap_itemav_(true) {}

    inline void set_mapper(Mapper* m) { mapper_ = m; }
    inline void set_map(const Map& map) { map_ = map; srcarea_ = map_.area(); }
    inline const Map& map() const { return map_; }

    bool Draw();
  private:
    void Swap();
    void Copy();

    int id_;
    Mapper* mapper_;
    Map map_;
    int srcarea_;
    int dstarea_;
    bool swap_map_;
    bool swap_conn_;
    bool swap_enemies_;
    bool swap_itemav_;
};

}  // namespace z2util
#endif // Z2UTIL_IMWIDGET_MAP_COMMAND_H
