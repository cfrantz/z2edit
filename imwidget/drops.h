#ifndef Z2UTIL_IMWIDGET_DROPS_H
#define Z2UTIL_IMWIDGET_DROPS_H
#include <vector>

#include "imwidget/imwidget.h"
#include "proto/rominfo.pb.h"

class Mapper;
namespace z2util {

class Drops: public ImWindowBase {
  public:
    Drops(): ImWindowBase(false) {};

    void Refresh() override;
    bool Draw() override;
    inline void set_mapper(Mapper* m) { mapper_ = m; };
  private:
    void DrawDropTable();
    void DrawDropper(const ItemDrop::Dropper& d, int n);
    int EnemyList(int world, const char **list, int n);

    void Unpack();
    void Pack();

    struct Unpacked {
        struct Drop {
            int item, enemy, hp;
        };
        int counter;
        int small[8];
        int large[8];
        std::vector<Drop> drop;
    };
    struct Unpacked data_;
    Mapper* mapper_;
    int drop_type_;
    bool changed_;
};

}  // namespace
#endif // Z2UTIL_IMWIDGET_DROPS_H
