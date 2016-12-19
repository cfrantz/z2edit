#ifndef Z2UTIL_NES_Z2OBJCACHE_H
#define Z2UTIL_NES_Z2OBJCACHE_H
#include <cstdint>
#include <map>
#include "proto/rominfo.pb.h"
#include "imwidget/glbitmap.h"

class Mapper;
class NesHardwarePalette;

namespace z2util {

class Z2ObjectCache {
  public:
    enum Schema {
        OVERWORLD = 0,
        MAP = 1,
        ITEM = 2,
    };
    Z2ObjectCache() {};
    explicit Z2ObjectCache(Mapper* mapper, NesHardwarePalette* hwpal)
        : mapper_(mapper), hwpal_(hwpal) {}

    void Init(const Map& map);
    void Init(const Address& addr, const Address& chr, Schema schema);
    GLBitmap& Get(uint8_t object);

    inline void set_mapper(Mapper* m) { mapper_ = m; }
    inline void set_hwpal(NesHardwarePalette* pal) { hwpal_ = pal; }
    inline void set_palette(const Address& pal) { palette_ = pal; }
    inline void Clear() { cache_.clear(); }
    const static int WIDTH = 16;
    const static int HEIGHT = 16;
  private:
    void CreateObject(uint8_t obj, uint32_t* dest);
    void BlitTile(uint32_t* dest, int x, int y, int tile, int pal, bool flip=false);

    Mapper* mapper_;
    NesHardwarePalette* hwpal_;

    Schema schema_;
    Address obj_[4];
    Address palette_;
    Address chr_;

    std::map<uint8_t, GLBitmap> cache_;
};

}  // namespace
#endif // Z2UTIL_NES_Z2OBJCACHE_H
