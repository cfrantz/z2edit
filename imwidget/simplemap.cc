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

    ImGui::SameLine();
    ImGui::PushItemWidth(100);
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
                decomp_.DecompressSideView(holder_.MapData().data());
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
    map_ = map;

    decomp_.Init();
    decomp_.Decompress(map);

    connection_.set_mapper(mapper_);
    cache_.set_mapper(mapper_);
    cache_.set_hwpal(hwpal_);
    cache_.Init(map);
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
