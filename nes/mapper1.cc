#include "nes/mapper1.h"
#include "nes/cartridge.h"

Mapper1::Mapper1(Cartridge* cart)
    : Mapper(cart),
    shift_register_(0x10),
    control_(0),
    prg_mode_(0), chr_mode_(0),
    prg_bank_(0), chr_bank0_(0), chr_bank1_(0),
    prg_offset_{0, 0}, chr_offset_{0, 0} {
        prg_offset_[1] = PrgBankOffset(-1);
}

uint8_t Mapper1::Read(uint16_t addr) {
    if (addr < 0x2000) {
        int bank = addr / 0x1000;
        int offset = addr % 0x1000;
        //printf("mapper1: %d -> %d\n", addr, chr_offset_[bank] + offset);
        return cartridge()->ReadChr(chr_offset_[bank] + offset);
    } else if (addr >= 0x6000 && addr < 0x8000) {
        fprintf(stderr, "No SRAM\n");
    } else if (addr >= 0x8000) {
        addr -= 0x8000;
        int bank = addr / 0x4000;
        int offset = addr % 0x4000;
        return cartridge()->ReadPrg(prg_offset_[bank] + offset);
    } else {
        fprintf(stderr, "Unhandled mapper1 read at %04x\n", addr);
    }
    return 0;
}

void Mapper1::Write(uint16_t addr, uint8_t val) {
    if (addr < 0x2000) {
        int bank = addr / 0x1000;
        int offset = addr % 0x1000;
        return cartridge()->WriteChr(chr_offset_[bank] + offset, val);
    } else if (addr >= 0x6000 && addr < 0x8000) {
        fprintf(stderr, "No SRAM\n");
    } else if (addr >= 0x8000) {
        addr -= 0x8000;
        int bank = addr / 0x4000;
        int offset = addr % 0x4000;
        return cartridge()->WritePrg(prg_offset_[bank] + offset, val);
    } else {
        fprintf(stderr, "Unhandled mapper1 write at %04x\n", addr);
    }
}

void Mapper1::DebugWriteReg(DebugConsole* console, int argc, char **argv) {
    if (argc != 3) {
        console->AddLog("[error] Usage %s <reg-or-offset> <value>", argv[0]);
        console->AddLog("[error] Register Names:");
        console->AddLog("[error]     prg_mode, chr_mode, prg_bank, chr_bank0, chr_bank1");
        console->AddLog("prg_mode=%02x prg_bank=%02x", prg_mode_, prg_bank_);
        console->AddLog("# prg offsets = 0x%04x 0x%04x",
                        prg_offset_[0], prg_offset_[1]);
        console->AddLog("chr_mode=%02x chr_bank0=%02x chr_bank1=%02x",
                        chr_mode_, chr_bank0_, chr_bank1_);
        console->AddLog("# chr offsets = 0x%04x 0x%04x",
                        chr_offset_[0], chr_offset_[1]);
        return;
    }
    uint8_t val = strtoul(argv[2], 0, 0);
    if (!strcmp(argv[1], "prg_mode")) {
        prg_mode_ = val;
    } else if (!strcmp(argv[1], "chr_mode")) {
        chr_mode_ = val;
    } else if (!strcmp(argv[1], "prg_bank")) {
        prg_bank_ = val;
    } else if (!strcmp(argv[1], "chr_bank0")) {
        chr_bank0_ = val;
    } else if (!strcmp(argv[1], "chr_bank1")) {
        chr_bank1_ = val;
    } else {
        uint16_t addr = strtoul(argv[1], 0, 0);
        if (addr >= 0x8000) {
            LoadRegister(addr, val);
        } else {
            console->AddLog("[error] Bad name or offset: %s", argv[1]);
            return;
        }
    }
    UpdateOffsets();
}

int Mapper1::PrgBankOffset(int index) {
    if (index >= 0x80)
        index -= 0x100;
    index = index % (cartridge()->prglen() / 0x4000);
    int result = index * 0x4000;
    if (result < 0)
        result += cartridge()->prglen();
    return result;
}

int Mapper1::ChrBankOffset(int index) {
    if (index >= 0x80)
        index -= 0x100;
    index = index % (cartridge()->chrlen() / 0x1000);
    int result = index * 0x1000;
    if (result < 0)
        result += cartridge()->chrlen();
    return result;
}

void Mapper1::WriteControl(uint8_t val) {
    control_ = val;
    chr_mode_ = (val >> 4) & 1;
    prg_mode_ = (val >> 2) & 3;
    switch(val & 3) {
        case 0: cartridge()->set_mirror(Cartridge::SINGLE0); break;
        case 1: cartridge()->set_mirror(Cartridge::SINGLE1); break;
        case 2: cartridge()->set_mirror(Cartridge::VERTICAL); break;
        case 3: cartridge()->set_mirror(Cartridge::HORIZONTAL); break;
    }
}

void Mapper1::UpdateOffsets() {
    switch (prg_mode_) {
    case 0:
    case 1:
        prg_offset_[0] = PrgBankOffset(prg_bank_ & 0xFE);
        prg_offset_[1] = PrgBankOffset(prg_bank_ | 0x01);
        break;
    case 2:
        prg_offset_[0] = 0;
        prg_offset_[1] = PrgBankOffset(prg_bank_);
        break;
    case 3:
        prg_offset_[0] = PrgBankOffset(prg_bank_);
        prg_offset_[1] = PrgBankOffset(-1);
        break;
    }

    switch (chr_mode_) {
    case 0:
        chr_offset_[0] = ChrBankOffset(chr_bank0_ & 0xFE);
        chr_offset_[1] = ChrBankOffset(chr_bank0_ | 0x01);
        break;
    case 1:
        chr_offset_[0] = ChrBankOffset(chr_bank0_);
        chr_offset_[1] = ChrBankOffset(chr_bank1_);
        break;
    }
}

void Mapper1::WriteRegister(uint16_t addr, uint8_t val) {
    if (addr < 0xA000) {
        // addr in [0x0000..0x9FFF]
        WriteControl(val);
        //printf("control = %02x\n", val);
    } else if (addr < 0xC000) {
        // addr in [0xA000..0xBFFF]
        chr_bank0_ = val;
        //printf("chrbank0 = %02x\n", val);
    } else if (addr < 0xE000) {
        // addr in [0xC000..0xDFFF]
        chr_bank1_ = val;
        //printf("chrbank1 = %02x\n", val);
    } else {
        // addr in [0xE000..0xFFFF]
        prg_bank_ = val & 0x0F;
        //printf("prgbank = %02x\n", val);
    }
    UpdateOffsets();
}

void Mapper1::LoadRegister(uint16_t addr, uint8_t val) {
    if (val & 0x80) {
        shift_register_ = 0x10;
        WriteControl(control_ & 0x0C);
        UpdateOffsets();
    } else {
        int complete = shift_register_ & 0x01;
        shift_register_ = (shift_register_ >> 1) | ((val & 0x01) << 4);
        if (complete) {
            WriteRegister(addr, shift_register_);
            shift_register_ = 0x10;
        }
    }
}

REGISTER_MAPPER(1, Mapper1);
REGISTER_MAPPER(5, Mapper1);
