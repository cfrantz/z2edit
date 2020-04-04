#include "nes/z2decompress.h"
#include <gflags/gflags.h>

#include "util/logging.h"
#include "util/config.h"

#ifdef NDEBUG
// Turn off logging in this module when not in debug mode, as logging to the
// console on windows is very slow and the decompressor is in the sideview
// editor's draw routine.
#undef LOG
#define LOG(...) do { } while(0)
#endif

DEFINE_int32(max_map_height, 0, "Maximum height of overworld maps");
DEFINE_int32(convert_unprogrammed_overworld_tiles, 0xFC,
            "Convert unprogrammed overworld tiles (0xFF) to this value.");
DECLARE_bool(hackjam2020);

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
        DecompressSideView(address(), nullptr);
    }
}


void Z2Decompress::DecompressOverWorld(const Map& map) {
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    uint8_t *mm = (uint8_t*)map_;
    // FIXME: how to determine the map length instead of specifying manually
    LOG(INFO, "DecompressOverWorld: bank=", map.address().bank(),
              " address=", HEX(map.address().address()),
              " length=", map.length());

    width_ = misc.overworld_width();
    if (width_ != 64) {
        LOG(FATAL, "Overworld width must be 64.");
    }
    height_ = misc.overworld_height();
    if (FLAGS_max_map_height) height_ = FLAGS_max_map_height;
    int i = 0;
    for(int n=0; n < width_ * height_; i++) {
        uint8_t val = Read(map.address(), i);
        //val = FLAGS_convert_unprogrammed_overworld_tiles;
        uint8_t type = val & 0x0f;
        uint8_t len = (val >> 4) + 1;
        if (FLAGS_hackjam2020 && type == 0x0F && len > 1) {
            // Handle expansion tiles.
            len--;
            type = Read(map.address(), ++i);
        }
        for(int j=0; j<len; j++) {
            *mm++ = type;
            n++;
        }
    }
    length_ = i;
}

const ItemInfo& Z2Decompress::EnemyInfo() {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();

    for (const auto& e : ri.enemies()) {
        if (compressed_map_.world() == e.world() &&
            compressed_map_.overworld() == e.overworld()) {
            LOGF(INFO, "EnemyInfo for world %d overworld %d ",
                 e.world(), e.overworld());
            return e;
        }
    }
    const auto& e = ri.enemies(0);
    LOGF(INFO, "EnemyInfo for world %d overworld %d ",
         e.world(), e.overworld());
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
    Address addr;
    if (compressed_map_.palettes_size()) {
        // FIXME: for palace maps, this should be an indirect lookup
        // through the palace palette table at bank 7 $cd35.
        addr = compressed_map_.palettes((back_ >> 3) & 7);
    } else {
        addr = compressed_map_.palette();
        addr.set_address(addr.address() + 16 * ((back_ >> 3) & 7));
    }
    return addr;
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
                                      const Address* foreground) {
    uint8_t len = Read(address, 0);
    uint8_t data[256];
    for(int i=0; i<len; i++) {
        data[i] = Read(address, i);
    }

    // Get the tileset of the foreground map, but use the floor and
    // ceiling parameters in the background map.
    bool collapse = !foreground;
    if (!foreground) foreground = &address;
    data[2] = (data[2] & 0x8f) | (Read(*foreground, 2) & 0x70);
    DecompressSideView(data, collapse);
}

void Z2Decompress::DecompressSideView(const uint8_t* data, bool collapse) {
    // Welcome to the shitshow: ground and back are needed to render
    // the background correctly, but rendering a background map will
    // modify them.  Rather than design this class proerply, I'm lazy
    // and just overwrite the values again after rendering the background.
    ground_ = data[2];
    back_ = data[3];
    const auto& bg = GetBackgroundInfo();
    if (back_ & 7) {
        Address backaddr = address();
        // The backing map pointers are stored at offset zero in the
        // current bank.
        backaddr.set_address(0);
        backaddr.set_address(ReadWord(backaddr, 2 * ((back_ & 7) - 1)));
        LOG(INFO, "Decompressing background map ", (back_&7), " at ", HEX(backaddr.address()));
        if (backaddr.address() == 0 || backaddr.address() == 0xFFFF) {
            LOG(ERROR, "Unexpected address for background map");
        } else {
            DecompressSideView(backaddr, &address());
            layer_++;
        }
    } else {
        layer_ = 0;
        memset(&map_[layer_], uint8_t(bg.background()), sizeof(map_[0]));
    }

    uint8_t len = data[0];
    length_ = len;
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

    width_ = 64;
    height_ = 13;
    mapwidth_ = (1 + ((flags_ >> 5) & 3)) * 16;
    uint8_t floor = ground_ & 0x0f;
    uint8_t ceiling = !(ground_ & 0x80);
    uint8_t objset = !!(flags_ & 0x80);


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
                        LOG(ERROR, "Couldn't find renderer ", info->render());
                        ; // Do nothing
                }
            } else {
                LOG(ERROR, "Couldn't look up ", HEX(obj), " for o=",
                           oindex, " f=", findex, " extra=", extra);
            }

            // Look up the put funtion and call it
            PutFn put = put_[fn];
            if (!put) {
                LOG(ERROR, "Couldn't find PutFn for '", fn, "': ",
                        info->DebugString());
                put = &Z2Decompress::Invalid;
            }
            (this->*put)(x, y, obj, info);
        }
    }
    while (x < width_) {
        DrawFloor(x++, floor, ceiling);
    }
exitloop:
    if (collapse) {
        CollapseLayers(layer_);
        layer_ = 0;
    }
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
