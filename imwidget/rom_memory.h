#ifndef Z2UTIL_IMWIDGET_ROM_MEMORY_H
#define Z2UTIL_IMWIDGET_ROM_MEMORY_H
#include <cstdint>
#include <map>
#include <vector>

#include "imwidget/imwidget.h"
#include "nes/mapper.h"
#include "imwidget/glbitmap.h"
#include "proto/rominfo.pb.h"

namespace z2util {

class RomMemory: public ImWindowBase {
  public:
    RomMemory()
      : ImWindowBase(false),
        mapper_(nullptr),
        bank_(1),
        scale_(4),
        refresh_(true),
        viz_(128, 128)
        {}

    void Init();
    void Refresh() override { refresh_ = true; }
    bool Draw() override;

    inline void set_mapper(Mapper* m) { mapper_ = m; }

  private:
    int GetOverworldLength(const Address& addr);
    void ProcessOverworlds();
    void ProcessSideview();
    void FillFreeSpace(uint32_t color);
    void FillKeepout(uint32_t color);
    int FillSideview(const Address& addr, uint32_t color);
    int FillOverworld(const Address& addr, uint32_t color);
    int FillAllocRegions();

    bool Repack();

    struct RomData {
        uint16_t orig;
        uint16_t address;
        std::vector<uint8_t> data;
    };
    struct Region {
        uint16_t address;
        uint16_t offset;
        uint16_t length;
    };

    RomData ReadLevel(const Address& addr);
    RomData ReadOverworld(const Address& addr);
    void WriteRomData(const RomData& rd);
    bool PlaceMap(std::vector<Region>* regions, RomData* map);
    void FixMapPointers(const RomData& rd,
                        std::map<uint16_t, std::vector<uint16_t>>& pointers);
    void FreeAllocRegions();

    Mapper* mapper_;
    int bank_;
    int scale_;
    bool refresh_;
    GLBitmap viz_;
};

}  // namespace z2util

#endif // Z2UTIL_IMWIDGET_ROM_MEMORY_H
