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
    subworld_(subworld)
{
    Read();
}

OverworldConnector::OverworldConnector(const OverworldConnector& other)
  : mapper_(other.mapper_),
    address_(other.address_),
    offset_(other.offset_)
{
    Read();
}

void OverworldConnector::Read() {
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    uint8_t y = mapper_->Read(address_, offset_ + 0x00);
    uint8_t x = mapper_->Read(address_, offset_ + 0x3f);
    uint8_t z = mapper_->Read(address_, offset_ + 0x7e);
    uint8_t w = mapper_->Read(address_, offset_ + 0xbd);

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
}

void OverworldConnector::Draw() {
    Read();
    if (ImGui::Button("View Area")) {
        if (dest_world_ == 0 && dest_overworld_ == 0) {
            MultiMap::New(mapper_,
                          dest_world_, overworld_, subworld_, map_);
        } else {
            MultiMap::New(mapper_,
                          dest_world_, 0, 0, map_);
        }
    }

    ImGui::PushID(offset_);
    ImGui::Text("%02d: ", offset_);

    ImGui::PushItemWidth(100);
    ImGui::SameLine();
    ImGui::InputInt("xpos", &x_);

    ImGui::SameLine();
    ImGui::InputInt("ypos", &y_);

    ImGui::SameLine();
    ImGui::InputInt("map", &map_);

    ImGui::PushItemWidth(40);
    ImGui::SameLine();
    ImGui::Combo("w##world", &dest_world_,
                 "0\0001\0002\0003\0004\0005\0006\0007\000\0\0");
    ImGui::SameLine();
    ImGui::Combo("ov##overworld", &dest_overworld_,
                 "0\0001\0002\0003\0\0");
    ImGui::PopItemWidth();

    ImGui::SameLine();
    ImGui::Combo("entry", &entry_, "0\000256\000512\000768\0\0");

    ImGui::SameLine();
    ImGui::Checkbox("extern", &ext_);

    ImGui::SameLine();
    ImGui::Checkbox("second", &second_);

    ImGui::SameLine();
    ImGui::Checkbox("2 lower", &exit_2_lower_);

    ImGui::SameLine();
    ImGui::Checkbox("right", &entry_right_);

    ImGui::SameLine();
    ImGui::Checkbox("passthru", &passthru_);

    ImGui::SameLine();
    ImGui::Checkbox("fall", &fall_);
    ImGui::PopItemWidth();
    ImGui::PopID();
    Write();
}

void OverworldConnector::DrawInPopup() {
    Read();
    if (ImGui::Button("View Area")) {
        if (dest_world_ == 0 && dest_overworld_ == 0) {
            MultiMap::New(mapper_,
                          dest_world_, overworld_, subworld_, map_);
        } else {
            MultiMap::New(mapper_,
                          dest_world_, 0, 0, map_);
        }
    }

    ImGui::Text("Position:");
    ImGui::PushItemWidth(100);
    ImGui::InputInt("xpos", &x_);

    ImGui::SameLine();
    ImGui::InputInt("ypos", &y_);


    ImGui::Text("Connects to:");
    ImGui::InputInt("map ", &map_);

    ImGui::PushItemWidth(40);
    ImGui::SameLine();
    ImGui::Combo("w##world", &dest_world_,
                 "0\0001\0002\0003\0004\0005\0006\0007\000\0\0");
    ImGui::SameLine();
    ImGui::Combo("ov##overworld", &dest_overworld_,
                 "0\0001\0002\0003\0\0");
    ImGui::PopItemWidth();

    ImGui::Text("Properties:");
    ImGui::Combo("entry", &entry_, "0\000256\000512\000768\0\0");

    ImGui::Checkbox("extern", &ext_);

    ImGui::SameLine();
    ImGui::Checkbox("second  ", &second_);

    ImGui::SameLine();
    ImGui::Checkbox("2 lower", &exit_2_lower_);


    ImGui::Checkbox("right ", &entry_right_);

    ImGui::SameLine();
    ImGui::Checkbox("passthru", &passthru_);

    ImGui::SameLine();
    ImGui::Checkbox("fall", &fall_);
    ImGui::PopItemWidth();
    Write();
}


OverworldConnectorList::OverworldConnectorList()
  : add_offset_(0)
{}

void OverworldConnectorList::Init(Mapper* mapper, Address address,
                                  int overworld, int subworld, int n) {
    list_.clear();
    show_ = true;
    for(int i=0; i<n; i++) {
        list_.emplace_back(mapper, address, i, overworld, subworld);
    }
}

void OverworldConnectorList::Draw() {
    ImGui::Text("Overworld Connections:");                                                
    ImGui::Checkbox("Show on map", &show_);
    ImGui::BeginChild("connectors",                                            
                      ImVec2(ImGui::GetWindowContentRegionWidth()*0.7, 300), 
                      true, ImGuiWindowFlags_HorizontalScrollbar);              
    for(auto& c : list_) {
        c.Draw();
    }
    ImGui::EndChild();
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

    list_[a].Write();
    list_[b].Write();
    return &list_[a];
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

    int offset = conn->offset();
    ImGui::PushID(offset);
    TextOutlined(ImColor(0xFFFF00FF), "%02d", conn->offset());
    bool focus = ImGui::IsItemHovered();
    if (ImGui::BeginPopupContextItem("Properties")) {
        if(ImGui::Combo("Swap", &offset, ids)) {
            conn = Swap(offset, conn->offset());
            ImGui::CloseCurrentPopup();
        } else {
            ImGui::Separator();
            conn->DrawInPopup();
        }
        ImGui::EndPopup();
    }
    ImGui::PopID();
    return focus;
}

void OverworldConnectorList::DrawAdd() {
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
        list_[add_offset_].DrawInPopup();
        ImGui::EndPopup();
    }
}

}  // namespace z2util
