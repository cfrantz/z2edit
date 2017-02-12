#include "imwidget/start_values.h"
#include "proto/rominfo.pb.h"
#include "util/config.h"
#include "imgui.h"

namespace z2util {

StartValues::StartValues()
  : ImWindowBase(false)
{}

void StartValues::Unpack() {
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    Address addr = misc.start_values();

    data_.atklvl = mapper_->Read(addr, 0);
    data_.maglvl = mapper_->Read(addr, 1);
    data_.lifelvl = mapper_->Read(addr, 2);

    data_.shield  = mapper_->Read(addr, 4) != 0;
    data_.jump    = mapper_->Read(addr, 5) != 0;
    data_.life    = mapper_->Read(addr, 6) != 0;
    data_.fairy   = mapper_->Read(addr, 7) != 0;
    data_.fire    = mapper_->Read(addr, 8) != 0;
    data_.reflex  = mapper_->Read(addr, 9) != 0;
    data_.spell   = mapper_->Read(addr, 10) != 0;
    data_.thunder = mapper_->Read(addr, 11) != 0;
    data_.magic   = mapper_->Read(addr, 12);
    data_.heart   = mapper_->Read(addr, 13);
    data_.candle  = mapper_->Read(addr, 14) != 0;
    data_.glove   = mapper_->Read(addr, 15) != 0;
    data_.raft    = mapper_->Read(addr, 16) != 0;
    data_.boots   = mapper_->Read(addr, 17) != 0;
    data_.flute   = mapper_->Read(addr, 18) != 0;
    data_.cross   = mapper_->Read(addr, 19) != 0;
    data_.hammer  = mapper_->Read(addr, 20) != 0;
    data_.key     = mapper_->Read(addr, 21) != 0;
    data_.crystals = mapper_->Read(addr, 29);
    int techs     = mapper_->Read(addr, 31);
    data_.downstab = !!(techs & 0x10);
    data_.upstab   = !!(techs & 0x04);

    addr = misc.start_lives();
    data_.lives = mapper_->Read(addr, 0);
}

void StartValues::Pack() {
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    Address addr = misc.start_values();

    mapper_->Write(addr, 0, data_.atklvl);
    mapper_->Write(addr, 1, data_.maglvl);
    mapper_->Write(addr, 2, data_.lifelvl);

    mapper_->Write(addr, 4, data_.shield);
    mapper_->Write(addr, 5, data_.jump);
    mapper_->Write(addr, 6, data_.life);
    mapper_->Write(addr, 7, data_.fairy);
    mapper_->Write(addr, 8, data_.fire);
    mapper_->Write(addr, 9, data_.reflex);
    mapper_->Write(addr, 10, data_.spell);
    mapper_->Write(addr, 11, data_.thunder);

    mapper_->Write(addr, 12, data_.magic);
    mapper_->Write(addr, 13, data_.heart);

    mapper_->Write(addr, 14, data_.candle);
    mapper_->Write(addr, 15, data_.glove);
    mapper_->Write(addr, 16, data_.raft);
    mapper_->Write(addr, 17, data_.boots);
    mapper_->Write(addr, 18, data_.flute);
    mapper_->Write(addr, 19, data_.cross);
    mapper_->Write(addr, 20, data_.hammer);
    mapper_->Write(addr, 21, data_.key);

    mapper_->Write(addr, 29, data_.crystals);
    mapper_->Write(addr, 31,
            (data_.downstab ? 0x10 : 0) | (data_.upstab ? 0x04 : 0));

    addr = misc.start_lives();
    mapper_->Write(addr, 0, data_.lives);
}

bool StartValues::Draw() {
    if (!visible_)
        return false;

    Unpack();
    ImGui::Begin("Start Values", &visible_);

    ImGui::PushItemWidth(100);
    ImGui::InputInt("Attack", &data_.atklvl);
    ImGui::SameLine();
    ImGui::InputInt("Magic", &data_.maglvl);
    ImGui::SameLine();
    ImGui::InputInt("Life", &data_.lifelvl);

    ImGui::InputInt("Heart Containers", &data_.heart);
    ImGui::InputInt("Magic Containers", &data_.magic);

    ImGui::InputInt("Crystals", &data_.crystals);
    ImGui::InputInt("Lives", &data_.lives);
    ImGui::PopItemWidth();

    ImGui::Checkbox("Downstab", &data_.downstab);
    ImGui::SameLine();
    ImGui::Checkbox("Upstab", &data_.upstab);

    ImGui::Columns(2, "items", true);

    ImGui::Checkbox("Sheild Magic", &data_.shield);
    ImGui::Checkbox("Jump Magic", &data_.jump);
    ImGui::Checkbox("Life Magic", &data_.life);
    ImGui::Checkbox("Fairy Magic", &data_.fairy);
    ImGui::Checkbox("Fire Magic", &data_.fire);
    ImGui::Checkbox("Relect Magic", &data_.reflex);
    ImGui::Checkbox("Spell Magic", &data_.spell);
    ImGui::Checkbox("Thunder Magic", &data_.thunder);

    ImGui::NextColumn();

    ImGui::Checkbox("Candle", &data_.candle);
    ImGui::Checkbox("Glove", &data_.glove);
    ImGui::Checkbox("Raft", &data_.raft);
    ImGui::Checkbox("Boots", &data_.boots);
    ImGui::Checkbox("Flute", &data_.flute);
    ImGui::Checkbox("Cross", &data_.cross);
    ImGui::Checkbox("Hammer", &data_.hammer);
    ImGui::Checkbox("Magic Key", &data_.key);

    ImGui::Columns(1);
    ImGui::End();
    Pack();
    return false;
}

}  // namespace z2util
