#include "imwidget/simplemap.h"
#include "imwidget/imutil.h"

SimpleMap::SimpleMap()
  : visible_(false)
{
}

void SimpleMap::BlitTile(int x, int y, int tile, int pal) {
    uint32_t bmp[8][8];
    for(int row=0; row<8; row++) {
        uint8_t a = mapper_->ReadChrBank(chr_.bank(),
                                         chr_.address() + 16*tile + row);
        uint8_t b = mapper_->ReadChrBank(chr_.bank(),
                                         chr_.address() + 16*tile + row + 8);
        for(int col=0; col<8; col++, a<<=1, b<<=1) {
            uint8_t color = (a & 0x80) >> 7 | (b & 0x80) >> 6;
            color = mapper_->ReadPrgBank(pal_.bank(),
                                         pal_.address() + pal * 4 + color);
            bmp[row][col] = hwpal_->palette(color);
        }
    }
    bitmap_->Blit(x, y, 8, 8, (uint32_t*)bmp);
}

void SimpleMap::BlitObject(int x, int y, int obj) {
    uint8_t tile, pal;
    uint8_t set = obj >> 6;
    uint8_t overworld_pal[] = {
        2, 2, 2, 2,
        3, 0, 0, 0,
        1, 1, 1, 1,
        3, 3, 1, 1,
    };
    obj &= 0x3f;
    if (decomp_.type() == z2util::MapType::OVERWORLD) {
        pal = overworld_pal[obj];
    } else {
        pal = set;
    }

    tile = mapper_->ReadPrgBank(obj_[set].bank(),
                                obj_[set].address() + obj*4 + 0);
    BlitTile(x*16, y*16, tile, pal);

    tile = mapper_->ReadPrgBank(obj_[set].bank(),
                                obj_[set].address() + obj*4 + 1);
    BlitTile(x*16, y*16+8, tile, pal);

    tile = mapper_->ReadPrgBank(obj_[set].bank(),
                                obj_[set].address() + obj*4 + 2);
    BlitTile(x*16+8, y*16, tile, pal);

    tile = mapper_->ReadPrgBank(obj_[set].bank(),
                                obj_[set].address() + obj*4 + 3);
    BlitTile(x*16+8, y*16+8, tile, pal);
}

void SimpleMap::Draw() {
    const char *names[256];
    int len=0, mapsel = mapsel_;
    if (!visible_)
        return;

    ImGui::Begin("SimpleMap", visible());
    for(const auto& m : rominfo_->map()) {
        names[len++] = m.name().c_str();
    }
    ImGui::Combo("Map", &mapsel_, names, len);
    if (mapsel_ != mapsel) {
        SetMap(rominfo_->map(mapsel_), *rominfo_);
    }

    // ImVec2 ul = ImGui::GetCursorScreenPos();
    // ImDrawList* dl = ImGui::GetWindowDrawList();
    ImVec2 cursor = ImGui::GetCursorPos();
    if (bitmap_)
        bitmap_->Draw();
    //for(int i=0; i<4; i++) {
    //    ImVec2 lr = ImVec2(ul.x+256, ul.y+208);
    //    dl->AddRect(ul, lr, 0xFFFFFFFF);
    //    ul.x += 256;
    //}

    if (map_.type() == z2util::MapType::OVERWORLD) {
        connector_.Draw();
        if (connector_.show()) {
            for(int i=0; i<63; i++) {
                int x, y;
                connector_.GetXY(i, &x, &y);
                x *= 16; y *= 16;
                ImGui::SetCursorPos(cursor + ImVec2(x + 1, y + 1));
                TextOutlined(ImColor(0xFFFF00FF), "%02x", i);
            }
        }
    } else {
        if (holder_.Draw()) {
            decomp_.Clear();
            decomp_.DecompressSideView(holder_.MapData().data());
            pal_ = decomp_.palette();
            for(int y=0; y<decomp_.height(); y++) {
                for(int x=0; x<decomp_.width(); x++) {
                    BlitObject(x, y, decomp_.map(x, y));
                }
            }
            bitmap_->Update();
        }
    }

    ImGui::End();
}

void SimpleMap::SetMap(const z2util::Map& map, const z2util::RomInfo& ri) {
    map_ = map;
    rominfo_ = &ri;
    pal_ = map.palette();

    for(int i=0; i<4 && i<map.objtable_size(); i++) {
        if (map.type() != z2util::MapType::OVERWORLD) {
            // FIXME: for sideview maps, the object table pointers 
            // are actually pointers to pointers.  Deref.
            z2util::Address a = map.objtable(i);
            obj_[i].set_bank(a.bank());
            obj_[i].set_address(
                mapper_->ReadPrgBank(a.bank(), a.address()) |
                mapper_->ReadPrgBank(a.bank(), a.address()+1) << 8);
        } else {
            obj_[i] = map.objtable(i);
        }
    }
    chr_ = map.chr();

    holder_.set_mapper(mapper_);
    decomp_.Init();
    decomp_.Decompress(map);
    // FIXME(cfrantz): stupid palette hack
    if (map.type() == z2util::MapType::OVERWORLD) {
        connector_.Init(mapper_, map.connector());
    } else {
        pal_ = decomp_.palette();
        holder_.Parse(map);
    }
    bitmap_.reset(new GLBitmap(decomp_.width() * 16, decomp_.height() * 16));

    for(int y=0; y<decomp_.height(); y++) {
        for(int x=0; x<decomp_.width(); x++) {
            BlitObject(x, y, decomp_.map(x, y));
        }
    }
    bitmap_->Update();
}

