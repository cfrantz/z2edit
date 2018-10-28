#include "imwidget/object_table.h"
#include "imwidget/imutil.h"
#include "util/config.h"

namespace z2util {

ObjectTable::ObjectTable()
  : ImWindowBase(false),
    group_(0),
    item_(0),
    palette_(0),
    scale_(2.0),
    selection_(0)
{}

void ObjectTable::Init() {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    cache_.set_mapper(mapper_);
    cache_.Init(ri.objtable(selection_));
}

bool ObjectTable::Draw() {
    if (!visible_)
        return false;

    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    auto set_palette = [&](int p) {
        Address pal;
        if (ri.objtable(selection_).palettes_size()) {
            pal = ri.objtable(selection_).palettes(p);
        } else {
            pal = ri.objtable(selection_).palette();
            pal.set_address(pal.address() + 16 * p);
        }
        cache_.set_palette(pal);
        cache_.Clear();
    };
    const char *names[ri.objtable().size()];
    int len = 0;

    for(const auto& o : ri.objtable()) {
        names[len++] = o.name().c_str();
    }

    ImGui::Begin("Object Table", &visible_);
    ImGui::PushItemWidth(200);
    if (ImGui::Combo("Table", &selection_, names, len)) {
        cache_.Init(ri.objtable(selection_));
        set_palette(palette_);
        item_ = 0;
    }
    ImGui::PopItemWidth();
    ImGui::SameLine();
    ImGui::PushItemWidth(150);
    ImGui::InputFloat("Scale", &scale_, 0.25, 1.0);
    Clamp(&scale_, 0.25f, 4.0f);
    ImGui::InputInt("Group", &group_);
    Clamp(&group_, 0, 3);
    if (ImGui::InputInt("Palette", &palette_)) {
        Clamp(&palette_, 0, 7);
        set_palette(palette_);
    }
    ImGui::PopItemWidth();

    ImGui::Separator();
    ImGui::Columns(2, "objects", true);
    ImGui::Text("Objects");

    for(int item=group_*64,y=0; y<8; y++) {
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
                item_, ri.objtable(selection_).chr().bank());

    Address ptr = ri.objtable(selection_).objtable(group_);
    Address base = mapper_->ReadAddr(
            ri.objtable(selection_).objtable(group_), 0);
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
