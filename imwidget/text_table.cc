#include <cstring>

#include "imwidget/text_table.h"

#include "imwidget/error_dialog.h"
#include "imwidget/imapp.h"
#include "imwidget/imutil.h"
#include "nes/mapper.h"
#include "proto/rominfo.pb.h"
#include "util/config.h"
#include "imgui.h"
#include <gflags/gflags.h>

DECLARE_bool(reminder_dialogs);

namespace z2util {

void TextTableEditor::Init() {
    Load();
}

int TextTableEditor::TotalLength() {
    int other = 1 - world_;
    int i;
    int total = 0;
    // Count up size of strings in the other world.
    for(i=0; i<pack_.Length(other); ++i) {
        total += pack_.Get(other, i).size() + 1;
    }
    // Count up size of strings actively under edit.
    for(i=0; i<pack_.Length(world_); ++i) {
        total += strlen(data_[i]) + 1;
    }
    return total;
}

bool TextTableEditor::Draw() {
    const auto& tt = ConfigLoader<RomInfo>::GetConfig().text_table();
    if (!visible_)
        return false;

    const char *names[2] = {"West Hyrule", "East Hyrule"};
    int world = world_;

    ImGui::Begin("TextTable Editor", &visible_);

    ImGui::PushItemWidth(200);
    if (ImGui::Combo("World", &world, names, 2)) {
        if (FLAGS_reminder_dialogs && changed_) {
            ErrorDialog::Spawn("Discard Chagnes", 
                ErrorDialog::OK | ErrorDialog::CANCEL,
                "Discard Changes to TextTables?")->set_result_cb([this, world](int result) {
                    if (result == ErrorDialog::OK) {
                        world_ = world;
                        Load();
                    }
            });
        } else {
            world_ = world;
            Load();
        }
    }
    ImGui::PopItemWidth();

    ImGui::SameLine();
    if (ImGui::Button("Commit to ROM")) {
        Save();
        ImApp::Get()->ProcessMessage("commit", "TextTable Edits");
    }
    ImApp::Get()->HelpButton("texttable", true);

    ImGui::Text("Space available: %d bytes (%d / %d used)",
            tt.text_data().length() - TotalLength(),
            TotalLength(), tt.text_data().length());

    ImGui::BeginChild("texttable", ImVec2(0, 0), true);
    int len = pack_.Length(world_);
    for(int i=0; i<len; i++) {
        ImGui::PushID(i);
        ImGui::Text("%3d: ", i);
        ImGui::SameLine();
        changed_ |= ImGui::InputText("##text", data_[i], sizeof(data_[0]));
        ImGui::PopID();
    }
    ImGui::EndChild();
    ImGui::End();
    return changed_;
}

void TextTableEditor::Load() {
    memset(data_, 0, sizeof(data_));
    pack_.set_mapper(mapper_);
    pack_.Unpack(3);
    pack_.CheckIndex();

    int len = pack_.Length(world_);
    for(int i=0; i<len; i++) {
        std::string s;
        pack_.Get(world_, i, &s);
        memcpy(data_[i], s.data(), s.size());
    }
    changed_ = false;
}

void TextTableEditor::Save() {
    const auto& tt = ConfigLoader<RomInfo>::GetConfig().text_table();
    int len = pack_.Length(world_);
    for(int i=0; i<len; i++) {
        pack_.Set(world_, i, data_[i]);
    }
    if (!pack_.Pack()) {
        ErrorDialog::Spawn("Error Saving Text Table",
                "Failed to save the text table.  You probably\n"
                "have too much text.  Adjust your text to have\n"
                "fewer than ", tt.text_data().length(), " bytes.");
                
        return;
    }
    // Reload to round-trip the text through the zelda2 text encoding.
    Load();
}

}  // namespace z2util
