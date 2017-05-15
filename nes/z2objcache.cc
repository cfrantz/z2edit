#include "nes/z2objcache.h"
#include "nes/mapper.h"
#include "imwidget/hwpalette.h"
#include "util/logging.h"

namespace z2util {

Z2ObjectCache::Z2ObjectCache() {}

void Z2ObjectCache::Init(const Map& map) {
    Clear();
    palette_ = map.palette();
    chr_ = map.chr();
    schema_ = map.type()== MapType::OVERWORLD ? Schema::OVERWORLD : Schema::MAP;

    for(int i=0; i<map.objtable_size(); i++) {
        if (schema_ == Schema::OVERWORLD) {
            obj_[i] = map.objtable(i);
        } else {
            // For sideview maps, the object table is a list of addresses
            // to the object table.  
            obj_[i] = mapper_->ReadAddr(map.objtable(i), 0);
        }
    }
}

void Z2ObjectCache::Init(const ItemInfo& info) {
    Clear();
    info_ = info;
    schema_ = Schema::ITEMINFO;
}

void Z2ObjectCache::Init(const Address& addr, const Address& chr,
                         Schema schema) {
    Clear();
    chr_ = chr;
    schema_ = schema;

    for(int i=0; i<4; i++)
        obj_[i] = addr;
}

GLBitmap& Z2ObjectCache::Get(uint8_t object) {
    auto item = cache_.find(object);
    if (item == cache_.end()) {
        CreateObject(object);
    }
    return cache_[object];
}


void Z2ObjectCache::BlitTile(uint32_t* dest, int x, int y, int tile, int pal,
                             int width, bool flip) {
    int bofs = 0;
    int height = 8;
    dest += y*width + x;

    if (schema_ == Schema::ITEM
        || schema_ == Schema::ITEMINFO
        || schema_ == Schema::TILE8x16) {
        bofs = tile & 1;
        height = 16;
        tile &= ~1;
    }

    for(int row=0; row<height; row++, dest+=width) {
        uint8_t a = mapper_->ReadChrBank(chr_.bank() + bofs,
                 chr_.address() + 16*(tile + (row / 8)) + (row & 7));
        uint8_t b = mapper_->ReadChrBank(chr_.bank() + bofs,
                 chr_.address() + 16*(tile + (row / 8)) + (row & 7) + 8);
        for(int col=0; col<8; col++, a<<=1, b<<=1) {
            uint8_t color = (a & 0x80) >> 7 | (b & 0x80) >> 6;
            color = mapper_->Read(palette_, pal * 4 + color);
            dest[flip ? 7-col : col] = 
                color == 0xFF ? 0 : NesHardwarePalette::Get()->palette(color);
        }
    }
}

void Z2ObjectCache::CreateObject(uint8_t obj) {
    uint32_t* dest = nullptr;
    int width = 16, height = 16;
    uint8_t tile, pal;
    uint8_t set = obj >> 6;
    uint8_t overworld_pal[] = {
        2, 2, 2, 2,
        3, 0, 0, 0,
        1, 1, 1, 1,
        3, 3, 1, 1,
    };

    // FIXME(cfrantz): Maybe rework this to use polymorphism instead of a
    // big if statement.
    if (schema_ == Schema::ITEM) {
        pal = 1;
        dest = new uint32_t[width * height]();
        int tile = mapper_->Read(obj_[set], obj*2 + 0);
        BlitTile(dest, 0, 0, tile, pal, width);

        int tile2 = mapper_->Read(obj_[set], obj*2 + 1);
        BlitTile(dest, 8, 0, tile2, pal, width, tile == tile2);
    } else if (schema_ == Schema::TILE8x8 || schema_ == Schema::TILE8x16) {
        width = 8;
        height = (schema_ == Schema::TILE8x8) ? 8 : 16;
        dest = new uint32_t[width * height]();
        BlitTile(dest, 0, 0, obj, 1, width);
    } else if (schema_ == Schema::ITEMINFO) {
        const auto& it = info_.info().find(obj);
        if (it != info_.info().end()) {
            const auto& item = it->second;
            width = item.width() ? item.width() : 16;
            height = item.height() ? item.height() : 16;
            dest = new uint32_t[width * height]();
            pal = item.palette();
            chr_ = item.chr();
            int n = 0;
            for(int y=0; y<height; y+=16) {
                int lasttile = -1;
                for(int x=0; x<width; x+=8, n++) {
                    int tile = item.id_size() ? item.id(n) :
                        mapper_->Read(item.table(), n);
                    if (tile == -1)
                        continue;
                    // Magic bits from tile IDs in the config control non-grid
                    // offset placements and mirroring.
                    bool mirror = tile==lasttile || tile & 0x1000000;
                    int xofs = (tile >> 16) & 0xff;
                    int yofs = (tile >> 8) & 0xff;
                    tile &= 0xff;
                    BlitTile(dest, x+xofs, y+yofs, tile, pal, width, mirror);
                    lasttile = tile;
                }
            }
        } else {
            dest = new uint32_t[width * height]();
        }
    } else {
        int offset = (obj & 0x3f) * 4;
        if (schema_ == Schema::OVERWORLD) {
            pal = overworld_pal[obj];
        } else {
            pal = set;
        }
        dest = new uint32_t[width * height]();
        tile = mapper_->Read(obj_[set], offset + 0);
        BlitTile(dest, 0, 0, tile, pal, width);

        tile = mapper_->Read(obj_[set], offset + 1);
        BlitTile(dest, 0, 8, tile, pal, width);

        tile = mapper_->Read(obj_[set], offset + 2);
        BlitTile(dest, 8, 0, tile, pal, width);

        tile = mapper_->Read(obj_[set], offset + 3);
        BlitTile(dest, 8, 8, tile, pal, width);

        // Hack to make walkable water tiles visible
        if (schema_ == Schema::OVERWORLD && obj == 13) {
            for(int i=0; i<256; i++) {
                dest[i] = ((dest[i] >> 1) & 0x7f7f7f7f) | 0xFF000000;
//                if (dest[i] == 0xFFFFB064) dest[i] = 0xFFC08030;
            }
        }
    }
    cache_.emplace(std::make_pair(obj, GLBitmap(width, height, dest)));
}

}  // namespace

