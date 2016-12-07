#ifndef Z2UTIL_NES_MAPPER_H
#define Z2UTIL_NES_MAPPER_H
#include <functional>
#include <map>
#include <cstdint>
#include "nes/cartridge.h"
#include "imwidget/debug_console.h"

class Mapper {
  public:
    Mapper(Cartridge* cart) : cartridge_(cart) {}
    virtual uint8_t Read(uint16_t addr) = 0;
    virtual void Write(uint16_t addr, uint8_t val) = 0;
    virtual void DebugWriteReg(DebugConsole* console, int argc, char** argv) {
        console->AddLog("Not implemented");
    }

    virtual uint8_t ReadPrgBank(uint8_t bank, uint32_t addr) {
        return cartridge_->ReadPrg(bank * 0x4000 + (addr & 0x3FFF));
    }
    virtual uint8_t ReadChrBank(uint8_t bank, uint32_t addr) {
        return cartridge_->ReadChr(bank * 0x1000 + (addr & 0x0FFF));
    }

    virtual void WritePrgBank(uint8_t bank, uint32_t addr, uint8_t val) {
        return cartridge_->WritePrg(bank * 0x4000 + (addr & 0x3FFF), val);
    }
    virtual void WriteChrBank(uint8_t bank, uint32_t addr, uint8_t val) {
        return cartridge_->WriteChr(bank * 0x1000 + (addr & 0x0FFF), val);
    }

    Cartridge* cartridge() { return cartridge_; }
  protected:
    Cartridge* cartridge_;
};

class MapperRegistry {
  public:
    MapperRegistry(int n, std::function<Mapper*(Cartridge*)> create);
    static Mapper* New(Cartridge* cart, int n);
  private:
    static std::map<int, std::function<Mapper*(Cartridge*)>>* mappers();
};

#define CONCAT_(x, y) x ## y
#define CONCAT(x, y) CONCAT_(x, y)

#define REGISTER_MAPPER(n_, type_) \
    MapperRegistry CONCAT(CONCAT(CONCAT(reg_, type_), __), __LINE__) (n_, \
            [](Cartridge* cart) { return new type_(cart); })

#endif // Z2UTIL_NES_MAPPER_H
