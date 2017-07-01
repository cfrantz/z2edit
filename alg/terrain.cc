#include "alg/terrain.h"

#define STB_PERLIN_IMPLEMENTATION 1
#include "util/stb_perlin.h"
namespace z2util {

std::unique_ptr<Terrain> Terrain::New(Type type) {
    std::unique_ptr<Terrain> ptr;
    switch(type) {
        case PERLIN:
            ptr.reset(new PerlinTerrain());
            break;
        case CELLULAR:
            ptr.reset(new CellularTerrain());
            break;
        case VORONOI:
            ptr.reset(new VoronoiTerrain());
            break;
        case MANHATTAN:
            ptr.reset(new ManhattanTerrain());
            break;
        default:
            ;
    }
    return std::move(ptr);
}

void PerlinTerrain::Generate(unsigned int seed) {
    Terrain::Generate(seed);
    std::uniform_real_distribution<float> u(-100, 100);
    elevation_z_ = u(rand_);
    moisture_z_ = u(rand_);
}

int PerlinTerrain::GetTile(int x, int y) {
    const float e = Elevation(x, y);
    const float m = Moisture(x, y);

    if (e > 0.030f) {
        if (m > 0.1) return TREES;
        return MOUNTAINS;
    } else if (e > 0.005f) {
        if (m > 0.3) return SWAMP;
        if (m > 0.1) return TREES;
        if (m > -0.2) return GRASS;
        return SAND;
    } else if (e > 0.0f) {
        return SAND;
    } else {
        return WATER;
    }
}

float PerlinTerrain::Elevation(int x, int y) const {
    const float d = x * x + y * y;
    const float e = Noise( 1 * x,  1 * y, elevation_z_) / 2.0f
        + Noise( 2 * x,  2 * y, elevation_z_) / 4.0f
        + Noise( 4 * x,  4 * y, elevation_z_) / 8.0f;
    return powf(e, 3.0f) + 0.03f - noise_zoom_ * 0.0001 * d;
}

float PerlinTerrain::Moisture(int x, int y) const {
    const float m = Noise( 1 * x,  1 * y, moisture_z_) / 2.0f
        + Noise( 2 * x,  2 * y, moisture_z_) / 4.0f
        + Noise( 4 * x,  4 * y, moisture_z_) / 8.0f;
    return m;
}

float PerlinTerrain::Noise(int x, int y, int z) const {
    return stb_perlin_noise3(noise_zoom_ * x, noise_zoom_ * y, z);
}

int CellularTerrain::GetTile(int x, int y) {
    if (x < 0 || x >= MAPWIDTH) return bg_;
    if (y < 0 || y >= MAPHEIGHT) return bg_;
    return data_[y][x];
}

void CellularTerrain::Generate(unsigned int seed) {
    Terrain::Generate(seed);
    std::uniform_real_distribution<float> u(0.0, 1.0f);

    for(int y=0; y<MAPHEIGHT; y++) {
        for(int x=0; x<MAPWIDTH; x++) {
            data_[y][x] = u(rand_) < probability_ ? bg_ : fg_;
        }
    }

    for (int i = 0; i < 4; ++i) {
        Iterate([this](int x, int y){
            return WallsWithin(x, y, 1) >= 5 || WallsWithin(x, y, 2) <= 2;
        });
    }
    /*
    for (int i = 0; i < 3; ++i) {
        Iterate([this](int x, int y){
            return WallsWithin(x, y, 1) >= 5;
        });
    }
    */
}

int CellularTerrain::WallsWithin(int x, int y, int r) {
    int count = 0;
    for(int yy=y-r; yy<y+r; yy++) {
        for(int xx=x-r; xx<x+r; xx++) {
            if (data_[yy][xx] == bg_) ++count;
        }
    }
    return count;
}

void CellularTerrain::Iterate(std::function<bool(int, int)> selector) {
    uint8_t d[MAPHEIGHT][MAPWIDTH];
    for (int y = 0; y < MAPHEIGHT; ++y) {
        for (int x = 0; x < MAPWIDTH; ++x) {
            d[y][x] = selector(x, y) ? bg_ : fg_;
        }
    }

    for (int y = 0; y < MAPHEIGHT; ++y) {
        for (int x = 0; x < MAPWIDTH; ++x) {
            data_[y][x] = d[y][x];
        }
    }
}


void VoronoiTerrain::Generate(unsigned int seed) {
    Terrain::Generate(seed);
    uint8_t types[] = {
        SAND, GRASS, TREES, SWAMP, GRAVE, MOUNTAINS, WATER,
    };
    std::uniform_real_distribution<float> tt(0, 7);
    std::uniform_real_distribution<float> xpos(0, width_);
    std::uniform_real_distribution<float> ypos(0, height_);

    points_.clear();
    for(int y=0; y<height_; y++) {
        for(int x=0; x<width_; x++) {
            data_[y][x].distance = 1e6;
            data_[y][x].point = 0;
        }
    }

    for(int i=0; i<num_; i++) {
        points_.emplace_back(Point{int(xpos(rand_)), int(ypos(rand_)),
                                   types[int(tt(rand_))]});
    }

    for(int y=0; y<height_; y++) {
        for(int x=0; x<width_; x++) {
            for(int i=0; i<num_; i++) {
                double d = DistanceFunction(x, y, i);
                if (d < data_[y][x].distance) {
                    data_[y][x].distance = d;
                    data_[y][x].point = i;
                }
            }
        }
    }
}

int VoronoiTerrain::GetTile(int x, int y) {
    if (num_ == 0) return SAND;
    int point = data_[y][x].point;
    return points_[point].terrain;
}

double VoronoiTerrain::DistanceFunction(int fromx, int fromy, int topoint) {
    double x0 = fromx;
    double y0 = fromy;
    double x1 = points_[topoint].x;
    double y1 = points_[topoint].y;
    double xd = x0 - x1;
    double yd = y0 - y1;
    return std::sqrt(xd*xd + yd*yd);
}

double ManhattanTerrain::DistanceFunction(int fromx, int fromy, int topoint) {
    double x0 = fromx;
    double y0 = fromy;
    double x1 = points_[topoint].x;
    double y1 = points_[topoint].y;
    double xd = x0 - x1;
    double yd = y0 - y1;
    return std::abs(xd) + std::abs(yd);
}

}  // namespace
