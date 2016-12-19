#include "nes/z2objcache.h"
#include "nes/mapper.h"
#include "imwidget/hwpalette.h"
#include "util/logging.h"

namespace z2util {

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
        uint32_t* data = new uint32_t[WIDTH * HEIGHT]();
        CreateObject(object, data);
        cache_.emplace(std::make_pair(object, GLBitmap(WIDTH, HEIGHT, data)));
    }
    return cache_[object];
}


void Z2ObjectCache::BlitTile(uint32_t* dest, int x, int y, int tile, int pal,
                             bool flip) {
    int bofs = 0;
    int height = 8;
    dest += y*WIDTH + x;

    if (schema_ == Schema::ITEM) {
        bofs = tile & 1;
        height = 16;
        tile &= ~1;
    }

    for(int row=0; row<height; row++, dest+=WIDTH-8) {
        uint8_t a = mapper_->ReadChrBank(chr_.bank() + bofs,
                                         chr_.address() + 16*(tile + (row / 8)) + (row & 7));
        uint8_t b = mapper_->ReadChrBank(chr_.bank() + bofs,
                                         chr_.address() + 16*(tile + (row / 8)) + (row & 7) + 8);
        for(int col=0; col<8; col++, a<<=1, b<<=1) {
            uint8_t color = (a & 0x80) >> 7 | (b & 0x80) >> 6;
            color = mapper_->Read(palette_, pal * 4 + color);
            dest[flip ? 7-col : col] = 
                color == 0xFF ? 0 : hwpal_->palette(color);
        }
        dest += 8;
    }
}

void Z2ObjectCache::CreateObject(uint8_t obj, uint32_t* dest) {
    uint8_t tile, pal;
    uint8_t set = obj >> 6;
    uint8_t overworld_pal[] = {
        2, 2, 2, 2,
        3, 0, 0, 0,
        1, 1, 1, 1,
        3, 3, 1, 1,
    };
    obj &= 0x3f;
    if (schema_ == Schema::OVERWORLD) {
        pal = overworld_pal[obj];
    } else {
        pal = set;
    }

    if (schema_ == Schema::ITEM) {
        pal = 1;
        int tile = mapper_->Read(obj_[set], obj*2 + 0);
        BlitTile(dest, 0, 0, tile, pal);

        int tile2 = mapper_->Read(obj_[set], obj*2 + 1);
        BlitTile(dest, 8, 0, tile2, pal, tile == tile2);
    } else {
        tile = mapper_->Read(obj_[set], obj*4 + 0);
        BlitTile(dest, 0, 0, tile, pal);

        tile = mapper_->Read(obj_[set], obj*4 + 1);
        BlitTile(dest, 0, 8, tile, pal);

        tile = mapper_->Read(obj_[set], obj*4 + 2);
        BlitTile(dest, 8, 0, tile, pal);

        tile = mapper_->Read(obj_[set], obj*4 + 3);
        BlitTile(dest, 8, 8, tile, pal);

        // Hack to make walkable water tiles visible
        if (schema_ == Schema::OVERWORLD && obj == 13) {
            for(int i=0; i<256; i++) {
                if (dest[i] == 0xFFFFB064) dest[i] = 0xFFC08030;
            }
        }
    }
}

}  // namespace

