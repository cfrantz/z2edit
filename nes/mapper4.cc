#include <cstdint>
#include "src/nes/mapper.h"
#include "src/nes/cartridge.h"
#include "src/nes/ppu.h"


class Mapper4: public Mapper {
  public:
    Mapper4(NES* nes);
    uint8_t Read(uint16_t addr) override;
    void Write(uint16_t addr, uint8_t val) override;
    void Emulate() override;

  private:
    int PrgBankOffset(int index);
    int ChrBankOffset(int index);
    void WriteBankSelect(uint8_t val);
    void WriteMirror(uint8_t val);
    void WriteRegister(uint16_t addr, uint8_t val);
    void UpdateOffsets();

    bool irqen_;
    uint8_t register_;
    uint8_t reload_;
    uint8_t counter_;
    uint8_t prg_mode_;
    uint8_t chr_mode_;
    uint8_t registers_[8];
    int prg_offset_[4];
    int chr_offset_[8];
};

Mapper4::Mapper4(NES* nes)
    : Mapper(nes),
    irqen_(false),
    register_(0),
    reload_(0),
    counter_(0),
    prg_mode_(0), chr_mode_(0),
    registers_{0,},
    prg_offset_{0,}, chr_offset_{0,} {
        prg_offset_[0] = PrgBankOffset(0);
        prg_offset_[1] = PrgBankOffset(1);
        prg_offset_[2] = PrgBankOffset(-2);
        prg_offset_[3] = PrgBankOffset(-1);
}

uint8_t Mapper4::Read(uint16_t addr) {
    if (addr < 0x2000) {
        int bank = addr / 0x400;
        int offset = addr % 0x400;
        return nes_->cartridge()->ReadChr(chr_offset_[bank] + offset);
    } else if (addr >= 0x6000 && addr < 0x8000) {
        return nes_->cartridge()->ReadSram(addr - 0x6000);
    } else if (addr >= 0x8000) {
        addr -= 0x8000;
        int bank = addr / 0x2000;
        int offset = addr % 0x2000;
        return nes_->cartridge()->ReadPrg(prg_offset_[bank] + offset);
    } else {
        fprintf(stderr, "Unhandled mapper4 read at %04x\n", addr);
    }
    return 0;
}

void Mapper4::Write(uint16_t addr, uint8_t val) {
    if (addr < 0x2000) {
        int bank = addr / 0x400;
        int offset = addr % 0x400;
        return nes_->cartridge()->WriteChr(chr_offset_[bank] + offset, val);
    } else if (addr >= 0x6000 && addr < 0x8000) {
        return nes_->cartridge()->WriteSram(addr - 0x6000, val);
    } else if (addr >= 0x8000) {
        WriteRegister(addr, val);
    } else {
        fprintf(stderr, "Unhandled mapper4 write at %04x\n", addr);
    }
}

void Mapper4::Emulate() {
    auto cycle = nes_->ppu()->cycle();
    auto scanline = nes_->ppu()->scanline();
    auto mask = nes_->ppu()->mask();

    // fogleman's NES uses 280 here (and comments that it should be 260)
    // Nimes uses 300.
    if (cycle != 260)
        return;
    if (scanline >= 240 && scanline <= 260)
        return;
    if (!mask.showbg and !mask.showsprites)
        return;

    if (counter_ == 0) {
        counter_ = reload_;
    } else {
        counter_--;
        if (counter_ == 0 && irqen_) {
            nes_->IRQ();
        }
    }
}

int Mapper4::PrgBankOffset(int index) {
    if (index >= 0x80)
        index -= 0x100;
    index = index % (nes_->cartridge()->prglen() / 0x2000);
    int result = index * 0x2000;
    if (result < 0)
        result += nes_->cartridge()->prglen();
    return result;
}

int Mapper4::ChrBankOffset(int index) {
    if (index >= 0x80)
        index -= 0x100;
    index = index % (nes_->cartridge()->chrlen() / 0x0400);
    int result = index * 0x0400;
    if (result < 0)
        result += nes_->cartridge()->chrlen();
    return result;
}

void Mapper4::UpdateOffsets() {
    switch (prg_mode_) {
    case 0:
        prg_offset_[0] = PrgBankOffset(registers_[6]);
        prg_offset_[1] = PrgBankOffset(registers_[7]);
        prg_offset_[2] = PrgBankOffset(-2);
        prg_offset_[3] = PrgBankOffset(-1);
        break;
    case 1:
        prg_offset_[0] = PrgBankOffset(-2);
        prg_offset_[1] = PrgBankOffset(registers_[7]);
        prg_offset_[2] = PrgBankOffset(registers_[6]);
        prg_offset_[3] = PrgBankOffset(-1);
        break;
    }

    switch (chr_mode_) {
    case 0:
        chr_offset_[0] = ChrBankOffset(registers_[0] & 0xFE);
        chr_offset_[1] = ChrBankOffset(registers_[0] | 0x01);
        chr_offset_[2] = ChrBankOffset(registers_[1] & 0xFE);
        chr_offset_[3] = ChrBankOffset(registers_[1] | 0x01);
        chr_offset_[4] = ChrBankOffset(registers_[2]);
        chr_offset_[5] = ChrBankOffset(registers_[3]);
        chr_offset_[6] = ChrBankOffset(registers_[4]);
        chr_offset_[7] = ChrBankOffset(registers_[5]);
        break;
    case 1:
        chr_offset_[0] = ChrBankOffset(registers_[2]);
        chr_offset_[1] = ChrBankOffset(registers_[3]);
        chr_offset_[2] = ChrBankOffset(registers_[4]);
        chr_offset_[3] = ChrBankOffset(registers_[5]);
        chr_offset_[4] = ChrBankOffset(registers_[0] & 0xFE);
        chr_offset_[5] = ChrBankOffset(registers_[0] | 0x01);
        chr_offset_[6] = ChrBankOffset(registers_[1] & 0xFE);
        chr_offset_[7] = ChrBankOffset(registers_[1] | 0x01);
        break;
    }
}

void Mapper4::WriteBankSelect(uint8_t val) {
    prg_mode_ = (val >> 6) & 1;
    chr_mode_ = (val >> 7) & 1;
    register_ = val & 7;
}

void Mapper4::WriteMirror(uint8_t val) {
    switch(val & 1) {
    case 0:
        nes_->cartridge()->set_mirror(Cartridge::MirrorMode::VERTICAL);
        break;
    case 1:
        nes_->cartridge()->set_mirror(Cartridge::MirrorMode::HORIZONTAL);
        break;
    }
}

void Mapper4::WriteRegister(uint16_t addr, uint8_t val) {
    if (addr < 0xA000) {
        if ((addr & 1) == 0)
            WriteBankSelect(val);
        else
            registers_[register_] = val;
        UpdateOffsets();
    } else if (addr < 0xC000) {
        if ((addr & 1) == 0)
            WriteMirror(val);
    } else if (addr < 0xE000) {
        if ((addr & 1) == 0)
            reload_ = val;
        else
            counter_ = 0;
    } else {
        irqen_ = addr & 1;
    }
}

REGISTER_MAPPER(4, Mapper4);
