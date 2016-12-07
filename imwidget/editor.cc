#include <string>

#include "imwidget/editor.h"
#include "util/stb_tilemap_editor.h"

#define MAX_PROP 5
ld36::Editor::PropertyInfo ld36::Editor::property_info_[MAX_PROP] = {
    { "VertClimb", STBTE_PROP_bool, 0, 1, 1 },
    { "LockedDoor", STBTE_PROP_bool, 0, 1, 1 },
    { "Spawn", STBTE_PROP_int, 0, 100, 1 },
    { "XRange", STBTE_PROP_int, 0, 100, 1 },
};

#define PROP_VERTCLIMB 0
#define PROP_DOOR 1
#define PROP_SPAWN 2
#define PROP_XRANGE 3

#define STBTE_MAX_TILEMAP_X      400
#define STBTE_MAX_TILEMAP_Y      200
#define STBTE_MAX_PROPERTIES     MAX_PROP


#define STBTE_DRAW_RECT(x0, y0, x1, y1, c) \
    ld36::Editor::Get()->DrawRect(x0, y0, x1, y1, c)

#define STBTE_DRAW_TILE(x, y, id, highlight, props) \
    ld36::Editor::Get()->DrawTile(x, y, id, highlight, props)

#define STBTE_PROP_TYPE(n, tiledata, params) \
    ld36::Editor::Get()->PropertyType(n, tiledata, params)

#define STBTE_PROP_NAME(n, tiledata, params) \
    ld36::Editor::Get()->PropertyName(n, tiledata, params)

#define STBTE_PROP_MIN(n, tiledata, params) \
    ld36::Editor::Get()->PropertyRange(n, tiledata, params, 0)

#define STBTE_PROP_MAX(n, tiledata, params) \
    ld36::Editor::Get()->PropertyRange(n, tiledata, params, 1)

#define STBTE_PROP_SCALE(n, tiledata, params) \
    ld36::Editor::Get()->PropertyRange(n, tiledata, params, 2)

#define STB_TILEMAP_EDITOR_IMPLEMENTATION
#include "util/stb_tilemap_editor.h"

static Editor* current;

Editor::Editor() :
    edit_mode_(false),
    bitmap(320,224)
{ }

Editor* Editor::Get() {
    return current;
}

Editor* Editor::Init(Renderer& r) {
    current = new Editor(r);
    current->ConvertFromMap(nullptr);
    return current;
}

void Editor::Resize(int x0, int y0, int x1, int y1) {
    stbte_set_display(x0, y0, x1, y1);
}

void Editor::DrawRect(int x0, int y0, int x1, int y1, uint32_t color) {
    bitmap->FilledBox(x0, y0, x1-x0, y1-y0, color);
}

void Editor::DrawTile(int x, int y, uint16_t tile, int mode, float* props) {
    ResourceLoader *loader = ResourceLoader::Global();
    sdlutil::GFX* g = sdlutil::GFX::Global();
    Sprite *s = loader->sprite(tile);
    if (!s) {
        fprintf(stderr, "Unknown tile id %d (%04x)\n", tile, tile);
        s = loader->sprite(1);
    }
    s->Draw(renderer_, ::SDL2pp::Point(x, y));
    if (props) {
        for(int i=0; i<MAX_PROP; i++) {
            if (props[i] != 0.0) {
                g->FilledCircle(x+16, y+16, 16, 0x80008000);
            }
        }
    }
}

void Editor::Draw() {
    stbte_draw(tm);
    stbte_tick(tm, 1.0/60.0);
}

void Editor::ProcessEvent(SDL_Event *e) {
   switch (e->type) {
      case SDL_MOUSEMOTION:
      case SDL_MOUSEBUTTONDOWN:
      case SDL_MOUSEBUTTONUP:
      case SDL_MOUSEWHEEL:
         stbte_mouse_sdl(tm, e, 1.0f,1.0f,0,0);
         break;

      case SDL_KEYDOWN:
         if (edit_mode_) {
            switch (e->key.keysym.sym) {
               case SDLK_RIGHT: stbte_action(tm, STBTE_scroll_right); break;
               case SDLK_LEFT : stbte_action(tm, STBTE_scroll_left ); break;
               case SDLK_UP   : stbte_action(tm, STBTE_scroll_up   ); break;
               case SDLK_DOWN : stbte_action(tm, STBTE_scroll_down ); break;
               default:
                   ;
            }
            switch (e->key.keysym.scancode) {
               case SDL_SCANCODE_S: stbte_action(tm, STBTE_tool_select); break;
               case SDL_SCANCODE_B: stbte_action(tm, STBTE_tool_brush ); break;
               case SDL_SCANCODE_E: stbte_action(tm, STBTE_tool_erase ); break;
               case SDL_SCANCODE_R: stbte_action(tm, STBTE_tool_rectangle ); break;
               case SDL_SCANCODE_I: stbte_action(tm, STBTE_tool_eyedropper); break;
               case SDL_SCANCODE_L: stbte_action(tm, STBTE_tool_link);       break;
               case SDL_SCANCODE_G: stbte_action(tm, STBTE_act_toggle_grid); break;
               default:
                   ;
            }
            if ((e->key.keysym.mod & KMOD_CTRL) && !(e->key.keysym.mod & ~KMOD_CTRL)) {
               switch (e->key.keysym.scancode) {
                  case SDL_SCANCODE_X: stbte_action(tm, STBTE_act_cut  ); break;
                  case SDL_SCANCODE_C: stbte_action(tm, STBTE_act_copy ); break;
                  case SDL_SCANCODE_V: stbte_action(tm, STBTE_act_paste); break;
                  case SDL_SCANCODE_Z: stbte_action(tm, STBTE_act_undo ); break;
                  case SDL_SCANCODE_Y: stbte_action(tm, STBTE_act_redo ); break;
                  case SDL_SCANCODE_S: SaveMapText(); break;
                  default:
                         ;
               }
            }
         }
         break;
      default:
         // nothing
         ;
   }
}

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
