#include <cstdio>
#include "imwidget/neschrview.h"
#include "imgui.h"

NesChrView::NesChrView()
  : ImWindowBase(false),
  bank_(0) {
    bitmap.reset(new GLBitmap(128, 128));
}

void NesChrView::RenderChr() {
    uint32_t pal[] = { 0xFF000000, 0xFF666666, 0xFFAAAAAA, 0xFFFFFFFF };
    uint32_t *image = bitmap->data();
    int tile = 0;
    // Draw the tiles on a 16x16 grid
    for(int y=0; y<16; y++) {
        for(int x=0; x<16; x++, tile++) {
            // Each tile is 8x8
            for(int row=0; row<8; row++) {
                uint8_t a = mapper_->ReadChrBank(bank_, 16*tile + row);
                uint8_t b = mapper_->ReadChrBank(bank_, 16*tile + row + 8);
                for(int col=0; col<8; col++, a<<=1, b<<=1) {
                    int color = ((a & 0x80) >> 7) | ((b & 0x80) >> 6);
                    image[128*(8*y + row) + 8*x + col] = pal[color];
                }
            }
        }
    }
    bitmap->Update();
}

void NesChrView::MakeLabels() {
    const char *lptrs[256];
    int i;
    // The mapper chrsz is 8k banks, but the mappers typically map chr
    // banks 4k at a time.
    for(i=0; i<mapper_->cartridge()->chrsz()*2; i++) {
        snprintf(labels_[i], sizeof(labels_[0]), "Chr Bank %02x", i);
        lptrs[i] = labels_[i];
    }
    nr_labels_ = i;
    ImGui::Combo("Bank", &bank_, lptrs, nr_labels_);
}

bool NesChrView::Draw() {
    if (!visible_)
        return false;

    RenderChr();

    ImGui::Begin("CHR Data", &visible_);
    MakeLabels();

    // Render the labels on the vertical axis
    ImGui::BeginGroup();
    ImGui::Text(" ");
    float y = ImGui::GetCursorPosY();
    for(int i=0; i<16; i++) {
        ImGui::SetCursorPosY(y + i*32);
        ImGui::Text("%x0", i);
    }
    ImGui::SetCursorPosY(y);
    ImGui::EndGroup();

    ImGui::SameLine();

    // Render the labels on the horizontal axis
    ImGui::BeginGroup();
    ImGui::Text(" ");
    float x = ImGui::GetCursorPosX();
    for(int i=0; i<16; i++) {
        ImGui::SameLine(x + i*32 + 8);
        ImGui::Text("%02x", i);
    }

    // Render the CHR image
    ImGui::SetCursorPosY(y);
    bitmap->Draw(512, 512);
    ImGui::EndGroup();

    ImGui::End();
    return false;
}
