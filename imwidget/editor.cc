#include <string>
#include <gflags/gflags.h>

#include "alg/terrain.h"
#include "imwidget/imapp.h"
#include "imwidget/editor.h"
#include "nes/z2decompress.h"
#include "util/config.h"
#include "util/imgui_impl_sdl.h"
#include "util/stb_tilemap_editor.h"
#include "imwidget/imutil.h"

DEFINE_int32(max_map_length, 0, "Maximum compressed map size");
DEFINE_bool(compress_boulders, false, "RLE compress boulders and spiders on "
                                      "overworld maps");
DECLARE_bool(reminder_dialogs);

#define STBTE_MAX_TILEMAP_X      64
#define STBTE_MAX_TILEMAP_Y      200
#define STBTE_MAX_PROPERTIES     0


#define STBTE_DRAW_RECT(x0, y0, x1, y1, c) \
    z2util::Editor::Get()->DrawRect(x0, y0, x1, y1, c)

#define STBTE_DRAW_TILE(x, y, id, highlight, props) \
    z2util::Editor::Get()->DrawTile(x, y, id, highlight, props)

#define STBTE_PROP_TYPE(n, tiledata, params) \
    z2util::Editor::Get()->PropertyType(n, tiledata, params)

#define STBTE_PROP_NAME(n, tiledata, params) \
    z2util::Editor::Get()->PropertyName(n, tiledata, params)

#define STBTE_PROP_MIN(n, tiledata, params) \
    z2util::Editor::Get()->PropertyRange(n, tiledata, params, 0)

#define STBTE_PROP_MAX(n, tiledata, params) \
    z2util::Editor::Get()->PropertyRange(n, tiledata, params, 1)

#define STBTE_PROP_SCALE(n, tiledata, params) \
    z2util::Editor::Get()->PropertyRange(n, tiledata, params, 2)

#define STB_TILEMAP_EDITOR_IMPLEMENTATION
#include "util/stb_tilemap_editor.h"

