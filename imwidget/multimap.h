#ifndef Z2UTIL_IMWIDGET_MULTIMAP_H
#define Z2UTIL_IMWIDGET_MULTIMAP_H
#include <cmath>
#include <memory>
#include <vector>

#include "alg/fdg.h"
#include "imwidget/glbitmap.h"
#include "nes/mapper.h"
#include "nes/z2decompress.h"
#include "nes/z2objcache.h"
#include "proto/rominfo.pb.h"
#include "imgui.h"

namespace z2util {

class MultiMap {
  public:
    static MultiMap* New(Mapper* m, int world, int map);

    MultiMap(Mapper* mapper, int world, int map)
        : mapper_(mapper), visible_(true), scale_(0.25),
        world_(world), start_(map), pauseconv_(true) {};

    void Init();
    void Draw();
    bool* visible() { return &visible_; }
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
    };
    fdg::Node* AddRoom(int room, int x, int y);
    Vec2 Position(const Vec2& pos);
    Vec2 Position(const DrawLocation& dl, Direction side);
    void DrawConnections(const DrawLocation& dl);
    void DrawOne(const DrawLocation& dl);
    void Traverse(int room, int x, int y, int from);
    void Sort();

    Mapper* mapper_;
    bool visible_;
    float scale_;
    int world_;
    int start_;
    std::string title_;
    int maxx_;
    int maxy_;
    Map maps_[64];
    bool visited_[64];
    int visited_room0_;
    std::map<int32_t, DrawLocation> location_;
    fdg::Graph graph_;

    Vec2 origin_;
    Vec2 absolute_;
    bool drag_;
    bool pauseconv_;

    static float xs_;
    static float ys_;
    static bool preconverge_;

    static const uint32_t RED    = 0xF00000FF;
    static const uint32_t GREEN  = 0xF000FF00;
    static const uint32_t BLUE   = 0xF0FF0000;
    static const uint32_t YELLOW = 0xF000EEFD;
    static const uint32_t GRAY   = 0x60808080;
};

}  // namespace z2util
#endif // Z2UTIL_IMWIDGET_MULTIMAP_H
