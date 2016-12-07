#ifndef Z2UTIL_IMWIDGET_SIMPLEMAP_H
#define Z2UTIL_IMWIDGET_SIMPLEMAP_H
#include <string>
#include "imwidget/glbitmap.h"
#include "imwidget/hwpalette.h"
#include "imwidget/map_command.h"
#include "nes/mapper.h"
#include "nes/z2decompress.h"
#include "proto/rominfo.pb.h"

class SimpleMap {
  public:
    SimpleMap();
    void set_mapper(Mapper* m) { mapper_ = m; decomp_.set_mapper(m); }
    void set_palette(NesHardwarePalette* p) { hwpal_ = p; }
    void BlitTile(int x, int y, int tile, int pal);
    void BlitObject(int x, int y, int obj);
    void Draw();
    void SetMap(const z2util::Map& map, const z2util::RomInfo& ri);
    inline bool* visible() { return &visible_; }
    inline const std::string& name() const { return decomp_.name(); }
  private:
    bool visible_;
    int width_;
    int height_;
    int mapsel_;
    std::unique_ptr<GLBitmap> bitmap_;
    z2util::Z2Decompress decomp_;
    z2util::MapHolder holder_;

    Mapper* mapper_;
    NesHardwarePalette* hwpal_;

    z2util::Address obj_[4];
    z2util::Address chr_;
    z2util::Address pal_;
    const z2util::RomInfo* rominfo_;
};

#endif // Z2UTIL_IMWIDGET_SIMPLEMAP_H
