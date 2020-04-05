#include <set>

#include "nes/enemylist.h"

#include "nes/mapper.h"
#include "proto/rominfo.pb.h"
#include "util/config.h"
#include <gflags/gflags.h>


DEFINE_int32(bank5_enemy_list_size, 432,
        "Size of the enemylist buffer in bank 5");

namespace z2util {

void EnemyListPack::LoadEncounters() {
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    Address addr = misc.overworld_encounters();
    addr.set_bank(bank_);

    for(int i=0; i<14; i++) {
        encounters_.push_back(mapper_->Read(addr, i) & 0x3f);
    }
}

bool EnemyListPack::IsEncounter(int area) {
    Address addr;

    // Can't be an overworld encounter in a bank that doesn't have an
    // overworld.
    addr.set_bank(bank_);
    addr.set_address(0x8500);
    uint16_t ovptr = mapper_->ReadWord(addr, 10);
    if (ovptr == 0 || ovptr == 0xFFFF)
        return false;

    for(const auto& e : encounters_) {
        if (e == area)
            return true;
    }
    return false;
}

void EnemyListPack::ReadOne(int area, Address addr) {
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    uint16_t orig_addr = addr.address();
    addr.set_address(addr.address() - misc.enemy_data_ram() +
                     misc.enemy_data_rom());

    List entry;
    entry.newaddr = 0;
    int lists = IsEncounter(area) ? 2 : 1;

    for(int j=0; j<lists; j++) {
        int i;
        uint8_t len = mapper_->Read(addr, 0);
        for(i=0; i<len; i++) {
            entry.data.push_back(mapper_->Read(addr, i));
        }
        addr.set_address(addr.address() + i);
    }

    entry_.insert(std::make_pair(orig_addr, entry));
    area_[area] = orig_addr;
}


void EnemyListPack::Unpack(int bank) {
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();

    bank_ = bank;
    // In bank5, the victory sequence starts at 0x8b50, so we have fewer
    // bytes available for the enemy list.
    // In the other banks, I've taken pains to move things around, but
    // bank5 is pretty full and has only 63 rooms instead of 126, so
    // ~half the space should be enough.
    size_ = (bank == 5) ? FLAGS_bank5_enemy_list_size : 1024;
    area_.resize(126, 0);

    LoadEncounters();

    std::set<int> single_list(misc.single_enemy_list_bank().cbegin(),
                              misc.single_enemy_list_bank().cend());
    int i = 0;
    for(Address addr : misc.enemy_pointer()) {
        addr.set_bank(bank_);
        for(int n=0; n<63; n++, i++) {
            Address pointer = mapper_->ReadAddr(addr, 2*n);
            uint16_t pa = pointer.address();
            if (pa >= misc.enemy_data_ram() &&
                    pa < misc.enemy_data_ram() + size_) {
                ReadOne(i, pointer);
            }
        } 
        if (single_list.find(bank_) != single_list.end()) {
            // If this bank only has a single enemy list, bail out.
            break;
        }
    }
}


void EnemyListPack::Add(int area, const std::vector<uint8_t>& data) {
    newareas_++;
    List entry{0, data};

    // Insert with a fake address so we can figure it out later
    entry_.insert(std::make_pair(-newareas_, entry));
    area_[area] = -newareas_;
}


bool EnemyListPack::Pack() {
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    std::vector<uint8_t> packed;
    
    // Insert an empty list to consolidate all empty rooms to the same
    // zero length list.
    packed.push_back(1);

    // Pack all of the enemy lists into the buffer
    for(int i=0; i<126; i++) {
        char side = 'A' + i/63;
        int room = i%63;
        int addr = area_[i];
        if (addr == 0)
            continue;

        List& entry = entry_[addr];
        if (entry.newaddr == 0) {
            if (entry.data.size() == 1 && !IsEncounter(room)) {
                // All empty lists (lists of length 1) point to the
                // empty-list sentinel at the beginning of the packed array.
                entry.newaddr = 0 + misc.enemy_data_ram();
                LOGF(INFO, "Packed room %c%d at %04x (%d bytes)",
                        side, room, entry.newaddr, entry.data.size());
            } else if (packed.size() + entry.data.size() > size_) {
                LOGF(ERROR, "Out of space for enemy list at %c%d (want %d+%d / %d)",
                        side, room, packed.size(), entry.data.size(), size_);
                return false;
            } else {
                entry.newaddr = packed.size() + misc.enemy_data_ram();
                packed.insert(packed.end(), entry.data.begin(), entry.data.end());
                LOGF(INFO, "Packed room %c%d at %04x (%d bytes)",
                        side, room, entry.newaddr, entry.data.size());
            }
        } else {
            LOGF(INFO, "Refer room  %c%d to %04x", side, room, entry.newaddr);
        }
    }

    // If everything fit, rewrite the map pointers
    int i = 0;
    for(Address pointer : misc.enemy_pointer()) {
        pointer.set_bank(bank_);
        for(int n=0; n<63; n++, i++) {
            int addr = area_[i];
            if (addr == 0)
                continue;

            List& entry = entry_[addr];
            mapper_->WriteWord(pointer, n*2, entry.newaddr);
        }
    }

    // And copy the enemy lists to the rom
    LOGF(INFO, "Extending pack from %d to %d bytes", packed.size(), size_);
    packed.resize(size_, 0xFF);
    Address addr;
    addr.set_bank(bank_);
    addr.set_address(misc.enemy_data_rom());
    for(size_t i=0; i<packed.size(); i++) {
        mapper_->Write(addr, i, packed[i]);
    }
    return true;
}

}  // namespace z2util
