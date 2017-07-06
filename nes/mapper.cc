#include "nes/mapper.h"
#include "nes/memory.h"

// The byte sequence A1 0C doesn't appear in the zelda2 ROM file.  We'll use
// this sequence as a magic number for the free-space allocator.
//
// Allocations of free space always request 4 extra bytes, which are used
// to mark that what follows is manged by the free-space allocator:
//
// A1 0C length-lsb length-msb <lenght bytes of user data>
#define ALLOC_TOKEN     0x0CA1


std::map<int, std::function<Mapper*(Cartridge*)>>* MapperRegistry::mappers() {
    static std::map<int, std::function<Mapper*(Cartridge*)>> reg;
    return &reg;
}

MapperRegistry::MapperRegistry(int n, std::function<Mapper*(Cartridge*)> create) {
    mappers()->insert(std::make_pair(n, create));
}

Mapper* MapperRegistry::New(Cartridge* cart, int n) {
    const auto& mi = mappers()->find(n);
    if (mi == mappers()->end()) {
        return nullptr;
    }
    return mi->second(cart);
}

z2util::Address Mapper::FindFreeSpace(z2util::Address addr, int length) {
    int end;
    int offset;

    addr.set_address(0);
    if (length < 8)
        length = 8;

    // Search backwards from the end for free space
    offset = 0x3fe0;
    while(offset > 0) {
        if (Read(addr, offset) == 0xFF) {
            end = offset;
            while(offset > 0 && Read(addr, offset) == 0xFF) {
                if (end - offset + 1 == length) {
                    z2util::Address startaddr = addr;
                    z2util::Address endaddr = addr;
                    startaddr.set_address(0x8000 | offset);
                    endaddr.set_address(0x8000 | end);
                    if (z2util::Memory::InKeepoutRegion(startaddr)
                        || z2util::Memory::InKeepoutRegion(endaddr)) {
                        break;
                    }
                    return startaddr;
                }
                offset--;
            }
        }
        offset--;
    }

    addr.set_address(0);
    return addr;
}

void Mapper::Erase(const z2util::Address& addr, uint16_t length) {
    for(uint16_t i=0; i<length; i++) {
        Write(addr, i, 0xFF);
    }
}

z2util::Address Mapper::Alloc(z2util::Address addr, int length) {
    addr = FindFreeSpace(addr, length+4);
    if (addr.address() != 0) {
        WriteWord(addr, 0, ALLOC_TOKEN);
        WriteWord(addr, 2, length);
        addr.set_address(addr.address() + 4);
    }
    return addr;
}

void Mapper::Free(z2util::Address addr) {
    if (ReadWord(addr, -4) == ALLOC_TOKEN) {
        uint16_t length = ReadWord(addr, -2);
        addr.set_address(addr.address() - 4);
        Erase(addr, length+4);
    }
}
