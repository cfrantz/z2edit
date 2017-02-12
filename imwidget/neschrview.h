#ifndef Z2UTIL_IMWIDGET_NESCHRVIEW_H
#define Z2UTIL_IMWIDGET_NESCHRVIEW_H
#include <memory>
#include <vector>

#include "nes/mapper.h"
#include "imwidget/glbitmap.h"
#include "imwidget/imwidget.h"

class NesChrView: public ImWindowBase {
  public:
    NesChrView();

    void RenderChr();
    void MakeLabels();
    bool Draw();
    inline void set_mapper(Mapper* mapper) { mapper_ = mapper; }
  private:
    Mapper* mapper_;
    int bank_;
    std::unique_ptr<GLBitmap> bitmap;
    char labels_[256][64];
    int nr_labels_;
};

#endif // Z2UTIL_IMWIDGET_NESCHRVIEW_H
