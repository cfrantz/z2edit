#include "imwidget/misc_hacks.h"

#include "nes/mapper.h"
#include "proto/rominfo.pb.h"
#include "util/config.h"
#include "imgui.h"


namespace z2util {

bool MiscellaneousHacks::Draw() {
    if (!visible_)
        return false;

    ImGui::Begin("Miscellaneous Hacks", &visible_);
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();

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

    ImGui::PopItemWidth();
    ImGui::End();
    return false;
}

}  // namespace
