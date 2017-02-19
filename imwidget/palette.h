#ifndef Z2UTIL_IMWIDGET_PALETTE_H
#define Z2UTIL_IMWIDGET_PALETTE_H
#include <vector>
#include "imwidget/imwidget.h"

class Mapper;
namespace z2util {

class PaletteEditor: public ImWindowBase {
  public:
    PaletteEditor()
        : ImWindowBase(false), mapper_(nullptr), changed_(false),grpsel_(0) {}
    void Init();
    bool Draw() override;

    inline void set_mapper(Mapper* m) { mapper_ = m; }
  private:
    struct Unpacked {
        int color[16];
    };
    void Load();
    void Save();
    bool ColorSelector(const char* name, int pnum, int cnum, int color);


    Mapper* mapper_;
    bool changed_;
    int grpsel_;
    std::vector<Unpacked> data_;
};

}  // namespace z2util
#endif // Z2UTIL_IMWIDGET_PALETTE_H
