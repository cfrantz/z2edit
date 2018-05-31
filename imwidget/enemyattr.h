#ifndef Z2UTIL_IMWIDGET_ENEMYATTR_H
#define Z2UTIL_IMWIDGET_ENEMYATTR_H
#include <vector>
#include "imwidget/imwidget.h"
#include "nes/z2objcache.h"

class Mapper;
namespace z2util {

class PointsTable: public ImWindowBase {
  public:
    PointsTable()
      : ImWindowBase(false), mapper_(nullptr), changed_(false) {}
    void Init();

    bool Draw() override;
    void Refresh() override { Init(); }
    inline void set_mapper(Mapper* m) { mapper_ = m; }
    inline int xp(int t) const { return data_.at(t).xp; }
    void Load();
    void Save();
  private:
    struct Unpacked {
        int xp;
        int gfx[2];
    };

    Mapper* mapper_;
    bool changed_;
    std::vector<Unpacked> data_;
    Z2ObjectCache cache_;
};

class EnemyEditor: public ImWindowBase {
  public:
    EnemyEditor()
      : ImWindowBase(false), mapper_(nullptr), changed_(false), category_(0) {}
    void Init();

    bool Draw() override;
    bool DrawPbags(const char* types);

    void Refresh() override { Init(); }
    inline void set_mapper(Mapper* m) { mapper_ = m; }
  private:
    struct Unpacked {
        int hp;
        int palette;
        bool steal_xp;
        bool need_fire;;
        int xp_type;
        int drop_group;
        bool no_beam;
        bool unknown1;
        int damage_code;
        bool no_thunder;
        bool regenerate;
        bool unknown2;
        bool no_sword;
        int unknown3;

    };
    void Load();
    void Save();

    Mapper* mapper_;
    bool changed_;
    int category_;
    std::vector<Unpacked> data_;
    int pbags_[4];
    PointsTable table_;
    const static int TABLE_LEN = 0x24;
};

}  // namespace z2util
#endif // Z2UTIL_IMWIDGET_ENEMYATTR_H
