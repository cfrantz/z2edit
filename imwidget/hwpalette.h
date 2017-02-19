#ifndef Z2UTIL_IMWIDGET_HWPALETTE_H
#define Z2UTIL_IMWIDGET_HWPALETTE_H
#include <cstdint>
#include "imgui.h"
#include "imwidget/imwidget.h"

class NesHardwarePalette: public ImWindowBase {
  public:
    static NesHardwarePalette* Get();
    NesHardwarePalette() : ImWindowBase(false) { Init(); }
    void Init();
    bool Draw() override;
    inline uint32_t palette(int color) const { return palette_[color]; }
    inline ImColor imcolor(int color) const { return fpal_[color]; }
  private:
    uint32_t palette_[64];
    ImColor fpal_[64];
    char label_[64][16];
};

#endif // Z2UTIL_IMWIDGET_HWPALETTE_H
