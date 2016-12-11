#include "nes/mapper.h"


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
    uint16_t end = 0x4000;
    uint16_t offset = addr.address() & 0x3FFF;

    addr.set_address(0);
    if (length < 16)
        length = 16;

    while(offset < end) {
        if ((offset & 0x000F) == 0 && Read(addr, offset) == 0xFF) {
            uint16_t run = offset + 1;
            while(run < end && Read(addr, run) == 0xFF) {
                run++;
            }
            if (run-offset >= length) {
                addr.set_address(offset);
                break;
            }
            offset = run;
        }
        offset++;
    }
    return addr;
}

void Mapper::Erase(const z2util::Address& addr, uint16_t length) {
    for(uint16_t i=0; i<length; i++) {
        Write(addr, i, 0xFF);
    }
}