namespace z2util {

static Editor* current;

Editor::Editor()
  : ImWindowBase(false),
    changed_(false),
    undo_len_(0),
    show_connections_(true),
    scale_(2.0),
    editor_(nullptr),
    map_(nullptr),
    mouse_origin_(0, 0),
    mouse_focus_(false),
    mapsel_(0)
{}

Editor* Editor::Get() {
    return current;
}

Editor* Editor::New() {
    current = new Editor();
    current->ConvertFromMap(nullptr);
    return current;
}

void Editor::ConvertFromMap(Map* map) {
    int width = 64;
    int height = 75;
    Z2Decompress decomp;

    decomp.set_mapper(mapper_);
    cache_.set_mapper(mapper_);
    encounters_.set_mapper(mapper_);

    if (map) {
        map_ = map;
        decomp.Decompress(*map);
        compressed_length_ = decomp.length();
        *map->mutable_address() = decomp.address();
        cache_.Init(*map);
        connections_.Init(mapper_, map->connector(), map->overworld(),
                          map->subworld());
        width = decomp.width();
        height = decomp.height();
        LOG(INFO, "map ", map_->name(),
                  " decompressed to ", width, "x", height);
        encounters_.set_map(*map);
        encounters_.Unpack();
    }
    editor_ = stbte_create_map(width, height, 1, 16, 16, 255);
    editor_->scroll_x = -80;
    editor_->scroll_y = -16;
    stbte_set_background_tile(editor_, 1);

    if (map) {
        for(int y=0; y<height; y++) {
            for(int x=0; x<width; x++) {
                editor_->data[y][x][0] = decomp.map(x, y);
            }
        }
    }

    for(int i=0; i<16; i++) {
        stbte_define_tile(editor_, i, 255, "basic");
    }
    changed_ = false;
    undo_len_ = editor_->undo_len;
}

std::vector<uint8_t> Editor::CompressMap() {
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    std::vector<uint8_t> data;

    for(int y=0; y<editor_->max_y; y++) {
        for(int x=0; x<editor_->max_x; x++) {
            uint8_t tile = editor_->data[y][x][0];
            uint8_t count = 0;
            // For palaces, write the ram address to the "turn to stone" offsets
            // in their respective rom banks.
            int conn = connections_.GetID(x, y);
            if (conn >= misc.palace_connection_id() &&
                conn < misc.palace_connection_id() + 4) {
                uint16_t addr = misc.overworld_ram() + data.size();
                int n = conn - misc.palace_connection_id();
                int w = map_->subworld() ? map_->subworld() : map_->overworld();
                if (w != 1 || (w == 1 && n == 0)) {
                    mapper_->WriteWord(misc.overworld_palace_ram(w), n*2, addr);
                } else {
                    LOGF(ERROR, "Palace at (%d, %d) connection ID %d not expected (%d).",
                            x, y, conn, misc.palace_connection_id());
                }
            } else {
                // Only log an error if its a Palace tile and the connection ID
                // exists (and is not zero, for north palace).
                if (tile == 0x02 && conn > 0) {
                    LOGF(ERROR, "Palace at (%d, %d) connection ID %d not in expected range %d-%d.",
                        x, y, conn, misc.palace_connection_id(),
                        misc.palace_connection_id() + 3);
                }
            }
            // Don't compress magic connection spots, boulders or the
            // spider/river devil.
            if (connections_.NoCompress(x, y) ||
                (!FLAGS_compress_boulders && (tile == 0x0E || tile == 0x0F))) {
                data.push_back(tile | count << 4);
                continue;
            }
            while(x+1 < editor_->max_x
                  && tile == editor_->data[y][x+1][0]
                  && !connections_.NoCompress(x+1, y)) {
                x++;
                count++;
                if (count == 15)
                    break;
            }
            data.push_back(tile | count << 4);
        }
    }
    return data;
}

void Editor::SaveMap() {
    std::vector<uint8_t> data = CompressMap();
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    int max_length = misc.overworld_length();
    if (FLAGS_max_map_length) max_length = FLAGS_max_map_length;

    if (!map_) {
        LOG(ERROR, "Can't save map: map_ == nullptr");
        return;
    }

    LOG(INFO, "Saving ", map_->name());
    LOG(INFO, "Map compresses to ", data.size(), " bytes.");

    if (data.size() > size_t(max_length)) {
        ErrorDialog::Spawn("Overworld Save Error",
            "Can't save ", map_->name(), " because it is ", data.size(),
            " bytes\n\n"
            "Overworld maps must be smaller than ", max_length, " bytes.\n");
        return;
    }

    compressed_length_ = int(data.size());
    Address addr = map_->address();
    addr.set_address(0);
    addr = mapper_->Alloc(addr, data.size());
    if (addr.address() == 0) {
        ErrorDialog::Spawn("Overworld Save Error",
            "Can't allocate free space for ", map_->name(), ".\n");
        LOG(ERROR, "Can't save map: can't find ", data.size(), " bytes"
                   " in bank=", addr.bank());
        return;
    }

    // Always mapped at 0x8000
    addr.set_address(0x8000 | addr.address());

    LOG(INFO, "Saving map to offset ", HEX(addr.address()),
              " in bank=", addr.bank());
    for(unsigned i=0; i<data.size(); i++) {
        mapper_->Write(addr, i, data[i]);
    }
    mapper_->WriteWord(map_->pointer(), 0, addr.address());
    mapper_->Free(map_->address());

    *(map_->mutable_address()) = addr;
    map_->set_length(data.size());
    encounters_.Save();
    connections_.Save();
    changed_ = false;
    undo_len_ = editor_->undo_len;
}

void Editor::Resize(int x0, int y0, int x1, int y1) {
    stbte_set_display(x0, y0, x1, y1);
}

void Editor::DrawRect(int x0, int y0, int x1, int y1, uint32_t color) {
    if (x0 < 0 || y0 < 0)
        return;
    if (x1 > size_.x || y1 > size_.y)
        return;

    float scale = scale_;
    auto* draw = ImGui::GetWindowDrawList();
    color |= 0xFF000000;
    draw->AddRectFilled(mouse_origin_ + ImVec2(x0, y0) * scale,
                        mouse_origin_ + ImVec2(x1, y1) * scale,
                        color);
}

void Editor::DrawTile(int x, int y, uint16_t tile, int mode, float* props) {
    if (x < 0 || y < 0)
        return;
    if (x + 16 > size_.x || y + 16 > size_.y)
        return;
    ImGui::SetCursorPos(origin_ + ImVec2(x, y) * scale_);
    cache_.Get(tile).Draw(16 * scale_, 16 * scale_);
    ImGui::SetCursorPos(origin_);
}

void Editor::Refresh() {
    auto* rominfo = ConfigLoader<RomInfo>::MutableConfig();
    ConvertFromMap(rominfo->mutable_map(mapsel_));
}

bool Editor::Draw() {
    const char *names[256];
    int len=0, mapsel = mapsel_;
    if (!visible_)
        return changed_;

    ImGui::SetNextWindowSize(ImVec2(1100, 700), ImGuiSetCond_FirstUseEver);
    ImGui::Begin("Map Editor", &visible_);

    auto* rominfo = ConfigLoader<RomInfo>::MutableConfig();
    for(const auto& m : rominfo->map()) {
        if (m.type() != MapType::OVERWORLD)
            continue;
        names[len++] = m.name().c_str();
    }
    ImGui::PushItemWidth(400);
    if (ImGui::Combo("Map", &mapsel_, names, len)) {
        if (FLAGS_reminder_dialogs && (changed_ || undo_len_ != editor_->undo_len)) {
            ErrorDialog::Spawn("Discard Changes",
                ErrorDialog::OK | ErrorDialog::CANCEL,
                "Discard Changes to map?")->set_result_cb([this, mapsel, rominfo](int result) {
                    if (result == ErrorDialog::OK) {
                        ConvertFromMap(rominfo->mutable_map(mapsel_));
                    } else {
                        mapsel_ = mapsel;
                    }
            });
        } else {
            ConvertFromMap(rominfo->mutable_map(mapsel_));
        }
    }
    ImGui::PopItemWidth();

    ImGui::SameLine();
    ImGui::PushItemWidth(100);
    ImGui::Checkbox("Show Connections", &show_connections_);

    ImGui::SameLine();
    if (ImGui::Button("Connections")) {
        ImGui::OpenPopup("Connections");
    }
    changed_ |= connections_.Draw();

    ImGui::SameLine();
    if (ImGui::Button("Encounters")) {
        ImGui::OpenPopup("Encounters");
    }
    changed_ |= encounters_.Draw();

    ImGui::SameLine();
    if (ImGui::Button("Randomize")) {
        ImGui::OpenPopup("Randomize");
    }
    changed_ |= randomize_.Draw(editor_, &connections_);

    ImGui::SameLine();
    if (ImGui::Button("Commit to ROM")) {
        SaveMap();
        ImApp::Get()->ProcessMessage(
                "commit", StrCat("Overworld edits to ", map_->name()).c_str());
    }

    ImGui::SameLine();
    ImGui::InputFloat("Zoom", &scale_, 0.25, 1.0);
    scale_ = Clamp(scale_, 0.25f, 8.0f);
    connections_.set_scale(16.0 * scale_);
    ImGui::PopItemWidth();

    ImGui::SameLine();
    ImApp::Get()->HelpButton("overworld-editor");

    ImGui::Text("Compressed map in bank %d address=%04x length=%d bytes.",
                map_->address().bank(), map_->address().address(),
                compressed_length_);

    ImGui::BeginChild("editarea", ImGui::GetContentRegionAvail());
    origin_ = ImGui::GetCursorPos();
    mouse_origin_ = ImGui::GetCursorScreenPos();
    size_ = ImGui::GetContentRegionAvail();
    ImVec2 mp = ImGui::GetMousePos();
    mouse_focus_ = (ImGui::IsWindowHovered() &&
                    mp.x >= mouse_origin_.x &&
                    mp.y >= mouse_origin_.y &&
                    mp.x < mouse_origin_.x + size_.x &&
                    mp.y < mouse_origin_.y + size_.y);

    size_ /= scale_;
    stbte_set_display(0, 0, size_.x, size_.y);
    stbte_draw(editor_);
    stbte_tick(editor_, 1.0/60.0);

    if (show_connections_) {
        int max_x = int(size_.x) & ~15;
        int max_y = int(size_.y) & ~15;
        for(int y=0; y<max_y; y+=16) {
            for(int x=0; x<max_x; x+=16) {
                ImGui::SetCursorPos(origin_ + ImVec2(x, y) * scale_);
                if (connections_.DrawInEditor((x + editor_->scroll_x)/16,
                                              (y + editor_->scroll_y)/16)) {
                    mouse_focus_ = false;
                    changed_ |= connections_.changed();
                }
            }
        }
    }

    for(auto& e : events_) {
        HandleEvent(&e);
    }
    events_.clear();

    ImGui::EndChild();
    ImGui::End();
    return changed_;
}

void Editor::ProcessEvent(SDL_Event* e) {
    if (!mouse_focus_)
        return;
    events_.push_back(*e);
}

void Editor::HandleEvent(SDL_Event* e) {
    const auto& keybinds = ConfigLoader<RomInfo>::GetConfig().overworld_editor_keybind();
    float hidpi = ImGui_ImplSdl_GetHiDPIScale();
    switch (e->type) {
        case SDL_MOUSEMOTION:
            e->motion.x /= hidpi; e->motion.y /= hidpi;
            e->motion.x -= mouse_origin_.x; e->motion.y -= mouse_origin_.y;
            e->motion.x /= scale_; e->motion.y /= scale_;
            stbte_mouse_sdl(editor_, e, 1.0f,1.0f,0,0);
            break;
        case SDL_MOUSEBUTTONDOWN:
        case SDL_MOUSEBUTTONUP:
            e->button.x /= hidpi; e->button.y /= hidpi;
            e->button.x -= mouse_origin_.x; e->button.y -= mouse_origin_.y;
            e->button.x /= scale_; e->button.y /= scale_;
            stbte_mouse_sdl(editor_, e, 1.0f,1.0f,0,0);
            break;
        case SDL_MOUSEWHEEL:
            e->wheel.x /= hidpi; e->wheel.y /= hidpi;
            e->wheel.x -= mouse_origin_.x; e->wheel.y -= mouse_origin_.y;
            e->wheel.x /= scale_; e->wheel.y /= scale_;
            stbte_mouse_sdl(editor_, e, 1.0f,1.0f,0,0);
            break;

        case SDL_KEYDOWN:
            for(const auto& kb : keybinds) {
                int mod = int(kb.mod());
                int scancode = int(kb.scancode());
                if (mod == 0 || (
                    (e->key.keysym.mod & mod) && !(e->key.keysym.mod & ~mod))) {
                    if (e->key.keysym.scancode == scancode) {
                        stbte_action(editor_,
                                static_cast<enum stbte_action>(kb.action()));
                        break;
                    }
                }
            }

            break;
        default:
            ; // nothing
    }
}

Editor::PropertyInfo Editor::property_info_[1];

int Editor::PropertyType(int n, int16_t* tiledata, float* params) {
    return property_info_[n].type;
}

char* Editor::PropertyName(int n, int16_t* tiledata, float* params) {
    return property_info_[n].name;
}

float Editor::PropertyRange(int n, int16_t* tiledata, float* params, int what) {
    switch(what) {
        case 0: return property_info_[n].min;
        case 1: return property_info_[n].max;
        case 2: return property_info_[n].scale;
        default: ;
    }
    return 0;
}

}  // namespace
