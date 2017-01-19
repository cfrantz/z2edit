#ifndef Z2UTIL_ROMFILE_H
#define Z2UTIL_ROMFILE_H
#include <cstdint>
#include "util/string.h"

class RomFile {
  public:
    RomFile();

    bool Load(const string& filename);
    inline void Hexdump(uint8_t bank, uint16_t addr, uint32_t length) const {
        Hexdump(FileOffset(bank, addr), length);
    }
    void Hexdump(uint32_t offset, uint32_t length,
                 uint32_t highlight=0, uint32_t hilen=0) const;

    void Grep(const std::string& pattern, bool wildcard);

    inline uint8_t Read8(uint32_t fileofs) const { return rom_.at(fileofs); }
    inline uint16_t Read16(uint32_t fileofs) const {
        return Read8(fileofs) | uint16_t(Read8(fileofs+1)) << 8;
    }
    inline uint8_t Read8(uint8_t bank, uint16_t addr) const {
        return Read8(FileOffset(bank, addr));
    }
    inline uint16_t Read16(uint8_t bank, uint16_t addr) const {
        return Read16(FileOffset(bank, addr));
    }
    static inline uint32_t FileOffset(uint8_t bank, uint16_t addr) {
        return (addr - 0x8000) + (bank * 0x4000) + 0x10;
    }
    void FindFreeSpaceInBank(int bank);
    void FindFreeSpace();
    void ReadEnemyLists(uint8_t bank);
  private:
    string rom_;
};

#endif // Z2UTIL_ROMFILE_H
