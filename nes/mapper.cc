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
