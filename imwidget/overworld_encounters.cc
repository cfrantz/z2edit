#include "imwidget/overworld_encounters.h"

#include "nes/mapper.h"
#include "util/config.h"
#include "imgui.h"

namespace z2util {

void OverworldEncounters::Unpack() {
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    Address addr = map_.pointer();

    // The overworld encounter table is fixed at address $8409
    // in the same bank as the overworld map
    addr.set_address(misc.overworld_encounters().address());
    data_.clear();
    for(int i=0; i<14; i++) {
        Unpacked data;

        uint8_t val = mapper_->Read(addr, i);
        data.area = val & 0x3f;
        data.screen = val >> 6;
        data_.push_back(data);
    }

    // The north/south dividing line is in a table in bank 7 at $cb32
    // Because of the whole death mountain/maze island fiasco, those worlds
    // share the north/south dividing line.
    int world = map_.world();
    if (world & 1) world &= 1;

    addr = misc.overworld_mason_dixon();
    south_side_ = mapper_->Read(addr, world) - misc.overworld_y_offset();
}

void OverworldEncounters::Draw() {
    int worlds = map_.world() & ~1;
    const char *screens = "0\0001\0002\0003\000\000";

    if (ImGui::BeginPopup("Encounters")) {
        ImGui::Text("Overworld Encounters for Worlds %d & %d", worlds, worlds+1);

        ImGui::PushItemWidth(100);
        ImGui::InputInt("North/South Dividing Line", &south_side_);
        ImGui::PopItemWidth();

        ImGui::Separator();
        ImGui::Columns(2, "ids", true);
        ImGui::Text("North Side");

        ImGui::PushItemWidth(100);
        ImGui::Text("Desert"); ImGui::SameLine();
        ImGui::InputInt("##ndesert",     &data_[0].area); ImGui::SameLine();
        ImGui::Combo("Screen##nsdesert", &data_[0].screen, screens);

        ImGui::Text(" Grass"); ImGui::SameLine();
        ImGui::InputInt("##ngrass",      &data_[2].area); ImGui::SameLine();
        ImGui::Combo("Screen##nsgrass",  &data_[2].screen, screens);
        ImGui::Text("Forest"); ImGui::SameLine();
        ImGui::InputInt("##nforest",     &data_[4].area); ImGui::SameLine();
        ImGui::Combo("Screen##nsforest", &data_[4].screen, screens);
        ImGui::Text(" Swamp"); ImGui::SameLine();
        ImGui::InputInt("##nswamp",      &data_[6].area); ImGui::SameLine();
        ImGui::Combo("Screen##nsswamp",  &data_[6].screen, screens);
        ImGui::Text(" Grave"); ImGui::SameLine();
        ImGui::InputInt("##ngrave",      &data_[8].area); ImGui::SameLine();
        ImGui::Combo("Screen##nsgrave",  &data_[8].screen, screens);
        ImGui::Text("  Road"); ImGui::SameLine();
        ImGui::InputInt("##nroad",       &data_[10].area); ImGui::SameLine();
        ImGui::Combo("Screen##nsroad",   &data_[10].screen, screens);
        ImGui::Text("  Lava"); ImGui::SameLine();
        ImGui::InputInt("##nlava",       &data_[12].area); ImGui::SameLine();
        ImGui::Combo("Screen##nslava",   &data_[12].screen, screens);
        ImGui::PopItemWidth();

        ImGui::NextColumn();
        ImGui::Text("South Side");

        ImGui::PushItemWidth(100);
        ImGui::InputInt("##sdesert",     &data_[1].area); ImGui::SameLine();
        ImGui::Combo("Screen##ssdesert", &data_[1].screen, screens);
        ImGui::InputInt("##sgrass",      &data_[3].area); ImGui::SameLine();
        ImGui::Combo("Screen##ssgrass",  &data_[3].screen, screens);
        ImGui::InputInt("##sforest",     &data_[5].area); ImGui::SameLine();
        ImGui::Combo("Screen##ssforest", &data_[5].screen, screens);
        ImGui::InputInt("##sswamp",      &data_[7].area); ImGui::SameLine();
        ImGui::Combo("Screen##ssswamp",  &data_[7].screen, screens);
        ImGui::InputInt("##sgrave",      &data_[9].area); ImGui::SameLine();
        ImGui::Combo("Screen##ssgrave",  &data_[9].screen, screens);
        ImGui::InputInt("##sroad",       &data_[11].area); ImGui::SameLine();
        ImGui::Combo("Screen##ssroad",   &data_[11].screen, screens);
        ImGui::InputInt("##slava",       &data_[13].area); ImGui::SameLine();
        ImGui::Combo("Screen##sslava",   &data_[13].screen, screens);
        ImGui::PopItemWidth();

        ImGui::EndPopup();
    }
}

void OverworldEncounters::Save() {
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    Address addr = map_.pointer();
    addr.set_address(misc.overworld_encounters().address());

    for(int i=0; i<int(data_.size()); i++) {
        uint8_t val = data_[i].area | (data_[i].screen << 6);
        mapper_->Write(addr, i, val);
    }

    int world = map_.world();
    if (world & 1) world &= 1;

    addr = misc.overworld_mason_dixon();
    mapper_->Write(addr, world, south_side_ + misc.overworld_y_offset());
}

}  // namespace z2util
