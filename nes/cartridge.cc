#include <cstdio>
#include <cstdlib>
#include <cstdint>

#include "nes/cartridge.h"
#include "util/file.h"

Cartridge::Cartridge()
    : prg_(nullptr), prglen_(0),
    chr_(nullptr), chrlen_(0),
    trainer_(nullptr) { }

Cartridge::Cartridge(const Cartridge& orig)
  : header_(orig.header_),
    prglen_(orig.prglen_),
    chrlen_(orig.chrlen_),
    mirror_(orig.mirror_) {
    if (header_.trainer) {
        trainer_.reset(new uint8_t[512]);
        memcpy(trainer_.get(), orig.trainer_.get(), 512);
    }
    prg_.reset(new uint8_t[prglen_]);
    memcpy(prg_.get(), orig.prg_.get(), prglen_);
    chr_.reset(new uint8_t[chrlen_]);
    memcpy(chr_.get(), orig.chr_.get(), chrlen_);
}

Cartridge::~Cartridge() { }

bool Cartridge::IsNESFile(const std::string& filename) {
    std::string buf;
    auto f = File::Open(filename, "rb");
    if (f && f->Read(&buf, 4)) {
        return (buf[0] == 'N' &&
                buf[1] == 'E' &&
                buf[2] == 'S' &&
                buf[3] == '\x1a');
    }
    return false;
}

void Cartridge::LoadRom(const std::string& rom) {
    const char *data = rom.data();
    unsigned offset = 0;
    unsigned size = rom.size();

    // Read the header
    memcpy(&header_, data, sizeof(header_));
    offset += sizeof(header_);

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
        memcpy(trainer_.get(), data + offset, 512);
        offset += 512;
    }

    if (offset + 16384 * header_.prgsz > size) {
        fprintf(stderr, "Couldn't read PRG.\n");
        abort();
    }
    memcpy(prg_.get(), data + offset, 16384 * header_.prgsz);
    offset += 16384 * header_.prgsz;

    if (offset + 8192 * header_.chrsz > size) {
        fprintf(stderr, "Couldn't read CHR.\n");
        abort();
    }
    memcpy(chr_.get(), data + offset, 8192 * header_.chrsz);
    offset += 8192 * header_.chrsz;
}

std::string Cartridge::SaveRom() {
    std::string rom;
    rom.append((const char*)(&header_), sizeof(header_));
    if (header_.trainer) {
        rom.append((const char*)(trainer_.get()), 512);
    }
    rom.append((const char*)(prg_.get()), 16384 * header_.prgsz);
    rom.append((const char*)(chr_.get()), 8192 * header_.chrsz);
    return rom;
}

void Cartridge::LoadFile(const std::string& filename) {
    std::string rom;
    File::GetContents(filename, &rom);
    LoadRom(rom);
}

void Cartridge::SaveFile(const std::string& filename) {
    File::SetContents(filename, SaveRom());
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
