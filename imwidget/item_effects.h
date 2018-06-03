#ifndef Z2UTIL_IMWIDGET_ITEM_EFFECTS_H
#define Z2UTIL_IMWIDGET_ITEM_EFFECTS_H

#include <vector>

#include "imwidget/imwidget.h"
#include "proto/rominfo.pb.h"

class Mapper;
namespace z2util {

class ItemEffects: public ImWindowBase {
  public:
    ItemEffects(): ImWindowBase(false) {};

    void Refresh() override;
    bool Draw() override;
    inline void set_mapper(Mapper* m) { mapper_ = m; };
  private:
    bool DrawItemTable();

    void Unpack();
    void Pack();

    struct UnpackedItemEffect {
        int town;
        int bit;
        int mc_count;
    };

    std::vector<UnpackedItemEffect> item_;
    Mapper* mapper_;
    bool changed_;
};

}  // namespace

#endif // Z2UTIL_IMWIDGET_ITEM_EFFECTS_H
