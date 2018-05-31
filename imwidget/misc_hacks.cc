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

    ImGui::RadioButton("Miscellaneous", &tab_, 0);
    ImGui::SameLine(); ImGui::RadioButton("Dynamic Banks", &tab_, 1);
    ImGui::SameLine(ImGui::GetWindowWidth() - 50);
    ImApp::Get()->HelpButton("misc");
    ImGui::Separator();

    bool changed = false;
    switch(tab_) {
    case 0:
        changed = DrawMiscHacks();
        break;
    case 1:
        changed = DrawDynamicBanks();
        break;
    default:
        ImGui::Text("Unknown tab value");
    }
    ImGui::End();
    return changed;
}

bool MiscellaneousHacks::DrawMiscHacks() {
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

    int elevator_speed = mapper_->Read(misc.elevator_speed(), 1);
    if (ImGui::InputInt("Elevator Speed", &elevator_speed)) {
        mapper_->Write(misc.elevator_speed(), 1, elevator_speed);
        mapper_->Write(misc.elevator_speed(), 2, -elevator_speed);
    }

    // 0x92aa: 10 18 e8 ff, 00 18 e8 ff, 00 18 e8 00
    int fairy_speed = mapper_->Read(misc.fairy_speed(), 1);
    if (ImGui::InputInt("Fairy Speed", &fairy_speed)) {
        mapper_->Write(misc.fairy_speed(), 1, fairy_speed);
        mapper_->Write(misc.fairy_speed(), 2, -fairy_speed);
        mapper_->Write(misc.fairy_speed(), 4+1, fairy_speed);
        mapper_->Write(misc.fairy_speed(), 4+2, -fairy_speed);
        mapper_->Write(misc.fairy_speed(), 8+1, fairy_speed);
        mapper_->Write(misc.fairy_speed(), 8+2, -fairy_speed);
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

    Hack("Overworld BreakBlocks", ri.overworld_breakblocks_size(),
        [&]() { return ri.overworld_breakblocks(); },
        [&](int n) { return ri.overworld_breakblocks(n); });

    return false;
}

bool MiscellaneousHacks::DrawDynamicBanks() {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    char roombuf[8], bankbuf[8];
    const char *overworlds[] = {"West", "DM/Maze", "East"};
    const char *worlds[] = {"Caves", "Towns", "Towns", "P125", "P346", "GP" };

    int banks = Hack("CHR Banks", ri.dynamic_banks_size(),
        [&]() { return ri.dynamic_banks(); },
        [&](int n) { return ri.dynamic_banks(n); });
    int ro = (banks == 0) ? ImGuiInputTextFlags_ReadOnly : 0;

    ImGui::Columns(9, NULL, true);
    ImGui::Text("Overworld World");
    ImGui::NextColumn();
    for(int i=0; i<8; i++) {
        ImGui::Text("Room CHR");
        ImGui::NextColumn();
    }
    ImGui::Separator();
    for(int ov=0; ov<3; ov++) {
        for(int world=0; world<6; world++) {
            ImGui::Text("%9s %5s", overworlds[ov], worlds[world]);
            ImGui::NextColumn();
            for(int i=0; i<16; i+=2) {
                int addr = 0xbe60 + (ov * 5 + world) * 16 + i;
                ImGui::PushID(addr);
                int room = mapper_->ReadPrgBank(0, addr + 0);
                int bank = mapper_->ReadPrgBank(0, addr + 1);
                snprintf(roombuf, sizeof(roombuf), "%d", room);
                snprintf(bankbuf, sizeof(bankbuf), "%02x", bank);
                ImGui::PushItemWidth(32);
                if (ImGui::InputText("##room", roombuf, sizeof(roombuf), ImGuiInputTextFlags_CharsDecimal | ro)) {
                    room = strtoul(roombuf, 0, 10);
                    mapper_->WritePrgBank(0, addr+0, room);
                }
                ImGui::SameLine();
                if (ImGui::InputText("##bank", bankbuf, sizeof(bankbuf), ImGuiInputTextFlags_CharsHexadecimal | ro)) {
                    bank = strtoul(bankbuf, 0, 16);
                    mapper_->WritePrgBank(0, addr+1, bank);
                }
                ImGui::PopItemWidth();
                ImGui::PopID();
                ImGui::NextColumn();
            }
            ImGui::Separator();
        }
    }
    ImGui::Columns(1);
    return false;
}


template<class GETALL, class GET>
int MiscellaneousHacks::Hack(const char* hackname, int n,
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
    return method;
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
