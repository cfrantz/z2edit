#include "imwidget/simplemap.h"
#include <gflags/gflags.h>

#include "imwidget/imapp.h"
#include "imwidget/imutil.h"
#include "imwidget/error_dialog.h"
#include "util/config.h"
#include "absl/strings/str_cat.h"

DEFINE_bool(render_items_in_known_banks, false,
            "Render items and enemies from known bank locations");
DECLARE_bool(reminder_dialogs);

namespace z2util {

SimpleMap::SimpleMap()
  : ImWindowBase(false),
    changed_(false),
    object_box_(true),
    enemy_box_(true),
    avail_box_(true),
    scale_(1.0),
    mapsel_(0),
    tab_(0),
    startscreen_(0),
    title_("Sideview Editor"),
    window_title_(title_),
    grayout_(256, 208) {
    for(int y=0; y<grayout_.height(); y++) {
        for(int x=0; x<grayout_.width(); x++) {
            grayout_.SetPixel(x, y, 0xc0000000);
        }
    }
    grayout_.Update();
}

SimpleMap::SimpleMap(Mapper* m, const Map& map, int startscreen)
  : SimpleMap() {
    set_mapper(m);
    SetMap(map);
    startscreen_ = startscreen;
    mapsel_ = -1;
    title_ = map.name();
    window_title_ = absl::StrCat(title_, "##", id_);
}

SimpleMap* SimpleMap::Spawn(Mapper* m, const Map& map, int startscreen) {
    SimpleMap *sm = new SimpleMap(m, map, startscreen);
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
                if (item != ELEVATOR && !avail_.get(x) && avail_.show()) {
                    float rx = sprite.width() * scale_ / 2.0;
                    float ry = sprite.height() * scale_ / 2.0;
                    draw->AddCircle(
                            ImVec2(sp.x + x*size + rx, sp.y + y*size + ry),
                            (rx+ry)/1.333, RED, 20, 2.0f);
                    draw->AddLine(
                            ImVec2(sp.x + x*size, sp.y + y*size),
                            ImVec2(sp.x + x*size + sprite.width() * scale_,
                                   sp.y + y*size + sprite.height() * scale_),
                            RED, 2.0f);

                }
            }
        }
    }
    for(const auto& e : enemies_.data()) {
        enemy_.Get(e.enemy).DrawAt(pos.x + e.x*size, pos.y + e.y*size, scale_);
    }
    int start = startscreen_ * 16;
    int end = start + decomp_.mapwidth();;
    for(int x=0; x<decomp_.width(); x+=16) {
        if (!(x >= start && x < end)) {
            grayout_.DrawAt(pos.x + x*size, pos.y + 0, scale_);
        }
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
    int start = startscreen_ * 16;
    int end = start + decomp_.mapwidth();;
    for(int x=0; x<decomp_.width(); x+=16) {
        if (!(x >= start && x < end)) {
            buffer->Blit(x*size, 0, grayout_.width(), grayout_.height(),
                         grayout_.data());
        }
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

    ImGui::SetNextWindowSize(ImVec2(1024, 700), ImGuiCond_FirstUseEver);
    ImGui::Begin(window_title_.c_str(), &visible_);
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    const char *names[ri.map().size()];
    int len=0, start=0;
    int mapsel = mapsel_;
    bool want_redraw = false;

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

    ImGui::PushItemWidth(80);
    ImGui::SameLine();
    if (ImGui::Button("Commit to ROM")) {
        holder_.Save([this]() {
            connection_.Save();
            enemies_.Save();
            avail_.Save();
            changed_ = false;
            ImApp::Get()->ProcessMessage(
                    "commit", absl::StrCat("Map edits to ", map_.name()).c_str());
        });
    }

    ImGui::SameLine();
    ImGui::InputInt("Screen", &startscreen_);
    startscreen_ = Clamp(startscreen_, 0, 3);
    ImGui::SameLine();
    ImGui::InputFloat("Zoom", &scale_, 0.25, 1.0);
    scale_ = Clamp(scale_, 0.25f, 8.0f);
    ImGui::PopItemWidth();

    ImGui::SameLine(); ImGui::Text("| Box:");
    ImGui::SameLine(); ImGui::Checkbox("Object", &object_box_);
    ImGui::SameLine(); ImGui::Checkbox("Enemy", &enemy_box_);
    ImGui::SameLine(); ImGui::Checkbox("Avail", &avail_box_);
    holder_.set_show_origin(object_box_);
    enemies_.set_show_origin(enemy_box_);
    avail_.set_show(avail_box_);


    ImGui::SameLine();
    ImApp::Get()->HelpButton("sideview-editor", true);

    ImGui::BeginChild("image", ImVec2(0, (decomp_.height()+2)*16.0*scale_),
                      true, ImGuiWindowFlags_HorizontalScrollbar);
    ImVec2 cursor = ImGui::GetCursorPos();
    DrawMap(cursor);
    ImGui::SetCursorPos(cursor);
    if (holder_.DrawPopup(scale_)) {
        LOG(INFO, "map changed");
        changed_ = true;
        want_redraw = true;
    }
    changed_ |= enemies_.DrawPopup(scale_);
    ImGui::EndChild();

    if (map_.type() != z2util::MapType::OVERWORLD) {
        ImGui::RadioButton("Map Commands", &tab_, 0); ImGui::SameLine();
        ImGui::RadioButton("Connections", &tab_, 1); ImGui::SameLine();
        ImGui::RadioButton("Enemy List", &tab_, 2); ImGui::SameLine();
        ImGui::RadioButton("Item Availability", &tab_, 3); ImGui::SameLine();
        ImGui::RadioButton("Swap/Copy", &tab_, 4);
        ImGui::Separator();

        MapHolder::DrawResult draw_result = MapHolder::DR_NONE;
        if (tab_ == 0) {
            draw_result = holder_.Draw();
            if (draw_result != MapHolder::DR_NONE) {
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
            if (draw_result == MapHolder::DR_PALETTE_CHANGED) {
                cache_.Clear();
                cache_.set_palette(decomp_.palette());
            }
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
    startscreen_ = 0;

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

    if (FLAGS_render_items_in_known_banks) {
        // Use the CHR banks set in the configuration.
        items_.set_use_iteminfo_chr(true);
        enemy_.set_use_iteminfo_chr(true);
    } else {
        // Use the CHR banks associated with this map.
        Address chr;
        chr.set_bank(cache_.chr().bank() & ~1);
        items_.set_chr(chr);
        enemy_.set_chr(chr);
    }

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
