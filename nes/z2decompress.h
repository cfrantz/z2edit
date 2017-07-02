#ifndef Z2UTIL_NES_Z2DECOMPRESS_H
#define Z2UTIL_NES_Z2DECOMPRESS_H
#include <map>
#include <string>

#include "proto/rominfo.pb.h"
#include "nes/mapper.h"

namespace z2util {

class Z2Decompress {
  public:
    Z2Decompress();
    void Init();

    void Print();
    void Decompress(const Map& map);
    void DecompressSideView(const uint8_t* data, bool collapse=true);
    void set_mapper(Mapper* m) { mapper_ = m; }

    inline uint8_t map(int x, int y) const {
        return map_[layer_][y][x];
    }
    inline uint8_t item(int x, int y) const {
        return type() == MapType::OVERWORLD ? 0xFF: items_[y][x];
    }

    inline void set_map(int x, int y, uint8_t val) {
        if (x >= 0 && y >= 0 && x < width_ && y < height_) {
            map_[layer_][y][x] = val;
        }
    }
    inline bool isbackground(int x, int y, int val) {
        return map(x, y) == 0 || map(x, y) == val;
    }
    inline MapType type() const {
        return compressed_map_.type();
    }
    inline bool cursor_moves_left() {
        return cursor_moves_left_;
    }
    const ItemInfo& EnemyInfo();
    void Clear();

    Address palette();
    inline int width() const { return width_; }
    inline int mapwidth() const { return mapwidth_; }
    inline int height() const { return height_; }

    inline const Address& address() const { return compressed_map_.address(); }
    inline int length() const { return length_; }
    inline const std::string& name() const { return compressed_map_.name(); }
    // areas: overword sideviews, towns, palaces, great palace
    const static int NR_AREAS = 4;
    // sets: small objects, object set 0, object set 1,
    // extra small objects, extra objects
    const static int NR_SETS = 5;
  private:
    int width_;
    int mapwidth_;
    int height_;
    typedef void (Z2Decompress::*PutFn)(int x, int y, uint8_t item,
                  const DecompressInfo* info);

    void DecompressOverWorld(const Map& map);
    void DecompressSideView(const Address& address, const Address* foreground);
    void CollapseLayers(int top_layer);
    const BackgroundInfo& GetBackgroundInfo();

    void DrawFloor(int x, uint8_t floor, uint8_t ceiling);

    void Invalid(int x, int y, uint8_t item,
                 const DecompressInfo* info);
    void NotYet(int x, int y, uint8_t item,
                const DecompressInfo* info);
    void RenderHorizontal(int x, int y, uint8_t item,
                          const DecompressInfo* info);
    void RenderVertical(int x, int y, uint8_t item,
                        const DecompressInfo* info);
    void RenderTopUnique(int x, int y, uint8_t item,
                         const DecompressInfo* info);
    void RenderBottomUnique(int x, int y, uint8_t item,
                            const DecompressInfo* info);
    void RenderGrid(int x, int y, uint8_t item,
                    const DecompressInfo* info);
    void RenderLava3High(int x, int y, uint8_t item,
                         const DecompressInfo* info);
    void RenderCactus1(int x, int y, uint8_t item,
                       const DecompressInfo* info);
    void RenderCactus2(int x, int y, uint8_t item,
                       const DecompressInfo* info);
    void RenderStonehenge(int x, int y, uint8_t item,
                          const DecompressInfo* info);
    void RenderBuilding(int x, int y, uint8_t item,
                        const DecompressInfo* info);
    void RenderWindow(int x, int y, uint8_t item,
                      const DecompressInfo* info);
    void RenderItem(int x, int y, uint8_t item,
                    const DecompressInfo* info);


    Mapper* mapper_;
    uint8_t map_[8][128][64];
    uint8_t items_[16][64];
    Map compressed_map_;

    uint8_t flags_;
    uint8_t ground_;
    uint8_t back_;
    int length_;
    int layer_;
    bool cursor_moves_left_;
    const DecompressInfo* info_[NR_AREAS][NR_SETS][16];
    static std::map<std::string, PutFn> put_;

    inline uint8_t Read(const Address& addr, uint16_t offset) {
        return mapper_->ReadPrgBank(addr.bank(), addr.address() + offset);
    }
    inline uint16_t ReadWord(const Address& addr, uint16_t offset) {
        return mapper_->ReadPrgBank(addr.bank(), addr.address() + offset) |
          mapper_->ReadPrgBank(addr.bank(), addr.address() + offset + 1) << 8;
    }

};

}  // namespace z2util
#endif // Z2UTIL_NES_Z2DECOMPRESS_H
