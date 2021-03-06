#include "imwidget/overworld_encounters.h"

#include "nes/mapper.h"
#include "util/config.h"
#include "imgui.h"

namespace z2util {

void OverworldEncounters::Unpack() {
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    Address addr = map_.pointer();

    // Check to see if this bank even has overworlds.  If not,
    // don't do anything
    addr.set_address(0x8500);
    uint16_t ovptr = mapper_->ReadWord(addr, 10);
    if (ovptr == 0 || ovptr == 0xFFFF)
        return;

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
    int overworld = map_.overworld();
    if (map_.subworld()) overworld = map_.subworld();

    addr = misc.overworld_mason_dixon();
    south_side_ = mapper_->Read(addr, overworld) - misc.overworld_y_offset();
}

bool OverworldEncounters::IsEncounter(int area) {
    for(const auto& data : data_) {
        if (area == data.area)
            return true;
    }
    return false;
}

bool OverworldEncounters::Draw() {
    const char *screens = "0\0001\0002\0003\000\000";
    bool c = false;

    if (ImGui::BeginPopup("Encounters")) {
        ImGui::Text("Overworld Encounters for %d-%d",
                    map_.overworld(), map_.subworld());

        ImGui::PushItemWidth(100);
        c |= ImGui::InputInt("North/South Dividing Line", &south_side_);
        ImGui::PopItemWidth();

        ImGui::Separator();
        ImGui::Columns(2, "ids", true);
        ImGui::Text("North Side");

        ImGui::PushItemWidth(100);
        ImGui::Text("Desert"); ImGui::SameLine();
        c |= ImGui::InputInt("##ndesert",     &data_[0].area); ImGui::SameLine();
        c |= ImGui::Combo("Screen##nsdesert", &data_[0].screen, screens);

        ImGui::Text(" Grass"); ImGui::SameLine();
        c |= ImGui::InputInt("##ngrass",      &data_[2].area); ImGui::SameLine();
        c |= ImGui::Combo("Screen##nsgrass",  &data_[2].screen, screens);
        ImGui::Text("Forest"); ImGui::SameLine();
        c |= ImGui::InputInt("##nforest",     &data_[4].area); ImGui::SameLine();
        c |= ImGui::Combo("Screen##nsforest", &data_[4].screen, screens);
        ImGui::Text(" Swamp"); ImGui::SameLine();
        c |= ImGui::InputInt("##nswamp",      &data_[6].area); ImGui::SameLine();
        c |= ImGui::Combo("Screen##nsswamp",  &data_[6].screen, screens);
        ImGui::Text(" Grave"); ImGui::SameLine();
        c |= ImGui::InputInt("##ngrave",      &data_[8].area); ImGui::SameLine();
        c |= ImGui::Combo("Screen##nsgrave",  &data_[8].screen, screens);
        ImGui::Text("  Road"); ImGui::SameLine();
        c |= ImGui::InputInt("##nroad",       &data_[10].area); ImGui::SameLine();
        c |= ImGui::Combo("Screen##nsroad",   &data_[10].screen, screens);
        ImGui::Text("  Lava"); ImGui::SameLine();
        c |= ImGui::InputInt("##nlava",       &data_[12].area); ImGui::SameLine();
        c |= ImGui::Combo("Screen##nslava",   &data_[12].screen, screens);
        ImGui::PopItemWidth();

        ImGui::NextColumn();
        ImGui::Text("South Side");

        ImGui::PushItemWidth(100);
        c |= ImGui::InputInt("##sdesert",     &data_[1].area); ImGui::SameLine();
        c |= ImGui::Combo("Screen##ssdesert", &data_[1].screen, screens);
        c |= ImGui::InputInt("##sgrass",      &data_[3].area); ImGui::SameLine();
        c |= ImGui::Combo("Screen##ssgrass",  &data_[3].screen, screens);
        c |= ImGui::InputInt("##sforest",     &data_[5].area); ImGui::SameLine();
        c |= ImGui::Combo("Screen##ssforest", &data_[5].screen, screens);
        c |= ImGui::InputInt("##sswamp",      &data_[7].area); ImGui::SameLine();
        c |= ImGui::Combo("Screen##ssswamp",  &data_[7].screen, screens);
        c |= ImGui::InputInt("##sgrave",      &data_[9].area); ImGui::SameLine();
        c |= ImGui::Combo("Screen##ssgrave",  &data_[9].screen, screens);
        c |= ImGui::InputInt("##sroad",       &data_[11].area); ImGui::SameLine();
        c |= ImGui::Combo("Screen##ssroad",   &data_[11].screen, screens);
        c |= ImGui::InputInt("##slava",       &data_[13].area); ImGui::SameLine();
        c |= ImGui::Combo("Screen##sslava",   &data_[13].screen, screens);
        ImGui::PopItemWidth();

        ImGui::EndPopup();
    }
    return c;
}

void OverworldEncounters::Save() {
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    Address addr = map_.pointer();
    addr.set_address(misc.overworld_encounters().address());

    for(int i=0; i<int(data_.size()); i++) {
        uint8_t val = data_[i].area | (data_[i].screen << 6);
        mapper_->Write(addr, i, val);
    }

    int overworld = map_.overworld();
    if (map_.subworld()) overworld = map_.subworld();

    addr = misc.overworld_mason_dixon();
    mapper_->Write(addr, overworld, south_side_ + misc.overworld_y_offset());
}

}  // namespace z2util
