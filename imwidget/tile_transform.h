#ifndef Z2UTIL_IMWIDGET_TILE_TRANSFORM_H
#define Z2UTIL_IMWIDGET_TILE_TRANSFORM_H
#include <vector>

#include "imwidget/imwidget.h"

class Mapper;
namespace z2util {

class TileTransform: public ImWindowBase {
  public:
    TileTransform(): ImWindowBase(false) {};

    void Refresh() override;
    bool Draw() override;
    inline void set_mapper(Mapper* m) { mapper_ = m; };
  private:
    void Unpack();
    void Pack();
    struct Unpacked {
        std::vector<int> from;
        std::vector<int> to;
    };

    Mapper* mapper_;
    Unpacked data_;
    bool changed_;
};

}  // namespace

#endif // Z2UTIL_IMWIDGET_TILE_TRANSFORM_H
