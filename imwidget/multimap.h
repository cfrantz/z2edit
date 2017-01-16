#ifndef Z2UTIL_IMWIDGET_MULTIMAP_H
#define Z2UTIL_IMWIDGET_MULTIMAP_H
#include <memory>
#include <vector>

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
        world_(world), start_(map) {};

    void Init();
    void Draw();
    bool* visible() { return &visible_; }
  private:
    struct DrawLocation {
        DrawLocation(int map_, int x_, int y_, GLBitmap* buf_)
            : map(map_), x(x_), y(y_), buffer(buf_) {};
        int map;
        int x, y;
        std::unique_ptr<GLBitmap> buffer;
    };
    enum From {
        NONE,
        LEFT,
        DOWN,
        UP,
        RIGHT,
    };
    void DrawOne(ImVec2 pos, const DrawLocation& dl);
    void Traverse(int room, int x, int y, From from);
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
    std::vector<DrawLocation> location_;

//    Z2Decompress decomp_;
//    Z2ObjectCache cache_;
};

}  // namespace z2util
#endif // Z2UTIL_IMWIDGET_MULTIMAP_H
