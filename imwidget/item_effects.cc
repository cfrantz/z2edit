#include "imwidget/item_effects.h"

#include "imwidget/imapp.h"
#include "imwidget/imutil.h"
#include "nes/mapper.h"
#include "proto/rominfo.pb.h"
#include "util/config.h"
#include "imgui.h"

namespace z2util {

namespace {
int log2b(int byte) {
    switch(byte) {
        case 0x01: return 0;
        case 0x02: return 1;
        case 0x04: return 2;
        case 0x08: return 3;
        case 0x10: return 4;
        case 0x20: return 5;
        case 0x40: return 6;
        case 0x80: return 7;
    }
    return -1;
}
}

void ItemEffects::Refresh() {
    changed_ = false;
    Unpack();
}

bool ItemEffects::Draw() {
    if (!visible_)
        return false;

    ImGui::Begin("ItemEffects", &visible_);
    if (ImGui::Button("Commit to ROM")) {
        Pack();
        ImApp::Get()->ProcessMessage("commit", "Item Effects Edits");
    }
    ImApp::Get()->HelpButton("item-effects", true);
    changed_ = DrawItemTable();
    ImGui::End();
    return changed_;
}

bool ItemEffects::DrawItemTable() {
    const char *bits =
        "bit 0\0bit 1\0bit 2\0bit 3\0bit 4\0bit 5\0bit 6\0bit 7\0\0";
    const char *towns =
        "Rauru\0Ruto\0Saria\0Mido\0Nabooru\0Darunia\0New Kasuto\0Old Kasuto\0\0";
    const char *names[] = {
        "Trophy", "Medicine", "Child", "Magic Containers"
    };
    bool changed = false;
    for(int i=0; i<4; i++) {
        if (i) ImGui::Separator();
        ImGui::AlignFirstTextHeightToWidgets();
        ImGui::PushID(i);
        ImGui::Text("%20s activates", names[i]);
        ImGui::SameLine();
        ImGui::PushItemWidth(70);
        changed |= ImGui::Combo("##bit", &item_[i].bit, bits);
        ImGui::PopItemWidth();
        ImGui::SameLine(); ImGui::Text("in town"); ImGui::SameLine();
        ImGui::PushItemWidth(100);
        changed |= ImGui::Combo("##town", &item_[i].town, towns);
        if (i == 3) {
            ImGui::Text("when player has"); ImGui::SameLine();
            changed |= ImGui::InputInt("containers ##mc", &item_[i].mc_count);
            Clamp(&item_[i].mc_count, 0, 8);
        }
        ImGui::PopItemWidth();
        ImGui::PopID();
    }
    return changed;
}


void ItemEffects::Unpack() {
    const auto& ie = ConfigLoader<RomInfo>::GetConfig().item_effects();
    int town, bit, n;

    item_.clear();

    town = mapper_->ReadWord(ie.trophy().load(), 0) - ie.town_base();
    bit = mapper_->Read(ie.trophy().bits(), 0);
    item_.emplace_back(UnpackedItemEffect{town, log2b(bit)});

    town = mapper_->ReadWord(ie.medicine().load(), 0) - ie.town_base();
    bit = mapper_->Read(ie.medicine().bits(), 0);
    item_.emplace_back(UnpackedItemEffect{town, log2b(bit)});

    town = mapper_->ReadWord(ie.child().load(), 0) - ie.town_base();
    bit = mapper_->Read(ie.child().bits(), 0);
    item_.emplace_back(UnpackedItemEffect{town, log2b(bit)});

    town = mapper_->ReadWord(ie.magic_containers().load(), 0) - ie.town_base();
    bit = mapper_->Read(ie.magic_containers().bits(), 0);
    n = mapper_->Read(ie.magic_container_count(), 0);
    item_.emplace_back(UnpackedItemEffect{town, log2b(bit), n});

    changed_ = false;
}

void ItemEffects::Pack() {
    const auto& ie = ConfigLoader<RomInfo>::GetConfig().item_effects();
    int16_t base = ie.town_base();

    mapper_->WriteWord(ie.trophy().load(), 0, item_[0].town + base);
    mapper_->WriteWord(ie.trophy().save(), 0, item_[0].town + base);
    mapper_->Write(ie.trophy().bits(), 0, 1 << item_[0].bit);

    mapper_->WriteWord(ie.medicine().load(), 0, item_[1].town + base);
    mapper_->WriteWord(ie.medicine().save(), 0, item_[1].town + base);
    mapper_->Write(ie.medicine().bits(), 0, 1 << item_[1].bit);

    mapper_->WriteWord(ie.child().load(), 0, item_[2].town + base);
    mapper_->WriteWord(ie.child().save(), 0, item_[2].town + base);
    mapper_->Write(ie.child().bits(), 0, 1 << item_[2].bit);

    mapper_->WriteWord(ie.magic_containers().load(), 0, item_[3].town + base);
    mapper_->WriteWord(ie.magic_containers().save(), 0, item_[3].town + base);
    mapper_->Write(ie.magic_containers().bits(), 0, 1 << item_[3].bit);
    mapper_->Write(ie.magic_container_count(), 0, item_[3].mc_count);

    changed_ = false;
}

}  // namespace
