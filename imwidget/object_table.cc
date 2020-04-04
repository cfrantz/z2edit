#include "imwidget/object_table.h"
#include "imwidget/imutil.h"
#include "util/config.h"

#include <gflags/gflags.h>
DECLARE_bool(hackjam2020);

namespace z2util {

ObjectTable::ObjectTable()
  : ImWindowBase(false),
    group_(0),
    item_(0),
    palette_(0),
    scale_(2.0),
    selection_(0),
    size_(FLAGS_hackjam2020 ? 64 : 16)
{}

void ObjectTable::Init() {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    objtable_.clear();
    names_.clear();
    for(const auto& m : ri.map()) {
        if (m.type() == MapType::OVERWORLD) {
            objtable_.push_back(&m);
            names_.push_back(m.name().c_str());
        }
    }
    // Pseudo-maps for the rest of the known sideview areas.
    for(const auto& m : ri.objtable()) {
        objtable_.push_back(&m);
        names_.push_back(m.name().c_str());
    }

    cache_.set_mapper(mapper_);
    cache_.Init(*objtable_[selection_]);
}

bool ObjectTable::Draw() {
    if (!visible_)
        return false;

    auto set_palette = [&](int p) {
        Address pal;
        if (objtable_[selection_]->palettes_size()) {
            pal = objtable_[selection_]->palettes(p);
        } else {
            pal = objtable_[selection_]->palette();
            pal.set_address(pal.address() + 16 * p);
        }
        cache_.set_palette(pal);
        cache_.Clear();
    };

    ImGui::Begin("Object Table", &visible_);
    ImGui::PushItemWidth(200);
    if (ImGui::Combo("Table", &selection_, names_.data(), names_.size())) {
        cache_.Init(*objtable_[selection_]);
        set_palette(palette_);
        item_ = 0;
        if (objtable_[selection_]->type() == MapType::OVERWORLD) {
            size_ = FLAGS_hackjam2020 ? 64 : 16;
        } else {
            size_ = 256;
        }
    }
    ImGui::PopItemWidth();
    ImGui::SameLine();
    ImGui::PushItemWidth(150);
    ImGui::InputFloat("Scale", &scale_, 0.25, 1.0);
    Clamp(&scale_, 0.25f, 4.0f);
    ImGui::InputInt("Group", &group_);
    Clamp(&group_, 0, 3);
    if (objtable_[selection_]->type() == MapType::OVERWORLD) {
        group_ = 0;
    }
    if (ImGui::InputInt("Palette", &palette_)) {
        Clamp(&palette_, 0, 7);
        if (objtable_[selection_]->type() == MapType::OVERWORLD) {
            palette_ = 0;
        }
        set_palette(palette_);
    }
    ImGui::PopItemWidth();

    ImGui::Separator();
    ImGui::Columns(2, "objects", true);
    ImGui::Text("Objects");

    for(int item=group_*64,y=0; y<8 && item<size_; y++) {
        ImGui::Text("%02x:", item);
        for(int x=0; x<8; x++, item++) {
            ImGui::SameLine();
            auto& bitmap = cache_.Get(item);
            ImTextureID id = reinterpret_cast<ImTextureID>(bitmap.texture_id());
            ImVec2 size(bitmap.width()*scale_, bitmap.height()*scale_);
            ImGui::PushID(item);
            if (ImGui::ImageButton(id, size)) {
                item_ = item;
            }
            ImGui::PopID();
        }
    }

    ImGui::NextColumn();
    ImGui::Text("Caution: Tile-ID edits change the ROM immediately.\n\n");
    ImGui::Text("Tile IDs for item %02X.  Tiles from CHR bank %02X.",
                item_, objtable_[selection_]->chr().bank());

    Address ptr, base;
    if (objtable_[selection_]->type() == MapType::OVERWORLD) {
        ptr.Clear();
        base = objtable_[selection_]->objtable(group_);
    } else {
        ptr = objtable_[selection_]->objtable(group_);
        base = mapper_->ReadAddr(objtable_[selection_]->objtable(group_), 0);
    }
    ImGui::Text("PRG = %02X Ptr = %04X Base = %04X\n\n",
                ptr.bank(), ptr.address(), base.address());

    bool changed = false;
    uint8_t tiles[4];
    char buf[4][4];
    int offset = (item_ & 0x3f) * 4;
    for(int i=0; i<4; i++) {
        tiles[i] = mapper_->Read(base, offset + i);
        sprintf(buf[i], "%02X", tiles[i]);
    }

    ImGui::PushItemWidth(32);
    ImGui::Text("        "); ImGui::SameLine();
    changed |= ImGui::InputText("##tl", buf[0], 4,
            ImGuiInputTextFlags_CharsHexadecimal | ImGuiInputTextFlags_EnterReturnsTrue);
    ImGui::SameLine();
    changed |= ImGui::InputText("##tr", buf[2], 4,
            ImGuiInputTextFlags_CharsHexadecimal | ImGuiInputTextFlags_EnterReturnsTrue);

    ImGui::Text("        "); ImGui::SameLine();
    changed |= ImGui::InputText("##bl", buf[1], 4,
            ImGuiInputTextFlags_CharsHexadecimal | ImGuiInputTextFlags_EnterReturnsTrue);
    ImGui::SameLine();
    changed |= ImGui::InputText("##br", buf[3], 4,
            ImGuiInputTextFlags_CharsHexadecimal | ImGuiInputTextFlags_EnterReturnsTrue);
    ImGui::PopItemWidth();

    ImGui::PushItemWidth(80);
    if (objtable_[selection_]->type() == MapType::OVERWORLD) {
        ImGui::Text("\n");
        ImGui::Text("Tile Palette: ");
        ImGui::SameLine();
        int offset = size_ * 4;
        int tilepal = mapper_->Read(base, offset+item_);
        if (ImGui::InputInt("##tp", &tilepal)) {
            Clamp(&tilepal, 0, 3);
            mapper_->Write(base, offset+item_, tilepal);
            cache_.Clear();
        }
    }
    ImGui::PopItemWidth();

    if (changed) {
        for(int i=0; i<4; i++) {
            uint8_t id = strtoul(buf[i], 0, 16);
            mapper_->Write(base, offset + i, id);
        }
        cache_.Clear();
    }
    ImGui::End();
    return false;
}

}  // namespace z2util
