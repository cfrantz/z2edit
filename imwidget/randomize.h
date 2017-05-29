#ifndef Z2UTIL_IMWIDGET_RANDOMIZE_H
#define Z2UTIL_IMWIDGET_RANDOMIZE_H
#include <cstdint>

typedef struct stbte_tilemap stbte_tilemap;
namespace z2util {

class OverworldConnectorList;
class RandomizeOverworld {
  public:
    RandomizeOverworld();
    bool Draw(stbte_tilemap* editor, OverworldConnectorList* connections);
  private:
    void InitParams(stbte_tilemap* editor);
    void SaveMap(stbte_tilemap* editor);
    void RestoreMap(stbte_tilemap* editor);
    struct RandomParams {
        bool initialized;
        bool keep_transfer_tiles;
        float p;
        int bg;
        int fg;
        int algorithm;
        int seed;
        int x0;
        int y0;
        int x1;
        int y1;
        int centerx;
        int centery;
        bool keep;
        uint8_t backup[100][64];
    };
    RandomParams random_params_;
};
}  // namespace z2util
#endif // Z2UTIL_IMWIDGET_RANDOMIZE_H
