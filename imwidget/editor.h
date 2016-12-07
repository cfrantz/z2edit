#ifndef Z2UTIL_IMWIDGET_EDITOR_H
#define Z2UTIL_IMWIDGET_EDITOR_H
#include <memory>
#include "imwidget/glbitmap.h"

typedef struct stbte_tilemap stbte_tilemap;

class Editor {
  public:
    static Editor* Get();
    Editor();
    void ProcessEvent(SDL_Event* e);
    void DrawTile(int x, int y, uint16_t tile, int mode, float* props);
    void DrawRect(int x0, int y0, int x1, int y1, uint32_t color);
    void Draw();

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
    bool edit_mode_;
    stbte_tilemap* tm;
    std::unique<GLBitmap> bitmap;
};

#endif // Z2UTIL_IMWIDGET_EDITOR_H
