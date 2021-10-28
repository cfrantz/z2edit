#include "imwidget/drops.h"

#include "imwidget/imapp.h"
#include "imwidget/map_command.h"
#include "nes/mapper.h"
#include "proto/rominfo.pb.h"
#include "util/config.h"
#include "imgui.h"

namespace z2util {

void Drops::Refresh() {
    changed_ = false;
    drop_type_ = 0;
    MapCommand::Init();
    Unpack();
}

bool Drops::Draw() {
    if (!visible_)
        return false;

    ImGui::Begin("Drops", &visible_);
    ImGui::Combo("##type", &drop_type_, "Drop Probabilities\0Hidden Drops\0\0");
    ImGui::SameLine();
    if (ImGui::Button("Commit to ROM")) {
        Pack();
        ImApp::Get()->ProcessMessage("commit", "Item/Enemy Drop Edits");
    }
    ImGui::Text("\n");
    ImGui::PushItemWidth(200);
    if (drop_type_ == 0) {
        DrawDropTable();
    } else if (drop_type_ == 1) {
        int i = 0;
        const auto& di = ConfigLoader<RomInfo>::GetConfig().drop_info();
        for(const auto& d : di.drops()) {
            DrawDropper(d, i);
            i++;
        }
    }
    ImGui::PopItemWidth();
    ImGui::End();
    return changed_;
}

void Drops::DrawDropTable() {
    changed_ |= ImGui::InputInt("Drop Count", &data_.counter);
    
    ImGui::Text("\nDrop Probabilities:");
    ImGui::Separator();
    ImGui::Text("    Small                         Large");
    for(int i=0; i<8; i++) {
        ImGui::PushID(i);
        ImGui::Text("%d: ", i+1);
        ImGui::SameLine();
        changed_ |= ImGui::Combo(
                "##small", data_.small+i,
                MapCommand::collectable_names_, MapCommand::MAX_COLLECTABLE);
        ImGui::SameLine();
        changed_ |= ImGui::Combo(
                "##large", data_.large+i,
                MapCommand::collectable_names_, MapCommand::MAX_COLLECTABLE);
        ImGui::PopID();
    }
}

int Drops::EnemyList(int world, const char **list, int n) {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    for(int i=0; i<n; i++)
        list[i] = "???";

    int max_names = 0;
    for(const auto& e : ri.enemies()) {
        if (e.world() == world) {
            for(const auto& info : e.info()) {
                list[info.first] = info.second.name().c_str();
                if (info.first >= max_names)
                    max_names = info.first+1;
            }
        }
    }
    return max_names;
}

void Drops::DrawDropper(const ItemDrop::Dropper& d, int n) {
    Unpacked::Drop* drop = &data_.drop[n];
    ImGui::PushID(n);
    ImGui::Text("%s", d.name().c_str());
    ImGui::Separator();
    if (d.item().address()) {
        changed_ |= ImGui::Combo(
                "Item", &drop->item,
                MapCommand::collectable_names_, MapCommand::MAX_COLLECTABLE);
    }
    if (d.enemy().address()) {
        const char* list[256];
        int max = EnemyList(d.world(), list, 256);
        changed_ |= ImGui::Combo("Enemy", &drop->enemy, list, max);
    }
    if (d.hp().address()) {
        ImGui::SameLine();
        changed_ |= ImGui::InputInt("HP", &drop->hp);
    }
    ImGui::PopID();
}

void Drops::Unpack() {
    const auto& di = ConfigLoader<RomInfo>::GetConfig().drop_info();
    
    data_.counter = mapper_->Read(di.counter(), 0);
    for(int i=0; i<8; i++) {
        data_.small[i] = mapper_->Read(di.drop_table(), i) & 0x7f;
        data_.large[i] = mapper_->Read(di.drop_table(), i+8) & 0x7f;
    }
    for(const auto& d : di.drops()) {
        int item = d.item().address() ? mapper_->Read(d.item(), 0) : 0;
        int enemy = d.enemy().address() ? mapper_->Read(d.enemy(), 0) : 0;
        int hp = d.hp().address() ? mapper_->Read(d.hp(), 0) : 0;
        data_.drop.emplace_back(Unpacked::Drop{item, enemy, hp});
    }
    changed_ = false;
}

void Drops::Pack() {
    const auto& di = ConfigLoader<RomInfo>::GetConfig().drop_info();
    
    mapper_->Write(di.counter(), 0, data_.counter);
    for(int i=0; i<8; i++) {
        mapper_->Write(di.drop_table(), i, data_.small[i] | 0x80);
        mapper_->Write(di.drop_table(), i+8, data_.large[i] | 0x80);
    }
    int i = 0;
    for(const auto& d : di.drops()) {
        if (d.item().address())
            mapper_->Write(d.item(), 0, data_.drop[i].item);
        if (d.enemy().address())
            mapper_->Write(d.enemy(), 0, data_.drop[i].enemy);
        if (d.hp().address())
            mapper_->Write(d.hp(), 0, data_.drop[i].hp);
        i++;
    }
    changed_ = false;
}

}  // namespace
