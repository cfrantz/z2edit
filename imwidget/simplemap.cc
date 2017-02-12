#include "imwidget/simplemap.h"
#include <gflags/gflags.h>

#include "imwidget/imapp.h"
#include "imwidget/imutil.h"
#include "imwidget/error_dialog.h"
#include "util/config.h"
#include "util/strutil.h"

DECLARE_bool(reminder_dialogs);

namespace z2util {

SimpleMap::SimpleMap()
  : ImWindowBase(false),
    changed_(false),
    scale_(1.0),
    mapsel_(0),
    tab_(0),
    title_("Sideview Editor"),
    window_title_(title_) {}

SimpleMap::SimpleMap(Mapper* m, const Map& map)
  : SimpleMap() {
    set_mapper(m);
    SetMap(map);
    mapsel_ = -1;
    title_ = map.name();
    window_title_ = StrCat(title_, "##", id_);
}

SimpleMap* SimpleMap::Spawn(Mapper* m, const Map& map) {
    SimpleMap *sm = new SimpleMap(m, map);
    sm->visible_ = true;
    ImApp::Get()->AddDrawCallback(sm);
    return sm;
}

void SimpleMap::DrawMap(const ImVec2& pos) {
    float size = 16.0 * scale_;
    auto* draw = ImGui::GetWindowDrawList();
    auto sp = ImGui::GetCursorScreenPos();
    for(int y=0; y<decomp_.height(); y++) {
        for(int x=0; x<decomp_.width(); x++) {
            cache_.Get(decomp_.map(x, y)).DrawAt(
                    pos.x + x*size, pos.y + y*size, scale_);

        }
    }
    for(int y=0; y<decomp_.height(); y++) {
        for(int x=0; x<decomp_.width(); x++) {
            uint8_t item = decomp_.item(x, y);
            if (item != 0xFF) {
                auto& sprite = items_.Get(item);
                sprite.DrawAt(pos.x + x*size, pos.y + y*size, scale_);
                if (item != ELEVATOR && !avail_.get(x)) {
                    draw->AddRect(
                            ImVec2(sp.x + x*size, sp.y + y*size),
                            ImVec2(sp.x + x*size + sprite.width() * scale_,
                                   sp.y + y*size + sprite.height() * scale_),
                            RED);
                    draw->AddLine(
                            ImVec2(sp.x + x*size, sp.y + y*size),
                            ImVec2(sp.x + x*size + sprite.width() * scale_,
                                   sp.y + y*size + sprite.height() * scale_),
                            RED);

                }
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

        }
    }
    for(int y=0; y<decomp_.height(); y++) {
        for(int x=0; x<decomp_.width(); x++) {
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


bool SimpleMap::Draw() {
    if (!visible_)
        return changed_;

    ImGui::SetNextWindowSize(ImVec2(1024, 700), ImGuiSetCond_FirstUseEver);
    ImGui::Begin(window_title_.c_str(), &visible_);
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    const char *names[ri.map().size()];
    int len=0, start=0;
    int mapsel = mapsel_;

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
        if (FLAGS_reminder_dialogs && changed_) {
            ErrorDialog::Spawn("Discard Chagnes", 
                ErrorDialog::OK | ErrorDialog::CANCEL,
                "Discard Changes to map?")->set_result_cb([this, mapsel, ri, start](int result) {
                    if (result == ErrorDialog::OK) {
                        SetMap(ri.map(start + mapsel_));
                    } else {
                        mapsel_ = mapsel;
                    }
            });
        } else {
            SetMap(ri.map(start + mapsel_));
        }
    }
    ImGui::PopItemWidth();

    ImGui::PushItemWidth(100);
    ImGui::SameLine();
    if (ImGui::Button("Commit to ROM")) {
        holder_.Save();
        connection_.Save();
        enemies_.Save();
        avail_.Save();
        changed_ = false;
    }

    ImGui::SameLine();
    ImGui::InputFloat("Zoom", &scale_, 0.25, 1.0);
    scale_ = Clamp(scale_, 0.25f, 8.0f);
    ImGui::PopItemWidth();

    ImGui::SameLine();
    ImApp::Get()->HelpButton("sideview-editor");

    ImGui::BeginChild("image", ImVec2(0, 16 + decomp_.height()*16.0*scale_),
                      true, ImGuiWindowFlags_HorizontalScrollbar);
    ImVec2 cursor = ImGui::GetCursorPos();
    DrawMap(cursor);
    ImGui::EndChild();

    if (map_.type() != z2util::MapType::OVERWORLD) {
        bool want_redraw = false;
        ImGui::RadioButton("Map Commands", &tab_, 0); ImGui::SameLine();
        ImGui::RadioButton("Connections", &tab_, 1); ImGui::SameLine();
        ImGui::RadioButton("Enemy List", &tab_, 2); ImGui::SameLine();
        ImGui::RadioButton("Item Availability", &tab_, 3); ImGui::SameLine();
        ImGui::RadioButton("Swap/Copy", &tab_, 4);
        ImGui::Separator();

        if (tab_ == 0) {
            if (holder_.Draw()) {
                changed_ = true;
                want_redraw = true;
            }
        } else if (tab_ == 1) {
            changed_ |= connection_.Draw();
        } else if (tab_ == 2) {
            changed_ |= enemies_.Draw();
        } else if (tab_ == 3) {
            changed_ |= avail_.Draw();
        } else if (tab_ == 4) {
            if (swapper_.Draw()) {
                SetMap(map_);
            }
        }
        if (want_redraw) {
            decomp_.Clear();
            decomp_.DecompressSideView(holder_.MapDataAbs().data());
            cache_.Clear();
            cache_.set_palette(decomp_.palette());
        }
    }
    ImGui::End();
    return changed_;
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
    ipal.set_bank(decomp_.palette().bank());
    ipal.set_address(0x809e);
    items_.Init(ri.items());
    items_.set_palette(ipal);

    enemy_.set_mapper(mapper_);
    enemy_.Init(decomp_.EnemyInfo());
    enemy_.set_palette(ipal);

    holder_.set_mapper(mapper_);
    enemies_.set_mapper(mapper_);
    avail_.set_mapper(mapper_);
    swapper_.set_mapper(mapper_);
    if (map_.type() != MapType::OVERWORLD) {
        cache_.set_palette(decomp_.palette());
        holder_.Parse(map);
        holder_.set_cursor_moves_left(decomp_.cursor_moves_left());
        connection_.Parse(map);
        enemies_.Parse(map);
        avail_.Parse(map);
        swapper_.set_map(map);
    }
    changed_ = false;
}

}  // namespace z2util
