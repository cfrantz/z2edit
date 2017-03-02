#include <map>
#include <set>
#include "nes/memory.h"

#include "proto/rominfo.pb.h"
#include "nes/mapper.h"
#include "util/config.h"
#include "util/logging.h"

namespace z2util {

Memory::Memory() {}

int Memory::CheckForKeepout(Address baseaddr, const std::string& name, int len,
                            bool move) {
    int result = 0;

    for(int i=0; i<len; i++, baseaddr.set_address(baseaddr.address() + 2)) {
        Address a = mapper_->ReadAddr(baseaddr, 0);
        if (InKeepoutRegion(a)) {
            if (move) {
                int dest = moved_[key(a)];
                if (dest) {
                    // Already moved this address, so just copy it
                    mapper_->WriteWord(baseaddr, 0, uint16_t(dest));
                } else {
                    // Not moved yet, so move it.
                    dest = MoveMapOutOfKeepout(baseaddr);
                    if (!dest) {
                        LOGF(ERROR, "Could not move %s %d out of keepout area",
                             name.c_str(), a.address());
                    }
                    LOGF(INFO, "In bank=%d, %s %d moved from %04x to %04x",
                       a.bank(), name.c_str(), i, a.address(), dest);
                    moved_[key(a)] = dest;
                }
            } else {
                LOGF(INFO, "In bank=%d, %s %d is in the keepout area (%04x)",
                     a.bank(), name.c_str(), i, a.address());
            }
            result += 1;
        }
    }
    return result;
}

int Memory::CheckBankForKeepout(int bank, bool move) {
    bool result = false;
    Address addr;
    addr.set_bank(bank);

    addr.set_address(0x8000);
    result += CheckForKeepout(addr, "background map", 7, move);

    addr.set_address(0x8523);
    result += CheckForKeepout(addr, "set one map", 63, move);

    addr.set_address(0xA000);
    result += CheckForKeepout(addr, "set two map", 63, move);

    return result;
}

void Memory::CheckAllBanksForKeepout(bool move) {
    std::set<int> banks;
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    for(const auto& k : ri.misc().allocator_keepout()) {
        banks.insert(k.bank());
    }
    for(const auto& bank : banks) {
        CheckBankForKeepout(bank, move);
    }
}

int Memory::MoveMapOutOfKeepout(const Address& pointer) {
    Address src = mapper_->ReadAddr(pointer, 0);
    uint8_t len = mapper_->Read(src, 0);
    
    // Alloc only uses 'bank' from 'src'
    Address dst = mapper_->Alloc(src, len);
    if (!dst.address())
        return false;

    // Copy to the new location, zero out the old location.
    for(int i=0; i<len; i++) {
        mapper_->Write(dst, i, mapper_->Read(src, i));
        mapper_->Write(src, i, 0);
    }
    mapper_->WriteWord(pointer, 0, uint16_t(dst.address()));
    return dst.address();
}

bool Memory::InKeepoutRegion(const Address& addr) {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    for(const auto& k : ri.misc().allocator_keepout()) {
        if (addr.bank() == k.bank() &&
            addr.address() >= k.address() &&
            addr.address() < k.address() + k.length()) {
            return true;
        }
    }
    return false;
}

}  // namespace
