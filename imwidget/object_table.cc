#include "imwidget/object_table.h"

namespace z2util {

ObjectTable::ObjectTable()
  : visible_(false)
{}

void ObjectTable::Init() {
    cache_.set_mapper(mapper_);
    cache_.set_hwpal(hwpal_);


    table_.set_bank(7); table_.set_address(0x2e51);
    chr_.set_bank(1);

    cache_.Init(table_, chr_, Z2ObjectCache::Schema::ITEM);

    Address ipal;
    ipal.set_bank(1); ipal.set_address(0x9e);
    cache_.set_palette(ipal);
}

void ObjectTable::Draw() {
    if (!visible_)
        return;

    int chr = chr_.bank();
    int tableb = table_.bank();
    int tablea = table_.address();
    bool init_cache = false;
    char abuf[8];
    sprintf(abuf, "%04x", tablea);

    ImGui::Begin("Object Table", visible());
    ImGui::PushItemWidth(150);
    init_cache |= ImGui::InputInt("CHR", &chr);
    init_cache |= ImGui::InputInt("Bank", &tableb);
    ImGui::SameLine();
//    init_cache |= ImGui::InputInt("Address", &tablea);
    init_cache |= ImGui::InputText("Address", abuf, sizeof(abuf),
                                ImGuiInputTextFlags_CharsHexadecimal |
                                ImGuiInputTextFlags_EnterReturnsTrue);

    if (init_cache) {
        chr_.set_bank(chr);
        table_.set_bank(tableb);
        tablea = strtol(abuf, 0, 16);
        table_.set_address(tablea);
        cache_.Init(table_, chr_, Z2ObjectCache::Schema::ITEM);
    }

    ImVec2 pos = ImGui::GetCursorPos();
    for(int y=0; y<8; y++) {
        for(int x=0; x<8; x++) {
            int item = y*8 + x;
            cache_.Get(item).DrawAt(pos.x + x*24, pos.y + y*24);
        }
    }
    ImGui::PopItemWidth();
    ImGui::End();
}

}  // namespace z2util
