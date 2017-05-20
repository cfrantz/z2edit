#include "imwidget/misc_hacks.h"

#include "imwidget/imapp.h"
#include "nes/mapper.h"
#include "proto/rominfo.pb.h"
#include "util/config.h"
#include "imgui.h"


namespace z2util {

bool MiscellaneousHacks::Draw() {
    if (!visible_)
        return false;

    ImGui::Begin("Miscellaneous Hacks", &visible_);
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    const auto& misc = ri.misc();

    // At bank=0, offset $71d is near the end of the subroutine which determines
    // where Link can walk on the overworld:
    //
    // 712: c90d          CMP #$0d
    // 714: d005          BNE $05
    // 716: ac8807        LDY $0788
    // 719: d004          BNE $04
    // 71b: c90b          CMP #$0b
    // 71d: b002          BCS $02
    // 71f: 18            CLC
    // 720: 60            RTS
    //
    // By re-writing the target of the BCS instruction, we can force the carry
    // flag to be clear all the time, thus allowing link to walk anywhere on
    // the overworld.
    bool walk_anywhere = mapper_->ReadPrgBank(0, 0x71e) == 0x00;
    if (ImGui::Checkbox("Walk Anywhere on Overworld", &walk_anywhere)) {
        if (walk_anywhere) {
            mapper_->WritePrgBank(0, 0x71e, 0);
        } else {
            mapper_->WritePrgBank(0, 0x71e, 2);
        }
    }

    ImGui::SameLine(ImGui::GetWindowWidth() - 50);
    ImApp::Get()->HelpButton("misc");

    ImGui::PushItemWidth(100);
    int item_delay = mapper_->Read(misc.item_pickup_delay(), 0);
    if (ImGui::InputInt("Item Pickup Delay", &item_delay)) {
        mapper_->Write(misc.item_pickup_delay(), 0, item_delay & 0xFF);
    }

    int text_delay = mapper_->Read(misc.text_delay(0), 0);
    if (ImGui::InputInt("Text Delay", &text_delay)) {
        text_delay &= 0xFF;
        int delay1 = text_delay - 0x1f; if (delay1 < 0) delay1 = 0;
        int delay2 = text_delay - 0x25; if (delay2 < 0) delay2 = 0;
        mapper_->Write(misc.text_delay(0), 0, text_delay);
        mapper_->Write(misc.text_delay(1), 0, delay1);
        mapper_->Write(misc.text_delay(2), 0, delay2);
    }

    int beam_time = 256 - mapper_->Read(misc.beam_sword_time(), 0);
    if (ImGui::InputInt("Beam Sword Time", &beam_time)) {
        mapper_->Write(misc.beam_sword_time(), 0, 256-beam_time);
    }

    int beam_speed = mapper_->Read(misc.beam_sword_speed(), 0);
    if (ImGui::InputInt("Beam Sword Speed", &beam_speed)) {
        mapper_->Write(misc.beam_sword_speed(), 0, beam_speed);
        mapper_->Write(misc.beam_sword_speed(), 1, -beam_speed);
    }

    ImGui::PopItemWidth();

    Hack("Palace 5 detect", ri.palace5_detect_size(),
        [&]() { return ri.palace5_detect(); },
        [&](int n) { return ri.palace5_detect(n); });

    Hack("Palace Continue", ri.palace_continue_size(),
        [&]() { return ri.palace_continue(); },
        [&](int n) { return ri.palace_continue(n); });

    Hack("Completed Places", ri.palace_to_stone_size(),
        [&]() { return ri.palace_to_stone(); },
        [&](int n) { return ri.palace_to_stone(n); });

    ImGui::End();
    return false;
}


template<class GETALL, class GET>
void MiscellaneousHacks::Hack(const char* hackname, int n,
                              GETALL getall, GET get) {
    const char *names[n];
    int len = 0;
    int method = 0;
    for(const auto& hack: getall()) {
        names[len] = hack.name().c_str();
        if (MemcmpHack(hack.hack(0))) {
            method = len;
        }
        len++;
    }
    ImGui::PushItemWidth(400);
    if (ImGui::Combo(hackname, &method, names, len)) {
        PutGameHack(get(method));
    }
    ImGui::PopItemWidth();
}


bool MiscellaneousHacks::MemcmpHack(const PokeData& data) {
    for(int i=0; i<data.data_size(); i++) {
        if (mapper_->Read(data.address(), i) != data.data(i)) {
            return false;
        }
    }
    return true;
}

void MiscellaneousHacks::PutPokeData(const PokeData& data) {
    for(int i=0; i<data.data_size(); i++) {
        mapper_->Write(data.address(), i, data.data(i));
    }
}

void MiscellaneousHacks::PutGameHack(const GameHack& hack) {
    for(const auto& h : hack.hack()) {
        PutPokeData(h);
    }
}

}  // namespace
