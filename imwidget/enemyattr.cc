#include "imwidget/enemyattr.h"

#include "imwidget/error_dialog.h"
#include "imwidget/imapp.h"
#include "nes/mapper.h"
#include "proto/rominfo.pb.h"
#include "util/config.h"
#include "imgui.h"
#include <gflags/gflags.h>

DECLARE_bool(reminder_dialogs);

namespace z2util {

void PointsTable::Init() {
    Load();
    Address none;
    Address chr;
    chr.set_bank(8);
    cache_.set_mapper(mapper_);
    cache_.Init(none, chr, Z2ObjectCache::Schema::TILE8x16);

    Address ipal;
    // FIXME(cfrantz): hardcoded palette location
    ipal.set_bank(1);
    ipal.set_address(0x809e);
    cache_.set_palette(ipal);

    cache_.Clear();
}

bool PointsTable::Draw() {
    if (!visible_)
        return changed_;

    char buf[16];
    ImGui::Begin("Experience Types", &visible_);
    int i = 0;
    ImGui::PushItemWidth(100);
    for(auto& val : data_) {
        ImGui::PushID(i);
        ImGui::AlignFirstTextHeightToWidgets();
        ImGui::Text("Type %x", i);

        ImGui::SameLine();
        changed_ |= ImGui::InputInt("Experience  ", &val.xp);

        ImGui::PushItemWidth(25);
        ImGui::SameLine();
        sprintf(buf, "%02x", val.gfx[0]);
        if (ImGui::InputText("", buf, sizeof(buf),
                             ImGuiInputTextFlags_CharsHexadecimal)) {
            changed_ = true;
            val.gfx[0] = strtoul(buf, 0, 16);
        }
        ImGui::SameLine();
        sprintf(buf, "%02x", val.gfx[1]);
        if (ImGui::InputText("Graphics", buf, sizeof(buf),
                             ImGuiInputTextFlags_CharsHexadecimal)) {
            changed_ = true;
            val.gfx[1] = strtoul(buf, 0, 16);
        }
        ImGui::PopItemWidth();
        ImGui::SameLine();
        ImVec2 cursor = ImGui::GetCursorPos();
        ImGui::Text(" ");
        cache_.Get(val.gfx[0]).DrawAt(cursor.x, cursor.y);
        cache_.Get(val.gfx[1]).DrawAt(cursor.x+8, cursor.y);
        ImGui::PopID();
        i++;
    }
    ImGui::PopItemWidth();
    ImGui::End();
    return changed_;
}

void PointsTable::Load() {
    const auto& xp = ConfigLoader<RomInfo>::GetConfig().xptable();

    data_.clear();
    for(int i=0; i<16; i++) {
        Unpacked val;
        val.xp = mapper_->Read(xp.lo(), i) | mapper_->Read(xp.hi(), i) << 8;
        val.gfx[0] = mapper_->Read(xp.gfx(), i);
        val.gfx[1] = mapper_->Read(xp.gfx(), i + 16);
        data_.push_back(val);
    }
    changed_ = false;
}

void PointsTable::Save() {
    const auto& xp = ConfigLoader<RomInfo>::GetConfig().xptable();
    int i = 0;
    for(const auto& val : data_) {
        mapper_->Write(xp.lo(), i, val.xp & 0xFF);
        mapper_->Write(xp.hi(), i, val.xp >> 8);
        mapper_->Write(xp.gfx(), i, val.gfx[0]);
        mapper_->Write(xp.gfx(), i+16, val.gfx[1]);
        i++;
    }
    changed_ = false;
}


void EnemyEditor::Init() {
    table_.set_mapper(mapper_);
    table_.Init();
    Load();
}

bool EnemyEditor::Draw() {
    if (!visible_)
        return changed_;

    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    char types[256];
    const char *all = "0\0001\0002\0003\0004\0005\0006\0007\0008\0009\000A\000B\000C\000D\000E\000F\000\0";
    memset(types, 0, sizeof(types));
    for(int i=0, n=0; i<16; i++) {
        n += sprintf(types+n, "%d", table_.xp(i));
        n++;
    }

    const char *names[ri.enemies_size()];
    for(int i=0; i<ri.enemies_size(); i++) {
        names[i] = ri.enemies(i).area().c_str();
    }

    ImGui::Begin("Enemy Attributes", &visible_);
    ImGui::PushItemWidth(400);
    int category = category_;
    if (ImGui::Combo("Category", &category, names, ri.enemies_size())) {
        if (FLAGS_reminder_dialogs && changed_) {
            ErrorDialog::Spawn("Discard Chagnes", 
                ErrorDialog::OK | ErrorDialog::CANCEL,
                "Discard Changes to Enemy Attributes?")->set_result_cb([this, category](int result) {
                    if (result == ErrorDialog::OK) {
                        category_ = category;
                        Load();
                    }
            });
        } else {
            category_ = category;
            Load();
        }
    }
    ImGui::PopItemWidth();

    ImGui::SameLine();
    if (ImGui::Button("Points Table")) {
        table_.set_visible(true);
    }
    changed_ |= table_.Draw();

    ImGui::SameLine();
    if (ImGui::Button("Commit to ROM")) {
        Save();
    }
    ImApp::Get()->HelpButton("enemy-attributes", true);

    ImGui::PushItemWidth(96);
    const auto& einfo = ri.enemies(category_);
    ImGui::Text("%-40s  %-14s %-8s %-8s %-8s %-14s %-8s %-7s %-7s %-7s "
                "%-7s %-7s %-7s %-7s %-7s",
            "Name", "HitPoints", "Palette", "StealXP", "NeedFire", "Points",
            "DropGrp", "BeamImm", "Unknown", "DmgType",
            "ThndImm", "Regen", "Unknown", "SwordImm", "Unknown"
            );
    ImGui::Separator();
    for(int i=0; i<TABLE_LEN; i++) {
        const auto& enemy = einfo.info().find(i);
        if (enemy == einfo.info().end())
            continue;

        ImGui::PushID(i);
        ImGui::AlignFirstTextHeightToWidgets();
        ImGui::Text("%-40.40s ", enemy->second.name().c_str());

        ImGui::SameLine();
        changed_ |= ImGui::InputInt("##hp", &data_[i].hp);
        ImGui::SameLine();
        ImGui::PushItemWidth(40);
        changed_ |= ImGui::Combo("   ##pal", &data_[i].palette, "0\0001\0002\0003\000\0");
        ImGui::PopItemWidth();

        ImGui::SameLine();
        changed_ |= ImGui::Checkbox("    ##steal", &data_[i].steal_xp);
        ImGui::SameLine();
        changed_ |= ImGui::Checkbox("    ##fire", &data_[i].need_fire);
        ImGui::SameLine();
        changed_ |= ImGui::Combo("##xp", &data_[i].xp_type, types);

        ImGui::SameLine();
        ImGui::PushItemWidth(60);
        changed_ |= ImGui::Combo("##dg", &data_[i].drop_group, "None\0Small\0Large\0Unknown\0\0");
        ImGui::PopItemWidth();
        ImGui::SameLine();
        changed_ |= ImGui::Checkbox("    ##nobeam", &data_[i].no_beam);
        ImGui::SameLine();
        changed_ |= ImGui::Checkbox("    ##unk1", &data_[i].unknown1);
        ImGui::SameLine();
        ImGui::PushItemWidth(40);
        changed_ |= ImGui::Combo("##damage", &data_[i].damage_code, all);
        ImGui::PopItemWidth();

        ImGui::SameLine();
        changed_ |= ImGui::Checkbox("    ##nothunder", &data_[i].no_thunder);
        ImGui::SameLine();
        changed_ |= ImGui::Checkbox("    ##regen", &data_[i].regenerate);
        ImGui::SameLine();
        changed_ |= ImGui::Checkbox("    ##unk2", &data_[i].unknown2);
        ImGui::SameLine();
        changed_ |= ImGui::Checkbox("    ##nosword", &data_[i].no_sword);
        ImGui::SameLine();
        ImGui::PushItemWidth(40);
        changed_ |= ImGui::Combo("##unk3", &data_[i].unknown3, all);
        ImGui::PopItemWidth();

        ImGui::PopID();
    }
    ImGui::PopItemWidth();
    ImGui::End();
    return changed_;
}

void EnemyEditor::Load() {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    const auto& einfo = ri.enemies(category_);
    table_.Load();
    data_.clear();
    for(int i=0; i<TABLE_LEN; i++) {
        uint8_t hp = mapper_->Read(einfo.hpinfo(), i);
        uint8_t val = mapper_->Read(einfo.xpinfo(), i);
        int palette = val >> 6;
        bool need_fire = !!(val & 0x20);
        bool steal_xp = !!(val & 0x10);
        int xp_type = val & 0x0f;

        val = mapper_->Read(einfo.xpinfo(), i + TABLE_LEN);
        int drop_group = val >> 6;
        bool no_beam = !!(val & 0x20);
        bool unknown1 = !!(val & 0x10);
        int damage_code = val & 0x0f;

        val = mapper_->Read(einfo.xpinfo(), i + 3*TABLE_LEN);
        bool no_thunder = !!(val & 0x80);
        bool regenerate = !!(val & 0x40);
        bool unknown2 = !!(val & 0x20);
        bool no_sword = !!(val & 0x10);
        int unknown3 = val & 0x0f;

        data_.emplace_back(
            Unpacked{hp,
                     palette, steal_xp, need_fire, xp_type,
                     drop_group, no_beam, unknown1, damage_code,
                     no_thunder, regenerate, unknown2, no_sword, unknown3 });
    }
    changed_ = false;
}

void EnemyEditor::Save() {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    const auto& einfo = ri.enemies(category_);
    table_.Save();
    for(const auto& enemy : einfo.info()) {
        const Unpacked& val = data_.at(enemy.first); 
        mapper_->Write(einfo.hpinfo(), enemy.first, val.hp);
        uint8_t b = (val.palette << 6)
            | (val.need_fire << 5)
            | (val.steal_xp << 4)
            | (val.xp_type);
        mapper_->Write(einfo.xpinfo(), enemy.first, b);

        b = (val.drop_group << 6)
            | (val.no_beam << 5)
            | (val.unknown1 << 4)
            | (val.damage_code);
        mapper_->Write(einfo.xpinfo(), enemy.first + TABLE_LEN, b);

        b = (val.no_thunder << 7)
            | (val.regenerate << 6)
            | (val.unknown2 << 5)
            | (val.no_sword << 4)
            | (val.unknown3);
        mapper_->Write(einfo.xpinfo(), enemy.first + 3*TABLE_LEN, b);
    }
    changed_ = false;
}


}  // namespace z2util
