#ifndef Z2UTIL_IMWIDGET_SIMPLEMAP_H
#define Z2UTIL_IMWIDGET_SIMPLEMAP_H
#include <string>
#include "imwidget/glbitmap.h"
#include "imwidget/imwidget.h"
#include "imwidget/map_command.h"
#include "nes/mapper.h"
#include "nes/z2decompress.h"
#include "nes/z2objcache.h"
#include "proto/rominfo.pb.h"

#include "imgui.h"

namespace z2util {

class SimpleMap: public ImWindowBase {
  public:
    static SimpleMap* Spawn(Mapper* m, const Map& map, int startscreen=0);
    SimpleMap();
    SimpleMap(Mapper* m, const Map& map, int startscreen=0);
    void set_mapper(Mapper* m) { mapper_ = m; decomp_.set_mapper(m); }
    void Refresh() override { SetMap(map_); }
    bool Draw() override;
    void DrawMap(const ImVec2& pos);
    void SetMap(const Map& map, int mapsel=-1);
    void StartEmulator(int screen);
    inline const std::string& name() const { return decomp_.name(); }
    void RenderToBuffer(GLBitmap *buffer);
    std::unique_ptr<GLBitmap> RenderToNewBuffer();
  private:
    bool changed_;
    bool object_box_;
    bool enemy_box_;
    bool avail_box_;
    bool bgmap_;
    int width_;
    int height_;
    float scale_;
    int mapsel_;
    int tab_;
    int startscreen_;
    std::string title_;
    std::string window_title_;
    GLBitmap grayout_;
    Z2Decompress decomp_;
    MapHolder holder_;
    MapConnection connection_;
    MapEnemyList enemies_;
    MapItemAvailable avail_;
    MapSwapper swapper_;

    Mapper* mapper_;

    Map map_;
    Z2ObjectCache cache_;
    Z2ObjectCache items_;
    Z2ObjectCache enemy_;
    static const uint32_t RED    = 0xFF0000FF;
    static const uint8_t ELEVATOR = 0xEE;
};

}  // namespace z2util
#endif // Z2UTIL_IMWIDGET_SIMPLEMAP_H
