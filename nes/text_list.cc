#include "nes/text_list.h"

#include "nes/mapper.h"
#include "nes/text_encoding.h"
#include "proto/rominfo.pb.h"
#include "util/config.h"
#include "util/logging.h"

namespace z2util {

std::string TextListPack::ReadNesString(const Address& addr) {
    int ch;
    std::string result;
    for(int i=0; ;i++) {
        ch = mapper_->Read(addr, i);
        if (ch == 0xFF)
            break;
        result.push_back(TextEncoding::FromZelda2(ch));

    }
    return result;
}

void TextListPack::WriteNesString(const Address& addr, const std::string& val) {
    unsigned i;
    for(i=0; i<val.size(); i++) {
        mapper_->Write(addr, i, TextEncoding::ToZelda2(val[i]));
    }
    mapper_->Write(addr, i, 0xFF);
}

void TextListPack::ReadOne(int world, int index, Address addr) {
    List entry;
    entry.newaddr = 0;
    entry.data = ReadNesString(addr);
    entry_.insert(std::make_pair(addr.address(), entry));
    index_[world][index] = addr.address();;
}


void TextListPack::Unpack(int bank) {
    const auto& tt = ConfigLoader<RomInfo>::GetConfig().text_table();
    bank_ = bank;

    index_.clear();
    entry_.clear();

    index_.resize(tt.length_size());
    int world = 0;
    for(const auto len : tt.length()) {
        Address table = mapper_->ReadAddr(tt.pointer(), world*2);
        index_[world].resize(len, 0);
        for(int i=0; i<len; ++i) {
            Address addr = mapper_->ReadAddr(table, i*2);
            ReadOne(world, i, addr);
        }
        ++world;
    }
}

void TextListPack::CheckIndex() {
    const auto& tt = ConfigLoader<RomInfo>::GetConfig().text_table();
    int world = -1;
    for(const auto len: tt.length()) {
        ++world;
        const auto& i0 = tt.index(world*2 + 0);
        const auto& i1 = tt.index(world*2 + 1);
        for(int i=0; i < i0.length(); ++i) {
            Address a;
            a.set_bank(i0.bank()); a.set_address(i0.address());
            int value = mapper_->Read(a, i);
            if (value >= len) {
                LOGF(ERROR, "Town %d: Enemy $%02x text1 index is out of bounds (%d)",
                        world*4 + i%4, 10+i/4, value);
            }

            if (i >= i1.length())
                continue;

            a.set_bank(i1.bank()); a.set_address(i1.address());
            value = mapper_->Read(a, i);
            if (value >= len) {
                LOGF(ERROR, "Town %d: Enemy $%02x text2 index is out of bounds (%d)",
                        world*4 + i%4, 10+i/4, value);
            }
        }
    }
}

void TextListPack::ResetAddrs() {
    for(auto& entry : entry_) {
        entry.second.newaddr = 0;
    }
}

bool TextListPack::Pack() {
    const auto& tt = ConfigLoader<RomInfo>::GetConfig().text_table();
    std::vector<uint8_t> packed;
    
    // Pack all of the text into the buffer
    int world = 0;
    for(const auto len : tt.length()) {
        for(int i=0; i<len; ++i) {
            int addr = index_[world][i];
            if (addr == 0)
                continue;

            List& entry = entry_[addr];
            int na = packed.size() + tt.text_data().address();
            LOGF(INFO, "Text w=%d i=%d addr=%04x->%04x '%s'", world, i, addr, na, entry.data.c_str());
            if (entry.newaddr == 0) {
                int newsize = packed.size() + entry.data.size() + 1;
                if (newsize > tt.text_data().length()) {
                    LOGF(ERROR, "Out of space for text list at world=%d index=%d", world, i);
                    LOGF(ERROR, "Want %d bytes, but only %d available.",
                         newsize, tt.text_data().length());
                    ResetAddrs();
                    return false;
                }
                entry.newaddr = packed.size() + tt.text_data().address();
                for(const auto& ch : entry.data) {
                    char zch = TextEncoding::ToZelda2(ch);
                    if (zch == 0) {
                        // Transform any unknown character to a question mark.
                        zch = TextEncoding::ToZelda2('?');
                    }
                    packed.push_back(zch);
                }
                // Terminate the string.
                packed.push_back(0xff);
            }
        }
        ++world;
    }

    // Copy text pointers to ROM.
    world = 0;
    for(const auto len : tt.length()) {
        Address table = mapper_->ReadAddr(tt.pointer(), world*2);
        for(int i=0; i<len; ++i) {
            int addr = index_[world][i];
            if (addr == 0)
                continue;

            List& entry = entry_[addr];
            mapper_->WriteWord(table, i*2, entry.newaddr);
        }
        ++world;
    }

    // And copy the text to the ROM.
    packed.resize(tt.text_data().length(), 0);
    for(size_t i=0; i<packed.size(); i++) {
        mapper_->Write(tt.text_data(), i, packed[i]);
    }
    return true;
}

const std::string& TextListPack::Get(int world, int index) {
    const auto& tt = ConfigLoader<RomInfo>::GetConfig().text_table();
    if (world < tt.length_size() && index < tt.length(world)) {
        uint16_t addr = index_[world][index];
        return entry_[addr].data;
    }
    return "";
}

bool TextListPack::Get(int world, int index, std::string* val) {
    const auto& tt = ConfigLoader<RomInfo>::GetConfig().text_table();
    if (world < tt.length_size() && index < tt.length(world)) {
        uint16_t addr = index_[world][index];
        *val = entry_[addr].data;
        return true;
    }
    return false;
}

bool TextListPack::Set(int world, int index, const std::string& val) {
    const auto& tt = ConfigLoader<RomInfo>::GetConfig().text_table();
    if (world < tt.length_size() && index < tt.length(world)) {
        uint16_t addr = index_[world][index];
        entry_[addr].data = val;
        return true;
    }
    return false;
}

int TextListPack::Length(int world) {
    const auto& tt = ConfigLoader<RomInfo>::GetConfig().text_table();
    if (world < tt.length_size())
        return int(index_[world].size());
    return 0;
}


}  // namespace z2util
