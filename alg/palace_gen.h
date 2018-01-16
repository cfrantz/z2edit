#ifndef Z2UTIL_ALG_PALACE_GEN_H
#define Z2UTIL_ALG_PALACE_GEN_H

#include <cstdint>
#include <random>
#include <functional>
#include <memory>
#include <vector>

#include "imwidget/map_command.h"
#include "nes/mapper.h"
#include "proto/generator.pb.h"
#include "proto/rominfo.pb.h"

namespace z2util {

class PalaceGenerator {
  public:
    struct Room {
        int room;
        uint8_t up;
        uint8_t down;
        uint8_t left;
        uint8_t right;
        uint8_t elevator;
        bool has_up, has_down, has_left, has_right;
        bool visited;
        bool dead_end;
        bool boss_room, item_room;
        int distance;
    };
    PalaceGenerator(PalaceGeneratorOptions& opt);

    void Generate();
    void SimplePrint();
    inline void set_mapper(Mapper* m) { mapper_ = m; }
  private:
    enum Direction { LEFT, RIGHT, UP, DOWN, };
    struct RoomGen {
        int xpos;
        int floor_val;
    };
    struct FloorCeiling {
        int ceiling, floor;
    };

    void InitPalaceMaps();
    void GenerateMaze();
    void VisitRooms(int x, int y);
    void WalkMaze(int r, int distance, Direction from);
    bool SelectSpecialRooms();
    void MapToRooms();
    void PrepareRoom(int r);
    void PrepareEntranceRoom(int r);
    void PrepareBossRoom(int r);
    void PrepareItemRoom(int r);
    void FixElevatorConnections();

    void CleanNearElevator(int r, int x);
    int MakeElevator(int r, int x, int floor_val);
    void MakeFakeCeiling(int r, int x, int w, int downto);
    void MakeFakeFloor(int r, int x, int w, int upto);
    void MakeGallery(int r, int w=-1);
    void MakeLavaPit(int r);
    void MakeCubby(int r, int w=-1);

    inline double real() { return real_(rng_); }
    inline bool bit(double prob=0.5) { return real() < prob; }
    inline int integer(int n) { return real() * double(n); }

    PalaceGeneratorOptions opt_;
    std::vector<std::vector<Room>> map_;
    std::vector<Room> rooms_;
    std::mt19937 rng_;
    std::uniform_real_distribution<double> real_;
    int room_;
    Mapper* mapper_;
    Map palace_maps_[64];
    std::unique_ptr<MapHolder> holder_;
    RoomGen gen_;

    static const FloorCeiling fpos_[];
    static const int MAXX = 62;
};

}  // namespace
#endif // Z2UTIL_ALG_PALACE_GEN_H
