#ifndef Z2UTIL_ALG_TERRAIN_H
#define Z2UTIL_ALG_TERRAIN_H
#include <cstdint>
#include <random>
#include <functional>
#include <memory>
namespace z2util {

// Terrain generators borrowed (with permission) from Quest Island
// by bentglasstube.

class Terrain {
  public:
    enum Type {
        PERLIN, CELLULAR
    };
    static std::unique_ptr<Terrain> New(Type type);
    Terrain() {}
    ~Terrain() {}
    virtual void Generate(unsigned int seed) { rand_.seed(seed); }
    virtual int GetTile(int x, int y) = 0;

    // Zelda 2 Terrain types
    static const uint8_t TOWN = 0;
    static const uint8_t CAVE = 1;
    static const uint8_t PALACE = 2;
    static const uint8_t BRIDGE = 3;
    static const uint8_t SAND = 4;
    static const uint8_t GRASS = 5;
    static const uint8_t TREES = 6;
    static const uint8_t SWAMP = 7;
    static const uint8_t GRAVE = 8;
    static const uint8_t ROAD = 9;
    static const uint8_t LAVA = 10;
    static const uint8_t MOUNTAINS = 11;
    static const uint8_t WATER = 12;
    static const uint8_t WALKWATER = 13;
    static const uint8_t BOULDER = 14;
    static const uint8_t SPIDER = 15;

    // Zelda2 overworlds are always 64 tiles wide
    const int MAPWIDTH = 64;
  protected:
    std::default_random_engine rand_;
};

class PerlinTerrain : public Terrain {
  public:
    PerlinTerrain() : noise_zoom_(0.03) {}
    void Generate(unsigned int seed) override;
    int GetTile(int x, int y) override;
    void set_noise_zoom(float nz) { noise_zoom_ = nz; }
  private:
    float Elevation(int x, int y) const;
    float Moisture(int x, int y) const;
    float Noise(int x, int y, int z) const;

    float elevation_z_;
    float moisture_z_;
    float noise_zoom_;
};

class CellularTerrain: public Terrain {
  public:
    CellularTerrain()
      : fg_(Terrain::SAND), bg_(Terrain::MOUNTAINS), probability_(0.25) {}
    void Generate(unsigned int seed) override;
    int GetTile(int x, int y) override;
    void set_fg(int fg) { fg_ = fg; }
    void set_bg(int bg) { bg_ = bg; }
    void set_probability(float p) { probability_ = p; }
  private:
    int WallsWithin(int x, int y, int r);
    void Iterate(std::function<bool(int, int)> selector);
    int fg_, bg_;
    float probability_;
    static const int WIDTH = 64;
    static const int HEIGHT = 100;
    uint8_t data_[HEIGHT][WIDTH];
};


}  // namespace

#endif // Z2UTIL_ALG_TERRAIN_H
