#include "util/logging.h"
#include "nes/z2decompress.h"

namespace z2util {

void Z2Decompress::Invalid(int x, int y, uint8_t item,
                           const DecompressInfo* info) {
    const char *dbg = "<null>";
    if (info)
        dbg = info->DebugString().c_str();
    LOG(ERROR, "Invalid item ", HEX(item), ": ", dbg);
}

void Z2Decompress::NotYet(int x, int y, uint8_t item,
                          const DecompressInfo* info) {
    const char *dbg = "<null>";
    if (info)
        dbg = info->DebugString().c_str();
    LOG(WARN, "Item ", HEX(item), " not implemented yet:", dbg);
}

// Render an 'item & 15' horizontal stripe of a 'height' tall object
void Z2Decompress::RenderHorizontal(int x, int y, uint8_t item,
                                    const DecompressInfo* info) {
    int width = info->width() ? info->width() : (item & 0xF) + 1;
    int height = info->height() ? info->height() : 1;
    int n = info->objid_size();
    if (info->fixed_y()) y = info->fixed_y();
    for(int yy=0; yy<height; yy++) {
        for(int xx=0; xx<width; xx++) {
            if (y + yy < height_) {
                set_map(x+xx, y+yy, info->objid(yy % n));
            }
        }
    }
}

// Render an 'item & 15' horizontal stripe of a building-type object
void Z2Decompress::RenderBuilding(int x, int y, uint8_t item,
                                 const DecompressInfo* info) {
    int width = info->width() ? info->width() : (item & 0xF) + 1;
    int height = info->height() ? info->height() : 13;
    int n = info->objid_size();
    if (n != 2) {
        LOG(ERROR, "Expecting exactly 2 objids in: ", info->DebugString());
        return;
    }
    if (info->fixed_y()) y = info->fixed_y();
    for(int yy=0; yy<height; yy++) {
        for(int xx=0; xx<width; xx++) {
            // FIXME(cfrantz): compute the real floor value
            int objid = (xx < width-1) ? info->objid(0) : info->objid(1);
            if (objid && y + yy < 11) {
                set_map(x+xx, y+yy, objid);
            }
        }
    }
}

// Render an 'item & 15' vertical stripe of a 'width' wide object
void Z2Decompress::RenderVertical(int x, int y, uint8_t item,
                                  const DecompressInfo* info) {
    int width = info->width() ? info->width() : 1;
    int height = info->height() ? info->height() : (item & 0xF) + 1;
    int n = info->objid_size();
    if (info->fixed_y()) y = info->fixed_y();
    for(int yy=0; yy<height; yy++) {
        for(int xx=0; xx<width; xx++) {
            if (y+yy < height_) {
                set_map(x+xx, y+yy, info->objid(xx % n));
            }
        }
    }
}

// Render an 'item & 15' vertical repeats of a house window
void Z2Decompress::RenderWindow(int x, int y, uint8_t item,
                                const DecompressInfo* info) {
    int height = info->height() ? info->height() : (item & 0xF) + 1;
    int n = info->objid_size();
    if (info->fixed_y()) y = info->fixed_y();
    height *= n;
    for(int yy=0; yy<height; yy++) {
        int objid = info->objid(yy % n);
        if (objid && y+yy < height_) {
            set_map(x, y+yy, objid);
        }
    }
}

// Render an 'item & 15' tall object with a unique top piece 
void Z2Decompress::RenderTopUnique(int x, int y, uint8_t item,
                                  const DecompressInfo* info) {
    int width = info->width() ? info->width() : 1;
    int height = info->height() ? info->height() : (item & 0xF) + 1;
    if (info->fixed_y()) y = info->fixed_y();
    for(int yy=0; yy<height; yy++) {
        for(int xx=0; xx<width; xx++) {
            if (yy == 0) {
                set_map(x+xx, y+yy, info->objid(0));
            } else if (y+yy < height_) {
                set_map(x+xx, y+yy, info->objid(1));
            }
        }
    }
}

// Render an 'item & 15' tall object with a unique bottom piece 
void Z2Decompress::RenderBottomUnique(int x, int y, uint8_t item,
                                      const DecompressInfo* info) {
    int width = info->width() ? info->width() : 1;
    int height = info->height() ? info->height() : (item & 0xF) + 1;
    if (info->fixed_y()) y = info->fixed_y();
    for(int yy=0; yy<height; yy++) {
        for(int xx=0; xx<width; xx++) {
            if (yy == height-1) {
                set_map(x+xx, y+yy, info->objid(1));
            } else if (y+yy < height_) {
                set_map(x+xx, y+yy, info->objid(0));
            }
        }
    }
}

void Z2Decompress::RenderGrid(int x, int y, uint8_t item,
                              const DecompressInfo* info) {
    int width = info->width();
    int height = info->height();
    int n = 0;
    if (info->fixed_y()) y = info->fixed_y();
    for(int yy=0; yy<height; yy++) {
        for(int xx=0; xx<width; xx++, n++) {
            if (y+yy < height_ && info->objid(n) != 0) {
                set_map(x+xx, y+yy, info->objid(n));
            }
        }
    }
}

void Z2Decompress::RenderStonehenge(int x, int y, uint8_t item,
                                    const DecompressInfo* info) {
    set_map(x+0, y+0, 0x56);
    set_map(x+1, y+0, 0x57);
    set_map(x+2, y+0, 0x57);
    set_map(x+3, y+0, 0x58);
    set_map(x+0, y+1, 0x59);
    set_map(x+0, y+2, 0x5a);

    set_map(x+3, y+1, 0x59);
    set_map(x+3, y+2, 0x5a);
}

void Z2Decompress::RenderLava3High(int x, int y, uint8_t item,
                                   const DecompressInfo* info) {
    DecompressInfo i = *info;
    i.set_width((item & 0xF) + 1);
    RenderTopUnique(x, 10, item, &i);
}

void Z2Decompress::RenderCactus1(int x, int y, uint8_t item,
                                 const DecompressInfo* info) {
    int height = (item & 0xF) + 1;
    RenderTopUnique(x, 10-height, item, info);
}

void Z2Decompress::RenderCactus2(int x, int y, uint8_t item,
                                 const DecompressInfo* info) {
    RenderTopUnique(x, 8, item, info);
}


std::map<std::string, Z2Decompress::PutFn> Z2Decompress::put_ = {
    { "RenderHorizontal",   &Z2Decompress::RenderHorizontal },
    { "RenderVertical",     &Z2Decompress::RenderVertical },
    { "RenderTopUnique",    &Z2Decompress::RenderTopUnique },
    { "RenderBottomUnique", &Z2Decompress::RenderBottomUnique },
    { "RenderGrid",         &Z2Decompress::RenderGrid },
    { "RenderLava3High",    &Z2Decompress::RenderLava3High },
    { "RenderCactus1",      &Z2Decompress::RenderCactus1 },
    { "RenderCactus2",      &Z2Decompress::RenderCactus2 },
    { "RenderStonehenge",   &Z2Decompress::RenderStonehenge },
    { "RenderBuilding",     &Z2Decompress::RenderBuilding },
    { "RenderWindow",       &Z2Decompress::RenderWindow },
    { "Invalid",            &Z2Decompress::Invalid },
};

}  // namespace z2util
