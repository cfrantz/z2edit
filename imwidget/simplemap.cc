#include "imwidget/simplemap.h"
#include "imwidget/imutil.h"
#include "util/config.h"

namespace z2util {

SimpleMap::SimpleMap()
  : visible_(false),
    scale_(1.0),
    mapsel_(0),
    tab_(0) {}

void SimpleMap::DrawMap(const ImVec2& pos) {
    float size = 16.0 * scale_;
    for(int y=0; y<decomp_.height(); y++) {
        for(int x=0; x<decomp_.width(); x++) {
            cache_.Get(decomp_.map(x, y)).DrawAt(
                    pos.x + x*size, pos.y + y*size, size, size);

            uint8_t item = decomp_.item(x, y);
            if (item != 0xFF) {
                items_.Get(item).DrawAt(
                    pos.x + x*size, pos.y + y*size, size, size);
            }
        }
    }
}

void SimpleMap::Draw() {
    if (!visible_)
        return;

    ImGui::Begin("SimpleMap", visible());
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    const char *names[ri.map().size()];
    int len=0;
    for(const auto& m : ri.map()) {
        names[len++] = m.name().c_str();
    }

    ImGui::PushItemWidth(400);
    if (ImGui::Combo("Map", &mapsel_, names, len)) {
        SetMap(ri.map(mapsel_));
    }
    ImGui::PopItemWidth();

    ImGui::PushItemWidth(100);
    ImGui::SameLine();
    if (ImGui::Button("Commit to ROM")) {
        holder_.Save();
        connection_.Save();
        enemies_.Save();
    }

    ImGui::SameLine();
    ImGui::InputFloat("Zoom", &scale_, 0.25, 1.0);
    scale_ = Clamp(scale_, 0.25f, 8.0f);
    ImGui::PopItemWidth();


    ImGui::BeginChild("image", ImVec2(0, 16 + decomp_.height()*16.0*scale_),
                      true, ImGuiWindowFlags_HorizontalScrollbar);
    ImVec2 cursor = ImGui::GetCursorPos();
    DrawMap(cursor);
    ImGui::EndChild();

    if (map_.type() != z2util::MapType::OVERWORLD) {
        ImGui::RadioButton("Map Commands", &tab_, 0); ImGui::SameLine();
        ImGui::RadioButton("Connections", &tab_, 1); ImGui::SameLine();
        ImGui::RadioButton("Enemy List", &tab_, 2);
        ImGui::Separator();

        if (tab_ == 0) {
            if (holder_.Draw()) {
                decomp_.Clear();
                decomp_.DecompressSideView(holder_.MapDataAbs().data());
            }
        } else if (tab_ == 1) {
            connection_.Draw();
        } else if (tab_ == 2) {
            enemies_.Draw();
        }
    }
    ImGui::End();
}

void SimpleMap::SetMap(const z2util::Map& map) {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    map_ = map;

    decomp_.Init();
    decomp_.Decompress(map);

    connection_.set_mapper(mapper_);
    cache_.set_mapper(mapper_);
    cache_.set_hwpal(hwpal_);
    cache_.Init(map);

    items_.set_mapper(mapper_);
    items_.set_hwpal(hwpal_);
    Address ipal;
    // FIXME(cfrantz): hardcoded palette location
    ipal.set_bank(1); ipal.set_address(0x9e);
    items_.Init(ri.items().sprite_table(), ri.items().chr(),
                Z2ObjectCache::Schema::ITEM);
    items_.set_palette(ipal);

    holder_.set_mapper(mapper_);
    enemies_.set_mapper(mapper_);
    if (map_.type() != MapType::OVERWORLD) {
        cache_.set_palette(decomp_.palette());
        holder_.Parse(map);
        connection_.Parse(map);
        enemies_.Parse(map);
    }
}

}  // namespace z2util
