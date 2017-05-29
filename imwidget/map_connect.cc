#include "imgui.h"
#include "imwidget/map_connect.h"
#include "imwidget/multimap.h"
#include "imwidget/imutil.h"
#include "util/config.h"

namespace z2util {

OverworldConnector::OverworldConnector(Mapper* mapper, Address address,
                               uint8_t offset, int overworld, int subworld)
  : mapper_(mapper),
    address_(address),
    offset_(offset),
    overworld_(overworld),
    subworld_(subworld),
    drag_(false),
    dx_(0),
    dy_(0)
{
    Read();
}

OverworldConnector::OverworldConnector(const OverworldConnector& other)
  : mapper_(other.mapper_),
    address_(other.address_),
    offset_(other.offset_),
    overworld_(other.overworld_),
    subworld_(other.subworld_),
    drag_(other.drag_),
    dx_(other.dx_),
    dy_(other.dy_)
{
    Read();
}

void OverworldConnector::Read() {
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    const auto& hpal = misc.hidden_palace();
    const auto& htown = misc.hidden_town();
    int overworld = subworld_ ? subworld_ : overworld_;

    uint8_t y = mapper_->Read(address_, offset_ + 0x00);
    uint8_t x = mapper_->Read(address_, offset_ + 0x3f);
    uint8_t z = mapper_->Read(address_, offset_ + 0x7e);
    uint8_t w = mapper_->Read(address_, offset_ + 0xbd);
    if (overworld == mapper_->Read(hpal.cmpov(), 0)
        && offset_ == mapper_->Read(hpal.connector(), 0)) {
        y = mapper_->Read(hpal.connector(), 2);
    } else if (overworld == mapper_->Read(hpal.cmpov(), 0)
               && offset_ == mapper_->Read(htown.connector(), 0)) {
        y = mapper_->Read(htown.connector(), 2);
    }

    y_ = (y & 0x7f) - misc.overworld_y_offset();
    ext_ = !!(y & 0x80);

    x_ = x & 0x3f;
    second_ = !!(x & 0x40);
    exit_2_lower_ = !!(x & 0x80);

    map_ = z & 0x3f;
    entry_ = z >> 6;

    dest_overworld_ = w & 0x3;
    dest_world_ = (w & 0x1c) >> 2;
    entry_right_ = !!(w & 0x20);
    passthru_ = !!(w & 0x40);
    fall_ = !!(w & 0x80);
}

void OverworldConnector::Write() {
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    const auto& hpal = misc.hidden_palace();
    const auto& htown = misc.hidden_town();
    int overworld = subworld_ ? subworld_ : overworld_;
    int raft = mapper_->Read(misc.raft_id(), 0);
    uint8_t y, x, z, w;

    y = (y_ + misc.overworld_y_offset()) | uint8_t(ext_) << 7;
    x = x_ | uint8_t(second_) << 6 | uint8_t(exit_2_lower_) << 7;
    z = map_ | entry_ << 6;
    w = dest_overworld_ | (dest_world_ << 2) | uint8_t(entry_right_) << 5 |
        uint8_t(passthru_) << 6 | uint8_t(fall_) << 7;

    mapper_->Write(address_, offset_ + 0x00, y);
    mapper_->Write(address_, offset_ + 0x3f, x);
    mapper_->Write(address_, offset_ + 0x7e, z);
    mapper_->Write(address_, offset_ + 0xbd, w);

    if (overworld == mapper_->Read(hpal.cmpov(), 0)
        && offset_ == mapper_->Read(hpal.connector(), 0)) {
        // Hidden palace Y coordinate includes the overworld_y_offset.
        // Furthermore, the "call" spot is 2 tiles north of the target
        // destination.
        mapper_->Write(hpal.connector(), 2, y);
        mapper_->Write(hpal.cmpy(), 0, (y & 0x7f) - 2);
        mapper_->Write(hpal.cmpx(), 0, x_);
        mapper_->Write(hpal.returny(), 0, (y & 0x7f));

        // PPU macros have their addresses in big-endian order.
        uint16_t ppu_addr = 0x2000 + 2 * (32*(y_ % 15) + (x_ % 16));
        mapper_->Write(hpal.ppu_macro(), 0, ppu_addr >> 8);
        mapper_->Write(hpal.ppu_macro(), 1, ppu_addr);
        ppu_addr += 32;
        mapper_->Write(hpal.ppu_macro(), 5, ppu_addr >> 8);
        mapper_->Write(hpal.ppu_macro(), 6, ppu_addr);
        // Lets be lazy and not deal with the palette change.
        mapper_->Write(hpal.ppu_macro(), 10, 0xff);

        mapper_->Write(address_, offset_ + 0x00, 0);
    } else if (overworld == mapper_->Read(htown.cmpov(), 0)
               && offset_ == mapper_->Read(htown.connector(), 0)) {
        // Hidden Town Y coordinate does not include the overworld_y_offset.
        // Furthermore, the x location seems to be 1 more than the actual
        // coordinate.
        mapper_->Write(htown.connector(), 2, y);
        mapper_->Write(htown.cmpy(), 0, y_);
        mapper_->Write(htown.cmpx(), 0, x_+1);
        mapper_->Write(htown.returny(), 0, (y & 0x7f));
        mapper_->Write(htown.discriminator(), 0, x_);

        mapper_->Write(address_, offset_ + 0x00, 0);
    } else if (offset_ == raft) {
        // The raft table layout is:
        // ov0_xpos, ov2_xpos, ov0_ypos, ov2_ypos
        int delta = !(overworld_ == 0);
        mapper_->Write(misc.raft_table(), delta+0, x & 0x3f);
        mapper_->Write(misc.raft_table(), delta+2, y & 0x7f);
    }
}

bool OverworldConnector::DrawInPopup() {
    bool chg = false;
    if (ImGui::Button("View Area")) {
        if (dest_world_ == 0 && dest_overworld_ == 0) {
            MultiMap::Spawn(mapper_,
                            dest_world_, overworld_, subworld_, map_);
        } else {
            MultiMap::Spawn(mapper_,
                            dest_world_, 0, 0, map_);
        }
    }

    ImGui::Text("Position:");
    ImGui::PushItemWidth(100);
    chg |= ImGui::InputInt("xpos", &x_);

    ImGui::SameLine();
    chg |= ImGui::InputInt("ypos", &y_);

    ImGui::Text("Connects to:");
    chg |= ImGui::InputInt("map ", &map_);

    ImGui::PushItemWidth(40);
    ImGui::SameLine();
    chg |= ImGui::Combo("w##world", &dest_world_,
                 "0\0001\0002\0003\0004\0005\0006\0007\000\0\0");
    ImGui::SameLine();
    chg |= ImGui::Combo("ov##overworld", &dest_overworld_,
                 "0\0001\0002\0003\0\0");
    ImGui::PopItemWidth();

    ImGui::Text("Properties:");
    chg |= ImGui::Combo("entry", &entry_, "0\000256\000512\000768\0\0");

    chg |= ImGui::Checkbox("extern", &ext_);

    ImGui::SameLine();
    chg |= ImGui::Checkbox("second  ", &second_);

    ImGui::SameLine();
    chg |= ImGui::Checkbox("2 lower", &exit_2_lower_);


    chg |= ImGui::Checkbox("right ", &entry_right_);

    ImGui::SameLine();
    chg |= ImGui::Checkbox("passthru", &passthru_);

    ImGui::SameLine();
    chg |= ImGui::Checkbox("fall", &fall_);
    ImGui::PopItemWidth();
    return chg;
}


OverworldConnectorList::OverworldConnectorList()
  : mapper_(nullptr),
    add_offset_(0),
    changed_(false),
    overworld_(0),
    subworld_(0)
{}

void OverworldConnectorList::Init(Mapper* mapper, Address address,
                                  int overworld, int subworld, int n) {
    mapper_ = mapper;
    list_.clear();
    show_ = true;
    changed_ = false;
    overworld_ = overworld;
    subworld_ = subworld;
    for(int i=0; i<n; i++) {
        list_.emplace_back(mapper, address, i, overworld, subworld);
    }
}

OverworldConnector* OverworldConnectorList::GetAtXY(int x, int y) {
    for(auto& c : list_) {
        if (x == c.xpos() && y == c.ypos())
            return &c;
    }
    return nullptr;
}

OverworldConnector* OverworldConnectorList::Swap(int a, int b) {
    std::swap(list_[a], list_[b]);

    list_[a].offset_ = a;
    list_[b].offset_ = b;
    return &list_[a];
}

void OverworldConnectorList::Save() {
    for(auto& c : list_) {
        c.Write();
    }
    changed_ = false;
}

bool OverworldConnectorList::DrawInEditor(int x, int y) {
    auto* conn = GetAtXY(x, y);
    if (!conn)
        return false;

    const char *ids =
        "00\00001\00002\00003\00004\00005\00006\00007\00008\00009\000"
        "10\00011\00012\00013\00014\00015\00016\00017\00018\00019\000"
        "20\00021\00022\00023\00024\00025\00026\00027\00028\00029\000"
        "30\00031\00032\00033\00034\00035\00036\00037\00038\00039\000"
        "40\00041\00042\00043\00044\00045\00046\00047\00048\00049\000"
        "50\00051\00052\00053\00054\00055\00056\00057\00058\00059\000"
        "60\00061\00062\000Delete\0\0";

    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    const auto& hpal = misc.hidden_palace();
    const auto& htown = misc.hidden_town();
    int overworld = subworld_ ? subworld_ : overworld_;
    int raft = mapper_->Read(misc.raft_id(), 0);

    int offset = conn->offset();
    ImGui::PushID(offset);
    ImVec2 pos = ImGui::GetCursorPos();
    pos.x += conn->dx();
    pos.y += conn->dy();
    ImGui::SetCursorPos(pos);
    if (offset == raft) {
        TextOutlined(ImColor(0xFFFF00FF), "%02dRAFT", offset);
    } else if (overworld == mapper_->Read(hpal.cmpov(), 0)
               && offset == mapper_->Read(hpal.connector(), 0)) {
        TextOutlined(ImColor(0xFFFF00FF), "%02dPAL", offset);
    } else if (overworld == mapper_->Read(htown.cmpov(), 0)
               && offset == mapper_->Read(htown.connector(), 0)) {
        TextOutlined(ImColor(0xFFFF00FF), "%02dTOWN", offset);
    } else {
        TextOutlined(ImColor(0xFFFF00FF), "%02d", offset);
    }
    ImGui::SetCursorPos(pos);
    ImGui::InvisibleButton("##button", ImVec2(16, 16));
    bool focus = ImGui::IsItemHovered();
    if (ImGui::IsItemActive()) {
        if (ImGui::IsMouseDragging()) {
            conn->drag_start();
            ImVec2 delta = ImGui::GetIO().MouseDelta;
            conn->drag(delta.x, delta.y);
        }
    } else {
        changed_ |= conn->drag_finalize(scale_);
    }
    if (ImGui::BeginPopupContextItem("Properties")) {
        if(ImGui::Combo("Swap", &offset, ids)) {
            conn = Swap(offset, conn->offset());
            changed_ = true;
            ImGui::CloseCurrentPopup();
        } else {
            ImGui::Separator();
            changed_ |= conn->DrawInPopup();
        }
        ImGui::EndPopup();
    }
    ImGui::PopID();
    return focus;
}

bool OverworldConnectorList::NoCompress(int x, int y) {
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    const auto& hpal = misc.hidden_palace();
    const auto& htown = misc.hidden_town();
    int overworld = subworld_ ? subworld_ : overworld_;

    for(const auto& c : list_) {
        if (x == c.xpos() && y == c.ypos()) {
            if (overworld == mapper_->Read(hpal.cmpov(), 0)
                && c.offset() == mapper_->Read(hpal.connector(), 0)) {
                return true;
            }
            if (overworld == mapper_->Read(htown.cmpov(), 0)
                && c.offset() == mapper_->Read(htown.connector(), 0)) {
                return true;
            }
        }
    }
    return false;
}


bool OverworldConnectorList::Draw() {
    const char *ids =
        "00\00001\00002\00003\00004\00005\00006\00007\00008\00009\000"
        "10\00011\00012\00013\00014\00015\00016\00017\00018\00019\000"
        "20\00021\00022\00023\00024\00025\00026\00027\00028\00029\000"
        "30\00031\00032\00033\00034\00035\00036\00037\00038\00039\000"
        "40\00041\00042\00043\00044\00045\00046\00047\00048\00049\000"
        "50\00051\00052\00053\00054\00055\00056\00057\00058\00059\000"
        "60\00061\00062\000\0";

    if (ImGui::BeginPopup("Connections")) {
        ImGui::Combo("Entry", &add_offset_, ids);
        ImGui::Separator();
        changed_ |= list_[add_offset_].DrawInPopup();
        ImGui::EndPopup();
    }
    return changed_;
}

}  // namespace z2util
