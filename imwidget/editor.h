#ifndef Z2UTIL_IMWIDGET_EDITOR_H
#define Z2UTIL_IMWIDGET_EDITOR_H
#include <memory>
#include <vector>
#include "imwidget/error_dialog.h"
#include "imwidget/glbitmap.h"
#include "imwidget/imwidget.h"
#include "imwidget/map_connect.h"
#include "imwidget/randomize.h"
#include "imwidget/overworld_encounters.h"
#include "nes/z2objcache.h"
#include "imgui.h"

#include <SDL2/SDL.h>

typedef struct stbte_tilemap stbte_tilemap;

class Mapper;
class NesHardwarePalette;

namespace z2util {
class Map;

class Editor: public ImWindowBase {
  public:
    static Editor* Get();
    static Editor* New();
    Editor();

    void ConvertFromMap(Map* map);
    std::vector<uint8_t> CompressMap();
    void SaveMap();

    void ProcessEvent(SDL_Event* e);
    bool Draw() override;
    void Refresh() override;

    inline void set_mapper(Mapper* m) { mapper_ = m; }

    void DrawTile(int x, int y, uint16_t tile, int mode, float* props);
    void DrawRect(int x0, int y0, int x1, int y1, uint32_t color);
    void Resize(int x0, int y0, int x1, int y1);
    int PropertyType(int n, int16_t* tiledata, float* params);
    char* PropertyName(int n, int16_t* tiledata, float* params);
    float PropertyRange(int n, int16_t* tiledata, float* params, int what);
    struct PropertyInfo {
        char *name;
        int type;
        float min, max, scale;
    };
    static PropertyInfo property_info_[];
  private:
    void HandleEvent(SDL_Event* e);

    bool changed_;
    int undo_len_;
    bool show_connections_;
    float scale_;
    stbte_tilemap* editor_;
    Map* map_;
    int compressed_length_;
    int hidden_palace_tile_;
    int hidden_town_tile_;

    Mapper* mapper_;
    NesHardwarePalette* hwpal_;
    Z2ObjectCache cache_;
    OverworldConnectorList connections_;
    OverworldEncounters encounters_;
    ImVec2 mouse_origin_;
    ImVec2 origin_;
    ImVec2 size_;
    bool mouse_focus_;
    int mapsel_;
    std::vector<SDL_Event> events_;
    std::unique_ptr<ErrorDialog> error_dialog_;
    RandomizeOverworld randomize_;
};

}  // namespace
#endif // Z2UTIL_IMWIDGET_EDITOR_H
