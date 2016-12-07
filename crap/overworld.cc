#include <cstdio>
#include "overworld.h"

#include "romfile.h"
#include "util/string.h"

//const char kTranslationTable[] = "0123456789ABCDEF";
const char kTranslationTable[] = "T^P=:.t#+ %Mw~o*";

void Overworld::Decompress(uint32_t start, uint32_t end) {
    for(uint32_t i=start; i<=end; i++) {
        uint8_t v = rom_->Read8(i);
        if (v == 0xff)
            break;
        map_.append((v>>4)+1, kTranslationTable[v & 0xF]);
    }
}

void Overworld::Print() {
    string::size_type i;
    for(i=0; i<map_.size(); i++) {
        if (i && i % 64 == 0)
            fputc('\n', stdout);
        fputc(map_.at(i), stdout);
    }
    fputc('\n', stdout);
}

void Overworld::PrintAreas(uint32_t start) {
    uint32_t i;

        printf("Area XPOS YPOS Map World Ext 2nd Lower Enter Right Thru Hole\n"
               "---- ---- ---- --- ----- --- --- ----- ----- ----- ---- ----\n");
    for(i=0; i<63; i++) {
        uint8_t ypos = rom_->Read8(start + i);
        uint8_t xpos = rom_->Read8(start + i + 0x3f);
        uint8_t map = rom_->Read8(start + i + 0x7e);
        uint8_t world = rom_->Read8(start + i + 0xbd);

        // Extract various flags bits
        bool ext = !!(ypos & 0x80);
        bool second = !!(xpos & 0x40);
        bool lower = !!(xpos & 0x80);
        uint32_t enter = (map >> 6) * 256;
        bool right = !!(world & 0x20);
        bool thru = !!(world & 0x40);
        bool hole = !!(world & 0x80);

        // Mask off flags bits since we've extracted them
        ypos &= 0x7f;
        xpos &= 0x3f;
        map &= 0x3f;
        world &= 0x1f;

        printf("%4d %4d %4d %3d %5d %3c %3c %5c %5d %5c %4c %4c\n",
               i, xpos, ypos, map, world,
               ext ? 'X' : ' ',
               second ? 'X' : ' ',
               lower ? 'X' : ' ',
               enter,
               right ? 'X' : ' ',
               thru ? 'X' : ' ',
               hole ? 'X' : ' ');
    }
}
