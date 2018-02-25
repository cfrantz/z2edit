#include "imwidget/tile_transform.h"

#include "imwidget/imapp.h"
#include "nes/mapper.h"
#include "proto/rominfo.pb.h"
#include "util/config.h"
#include "imgui.h"

namespace z2util {

void TileTransform::Refresh() {
    changed_ = false;
    Unpack();
}

bool TileTransform::Draw() {
    if (!visible_)
        return false;
    const auto& tt = ConfigLoader<RomInfo>::GetConfig().tile_transform_table();
    const char* tiles = "Town\0Cave\0Palace\0Bridge\0Desert\0Grass\0Forest\0"
                        "Swamp\0Graveyard\0Road\0Lava\0Mountain\0Water\0"
                        "WalkableWater\0Rock\0Spider\0\0";

    ImGui::Begin("Tile Transforms", &visible_);
    if (ImGui::Button("Commit to ROM")) {
        Pack();
        ImApp::Get()->ProcessMessage("commit", "Tile Transforms");
    }
    ImGui::Text("\n");
    ImGui::Text("From Tile                     To Tile");
    ImGui::Separator();
    ImGui::PushItemWidth(200);
    for(int i=0; i<tt.length(); i++) {
        ImGui::PushID(i);
        ImGui::Combo("##from", &data_.from[i], tiles);
        ImGui::SameLine();
        ImGui::Combo("##to", &data_.to[i], tiles);
        ImGui::PopID();
    }
    ImGui::PopItemWidth();
    ImGui::End();
    return changed_;
}

void TileTransform::Unpack() {
    const auto& tt = ConfigLoader<RomInfo>::GetConfig().tile_transform_table();
    
    data_.from.clear();
    data_.to.clear();
    
    for(int i=0; i<tt.length(); i++) {
        data_.from.push_back(mapper_->Read(tt.from_tile(), i));
        data_.to.push_back(mapper_->Read(tt.to_tile(), i));
    }
    changed_ = false;
}

void TileTransform::Pack() {
    const auto& tt = ConfigLoader<RomInfo>::GetConfig().tile_transform_table();
    
    for(int i=0; i<tt.length(); i++) {
        mapper_->Write(tt.from_tile(), i, data_.from.at(i));
        mapper_->Write(tt.to_tile(), i, data_.to.at(i));
    }
    changed_ = false;
}

}  // namespace
