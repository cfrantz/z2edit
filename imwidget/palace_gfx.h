#ifndef Z2UTIL_IMWIDGET_PALACE_GFX_H
#define Z2UTIL_IMWIDGET_PALACE_GFX_H
#include <vector>
#include "imwidget/imwidget.h"

class Mapper;
namespace z2util {

class PalaceGraphics: public ImWindowBase {
  public:
    PalaceGraphics(): ImWindowBase(false) {}

    void Refresh() override { Load(); }
    bool Draw() override;
    inline void set_mapper(Mapper* m) { mapper_ = m; };
  private:
    Mapper* mapper_;
    std::vector<int> graphics_;
    std::vector<int> palette_;

    void Load();
    void Save();
};

}  // namespace
#endif // Z2UTIL_IMWIDGET_MISC_HACKS_H
