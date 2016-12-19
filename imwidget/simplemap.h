#ifndef Z2UTIL_IMWIDGET_SIMPLEMAP_H
#define Z2UTIL_IMWIDGET_SIMPLEMAP_H
#include <string>
#include "imwidget/glbitmap.h"
#include "imwidget/hwpalette.h"
#include "imwidget/map_command.h"
#include "nes/mapper.h"
#include "nes/z2decompress.h"
#include "nes/z2objcache.h"
#include "proto/rominfo.pb.h"

#include "imgui.h"

namespace z2util {

class SimpleMap {
  public:
    SimpleMap();
    void set_mapper(Mapper* m) { mapper_ = m; decomp_.set_mapper(m); }
    void set_palette(NesHardwarePalette* p) { hwpal_ = p; }
    void Draw();
    void DrawMap(const ImVec2& pos);
    void SetMap(const Map& map);
    inline bool* visible() { return &visible_; }
    inline const std::string& name() const { return decomp_.name(); }
  private:
    bool visible_;
    int width_;
    int height_;
    float scale_;
    int mapsel_;
    int tab_;
    Z2Decompress decomp_;
    MapHolder holder_;
    MapConnection connection_;
    MapEnemyList enemies_;;

    Mapper* mapper_;
    NesHardwarePalette* hwpal_;

    Map map_;
    Z2ObjectCache cache_;
    Z2ObjectCache items_;;
};

}  // namespace z2util
#endif // Z2UTIL_IMWIDGET_SIMPLEMAP_H
