#ifndef Z2UTIL_IMWIDGET_HWPALETTE_H
#define Z2UTIL_IMWIDGET_HWPALETTE_H
#include <cstdint>
#include "imgui.h"

class NesHardwarePalette {
  public:
    NesHardwarePalette() : visible_(false) { Init(); }
    void Init();
    void Draw();
    inline uint32_t palette(int color) const { return palette_[color]; }
    inline bool* visible() { return &visible_; }
  private:
    bool visible_;
    uint32_t palette_[64];
    ImColor fpal_[64];
    char label_[64][16];
};

#endif // Z2UTIL_IMWIDGET_HWPALETTE_H
