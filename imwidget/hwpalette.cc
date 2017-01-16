#include <cstdio>
#include "imwidget/hwpalette.h"

// Color values as ARGB
const uint32_t standard_palette[] = {
    0xFF666666, 0xFF002A88, 0xFF1412A7, 0xFF3B00A4,
    0xFF5C007E, 0xFF6E0040, 0xFF6C0600, 0xFF561D00,
    0xFF333500, 0xFF0B4800, 0xFF005200, 0xFF004F08,
    0xFF00404D, 0xFF000000, 0xFF000000, 0xFF000000,
    0xFFADADAD, 0xFF155FD9, 0xFF4240FF, 0xFF7527FE,
    0xFFA01ACC, 0xFFB71E7B, 0xFFB53120, 0xFF994E00,
    0xFF6B6D00, 0xFF388700, 0xFF0C9300, 0xFF008F32,
    0xFF007C8D, 0xFF000000, 0xFF000000, 0xFF000000,
    0xFFFFFEFF, 0xFF64B0FF, 0xFF9290FF, 0xFFC676FF,
    0xFFF36AFF, 0xFFFE6ECC, 0xFFFE8170, 0xFFEA9E22,
    0xFFBCBE00, 0xFF88D800, 0xFF5CE430, 0xFF45E082,
    0xFF48CDDE, 0xFF4F4F4F, 0xFF000000, 0xFF000000,
    0xFFFFFEFF, 0xFFC0DFFF, 0xFFD3D2FF, 0xFFE8C8FF,
    0xFFFBC2FF, 0xFFFEC4EA, 0xFFFECCC5, 0xFFF7D8A5,
    0xFFE4E594, 0xFFCFEF96, 0xFFBDF4AB, 0xFFB3F3CC,
    0xFFB5EBF2, 0xFFB8B8B8, 0xFF000000, 0xFF000000,
};

NesHardwarePalette* NesHardwarePalette::Get() {
    static NesHardwarePalette* singleton = new NesHardwarePalette();
    return singleton;
}

void NesHardwarePalette::Init() {
    for(int i=0; i<64; i++) {
        uint32_t color = standard_palette[i];
        // Flip the color ordering to ABGR
        color = (color & 0xFF00FF00) |
                (color & 0x00FF0000) >> 16 |
                (color & 0x000000FF) << 16;
        palette_[i] = color;
        fpal_[i] = ImColor(color);
        sprintf(label_[i], "Edit Color %02x", i);
    }
}

void NesHardwarePalette::Draw() {
    int x, y, i;
    if (!visible_)
        return;

    ImGui::Begin("Hardware Palette", visible());
    ImGui::Text("Right click to edit colors.");
    for(x=0; x<16; x++) {
        if (x) ImGui::SameLine();
        ImGui::BeginGroup();
        ImGui::Text(x == 0 ? "   %02x" : "%02x", x);
        for(y=0; y<4; y++) {
            if (x == 0) {
                ImGui::Text("%x0", y);
                ImGui::SameLine();
            }
            i = y * 16 + x;
            ImGui::ColorButton(fpal_[i]);
            if (ImGui::BeginPopupContextItem(label_[i])) {
                ImGui::Text("Edit Color");
                ImGui::ColorEdit3("##edit", (float*)&fpal_[i]);
                if (ImGui::Button("Close"))
                    ImGui::CloseCurrentPopup();
                ImGui::EndPopup();
                palette_[i] = ImU32(fpal_[i]);
            }
        }
        ImGui::EndGroup();
    }
    ImGui::End();
}
