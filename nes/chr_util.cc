#include "nes/chr_util.h"

#include <cstdint>
#include "nes/mapper.h"

namespace z2util {
const uint8_t chrxdigits[16][8] = {
    { 2, 5, 5, 5, 5, 5, 2, 0 },    // 0
    { 2, 6, 2, 2, 2, 2, 7, 0 },    // 1
    { 2, 5, 1, 2, 4, 4, 7, 0 },    // 2
    { 2, 5, 1, 3, 1, 5, 2, 0 },    // 3
    { 5, 5, 5, 7, 1, 1, 1, 0 },    // 4
    { 7, 4, 7, 1, 1, 5, 2, 0 },    // 5
    { 3, 4, 4, 4, 7, 5, 7, 0 },    // 6
    { 7, 1, 1, 2, 2, 4, 4, 0 },    // 7
    { 2, 5, 5, 2, 5, 5, 2, 0 },    // 8
    { 7, 5, 7, 1, 1, 1, 1, 0 },    // 9
    { 2, 5, 5, 7, 5, 5, 5, 0 },    // A
    { 6, 5, 5, 6, 5, 5, 6, 0 },    // B
    { 2, 5, 4, 4, 4, 5, 2, 0 },    // C
    { 6, 5, 5, 5, 5, 5, 6, 0 },    // D
    { 7, 4, 4, 7, 4, 4, 7, 0 },    // E
    { 7, 4, 4, 7, 4, 4, 4, 0 },    // F
};

void ChrUtil::Clear(int bank, uint8_t ch, bool with_id) {
    uint16_t addr = 16 * ch;
    for(uint16_t y=0; y<8; y++) {
        uint8_t val = 0;
        if (with_id) {
            val = chrxdigits[ch>>4][y] << 4 | chrxdigits[ch & 0x0F][y];
        }
        mapper_->WriteChrBank(bank, addr+y, val);
        mapper_->WriteChrBank(bank, addr+y+8, val);
    }
}

void ChrUtil::Copy(int dbank, uint8_t dst, int sbank, uint8_t src) {
    uint16_t daddr = 16 * dst;
    uint16_t saddr = 16 * src;
    for(uint16_t y=0; y<16; y++) {
        mapper_->WriteChrBank(dbank, daddr+y,
                mapper_->ReadChrBank(sbank, saddr+y));
    }
}

void ChrUtil::Swap(int dbank, uint8_t dst, int sbank, uint8_t src) {
    uint16_t daddr = 16 * dst;
    uint16_t saddr = 16 * src;
    for(uint16_t y=0; y<16; y++) {
        uint8_t d = mapper_->ReadChrBank(dbank, daddr+y);
        uint8_t s = mapper_->ReadChrBank(sbank, saddr+y);
        mapper_->WriteChrBank(dbank, daddr+y, s);
        mapper_->WriteChrBank(sbank, saddr+y, d);
    }
}


}  // namespace
