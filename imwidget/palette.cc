#include "imwidget/palette.h"

#include "imwidget/error_dialog.h"
#include "imwidget/hwpalette.h"
#include "imwidget/imutil.h"
#include "nes/mapper.h"
#include "proto/rominfo.pb.h"
#include "util/config.h"
#include "imgui.h"
#include <gflags/gflags.h>

DECLARE_bool(reminder_dialogs);

namespace z2util {

void PaletteEditor::Init() {
    Load();
}

bool PaletteEditor::Draw() {
    if (!visible_)
        return false;

    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    const auto* hw = NesHardwarePalette::Get();
    const char *groupnames[ri.palettes_size()];
    char names[256][16], popups[256][32];
    int grplen = 0;
    int grpsel = grpsel_;

    for(const auto& p : ri.palettes()) {
        groupnames[grplen++] = p.name().c_str();
    }
    ImGui::Begin("Palette Editor", &visible_);

    ImGui::PushItemWidth(400);
    if (ImGui::Combo("Palette Group", &grpsel, groupnames, grplen)) {
        if (FLAGS_reminder_dialogs && changed_) {
            ErrorDialog::Spawn("Discard Chagnes", 
                ErrorDialog::OK | ErrorDialog::CANCEL,
                "Discard Changes to Palettes?")->set_result_cb([this, grpsel](int result) {
                    if (result == ErrorDialog::OK) {
                        grpsel_ = grpsel;
                        Load();
                    }
            });
        } else {
            grpsel_ = grpsel;
            Load();
        }
    }
    ImGui::PopItemWidth();

    ImGui::SameLine();
    if (ImGui::Button("Commit to ROM")) {
        Save();
    }

    int id = 0;
    int n = 0;
    ImGui::Text("%-20s   %-18s %-18s%-18s %-18s",
            "Title", "Palette 0", "Palette 1", "Palette 2", "Palette 3");
    ImGui::Separator();
    for(const auto& p: ri.palettes(grpsel_).palette()) {
        ImGui::Text("%20s", p.name().c_str());
        for(int i=0; i<16; i++, id++) {
            ImGui::SameLine();
            if ((i % 4) == 0) {
                ImGui::Text(" ");
                ImGui::SameLine();
            }
            int col = data_[n].color[i];
            sprintf(names[id], "%02x##%d", col, id);
            sprintf(popups[id], "Color Selector##%d", id);
            ImGui::PushStyleColor(ImGuiCol_Button, hw->imcolor(col));
            if (ImGui::Button(names[id])) {
                ImGui::OpenPopup(popups[id]);
            }
            ImGui::PopStyleColor(1);
            changed_ |= ColorSelector(popups[id], n, i, col);
        }
        n++;
    }

    ImGui::End();
    return changed_;
}

void PaletteEditor::Load() {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    data_.clear();

    for(const auto& p: ri.palettes(grpsel_).palette()) {
        Unpacked elem;
        for(int i=0; i<16; i++) {
            elem.color[i] = mapper_->Read(p.address(), i);
        }
        data_.push_back(elem);
    }
    changed_ = false;
}

void PaletteEditor::Save() {
    int j = 0;
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    for(const auto& p: ri.palettes(grpsel_).palette()) {
        for(int i=0; i<16; i++) {
            mapper_->Write(p.address(), i, data_[j].color[i]);
        }
        j++;
    }
    changed_ = false;
}

bool PaletteEditor::ColorSelector(const char *name, int pnum, int cnum, int color) {
    const auto* hw = NesHardwarePalette::Get();
    int x, y, i;
    char names[64][32];
    bool changed = false;

    if (ImGui::BeginPopup(name)) {
        for(x=0; x<16; x++) {
            if (x) ImGui::SameLine();
            ImGui::BeginGroup();
            ImGui::Text(x == 0 ? "   %02x" : "%02x", x);
            for(y=0; y<4; y++) {
                if (x == 0) {
                    ImGui::Text("%x0", y);
                    ImGui::SameLine();
                }
                i = y * 16 + x;
                sprintf(names[i], "  ##sel%02x", i);
                ImGui::PushStyleColor(ImGuiCol_Button, hw->imcolor(i));
                ImGui::PushStyleColor(ImGuiCol_ButtonHovered, Brighter(hw->imcolor(i)));
                ImGui::PushStyleColor(ImGuiCol_ButtonActive, Brighter(hw->imcolor(i)));
                if (ImGui::Button(names[i])) {
                    data_[pnum].color[cnum] = i;
                    ImGui::CloseCurrentPopup();
                    changed = true;
                }
                ImGui::PopStyleColor(3);
            }
            ImGui::EndGroup();
        }
        ImGui::EndPopup();
    }
    return changed;
}

}  // namespace z2util
