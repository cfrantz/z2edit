#include <cstdio>
#include <cstdlib>
#include <cstdint>

#include "nes/cartridge.h"

Cartridge::Cartridge()
    : prg_(nullptr), prglen_(0),
    chr_(nullptr), chrlen_(0),
    trainer_(nullptr) { }

Cartridge::~Cartridge() { }

void Cartridge::LoadFile(const std::string& filename) {
    FILE* fp = fopen(filename.c_str(), "rb");

    if (fp == nullptr) {
        fprintf(stderr, "Couldn't read %s.\n", filename.c_str());
        abort();
    }
    if (fread(&header_, sizeof(header_), 1, fp) != 1) {
        fprintf(stderr, "Couldn't read header.\n");
        abort();
    }

    mirror_ = MirrorMode(header_.mirror0 | (header_.mirror1 << 1));
    prglen_ = 16384 * header_.prgsz;
    prg_.reset(new uint8_t[prglen_]);

    if (header_.chrsz) {
        chrlen_ = 8192 * header_.chrsz;
    } else {
        // No chr rom, need and 8k buffer (ram?)
        chrlen_ = 8192;
    }
    chr_.reset(new uint8_t[chrlen_]);

    if (header_.trainer) {
        trainer_.reset(new uint8_t[512]);
        if (fread(trainer_.get(), 1, 512, fp) != 512) {
            fprintf(stderr, "Couldn't read trainer.\n");
            abort();
        }
    }

    if (fread(prg_.get(), 16384, header_.prgsz, fp) != header_.prgsz) {
        fprintf(stderr, "Couldn't read PRG.\n");
        abort();
    }

    if (fread(chr_.get(), 8192, header_.chrsz, fp) != header_.chrsz) {
        fprintf(stderr, "Couldn't read CHR.\n");
        abort();
    }
    fclose(fp);
}

void Cartridge::SaveFile(const std::string& filename) {
    FILE* fp = fopen(filename.c_str(), "wb");
    if (fp == nullptr) {
        fprintf(stderr, "Couldn't open %s.\n", filename.c_str());
        return;
    }
    if (fwrite(&header_, sizeof(header_), 1, fp) != 1) {
        fprintf(stderr, "Couldn't write header.\n");
        return;
    }

    if (header_.trainer) {
        if (fwrite(trainer_.get(), 1, 512, fp) != 512) {
            fprintf(stderr, "Couldn't write trainer.\n");
            return;
        }
    }
    if (fwrite(prg_.get(), 16384, header_.prgsz, fp) != header_.prgsz) {
        fprintf(stderr, "Couldn't read PRG.\n");
        return;
    }

    if (fwrite(chr_.get(), 8192, header_.chrsz, fp) != header_.chrsz) {
        fprintf(stderr, "Couldn't read CHR.\n");
        return;
    }
    fclose(fp);
}

void Cartridge::LoadFile(DebugConsole* console, int argc, char **argv) {
    if (argc != 2) {
        console->AddLog("[error] Usage: %s [filename]", argv[0]);
        return;
    }
    LoadFile(argv[1]);
}

void Cartridge::SaveFile(DebugConsole* console, int argc, char **argv) {
    if (argc != 2) {
        console->AddLog("[error] Usage: %s [filename]", argv[0]);
        return;
    }
    SaveFile(argv[1]);
}

void Cartridge::PrintHeader(DebugConsole* console, int argc, char **argv) {
    uint8_t *bytes = (uint8_t*)&header_;
    console->AddLog("NES header:\n");
    console->AddLog("  %02x %02x %02x %02x %02x %02x %02x %02x\n",
                    bytes[0], bytes[1], bytes[2], bytes[3],
                    bytes[4], bytes[5], bytes[6], bytes[7]);
    bytes += 8;
    console->AddLog("  %02x %02x %02x %02x %02x %02x %02x %02x\n",
                    bytes[0], bytes[1], bytes[2], bytes[3],
                    bytes[4], bytes[5], bytes[6], bytes[7]);

    console->AddLog("  PRG banks: %d\n", header_.prgsz);
    console->AddLog("  CHR banks: %d\n", header_.chrsz);
    console->AddLog("  Mirroring: %d\n", mirror_);
    console->AddLog("  Has SRAM:  %d\n", header_.sram);
    console->AddLog("  Has trainer: %d\n", header_.trainer);
    console->AddLog("  Mapper: %d\n", mapper());
}

void Cartridge::InsertPrg(int bank, uint8_t *data) {
    uint8_t *newprg = new uint8_t[prglen_ + 16384];
    std::unique_ptr<uint8_t[]> data2;
    if (data == nullptr) {
        data = new uint8_t[16384]();
        data2.reset(data);
    }

    memcpy(newprg, prg_.get(), bank * 16384);
    memcpy(newprg + bank*16384, data, 16384);
    memcpy(newprg + (bank+1)*16384, prg_.get()+(bank*16384),
           (header_.prgsz-bank) * 16384);

    prglen_ += 16384;
    header_.prgsz++;
    prg_.reset(newprg);
}

void Cartridge::InsertChr(int bank, uint8_t *data) {
    uint8_t *newchr = new uint8_t[chrlen_ + 8192];
    std::unique_ptr<uint8_t[]> data2;
    if (data == nullptr) {
        data = new uint8_t[8192]();
        data2.reset(data);
    }

    memcpy(newchr, chr_.get(), bank * 8192);
    memcpy(newchr + bank*8192, data, 8192);
    memcpy(newchr + (bank+1)*8192, chr_.get()+(bank*8192),
           (header_.chrsz-bank) * 8192);

    chrlen_ += 8192;
    header_.chrsz++;
    chr_.reset(newchr);
}
