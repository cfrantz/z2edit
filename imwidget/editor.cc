#include <string>

#include "imwidget/editor.h"
#include "nes/z2decompress.h"
#include "util/config.h"
#include "util/imgui_impl_sdl.h"
#include "util/stb_tilemap_editor.h"
#include "imwidget/imutil.h"
#include "imwidget/error_dialog.h"

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
  : visible_(false),
    show_connections_(true),
    scale_(2.0),
    editor_(nullptr),
    map_(nullptr),
    mouse_origin_(0, 0),
    mouse_focus_(false),
    mapsel_(0)
{ }

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
    int height = 72;
    Z2Decompress decomp;

    decomp.set_mapper(mapper_);
    cache_.set_mapper(mapper_);

    if (map) {
        map_ = map;
        decomp.Decompress(*map);
        cache_.Init(*map);
        connections_.Init(mapper_, map->connector(), map->world());
        width = decomp.width();
        height = decomp.height();
        LOG(INFO, "map ", map_->name(),
                  " decompressed to ", width, "x", height);
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
}

std::vector<uint8_t> Editor::CompressMap() {
    std::vector<uint8_t> data;

    for(int y=0; y<editor_->max_y; y++) {
        for(int x=0; x<editor_->max_x; x++) {
            uint8_t tile = editor_->data[y][x][0];
            uint8_t count = 0;
            while(x+1 < editor_->max_x && tile == editor_->data[y][x+1][0]) {
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

    if (!map_) {
        LOG(ERROR, "Can't save map: map_ == nullptr");
        return;
    }
    if (data.size() > 1024) {
        ErrorDialog::New("Overworld Save Error",
            "Can't save ", map_->name(), " because it is ", data.size(),
            " bytes\n\n"
            "Overworld maps must be smaller than 1024 bytes.\n");
    }

    LOG(INFO, "Saving ", map_->name());
    LOG(INFO, "Map compresses to ", data.size(), " bytes.");

    Address addr = map_->address();
    addr.set_address(0);
    addr = mapper_->Alloc(addr, data.size());
    if (addr.address() == 0) {
        ErrorDialog::New("Overworld Save Error",
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
    ImGui::SetCursorPos(origin_ + ImVec2(x, y) * scale_);
    if (show_connections_) {
        if (connections_.DrawInEditor((x + editor_->scroll_x)/16,
                                      (y + editor_->scroll_y)/16)) {
            mouse_focus_ = false;
        }
    }
    ImGui::SetCursorPos(origin_);
}

void Editor::Draw() {
    const char *names[256];
    int len=0, mapsel = mapsel_;
    if (!visible_)
        return;

    ImGui::Begin("Map Editor", visible());

    auto* rominfo = ConfigLoader<RomInfo>::MutableConfig();
    for(const auto& m : rominfo->map()) {
        if (m.type() != MapType::OVERWORLD)
            continue;
        names[len++] = m.name().c_str();
    }
    ImGui::PushItemWidth(400);
    if (ImGui::Combo("Map", &mapsel_, names, len)) {
        ConvertFromMap(rominfo->mutable_map(mapsel_));
    }
    ImGui::PopItemWidth();

    ImGui::SameLine();
    ImGui::PushItemWidth(100);
    ImGui::Checkbox("Show Connections", &show_connections_);

    ImGui::SameLine();
    if (ImGui::Button("Connections")) {
        ImGui::OpenPopup("Connections");
    }
    connections_.DrawAdd();

    ImGui::SameLine();
    if (ImGui::Button("Commit to ROM")) {
        SaveMap();
    }

    ImGui::SameLine();
    ImGui::InputFloat("Zoom", &scale_, 0.25, 1.0);
    scale_ = Clamp(scale_, 0.25f, 8.0f);
    ImGui::PopItemWidth();

    mouse_origin_ = ImGui::GetCursorScreenPos();
    origin_ = ImGui::GetCursorPos();
    size_ = ImGui::GetContentRegionAvail();
    ImGui::InvisibleButton("canvas", size_);
    ImGui::SetCursorPos(origin_);
    mouse_focus_ = ImGui::IsItemHovered();

    size_ /= scale_;
    stbte_set_display(0, 0, size_.x, size_.y);
    stbte_draw(editor_);
    stbte_tick(editor_, 1.0/60.0);

    for(auto& e : events_) {
        HandleEvent(&e);
    }
    events_.clear();

    ImGui::End();
}

void Editor::ProcessEvent(SDL_Event* e) {
    if (!mouse_focus_)
        return;
    events_.push_back(*e);
}

void Editor::HandleEvent(SDL_Event* e) {
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
            switch (e->key.keysym.sym) {
                case SDLK_RIGHT: stbte_action(editor_, STBTE_scroll_right); break;
                case SDLK_LEFT : stbte_action(editor_, STBTE_scroll_left ); break;
                case SDLK_UP    : stbte_action(editor_, STBTE_scroll_up    ); break;
                case SDLK_DOWN : stbte_action(editor_, STBTE_scroll_down ); break;
                default:
                    ; // nothing
            }
            switch (e->key.keysym.scancode) {
                case SDL_SCANCODE_S: stbte_action(editor_, STBTE_tool_select); break;
                case SDL_SCANCODE_B: stbte_action(editor_, STBTE_tool_brush ); break;
                case SDL_SCANCODE_E: stbte_action(editor_, STBTE_tool_erase ); break;
                case SDL_SCANCODE_R: stbte_action(editor_, STBTE_tool_rectangle ); break;
                case SDL_SCANCODE_I: stbte_action(editor_, STBTE_tool_eyedropper); break;
                case SDL_SCANCODE_L: stbte_action(editor_, STBTE_tool_link);         break;
                case SDL_SCANCODE_G: stbte_action(editor_, STBTE_act_toggle_grid); break;
                default:
                    ; // nothing
            }
            if ((e->key.keysym.mod & KMOD_CTRL) && !(e->key.keysym.mod & ~KMOD_CTRL)) {
                switch (e->key.keysym.scancode) {
                    case SDL_SCANCODE_X: stbte_action(editor_, STBTE_act_cut  ); break;
                    case SDL_SCANCODE_C: stbte_action(editor_, STBTE_act_copy ); break;
                    case SDL_SCANCODE_V: stbte_action(editor_, STBTE_act_paste); break;
                    case SDL_SCANCODE_Z: stbte_action(editor_, STBTE_act_undo ); break;
                    case SDL_SCANCODE_Y: stbte_action(editor_, STBTE_act_redo ); break;
                    default:
                        ; // nothing
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
