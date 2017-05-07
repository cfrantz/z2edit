#include "imwidget/xptable.h"

#include "imwidget/imapp.h"
#include "nes/mapper.h"
#include "proto/rominfo.pb.h"
#include "util/config.h"
#include "imgui.h"

namespace z2util {

void ExperienceTable::Init() {
    Load();
}
bool ExperienceTable::Draw() {
    if (!visible_)
        return changed_;

    ImGui::Begin("Experience Table", &visible_);
    if (ImGui::Button("Commit to ROM")) {
        Save();
        ImApp::Get()->ProcessMessage("commit", "Experience Table");
    }
    ImApp::Get()->HelpButton("experience-table", true);

    ImGui::PushItemWidth(97);
    ImGui::Text("%-20s %-14s %-14s %-14s %-14s %-14s %-14s %-14s %-14s",
            "Type",
            "Level 1", "Level 2", "Level 3", "Level 4",
            "Level 5", "Level 6", "Level 7", "Level 8");
    ImGui::Separator();
    int n = 0;
    for(auto& item : data_) {
        if (n == 3) ImGui::Text(" ");
        if (n == 11) {
            ImGui::Text(
                "\n%-20s %-14s %-14s %-14s %-14s %-14s %-14s %-14s %-14s",
                "",
                "Shield", "Jump", "Life", "Fairy",
                "Fire", "Reflect", "Spell", "Thunder");
            ImGui::Separator();
        }
        if (n == 12) {
            ImGui::Text(
                "\n%-20s %-14s %-14s %-14s %-14s %-14s %-14s %-14s %-14s",
                "XP for Next Level",
                "Level 2", "Level 3", "Level 4", "Level 5",
                "Level 6", "Level 7", "Level 8", "1 UP");
            ImGui::Separator();
        }
        ImGui::Text("%-20.20s", item.name);
        for(int i=0; i<8; i++) {
            ImGui::PushID(n*100 + i);
            ImGui::SameLine();
            changed_ |= ImGui::InputInt("", &item.val[i]);
            ImGui::PopID();
        }
        n++;
    }

    ImGui::PopItemWidth();
    ImGui::End();
    return changed_;
}

void ExperienceTable::LoadSaveWorker(
        std::function<void(const char*, const Address&)> load8,
        std::function<void(const char*, const Address&, int)> loadlevelup) {
    const auto& xpt = ConfigLoader<RomInfo>::GetConfig().xptable();

    load8("Sword Power", xpt.sword());
    load8("Small Damage", xpt.small_damage());
    load8("Large Damage", xpt.large_damage());

    const char* names[] = {
        "Shield", "Jump", "Life", "Fairy", "Fire", "Reflect", "Spell", "Thunder"
    };
    for(int i=0; i<8; i++) {
        load8(names[i], xpt.magic(i));
    }
    load8("Magic Effects", xpt.magic_effects());

    loadlevelup("Attack", xpt.levelup(), 0);
    loadlevelup("Magic", xpt.levelup(), 8);
    loadlevelup("Life", xpt.levelup(), 16);
    changed_ = false;
}

void ExperienceTable::Load() {
    auto load8 = [this](const char* name, const Address& addr) {
        Unpacked elem{name};
        for(int i=0; i<8; i++)
            elem.val[i] = mapper_->Read(addr, i);
        data_.push_back(elem);
    };
    auto loadlevelup = [this](const char* name, const Address& addr, int offset) {
        Unpacked elem{name};
        for(int i=0; i<8; i++)
            elem.val[i] = mapper_->Read(addr, offset+i) << 8
                        | mapper_->Read(addr, offset+i+24);
        data_.push_back(elem);
    };
    LoadSaveWorker(load8, loadlevelup);
}

void ExperienceTable::Save() {
    std::vector<Unpacked> copy = data_;
    auto save8 = [this, &copy](const char* name, const Address& addr) {
        for(int i=0; i<8; i++)
            mapper_->Write(addr, i, copy.at(0).val[i]);
        copy.erase(copy.begin());
    };
    auto savelevelup = [this, &copy](const char* name, const Address& addr, int offset) {
        for(int i=0; i<8; i++) {
            mapper_->Write(addr, offset+i,     copy.at(0).val[i] >> 8);
            mapper_->Write(addr, offset+i+24, copy.at(0).val[i] & 0xFF);
        }
        copy.erase(copy.begin());
    };
    LoadSaveWorker(save8, savelevelup);
    
    const auto& xpt = ConfigLoader<RomInfo>::GetConfig().xptable();
    for(int n=0,factor=10; n<3; n++, factor *= 10) {
        for(int j=0; j<3; j++) {
            for(int i=0; i<8; i++) {
                int value = data_.at(12+j).val[i];
                int digit = (value < factor) ? 0x24 : (value/factor) % 10;
                mapper_->Write(xpt.levelup_gfx(), n*24 + j*8 + i, 0xd0 + digit);
            }
        }
    }
}

}  // z2util
