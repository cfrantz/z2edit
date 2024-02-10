#include "nes/address.h"

#include "absl/log/log.h"
#include "absl/strings/str_cat.h"

namespace nes {
Address Address::File(uint32_t offset) {
    return Address(address::File(offset));
}
Address Address::Prg(int bank, uint16_t offset) {
    return Address(address::Prg(bank, offset));
}
Address Address::Chr(int bank, uint16_t offset) {
    return Address(address::Chr(bank, offset));
}

uint32_t Address::offset(const Layout& layout) const {
    if (std::holds_alternative<address::File>(addr)) {
        auto& file = std::get<address::File>(addr);
        return file.offset;
    } else if (std::holds_alternative<address::Prg>(addr)) {
        auto& prg = std::get<address::Prg>(addr);
        int bank = (prg.bank < 0) ? layout.prg.len + prg.bank : prg.bank;
        if (bank < 0 || bank >= int(layout.prg.len)) {
            LOG(FATAL) << "Invalid prg bank: " << bank;
        }
        uint32_t sz = layout.prg.bank_size;
        return layout.prg.start + bank * sz + prg.offset % sz;

    } else if (std::holds_alternative<address::Chr>(addr)) {
        auto& chr = std::get<address::Chr>(addr);
        int bank = (chr.bank < 0) ? layout.chr.len + chr.bank : chr.bank;
        if (bank < 0 || bank >= int(layout.chr.len)) {
            LOG(FATAL) << "Invalid chr bank: " << bank;
        }
        uint32_t sz = layout.chr.bank_size;
        return layout.chr.start + bank * sz + chr.offset % sz;
    }
    LOG(FATAL) << "Invalid address variant";
}

std::string Address::to_string() const {
    if (std::holds_alternative<address::File>(addr)) {
        auto& file = std::get<address::File>(addr);
        return absl::StrCat("File(", file.offset, ")");
    } else if (std::holds_alternative<address::Prg>(addr)) {
        auto& prg = std::get<address::Prg>(addr);
        return absl::StrCat("Prg(", prg.bank, ", 0x", absl::Hex(prg.offset),
                            ")");
    } else if (std::holds_alternative<address::Chr>(addr)) {
        auto& chr = std::get<address::Chr>(addr);
        return absl::StrCat("Chr(", chr.bank, ", 0x", absl::Hex(chr.offset),
                            ")");
    } else {
        return "AddressUnknown";
    }
}

Address& Address::operator+=(int v) {
    if (std::holds_alternative<address::File>(addr)) {
        auto& file = std::get<address::File>(addr);
        file.offset += v;
    } else if (std::holds_alternative<address::Prg>(addr)) {
        auto& prg = std::get<address::Prg>(addr);
        prg.offset += v;
    } else if (std::holds_alternative<address::Chr>(addr)) {
        auto& chr = std::get<address::Chr>(addr);
        chr.offset += v;
    }
    return *this;
}

Address& Address::operator-=(int v) { return *this += -v; }

}  // namespace nes
