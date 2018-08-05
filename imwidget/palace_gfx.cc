#include "imwidget/palace_gfx.h"

#include "imwidget/imapp.h"
#include "nes/mapper.h"
#include "proto/rominfo.pb.h"
#include "util/config.h"
#include "imgui.h"

namespace z2util {

bool PalaceGraphics::Draw() {
    if (!visible_)
        return false;

    if (!graphics_.size()) {
        Load();
    }

    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    char chr[misc.palace_table_len()][16];
    char pal[misc.palace_table_len()][16];

    ImGui::Begin("Palace Graphics & Palettes", &visible_);
    int start = 52;
    ImGui::Text("West   DM/MZ  East   CHR bank           Palette Offset");

    ImGui::SameLine(ImGui::GetWindowWidth() - 50);
    ImApp::Get()->HelpButton("palace-graphics");
    ImGui::Separator();

    for(int i=0; i<misc.palace_table_len(); i++) {
        sprintf(chr[i], "CHR##%d", i);
        sprintf(pal[i], "Palette##%d", i);

        for(int world=0; world<3; world++) {
            int n = start + i - world * 4;
            ImGui::AlignFirstTextHeightToWidgets();
            if (n >= 52 && n < 56) {
                ImGui::Text("%02d    ", n);
            } else if (n >= 62) {
                ImGui::Text("      ");
            } else {
                ImGui::TextColored(ImColor(0xFF808080), "%02d    ", n);
            }
            ImGui::SameLine();
        }
    
        ImGui::PushItemWidth(100);
        ImGui::SameLine();
        ImGui::InputInt(chr[i], &graphics_[i]);
        ImGui::SameLine();
        ImGui::InputInt(pal[i], &palette_[i]);
        ImGui::PopItemWidth();
    }
    if (ImGui::Button("Commit to ROM")) {
        Save();
        ImApp::Get()->ProcessMessage("commit", "Palace Graphics/Palette IDs");
    }
    ImGui::End();
    return false;
}

void PalaceGraphics::Load() {
    graphics_.clear();
    palette_.clear();
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    for(int i=0; i<misc.palace_table_len(); i++) {
        graphics_.push_back(mapper_->Read(misc.palace_graphics(), i));
        palette_.push_back(mapper_->Read(misc.palace_palette(), i));
    }
}
void PalaceGraphics::Save() {
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    for(int i=0; i<misc.palace_table_len(); i++) {
        mapper_->Write(misc.palace_graphics(), i, graphics_.at(i));
        mapper_->Write(misc.palace_palette(), i, palette_.at(i));
    }
}

}  // namespace
