#include "imwidget/rom_memory.h"

#include "proto/rominfo.pb.h"
#include "imwidget/imapp.h"
#include "imwidget/imutil.h"
#include "util/config.h"
#include "util/logging.h"
#include <gflags/gflags.h>

DEFINE_bool(repack_erase_only, false, "Erase only during repack");
DEFINE_bool(free_abandoned_regions, false,
            "Erase regions that appear to be abandonded");

namespace z2util {
#define ALLOC_TOKEN     0x0CA1

void RomMemory::Init() {}

void RomMemory::FillFreeSpace(uint32_t color) {
    Address addr;
    addr.set_bank(bank_);
    addr.set_address(0x8000);
    int len = 16384;
    int freespace = 0;
    int j;
    for(int i=0; i<len; ++i) {
        int val = mapper_->Read(addr, i);
        if (val == 255) {
            if (!freespace) {
                // We only consider it freespace if its >= 8 bytes of 0xFF.
                for(j=0; j<8 && i+j<len; ++j) {
                    if (mapper_->Read(addr, i+j) != 255) {
                        break;
                    }
                }
                if (j == 8) {
                    freespace = 1;
                }
            }
            if (freespace) {
                viz_.SetPixel(i%128, i/128, color);
            }
        } else {
            freespace = 0;
        }
    }
}

void RomMemory::FillKeepout(uint32_t color) {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    for(const auto ko : ri.misc().allocator_keepout()) {
        if (ko.bank() == bank_) {
            for(int i=0; i<ko.length(); ++i) {
                int pix = (ko.address() + i) & 0x3FFF;
                viz_.SetPixel(pix % 128, pix/128, color);
            }
        }
    }
}

int RomMemory::FillSideview(const Address& addr, uint32_t color) {
    if (addr.address() == 0 || addr.address() == 0xFFFF)
        return 0;

    int len = mapper_->Read(addr, 0);
    if (len == 255) len = 0;
    for(int i=0; i<len; ++i) {
        int pix = (addr.address() + i) & 0x3FFF;
        viz_.SetPixel(pix % 128, pix/128, color);
    }
    return len;
}

int RomMemory::GetOverworldLength(const Address& addr) {
    if (addr.address() == 0 || addr.address() == 0xFFFF)
        return 0;

    int len = mapper_->IsAlloc(addr);
    if (len == 0) {
        const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
        for(const auto ov : misc.vanilla_overworld()) {
            if (ov.bank() == addr.bank() && ov.address() == addr.address()) {
                len = ov.length();
                break;
            }
        }
    }
    return len;
}

int RomMemory::FillOverworld(const Address& addr, uint32_t color) {
    int len = GetOverworldLength(addr);
    if (len == 0) {
        LOGF(ERROR, "Can't determine length of overworld at "
                "{bank: %d address: 0x%04x}", addr.bank(), addr.address());
    }
    for(int i=0; i<len; ++i) {
        int pix = (addr.address() + i) & 0x3FFF;
        viz_.SetPixel(pix % 128, pix/128, color);
    }
    return len;
}

int RomMemory::FillAllocRegions() {
    Address addr;
    addr.set_bank(bank_);
    addr.set_address(0x8000);
    int len = 16384;
    int totallen = 0;
    for(int i=0; i<len; ++i) {
        int val = mapper_->ReadWord(addr, i);
        if (val == ALLOC_TOKEN) {
            int regionlen = mapper_->ReadWord(addr, i+2);
            totallen += regionlen;
            uint32_t color = 0xFFFF00FF;
            for(int j=0; i<len && j<regionlen+4; ++j, ++i) {
                viz_.SetPixel(i%128, i/128, color);
            }
            --i;
        }
    }
    // Length of allocations minus overhead
    return len;
}

void RomMemory::ProcessOverworlds() {
    Address ovptr;
    ovptr.set_bank(bank_);
    ovptr.set_address(0x8508);
    int len1=0, len2=0;

    Address ov1 = mapper_->ReadAddr(ovptr, 0);
    if (ov1.address() && ov1.address() != 0xFFFF) {
        len1 = FillOverworld(ov1, 0xFF808080);
    }
    Address ov2 = mapper_->ReadAddr(ovptr, 2);
    if (ov2.address() && ov2.address() != 0xFFFF) {
        len2 = FillOverworld(ov2, 0xFFCCCCCC);
    }
    ImGui::Text("Overworlds: $%04x $%04x", ov1.address(), ov2.address());
    ImGui::Text("Lengths:     %4d  %4d\n\n", len1, len2);
}

void RomMemory::ProcessSideview() {
    std::map<int, int> usage;
    int leng=0, lena=0, lenb=0;
    char bufa[64*8], bufb[64*8], bufg[64*2];
    int ofsa=0, ofsb=0, ofsg=0;

    Address baseg;
    baseg.set_bank(bank_);
    baseg.set_address(0x8000);
    for(int room=0; room < 7; ++room) {
        Address g = mapper_->ReadAddr(baseg, room*2);
        usage[g.address()] = FillSideview(g, 0xFF0000FF + ((room*2) << 16));
        ofsg += sprintf(bufg+ofsg, "$%04x ", g.address());
    }
    for(const auto& u : usage) {
        leng += u.second;
    }

    Address basea, baseb;
    basea.set_bank(bank_);
    basea.set_address(0x8523);
    baseb.set_bank(bank_);
    baseb.set_address(0xa000);

    for(int room=0; room < 63; ++room) {
        Address a = mapper_->ReadAddr(basea, room*2);
        usage[a.address()] = FillSideview(a, 0xFFFF0000 + room*2);
        if (room && room % 8 == 0)
            ofsa += sprintf(bufa+ofsa, "\n");
        ofsa += sprintf(bufa+ofsa, "$%04x ", a.address());
    }
    for(const auto& u : usage) {
        lena += u.second;
    }

    ImGui::Text("BG Maps:\n%s\n", bufg);
    ImGui::Text("Rooms A:\n%s\n", bufa);
    if (!(bank_==3 || bank_==5)) {
        for(int room=0; room < 63; ++room) {
            Address b = mapper_->ReadAddr(baseb, room*2);
            usage[b.address()] = FillSideview(b, 0xFF00FF00 + room*2);
            if (room && room % 8 == 0)
                ofsb += sprintf(bufb+ofsb, "\n");
            ofsb += sprintf(bufb+ofsb, "$%04x ", b.address());
        }
        for(const auto& u : usage) {
            lenb += u.second;
        }
        ImGui::Text("Rooms B:\n%s\n", bufb);
    }
    if (!(bank_==3 || bank_==5)) {
        ImGui::Text("Lengths: BG=%d A=%d B=%d",
                leng, lena-leng, lenb-(lena+leng));
    } else {
        ImGui::Text("Lengths: BG=%d A=%d", leng, lena-leng);
    }
}

RomMemory::RomData RomMemory::ReadLevel(const Address& addr) {
    RomData rd{};
    int len = mapper_->Read(addr, 0);
    for(int i=0; i<len; i++) {
        rd.data.emplace_back(mapper_->Read(addr, i));
        mapper_->Write(addr, i, 0xff);
    }
    mapper_->Free(addr);
    return rd;
}

RomMemory::RomData RomMemory::ReadOverworld(const Address& addr) {
    RomData rd{};
    int len = GetOverworldLength(addr);
    for(int i=0; i<len; i++) {
        rd.data.emplace_back(mapper_->Read(addr, i));
        mapper_->Write(addr, i, 0xff);
    }
    if (len) {
        mapper_->Free(addr);
    }
    return rd;
}

void RomMemory::FreeAllocRegions() {
    Address addr;
    addr.set_bank(bank_);
    addr.set_address(0x8000);
    int len = 16384;
    int totallen = 0;
    int j;
    for(int i=0; i<len; ++i) {
        int val = mapper_->ReadWord(addr, i);
        if (val == ALLOC_TOKEN) {
            int regionlen = mapper_->ReadWord(addr, i+2);

            // Check if the region appears to already be empty (all FF).
            for(j=0; j<regionlen && (j+i+4) < len; ++j) {
                if (mapper_->Read(addr, i+j+4) != 255)
                    break;
            }

            // If the region was empty, free it unconditionally.
            // Otherwise, only free it if the flag is set.
            Address f;
            f.set_bank(bank_);
            f.set_address(addr.address() + i + 4);
            if (j == regionlen) {
                LOGF(INFO, "Freeing erased region at { bank: %d address: 0x%04x }",
                        f.bank(), f.address());
                mapper_->Free(f);
            } else if (FLAGS_free_abandoned_regions) {
                LOGF(INFO, "Freeing abandoned region at { bank: %d address: 0x%04x }",
                        f.bank(), f.address());
                mapper_->Free(f);
            }
        }
    }
}

void RomMemory::WriteRomData(const RomData& rd) {
    Address addr;
    addr.set_bank(bank_);
    addr.set_address(rd.address);
    for(size_t i=0; i<rd.data.size(); ++i) {
        mapper_->Write(addr, i, rd.data[i]);
    }
}

bool RomMemory::PlaceMap(std::vector<Region>* regions, RomData* map) {
    if (regions) {
        for(auto& region: *regions) {
            if (region.offset + map->data.size() < region.length) {
                map->address = region.address + region.offset;
                WriteRomData(*map);
                region.offset += map->data.size();
                return true;
            }
        }
    }

    Address a;
    a.set_bank(bank_);
    a = mapper_->Alloc(a, map->data.size());
    if (a.address()) {
        map->address = a.address();
        WriteRomData(*map);
        return true;
    }
    return false;
}

void RomMemory::FixPointers(uint16_t addr, const RomData& rd) {
    Address base;
    base.set_bank(bank_);
    base.set_address(0x8000);
    for(int room=0; room<7; room++) {
        Address a = mapper_->ReadAddr(base, room*2);
        if (a.address() == addr) {
            mapper_->WriteWord(base, room*2, rd.address);
        }
    }
    base.set_address(0x8523);
    for(int room=0; room<63; room++) {
        Address a = mapper_->ReadAddr(base, room*2);
        if (a.address() == addr) {
            a.set_address(rd.address);
            mapper_->WriteWord(base, room*2, rd.address);
        }
    }
    if (!(bank_==3 || bank_==5)) {
        base.set_address(0xa000);
        for(int room=0; room<63; room++) {
            Address a = mapper_->ReadAddr(base, room*2);
            if (a.address() == addr) {
                mapper_->WriteWord(base, room*2, rd.address);
            }
        }
    }
}

bool RomMemory::Repack() {
    Address base;
    std::map<uint16_t, RomData> maps;

    char buf[64];
    sprintf(buf, "Before repack maps in bank %d", bank_);
    ImApp::Get()->ProcessMessage("commit", buf);

    // Read all maps into memory and erase them from the ROM.
    base.set_bank(bank_);
    base.set_address(0x8000);
    for(int room=0; room<7; room++) {
        Address a = mapper_->ReadAddr(base, room*2);
        if (a.address() == 0 || a.address() == 0xFFFF)
            continue;
        if (maps.find(a.address()) == maps.end())
            maps[a.address()] = ReadLevel(a);
    }
    base.set_address(0x8523);
    for(int room=0; room<63; room++) {
        Address a = mapper_->ReadAddr(base, room*2);
        if (a.address() == 0 || a.address() == 0xFFFF)
            continue;
        if (maps.find(a.address()) == maps.end())
            maps[a.address()] = ReadLevel(a);
    }
    if (!(bank_==3 || bank_==5)) {
        base.set_address(0xa000);
        for(int room=0; room<63; room++) {
            Address a = mapper_->ReadAddr(base, room*2);
            if (a.address() == 0 || a.address() == 0xFFFF)
                continue;
            if (maps.find(a.address()) == maps.end())
                maps[a.address()] = ReadLevel(a);
        }
    }

    base.set_address(0x8508);
    RomData ov1 = ReadOverworld(mapper_->ReadAddr(base, 0));
    RomData ov2 = ReadOverworld(mapper_->ReadAddr(base, 2));

    // Everything should be erased, so check for abandoned regions.
    FreeAllocRegions();

    // Set up buffers for the static areas in the rom.
    std::vector<Region> regions;
    const auto& misc = ConfigLoader<RomInfo>::GetConfig().misc();
    for(const auto& r : misc.static_regions()) {
        if (r.bank() == bank_) {
            regions.emplace_back(
                    Region{uint16_t(r.address()), 0, uint16_t(r.length())});
        }
    }

    if (FLAGS_repack_erase_only)
        return true;

    // Place all the sideview maps first, trying to pack as much as possible
    // into the static regions.
    for(auto& m : maps) {
        PlaceMap(&regions, &m.second);
        FixPointers(m.first, m.second);
    }

    // Now place the overworld maps.  We always delegate this to the frespace
    // allocator so the maps will carry their length in the allocator metadata.
    if (ov1.data.size()) {
        PlaceMap(nullptr, &ov1);
        mapper_->WriteWord(base, 0, ov1.address);
    }
    if (ov2.data.size()) {
        PlaceMap(nullptr, &ov2);
        mapper_->WriteWord(base, 2, ov2.address);
    }


    return true;
}

bool RomMemory::Draw() {
    if (!visible_) return false;

    ImGui::Begin("Rom Memory", &visible_);
    ImGui::PushItemWidth(100);
    if (ImGui::InputInt("Bank", &bank_)) {
        Clamp(&bank_, 1, 5);
        refresh_ = true;
    }
    ImGui::SameLine();
    ImGui::InputInt("Scale", &scale_);
    if (ImGui::Button("Re-Pack maps")) {
        Repack();
    }
    ImGui::PopItemWidth();
    ImGui::Separator();

    ImGui::Columns(2, "Memory", true);
    viz_.FilledBox(0, 0, 128, 128, 0xFF000000);
    FillFreeSpace(0xFF202020);
    FillKeepout(0xFF008AFF);

    ProcessOverworlds();
    ProcessSideview();

    FillAllocRegions();
    if (refresh_) {
        viz_.Update();
    }

    ImGui::NextColumn();

    ImGui::Text("Legend:\n\n");
    ImGui::TextColored(ImColor(0xFF000000), "Black: Game code & data");
    ImGui::TextColored(ImColor(0xFF000000), "Dark Gray: Potential freespace");
    ImGui::TextColored(ImColor(0xFF0000FF), "Red: Background maps");
    ImGui::TextColored(ImColor(0xFFFF0000), "Blue: Type A maps");
    ImGui::TextColored(ImColor(0xFF00FF00), "Green: Type B maps");
    ImGui::TextColored(ImColor(0xFFCCCCCC), "White: Vanilla overworlds");
    ImGui::TextColored(ImColor(0xFFFF00FF), "Magenta: Allocated freespace");
    ImGui::TextColored(ImColor(0xFF008AFF), "Orange: Keepout regions");

    ImGui::Columns(1);
    ImGui::Separator();
    ImVec2 cursor = ImGui::GetCursorScreenPos();
    viz_.Draw(128*scale_, 128*scale_);
    if (ImGui::IsItemHovered()) {
        ImVec2 p = (ImGui::GetIO().MousePos - cursor) / scale_;
        Address a;
        a.set_bank(bank_);
        a.set_address(0x8000 + int(p.y)*128 + p.x);
        uint8_t v = mapper_->Read(a, 0);
        ImGui::SetTooltip("$%04x: %02x", a.address(), v);
    }
    ImGui::End();
    return false;
}

}  // namespace z2util
