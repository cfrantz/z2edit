#include "util/logging.h"
#include "nes/z2decompress.h"
#include "util/config.h"

namespace z2util {

Z2Decompress::Z2Decompress() {}

void Z2Decompress::Init() {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    memset(info_, 0, sizeof(info_));
    layer_ = 0;
    for(const auto& d : ri.decompress()) {
        int id = d.id();
        if (id & 0xF0) {
            id >>= 4;
        }
        info_[d.area()][d.type()][id] = &d;
    }
}

void Z2Decompress::Clear() {
    layer_ = 0;
    cursor_moves_left_ = false;
    memset(map_, 0, sizeof(map_));
    memset(items_, 0xFF, sizeof(items_));
}

void Z2Decompress::Decompress(const Map& map) {
    compressed_map_ = map;

    if (map.pointer().address()) {
        LOG(INFO, "Map pointer at bank=", map.pointer().bank(),
                  " address=", HEX(map.pointer().address()));
        *compressed_map_.mutable_address() =
            mapper_->ReadAddr(map.pointer(), 0);
    }

    Clear();
    LOG(INFO, "Map at bank=", map.address().bank(),
              " address=", HEX(map.address().address()));
    if (map.type() == MapType::OVERWORLD) {
        DecompressOverWorld(compressed_map_);
    } else {
        DecompressSideView(compressed_map_);
    }
}


void Z2Decompress::DecompressOverWorld(const Map& map) {
    uint8_t *mm = (uint8_t*)map_;
    // FIXME: how to determine the map length instead of specifying manually
    LOG(INFO, "DecompressOverWorld: bank=", map.address().bank(),
              " address=", HEX(map.address().address()),
              " length=", map.length());
    for(int i=0; i<map.length(); i++) {
        uint8_t val = Read(map.address(), i);
        uint8_t type = val & 0x0f;
        uint8_t len = (val >> 4) + 1;
        for(int j=0; j<len; j++) {
            *mm++ = type;
        }
    }
    width_ = 64;
    height_ = (mm - (uint8_t*)map_) / width_;
}

const ItemInfo& Z2Decompress::EnemyInfo() {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();

    for (const auto& e : ri.enemies()) {
        if (compressed_map_.world() == e.world() ||
            (1 << compressed_map_.world()) & e.valid_worlds()) {
            LOG(INFO, "EnemyInfo for world ", e.world());
            return e;
        }
    }
    const auto& e = ri.enemies(0);
    LOG(INFO, "Default EnemyInfo for world ", e.world());
    return e;
}

const BackgroundInfo& Z2Decompress::GetBackgroundInfo() {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    int n = (ground_ >> 4) & 0x7;
    for (const auto& bg : ri.background()) {
        if (n == bg.index() && compressed_map_.type() == bg.type()) {
            return bg;
        }
    }
    LOG(ERROR, "Could not find background info for type=",
            compressed_map_.type(), " index=", n);
    return ri.background(0);
}


void Z2Decompress::DrawFloor(int x, uint8_t floor, uint8_t ceiling) {
    int y;
    const auto& bg = GetBackgroundInfo();
    uint8_t bgtile = uint8_t(bg.background());

    // These two magic bits determine what type of background will be
    // on rows #9 and #10.  I think they're mutually exclusive, and
    // depend on what tile set is selected.
    // Note: dwedit's documenation on these bits is backwards.
    if (flags_ & 0x08) {
        if (isbackground(x, 10, bgtile))
            set_map(x, 10, bg.alternate());
    }
    if (flags_ & 0x04) {
        if (isbackground(x, 9, bgtile))
            set_map(x, 9, bg.alternate());
    }

    if (floor < 8) {
        y = height_ - floor - 2;
        if (isbackground(x, y, bgtile))
            set_map(x, y, bg.floor(0));
        for(y++; y<height_; y++) {
            if (isbackground(x, y, bgtile))
                set_map(x, y, bg.floor(1));
        }
        if (ceiling) {
            if (isbackground(x, 0, bgtile))
                set_map(x, 0, bg.ceiling(1));
        }
    } else if (floor < 15) {
        if (ceiling) {
            ceiling = floor - 6;
            for(y=0; y<ceiling-1; y++) {
                if (isbackground(x, y, bgtile))
                    set_map(x, y, bg.ceiling(0));
            }
            if (isbackground(x, y, bgtile))
                set_map(x, y, bg.ceiling(1));
        }
        if (isbackground(x, 11, bgtile))
            set_map(x, 11, bg.floor(0));
        if (isbackground(x, 12, bgtile))
            set_map(x, 12, bg.floor(1));
    } else {
        for(y=0; y<height_; y++) {
            if (isbackground(x, y, bgtile))
                set_map(x, y, bg.floor(1));
        }
    }
}

Address Z2Decompress::palette() {

    Address addr = compressed_map_.palette();
    addr.set_address(addr.address() + 16 * ((back_ >> 3) & 7));
    return addr;
}

void Z2Decompress::DecompressSideView(const Map& map) {
    // FIXME: for side view maps, the map address is the address of a pointer
    // to the real address.  Read it and set the real address.
    Address address = map.address();
    //address.set_address(ReadWord(map.address(), 0));

    uint8_t len = Read(address, 0);
    LOG(INFO, "### ", map.name());
    LOG(INFO, "### dbp b=", address.bank(), " ", HEX(address.address()),
              " ", len);

    layer_ = 0;

    uint8_t backmap = Read(address, 3) & 7;
    if (backmap) {
        Address backaddr;
        // The backing map pointers are stored at offset zero in the
        // current bank.
        backaddr.set_bank(address.bank());
        backaddr.set_address(0);
        backaddr.set_address(ReadWord(backaddr, (backmap - 1) * 2));
        LOG(INFO, "Decompressing background map at ", HEX(backaddr.address()));
        DecompressSideView(backaddr, address);
        layer_++;
    }
    LOG(INFO, "Decompressing foreground map at ", HEX(address.address()));
    DecompressSideView(address, address);
    CollapseLayers(layer_);
    layer_ = 0;
}

void Z2Decompress::CollapseLayers(int top_layer) {
    const auto& bg = GetBackgroundInfo();
    uint8_t bgtile = uint8_t(bg.background());
    for(int y=0; y<height_; y++) {
        for(int x=0; x<width_; x++) {
            for(int t = top_layer; t>0; t--) {
                uint8_t val = map_[t][y][x];
                if (val && val != bgtile) {
                    map_[0][y][x] = val;
                    break;
                }
            }
        }
    }
}

void Z2Decompress::DecompressSideView(const Address& address,
                                      const Address& foreground) {
    uint8_t len = Read(address, 0);
    uint8_t data[256];
    for(int i=0; i<len; i++) {
        data[i] = Read(address, i);
    }
    // Read the map parameter bytes from the foreground map
    //data[1] = Read(foreground, 1);

    // Get the tileset of the foreground map, but use the floor and
    // ceiling parameters in the background map.
    data[2] = (data[2] & 0x8f) | (Read(foreground, 2) & 0x70);

    //data[3] = Read(foreground, 3);
    DecompressSideView(data);
}

void Z2Decompress::DecompressSideView(const uint8_t* data) {
    uint8_t len = data[0];
    flags_ = data[1];
    ground_ = data[2];
    back_ = data[3];

    LOG(VERBOSE, "Map Length: ", HEX(len));
    LOG(VERBOSE, "Flags: objset=", !!(flags_ & 0x80),
                 " width=", (flags_ >> 5) & 3,
                 " grass=" , !!(flags_ & 8),
                 " bushes=", !!(flags_ & 4));
    LOG(VERBOSE, "HasCeiling=", !!(ground_ & 0x80),
                 " GroundTiles=", (ground_ >> 4) & 7,
                 " Floor=", ground_ & 0xF);
    LOG(VERBOSE, "SpritePal=", (back_ >> 6) & 3,
                 " BackPal=", (back_ >> 3) & 7,
                 " BackMap=", back_ & 7);

    width_ = (1 + ((flags_ >> 5) & 3)) * 16;
    height_ = 13;
    uint8_t floor = ground_ & 0x0f;
    uint8_t ceiling = !(ground_ & 0x80);
    uint8_t objset = !!(flags_ & 0x80);

    const auto& bg = GetBackgroundInfo();
    if ((back_ & 7) == 0) {
        memset(&map_[layer_], uint8_t(bg.background()), sizeof(map_[0]));
    }

    int x, y, xspace;
    x = y = 0;
    for(int i=4; i<len; i+=2) {
        bool collectable = false, extra = false;
        uint8_t pos = data[i];
        uint8_t obj = data[i+1];
//        if (i>4) Print();
        LOG(VERBOSE, "op = ", HEX(pos), " ", HEX(obj));

        xspace = pos & 0xf;
        y = pos >> 4;
        // Deal with the various magic values
        if (y < 13 && obj == 15) {
            i++;
            obj = data[i+1];
            collectable = true;
        } else if (y == 13) {
            for(int j=0; j < xspace; j++, x++) {
                if (x >= width_)
                    goto exitloop;
                DrawFloor(x, floor, ceiling);
            }
            floor = obj & 0x0f;
            ceiling = !(obj & 0x80);
            continue;
        } else if (y == 14) {
            // Set cursor directly to "xspace * 16"
            xspace *= 16;
            if (xspace < x)
                cursor_moves_left_ = true;
            while(x < xspace) {
                DrawFloor(x++, floor, ceiling);
                if (x >= width_)
                    goto exitloop;
            }
            x = xspace;
            continue;
        } else if (y == 15) {
            extra = true;
        }

        for(int j=0; j < xspace; j++, x++) {
            if (x >= width_)
                goto exitloop;
            DrawFloor(x, floor, ceiling);
        }

        DrawFloor(x, floor, ceiling);
        if (collectable) {
            items_[y][x] = obj;
        } else {
            // Adjust the objset index to the PutFn array and set the
            // function index
            uint8_t findex, oindex;
            if (extra) {
                if ((obj & 0xF0) == 0) {
                    oindex = 3;
                    findex = obj & 0x0F;
                } else {
                    oindex = 4;
                    findex = obj >> 4;
                }
            } else if ((obj & 0xF0) == 0) {
                oindex = 0;
                findex = obj & 0x0F;
            } else {
                oindex = objset + 1;
                findex = obj >> 4;
            }

            std::string fn = "Invalid";
            int type = compressed_map_.type();
            const DecompressInfo* info = info_[type][oindex][findex];
            if (info) {
                switch(info->render()) {
                    case DecompressInfo::RENDER_HORIZONTAL:
                        fn = "RenderHorizontal"; break;
                    case DecompressInfo::RENDER_VERTICAL:
                        fn = "RenderVertical"; break;
                    case DecompressInfo::RENDER_TOP_UNIQUE:
                        fn = "RenderTopUnique"; break;
                    case DecompressInfo::RENDER_BOTTOM_UNIQUE:
                        fn = "RenderBottomUnique"; break;
                    case DecompressInfo::RENDER_CUSTOM:
                        fn = info->custom(); break;
                    default:
                        ; // Do nothing
                }
            } else {
                LOG(ERROR, "Couldn't look up ", HEX(obj), " for ",
                           oindex, " ", findex);
            }

            // Look up the put funtion and call it
            PutFn put = put_[fn];
            if (!put) {
                put = &Z2Decompress::Invalid;
            }
            (this->*put)(x, y, obj, info);
        }
    }
    while (x < width_) {
        DrawFloor(x++, floor, ceiling);
    }
exitloop:
    //Print();
    return;
}

void Z2Decompress::Print() {
    printf("   ");
    for(int x=0; x<width_; x++) {
        printf("%x", x & 0xF);
    }
    printf("\n");
    for(int y=0; y<height_; y++) {
        printf("%2d:", y);
        for(int x=0; x<width_; x++) {
            uint8_t val = map(x, y);
            if (val & 0x3f) {
                if (val > 0x7f) val &= 0x7f;
                if (val < 0x20) val += 0x20;
                printf("%c", val);
            } else {
                printf(" ");
            }
        }
        printf("\n");
    }
}

}  // namespace z2util
