#include <cstdio>
#include "imwidget/error_dialog.h"
#include "imwidget/imapp.h"
#include "imwidget/neschrview.h"
#include "imgui.h"

#ifdef HAVE_NFD
#include "nfd.h"
#endif

NesChrView::NesChrView(int bank)
  : ImWindowBase(false),
  bank_(bank), mode_(true), grid_(true) {
    int sz = grid_ ? 160 : 128;
    bitmap_.reset(new GLBitmap(sz, sz));
}

NesChrView::NesChrView() : NesChrView(0) {}

void NesChrView::RenderChr() {
    uint32_t pal[] = { 0xFF000000, 0xFF666666, 0xFFAAAAAA, 0xFFFFFFFF };
    uint32_t *image = bitmap_->data();
    int tile = 0;
    int inc = mode_ + 1;
    int width = grid_ ? 160 : 128;
    int sz = grid_ ? 10 : 8;

    if (grid_) {
        for(int i=0; i<width*width; i++) {
            image[i] = 0xFFFF00FF;
        }
        image += width*inc + 1;
    }
    // Draw the tiles on a 16x16 grid
    for(int y=0; y<16/inc; y++) {
        for(int x=0; x<16; x++, tile+=inc) {
            // Each tile is 8x8 or 8x16
            for(int row=0; row<8*inc; row++) {
                uint8_t a = mapper_->ReadChrBank(
                        bank_, 16*(tile + !!(row&8)) + (row & 7));
                uint8_t b = mapper_->ReadChrBank(
                        bank_, 16*(tile + !!(row&8)) + (row & 7) + 8);
                for(int col=0; col<8; col++, a<<=1, b<<=1) {
                    int color = ((a & 0x80) >> 7) | ((b & 0x80) >> 6);
                    image[width*(sz*inc*y + row) + sz*x + col] = pal[color];
                }
            }
        }
    }
    bitmap_->Update();
}

void NesChrView::Export(const std::string& filename) {
    if (!bitmap_->Save(filename)) {
        ErrorDialog::Spawn("Error Saving File",
                "There was an error saving ", filename);
    }
}

void NesChrView::Import(const std::string& filename) {
    if (!bitmap_->Load(filename)) {
        ErrorDialog::Spawn("Error Loading File",
                "There was an error loading ", filename);
        return;
    }

    uint32_t *image = bitmap_->data();
    int tile = 0;
    int inc = mode_ + 1;
    int width = grid_ ? 160 : 128;
    int sz = grid_ ? 10 : 8;

    if (grid_) {
        image += width*inc + 1;
    }
    for(int y=0; y<16/inc; y++) {
        for(int x=0; x<16; x++, tile+=inc) {
            // Each tile is 8x8 or 8x16
            for(int row=0; row<8*inc; row++) {
                uint8_t a=0, b=0, bit=0x80;
                for(int col=0; col<8; col++, bit>>=1) {
                    uint32_t pixel = image[width*(sz*inc*y + row) + sz*x + col];
                    uint8_t blue = pixel >> 16, green = pixel >> 8, red = pixel;
                    int val = (red + green + blue) / (3 * 0x40);
                    a |= (val & 1) ? bit : 0;
                    b |= (val & 2) ? bit : 0;
                }
                mapper_->WriteChrBank(
                        bank_, 16*(tile + !!(row&8)) + (row & 7), a);
                mapper_->WriteChrBank(
                        bank_, 16*(tile + !!(row&8)) + (row & 7) + 8, b);
            }
        }
    }
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
    ImGui::PushItemWidth(200);
    ImGui::Combo("Bank", &bank_, lptrs, nr_labels_);
    ImGui::PopItemWidth();
}

bool NesChrView::Draw() {
    if (!visible_)
        return false;


    int sz = grid_ ? 10 : 8;
    ImGui::Begin("CHR Viewer", &visible_);
    MakeLabels();

    ImGui::SameLine();
    ImGui::PushItemWidth(100);
    ImGui::Combo("Mode", &mode_, "8x8\0008x16\000\0");
    ImGui::PopItemWidth();

    ImGui::SameLine();
    if (ImGui::Checkbox("Grid", &grid_)) {
        sz = grid_ ? 10 : 8;
        bitmap_.reset(new GLBitmap(16*sz, 16*sz));
    }

#ifdef HAVE_NFD
    ImGui::SameLine();
    if (ImGui::Button("Export")) {
        char *filename = nullptr;
        auto result = NFD_SaveDialog("bmp", nullptr, &filename);
        if (result == NFD_OKAY) {
            Export(filename);
        }
        free(filename);
    }
    ImGui::SameLine();
    if (ImGui::Button("Import")) {
        char *filename = nullptr;
        auto result = NFD_OpenDialog("bmp", nullptr, &filename);
        if (result == NFD_OKAY) {
            Import(filename);
        }
        free(filename);
    }
#endif

    ImApp::Get()->HelpButton("chr-viewer", true);

    // Render the labels on the vertical axis
    ImGui::BeginGroup();
    ImGui::Text(" ");
    float y = ImGui::GetCursorPosY();
    for(int i=0; i<16; i += mode_+1) {
        ImGui::SetCursorPosY(y + i*4*sz);
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
        ImGui::SameLine(x + i*4*sz + sz);
        ImGui::Text("%02x", i * (mode_+1));
    }

    // Render the CHR image
    RenderChr();
    ImGui::SetCursorPosY(y);
    bitmap_->Draw(sz*4*16, sz*4*16);
    ImGui::EndGroup();

    ImGui::End();
    return false;
}
