#include "imwidget/start_values.h"
#include "proto/rominfo.pb.h"
#include "imgui.h"

namespace z2util {

StartValues::StartValues()
  : visible_(false)
{}

void StartValues::Unpack() {
    Address addr;
    addr.set_bank(5);
    addr.set_address(0x3ae7);

    data_.shield  = mapper_->Read(addr, 0) != 0;
    data_.jump    = mapper_->Read(addr, 1) != 0;
    data_.life    = mapper_->Read(addr, 2) != 0;
    data_.fairy   = mapper_->Read(addr, 3) != 0;
    data_.fire    = mapper_->Read(addr, 4) != 0;
    data_.reflex  = mapper_->Read(addr, 5) != 0;
    data_.spell   = mapper_->Read(addr, 6) != 0;
    data_.thunder = mapper_->Read(addr, 7) != 0;
    data_.magic   = mapper_->Read(addr, 8);
    data_.heart   = mapper_->Read(addr, 9);
    data_.candle  = mapper_->Read(addr, 10) != 0;
    data_.glove   = mapper_->Read(addr, 11) != 0;
    data_.raft    = mapper_->Read(addr, 12) != 0;
    data_.boots   = mapper_->Read(addr, 13) != 0;
    data_.flute   = mapper_->Read(addr, 14) != 0;
    data_.cross   = mapper_->Read(addr, 15) != 0;
    data_.hammer  = mapper_->Read(addr, 16) != 0;
    data_.key     = mapper_->Read(addr, 17) != 0;
    data_.crystals = mapper_->Read(addr, 25);
    int techs     = mapper_->Read(addr, 27);
    data_.downstab = !!(techs & 0x10);
    data_.upstab   = !!(techs & 0x04);

    addr.set_bank(7);
    addr.set_address(0x0359);
    data_.lives = mapper_->Read(addr, 0);
}

void StartValues::Pack() {
    Address addr;
    addr.set_bank(5);
    addr.set_address(0x3ae7);

    mapper_->Write(addr, 0, data_.shield);
    mapper_->Write(addr, 1, data_.jump);
    mapper_->Write(addr, 2, data_.life);
    mapper_->Write(addr, 3, data_.fairy);
    mapper_->Write(addr, 4, data_.fire);
    mapper_->Write(addr, 5, data_.reflex);
    mapper_->Write(addr, 6, data_.spell);
    mapper_->Write(addr, 7, data_.thunder);

    mapper_->Write(addr, 8, data_.magic);
    mapper_->Write(addr, 9, data_.heart);

    mapper_->Write(addr, 10, data_.candle);
    mapper_->Write(addr, 11, data_.glove);
    mapper_->Write(addr, 12, data_.raft);
    mapper_->Write(addr, 13, data_.boots);
    mapper_->Write(addr, 14, data_.flute);
    mapper_->Write(addr, 15, data_.cross);
    mapper_->Write(addr, 16, data_.hammer);
    mapper_->Write(addr, 17, data_.key);

    mapper_->Write(addr, 25, data_.crystals);
    mapper_->Write(addr, 27,
            (data_.downstab ? 0x10 : 0) | (data_.upstab ? 0x04 : 0));

    addr.set_bank(7);
    addr.set_address(0x0359);
    mapper_->Write(addr, 0, data_.lives);
}

void StartValues::Draw() {
    if (!visible_)
        return;

    Unpack();
    ImGui::Begin("Start Values", &visible_);

    ImGui::PushItemWidth(100);
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
}

}  // namespace z2util
