#ifndef Z2UTIL_IMWIDGET_NESCHRVIEW_H
#define Z2UTIL_IMWIDGET_NESCHRVIEW_H
#include <memory>
#include <vector>

#include "nes/mapper.h"
#include "imwidget/glbitmap.h"
#include "imwidget/imwidget.h"

class NesChrView: public ImWindowBase {
  public:
    NesChrView(int bank);
    NesChrView();

    void RenderChr();
    void RenderChr8x8();
    void RenderChr8x16();
    void MakeLabels();
    bool Draw();
    inline void set_mapper(Mapper* mapper) { mapper_ = mapper; }
    void Export(const std::string& filename);
    void Import(const std::string& filename);
  private:
    Mapper* mapper_;
    int bank_;
    std::unique_ptr<GLBitmap> bitmap_;
    char labels_[256][64];
    int nr_labels_;
    int mode_;
    bool grid_;
};

#endif // Z2UTIL_IMWIDGET_NESCHRVIEW_H
