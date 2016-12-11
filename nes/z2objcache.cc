#include "nes/z2objcache.h"
#include "nes/mapper.h"
#include "imwidget/hwpalette.h"

namespace z2util {

void Z2ObjectCache::Init(const Map& map) {
    Clear();
    palette_ = map.palette();
    chr_ = map.chr();
    type_ = map.type();

    for(int i=0; i<map.objtable_size(); i++) {
        if (type_ == MapType::OVERWORLD) {
            obj_[i] = map.objtable(i);
        } else {
            // For sideview maps, the object table is a list of addresses
            // to the object table.  
            obj_[i] = mapper_->ReadAddr(map.objtable(i), 0);
        }
    }
}

GLBitmap& Z2ObjectCache::Get(uint8_t object) {
    auto item = cache_.find(object);
    if (item == cache_.end()) {
        uint32_t* data = new uint32_t[WIDTH * HEIGHT];
        CreateObject(object, data);
        cache_.emplace(std::make_pair(object, GLBitmap(WIDTH, HEIGHT, data)));
    }
    return cache_[object];
}


void Z2ObjectCache::BlitTile(uint32_t* dest, int x, int y, int tile, int pal) {
    dest += y*WIDTH + x;

    for(int row=0; row<8; row++, dest+=WIDTH-8) {
        uint8_t a = mapper_->ReadChrBank(chr_.bank(),
                                         chr_.address() + 16*tile + row);
        uint8_t b = mapper_->ReadChrBank(chr_.bank(),
                                         chr_.address() + 16*tile + row + 8);
        for(int col=0; col<8; col++, a<<=1, b<<=1, dest++) {
            uint8_t color = (a & 0x80) >> 7 | (b & 0x80) >> 6;
            color = mapper_->Read(palette_, pal * 4 + color);
            *dest = hwpal_->palette(color);
        }
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
    if (type_ == MapType::OVERWORLD) {
        pal = overworld_pal[obj];
    } else {
        pal = set;
    }

    tile = mapper_->Read(obj_[set], obj*4 + 0);
    BlitTile(dest, 0, 0, tile, pal);

    tile = mapper_->Read(obj_[set], obj*4 + 1);
    BlitTile(dest, 0, 8, tile, pal);

    tile = mapper_->Read(obj_[set], obj*4 + 2);
    BlitTile(dest, 8, 0, tile, pal);

    tile = mapper_->Read(obj_[set], obj*4 + 3);
    BlitTile(dest, 8, 8, tile, pal);

    // Hack to make walkable water tiles visible
    if (type_ == MapType::OVERWORLD && obj == 13) {
        for(int i=0; i<256; i++) {
            if (dest[i] == 0xFFFFB064) dest[i] = 0xFFC08030;
        }
    }
}

}  // namespace

