#include "imwidget/simplemap.h"
#include "imwidget/imutil.h"
#include "util/config.h"
#include "imapp-util.h"

namespace z2util {

SimpleMap::SimpleMap()
  : visible_(false),
    scale_(1.0),
    mapsel_(0),
    tab_(0),
    title_("Sideview Editor") {}

SimpleMap::SimpleMap(Mapper* m, const Map& map)
  : SimpleMap() {
    set_mapper(m);
    SetMap(map);
    mapsel_ = -1;
    title_ = map.name();
}

SimpleMap* SimpleMap::New(Mapper* m, const Map& map) {
    SimpleMap *sm = new SimpleMap(m, map);
    sm->visible_ = true;
    AddDrawCallback([sm]() {
        bool vis = sm->visible_;
        if (vis) {
            sm->Draw();
        } else {
            delete sm;
        }
        return vis;
    });
    return sm;
}

void SimpleMap::DrawMap(const ImVec2& pos) {
    float size = 16.0 * scale_;
    for(int y=0; y<decomp_.height(); y++) {
        for(int x=0; x<decomp_.width(); x++) {
            cache_.Get(decomp_.map(x, y)).DrawAt(
                    pos.x + x*size, pos.y + y*size, scale_);

            uint8_t item = decomp_.item(x, y);
            if (item != 0xFF) {
                items_.Get(item).DrawAt(
                    pos.x + x*size, pos.y + y*size, scale_);
            }
        }
    }
    for(const auto& e : enemies_.data()) {
        enemy_.Get(e.enemy).DrawAt(pos.x + e.x*size, pos.y + e.y*size, scale_);
    }
}

void SimpleMap::RenderToBuffer(GLBitmap *buffer) {
    int size = 16;
    for(int y=0; y<decomp_.height(); y++) {
        for(int x=0; x<decomp_.width(); x++) {
            buffer->Blit(x*size, y*size, size, size,
                         cache_.Get(decomp_.map(x, y)).data());

            uint8_t item = decomp_.item(x, y);
            if (item != 0xFF) {
                auto& sprite = items_.Get(item);
                buffer->Blit(x*size, y*size, sprite.width(), sprite.height(),
                             sprite.data());
            }
        }
    }
    for(const auto& e : enemies_.data()) {
        auto& sprite = enemy_.Get(e.enemy);
        buffer->Blit(e.x*size, e.y*size, sprite.width(), sprite.height(),
                     sprite.data());
    }
}

std::unique_ptr<GLBitmap> SimpleMap::RenderToNewBuffer() {
    std::unique_ptr<GLBitmap> buffer(new GLBitmap(decomp_.width()*16,
                                                  decomp_.height()*16));
    RenderToBuffer(buffer.get());
    return buffer;
}


void SimpleMap::Draw() {
    if (!visible_)
        return;

    ImGui::Begin(title_.c_str(), visible());
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    const char *names[ri.map().size()];
    int len=0, start=0;;
    for(const auto& m : ri.map()) {
        start++;
        if (m.type() == z2util::MapType::OVERWORLD)
            continue;
        names[len] = m.name().c_str();
        if (mapsel_ == -1 && title_ == m.name()) {
            mapsel_ = len;
        }
        len++;
    }
    start -= len;

    ImGui::PushItemWidth(400);
    if (ImGui::Combo("Map", &mapsel_, names, len)) {
        SetMap(ri.map(start + mapsel_));
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
                cache_.Clear();
                cache_.set_palette(decomp_.palette());
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
    cache_.Init(map);

    items_.set_mapper(mapper_);
    Address ipal;
    // FIXME(cfrantz): hardcoded palette location
    ipal.set_bank(1); ipal.set_address(0x9e);
    items_.Init(ri.items());
    items_.set_palette(ipal);

    enemy_.set_mapper(mapper_);
    enemy_.Init(decomp_.EnemyInfo());
    enemy_.set_palette(ipal);

    holder_.set_mapper(mapper_);
    enemies_.set_mapper(mapper_);
    if (map_.type() != MapType::OVERWORLD) {
        cache_.set_palette(decomp_.palette());
        holder_.Parse(map);
        holder_.set_cursor_moves_left(decomp_.cursor_moves_left());
        connection_.Parse(map);
        enemies_.Parse(map);
    }
}

}  // namespace z2util
