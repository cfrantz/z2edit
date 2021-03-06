#ifndef Z2UTIL_IMWIDGET_MULTIMAP_H
#define Z2UTIL_IMWIDGET_MULTIMAP_H
#include <cmath>
#include <memory>
#include <vector>

#include "alg/fdg.h"
#include "imwidget/glbitmap.h"
#include "imwidget/imutil.h"
#include "imwidget/imwidget.h"
#include "nes/mapper.h"
#include "nes/z2decompress.h"
#include "nes/z2objcache.h"
#include "proto/rominfo.pb.h"
#include "proto/generator.pb.h"
#include "proto/session.pb.h"
#include "imgui.h"

namespace z2util {

class MultiMap: public ImWindowBase {
  public:
    static MultiMap* Spawn(Mapper* m,
                           int world, int overworld, int subworld,int map);

    MultiMap(Mapper* mapper, int world, int overworld, int subworld, int map)
        : ImWindowBase(),
        mapper_(mapper),
        world_(world),
        overworld_(overworld),
        subworld_(subworld),
        start_(map)
    {}

    void Init();
    void Refresh() override { Init(); }
    bool Draw() override;
  private:
    struct DrawLocation {
        fdg::Node *node;
        std::unique_ptr<GLBitmap> buffer;
    };
    enum Direction {
        NONE,
        LEFT,
        DOWN,
        UP,
        RIGHT,
        DOOR1,
        DOOR2,
        DOOR3,
        DOOR4,
    };
    fdg::Node* AddRoom(int room, double x, double y);
    Vec2 Position(const Vec2& pos);
    Vec2 Position(const DrawLocation& dl, Direction side);
    void DrawArrow(const Vec2& a, const Vec2&b, uint32_t color,
                   float width=2.0, float arrowpos=0.1, float rootsize=10.0);
    void DrawConnections(const DrawLocation& dl);
    void DrawOne(const DrawLocation& dl);
    void DrawGen();
    void Traverse(int room, double x, double y, int from, double strength=1.0);
    void Sort();
    void DrawLegend();

    int id_;
    Mapper* mapper_;
    int world_;
    int overworld_;
    int subworld_;
    int start_;
    std::string title_;
    int maxx_;
    int maxy_;
    Map maps_[64];
    bool visited_[64];
    int visited_room0_;
    std::map<int32_t, DrawLocation> location_;
    fdg::Graph graph_;

    MultiMapConfig* mcfg_;
    PalaceGeneratorOptions pgo_;

    Vec2 origin_;
    Vec2 absolute_;
    bool drag_;

    static const uint32_t RED    = 0xF00000FF;
    static const uint32_t GREEN  = 0xF000FF00;
    static const uint32_t BLUE   = 0xF0FF0000;
    static const uint32_t YELLOW = 0xF000EEFD;
    static const uint32_t ORANGE = 0xF0008BFF;
    static const uint32_t GRAY   = 0x60808080;
};

}  // namespace z2util
#endif // Z2UTIL_IMWIDGET_MULTIMAP_H
