#include <vector>
#include <algorithm>
#include <inttypes.h>
#include "romfile.h"

#include "util/file.h"

RomFile::RomFile() {}

bool RomFile::Load(const string& filename) {
    return File::GetContents(filename, &rom_);
}

void RomFile::Hexdump(uint32_t offset, uint32_t length,
                      uint32_t highlight, uint32_t hilen) const {
    char buf[17];
    uint32_t i;
    uint32_t addr;

    printf("address   ");

    for(i=0; i<16; i++) {
        printf(" %02x", (offset + i) & 15);
    }
    printf("  text\n");
    printf("--------   -----------------------------------------------  ----------------\n");

    for(i=0; i<length; i++) {
        addr = offset + i;
        uint8_t v = Read8(addr);
        if (i % 16 == 0) {
            if (i) {
                printf("  %s\n%08" PRIx32 ": ", buf, addr);
            } else {
                printf("%08" PRIx32 ": ", addr);
            }
            memset(buf, 0, sizeof(buf));
        }
        if (addr >= highlight && addr < highlight+hilen) {
            printf(" \033[1m%02x\033[0m", v);
        } else {
            printf(" %02x", v);
        }
        buf[i%16] = (v >= 32 && v < 127) ? v : '.';
    }

    if (i % 16) {
        i = 3*(16-i%16);
    } else {
        i = 0;
    }
    printf("  %*s%s\n", i, "", buf);
}

void *memmemx(const void *data, int len, const void *pat, int plen) {
    const uint8_t *d = (uint8_t*)data, *p = (uint8_t*)pat;
    int i, m;
    if (len<plen)
        return nullptr;

    for(i=0; i<(len-plen); i++) {
        for(m=0; m<plen; m++) {
            if (p[m] != 0xff && p[m] != d[i+m])
                break;
        }
        if (m == plen)
            return (void*)(intptr_t(d)+i);
    }
    return nullptr;
}


void RomFile::Grep(const std::string& pattern, bool wildcard) {
    const char *data = rom_.c_str();
    uint32_t sz = rom_.size();
    uint8_t pat[128];
    int plen = 0;
    const char *found;
    const char *sp = pattern.c_str();
    char *ep;
    uint32_t total = 0;
    
    for(;;) {
        pat[plen++] = strtoul(sp, &ep, 16);
        if (!*ep)
            break;
        sp = ep+1;
    }

    for(;;) {
        if (wildcard) {
            found = (char*)memmemx(data, sz, pat, plen);
        } else {
            found = (char*)memmem(data, sz, pat, plen);
        }
        if (found) {
            uint32_t offset = found - data;
            uint32_t start = offset-32;
            uint32_t end  = offset + plen + 32;
            if (start < 0) start = 0;
            Hexdump((total + start) & ~15, end-start, total + offset, plen);
            printf("\n");
            
            offset += 1;
            total += offset;
            data += offset;
            sz -= offset;
        } else {
            break;
        }
    }
}

void RomFile::FindFreeSpaceInBank(int bank) {
    int start;
    int end;
    int total = 0;

    printf("## Bank %d\n", bank);
    printf("| Address | Size (bytes) |\n");
    printf("|---|---|\n");
    for(int i=0x8000; i<0xc000; i++) {
        if (Read8(bank, i) == 0xFF) {
            start = i;
            for(;i<0xc000; i++) {
                if (Read8(bank, i) != 0xFF)
                    break;
            }
            end = i;

            if (end-start > 15) {
                printf("| `$%04x` | `%d` |\n", start, end-start);
                total += (end - start);
            }
        }
    }
    printf("| Total | `%d` |\n\n", total);
}

void RomFile::FindFreeSpace() {
    for(int i=0; i<8; i++) {
        FindFreeSpaceInBank(i);
    }
}

void RomFile::ReadEnemyLists(uint8_t bank, bool sort) {
    std::vector<uint16_t> ptr;
    int i;

    for(i=0; i<63; i++)
        ptr.push_back(Read16(bank, 0x85A1 + i*2));
    for(i=0; i<63; i++)
        ptr.push_back(Read16(bank, 0xA07E + i*2));

    if (sort)
        std::sort(ptr.begin(), ptr.end());
    for(i=0; i<126; i++) {
        printf("ptr[%d] = %04x -> %04x\n",
                i, ptr[i], 0x88a0 + (ptr[i] & 0x0FFF));
    }
}
