#ifndef Z2UTIL_NES_MAPPER_H
#define Z2UTIL_NES_MAPPER_H
#include <functional>
#include <map>
#include <cstdint>
#include "nes/cartridge.h"
#include "imwidget/debug_console.h"

#include "proto/rominfo.pb.h"

class Mapper {
  public:
    Mapper(Cartridge* cart) : cartridge_(cart) {}
    virtual uint8_t Read(uint16_t addr) = 0;
    virtual void Write(uint16_t addr, uint8_t val) = 0;
    virtual void DebugWriteReg(DebugConsole* console, int argc, char** argv) {
        console->AddLog("Not implemented");
    }

    virtual uint8_t ReadPrgBank(int bank, uint32_t addr) {
        if (bank < 0) bank += cartridge_->prgsz();
        return cartridge_->ReadPrg(bank * 0x4000 + (addr & 0x3FFF));
    }
    virtual uint8_t ReadChrBank(int bank, uint32_t addr) {
        if (bank < 0) bank += cartridge_->chrsz();
        return cartridge_->ReadChr(bank * 0x1000 + (addr & 0x0FFF));
    }

    virtual void WritePrgBank(int bank, uint32_t addr, uint8_t val) {
        if (bank < 0) bank += cartridge_->prgsz();
        return cartridge_->WritePrg(bank * 0x4000 + (addr & 0x3FFF), val);
    }
    virtual void WriteChrBank(int bank, uint32_t addr, uint8_t val) {
        if (bank < 0) bank += cartridge_->chrsz();
        return cartridge_->WriteChr(bank * 0x1000 + (addr & 0x0FFF), val);
    }

    uint8_t Read(const z2util::Address& addr, int offset) {
        return ReadPrgBank(addr.bank(), addr.address() + offset);
    }
    uint16_t ReadWord(const z2util::Address& addr, int offset) {
        return Read(addr, offset) | uint16_t(Read(addr, offset + 1)) << 8;
    }
    z2util::Address ReadAddr(z2util::Address addr, int offset) {
        addr.set_address(ReadWord(addr, offset));
        return addr;
    }

    uint8_t Read(const z2util::MemoryRegion& addr, int offset) {
        return ReadPrgBank(addr.bank(), addr.address() + offset);
    }
    uint16_t ReadWord(const z2util::MemoryRegion& addr, int offset) {
        return Read(addr, offset) | uint16_t(Read(addr, offset + 1)) << 8;
    }
    z2util::Address ReadAddr(z2util::MemoryRegion& r, int offset) {
        z2util::Address addr;
        addr.set_bank(r.bank()); addr.set_address(r.address());
        addr.set_address(ReadWord(addr, offset));
        return addr;
    }

    void Write(const z2util::Address& addr, int offset, uint8_t data) {
        WritePrgBank(addr.bank(), addr.address() + offset, data);
    }
    void WriteWord(const z2util::Address& addr, int offset, uint16_t data) {
        Write(addr, offset, uint8_t(data));
        Write(addr, offset+1, uint8_t(data >> 8));
    }

    void Write(const z2util::MemoryRegion& addr, int offset, uint8_t data) {
        WritePrgBank(addr.bank(), addr.address() + offset, data);
    }
    void WriteWord(const z2util::MemoryRegion& addr, int offset, uint16_t data) {
        Write(addr, offset, uint8_t(data));
        Write(addr, offset+1, uint8_t(data >> 8));
    }

    z2util::Address FindFreeSpace(z2util::Address start, int length);
    void Erase(const z2util::Address& start, uint16_t length);

    z2util::Address Alloc(z2util::Address start, int length);
    uint16_t IsAlloc(z2util::Address start);
    void Free(z2util::Address start);

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
