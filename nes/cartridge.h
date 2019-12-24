#ifndef Z2UTIL_NES_CARTRIDGE_H
#define Z2UTIL_NES_CARTRIDGE_H
#include <string>
#include <memory>
#include <cstdint>

#include "imwidget/debug_console.h"

class Cartridge {
  public:
    struct iNESHeader {
        uint32_t signature;
        uint8_t prgsz;
        uint8_t chrsz;
        uint8_t mirror0: 1;
        uint8_t sram: 1;
        uint8_t trainer: 1;
        uint8_t mirror1: 1;
        uint8_t mapperl: 4;
        uint8_t vs_unisystem: 1;
        uint8_t playchoice_10: 1;
        uint8_t version: 2;
        uint8_t mapperh: 4;
        uint8_t unused[8];
    };
    enum MirrorMode {
        HORIZONTAL,
        VERTICAL,
        SINGLE0,
        SINGLE1,
        FOUR,
    };
    Cartridge();
    Cartridge(const Cartridge& orig);
    ~Cartridge();

    static bool IsNESFile(const std::string& filename);

    std::string SaveRom();
    void LoadRom(const std::string& rom);

    void LoadFile(const std::string& filename);
    void SaveFile(const std::string& filename);
    void PrintHeader();
    inline uint8_t mirror() const { return mirror_; }
    inline void set_mirror(MirrorMode m) { mirror_ = m; }
    inline bool battery() const {
        return header_.sram;
    }
    inline uint8_t mapper() const {
        return header_.mapperl | header_.mapperh << 4;
    }
    inline void set_mapper(uint8_t m) {
        header_.mapperl = m;
        header_.mapperh = m>>4;
    }
    inline uint32_t prglen() const { return prglen_; }
    inline uint32_t chrlen() const { return chrlen_; }

    inline uint8_t prgsz() const { return header_.prgsz; }
    inline uint8_t chrsz() const { return header_.chrsz; }

    inline uint8_t* prg() const { return prg_.get(); }
    inline uint8_t* chr() const { return chr_.get(); }

    inline uint8_t ReadPrg(uint32_t addr) { return prg_[addr]; }
    inline uint8_t ReadChr(uint32_t addr) { return chr_[addr]; }
    inline void WritePrg(uint32_t addr, uint8_t val) { prg_[addr] = val; }
    inline void WriteChr(uint32_t addr, uint8_t val) { chr_[addr] = val; }

    void PrintHeader(DebugConsole* console, int argc, char **argv);
    void LoadFile(DebugConsole* console, int argc, char **argv);
    void SaveFile(DebugConsole* console, int argc, char **argv);

    void InsertPrg(int bank, uint8_t* newprg);
    void InsertChr(int bank, uint8_t* newchr);
  private:
    struct iNESHeader header_;
    std::unique_ptr<uint8_t[]> prg_;
    uint32_t prglen_;
    std::unique_ptr<uint8_t[]> chr_;
    uint32_t chrlen_;
    std::unique_ptr<uint8_t[]> trainer_;
    MirrorMode mirror_;
};

#endif // Z2UTIL_NES_CARTRIDGE_H
