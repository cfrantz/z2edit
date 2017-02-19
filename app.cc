#include <gflags/gflags.h>
#include "app.h"
#include "imgui.h"
#include "nes/cpu6502.h"
#include "proto/rominfo.pb.h"
#include "util/browser.h"
#include "util/config.h"
#include "util/os.h"
#include "util/logging.h"
#include "util/imgui_impl_sdl.h"

#ifdef HAVE_NFD
#include "nfd.h"
#endif


DEFINE_string(emulator, "fceux", "Emulator to run for testing");
DEFINE_string(romtmp, "/tmp/zelda2-test.nes", "Temporary filename for running under test");
DECLARE_bool(move_from_keepout);
DECLARE_string(config);

namespace z2util {

void Z2Edit::Init() {
    RegisterCommand("load", "Load a NES ROM.", this, &Z2Edit::LoadFile);
    RegisterCommand("save", "Save a NES ROM.", this, &Z2Edit::SaveFile);
    RegisterCommand("wm", "Write mapper register.", this, &Z2Edit::WriteMapper);
    RegisterCommand("header", "Print iNES header.",
            &cartridge_, &Cartridge::PrintHeader);

    RegisterCommand("db", "Hexdump bytes (via mapper).", this, &Z2Edit::HexdumpBytes);
    RegisterCommand("dbp", "Hexdump PRG bytes.", this, &Z2Edit::HexdumpBytes);
    RegisterCommand("dbc", "Hexdump CHR bytes.", this, &Z2Edit::HexdumpBytes);
    RegisterCommand("dw", "Hexdump words (via mapper).", this, &Z2Edit::HexdumpWords);
    RegisterCommand("dwp", "Hexdump PRG words.", this, &Z2Edit::HexdumpWords);
    RegisterCommand("dwc", "Hexdump CHR words.", this, &Z2Edit::HexdumpWords);
    RegisterCommand("wb", "Write bytes (via mapper).", this, &Z2Edit::WriteBytes);
    RegisterCommand("wbp", "Write PRG bytes.", this, &Z2Edit::WriteBytes);
    RegisterCommand("wbc", "Write CHR bytes.", this, &Z2Edit::WriteBytes);
    RegisterCommand("ww", "Write words (via mapper).", this, &Z2Edit::WriteWords);
    RegisterCommand("wwp", "Write PRG words.", this, &Z2Edit::WriteWords);
    RegisterCommand("wwc", "Write CHR words.", this, &Z2Edit::WriteWords);
    RegisterCommand("elist", "Dump Enemy List.", this, &Z2Edit::EnemyList);
    RegisterCommand("u", "Disassemble Code.", this, &Z2Edit::Unassemble);
    RegisterCommand("insertprg", "Insert a PRG bank.", this, &Z2Edit::InsertPrg);
    RegisterCommand("copyprg", "Copy a PRG bank to another bank.", this, &Z2Edit::CopyPrg);
    RegisterCommand("memmove", "Move memory within a PRG bank.", this, &Z2Edit::MemMove);

    loaded_ = false;
    hwpal_ = NesHardwarePalette::Get();
    chrview_.reset(new NesChrView);
    simplemap_.reset(new z2util::SimpleMap);
    misc_hacks_.reset(new z2util::MiscellaneousHacks);
    palace_gfx_.reset(new z2util::PalaceGraphics);
    palette_editor_.reset(new z2util::PaletteEditor);
    start_values_.reset(new z2util::StartValues);
    object_table_.reset(new z2util::ObjectTable);
    enemy_editor_.reset(new z2util::EnemyEditor);
    editor_.reset(z2util::Editor::New());
}

void Z2Edit::Load(const std::string& filename) {
    loaded_ = true;
    cartridge_.LoadFile(filename);
    mapper_.reset(MapperRegistry::New(&cartridge_, cartridge_.mapper()));
    chrview_->set_mapper(mapper_.get());

    simplemap_->set_mapper(mapper_.get());
    auto* ri = ConfigLoader<RomInfo>::MutableConfig();
    for(const auto& m : ri->map()) {
        if (m.name().find("North Palace") != std::string::npos) {
            simplemap_->SetMap(m);
            break;
        }
    }

    editor_->set_mapper(mapper_.get());
    editor_->ConvertFromMap(ri->mutable_map(0));

    misc_hacks_->set_mapper(mapper_.get());
    palace_gfx_->set_mapper(mapper_.get());
    palette_editor_->set_mapper(mapper_.get());
    start_values_->set_mapper(mapper_.get());

    object_table_->set_mapper(mapper_.get());
    enemy_editor_->set_mapper(mapper_.get());

    object_table_->Init();
    palette_editor_->Init();
    enemy_editor_->Init();

    memory_.set_mapper(mapper_.get());
    memory_.CheckBankForKeepout(1);
    memory_.CheckBankForKeepout(2);
    memory_.CheckBankForKeepout(3);
    memory_.CheckBankForKeepout(4);
    memory_.CheckBankForKeepout(5);

    if (FLAGS_move_from_keepout) {
        memory_.CheckBankForKeepout(1, true);
        memory_.CheckBankForKeepout(2, true);
        memory_.CheckBankForKeepout(3, true);
        memory_.CheckBankForKeepout(4, true);
        memory_.CheckBankForKeepout(5, true);
    }
}

void Z2Edit::LoadFile(DebugConsole* console, int argc, char **argv) {
    if (argc != 2) {
        console->AddLog("[error] Usage: %s [filename]", argv[0]);
        return;
    }
    Load(argv[1]);
}

void Z2Edit::WriteMapper(DebugConsole* console, int argc, char **argv) {
    mapper_->DebugWriteReg(console, argc, argv);
}

void Z2Edit::SaveFile(DebugConsole* console, int argc, char **argv) {
    if (argc != 2) {
        console->AddLog("[error] Usage: %s [filename]", argv[0]);
        return;
    }
    cartridge_.SaveFile(argv[1]);
}

void Z2Edit::HexdumpBytes(DebugConsole* console, int argc, char **argv) {
    // The hexdump command will be one of 'db', 'dbp' or 'dbc', standing
    // for 'dump bytes', 'dump bytes prg' and 'dump bytes chr'.  The
    // prg and chr versions can optionally take a bank number.
    uint8_t bank = 0;
    int mode = argv[0][2];
    int index = 0;
    if (argc < 2) {
        console->AddLog("[error] %s: Wrong number of arguments.", argv[0]);
        if (mode) {
            console->AddLog("[error] %s [b=<bank>] <addr> <length>", argv[0]);
        } else {
            console->AddLog("[error] %s <addr> <length>", argv[0]);
        }
        return;
    }

    if (mode && !strncmp(argv[1], "b=", 2)) {
        bank = strtoul(argv[1]+2, 0, 0);
        index++;
    }
    uint32_t addr = strtoul(argv[index+1], 0, 0);
    int len = (argc == 3 + index) ? strtoul(argv[index+2], 0, 0) : 64;

    char line[128], chr[17];
    int i, n;
    uint8_t val;

    for(i=n=0; i < len; i++) {
        if (mode == 'p') {
            val = mapper_->ReadPrgBank(bank, addr+i);
        } else if (mode == 'c') {
            val = mapper_->ReadChrBank(bank, addr+i);
        } else {
            val = mapper_->Read(addr+i);
        }
        if (i % 16 == 0) {
            if (i) {
                n += sprintf(line+n, "  %s", chr);
                console->AddLog("%s", line);
            }
            n = sprintf(line, "%04x: ", addr+i);
            memset(chr, 0, sizeof(chr));
        }
        n += sprintf(line+n, " %02x", val);
        chr[i%16] = (val>=32 && val<127) ? val : '.';
    }
    if (i % 16) {
        i = 3*(16 - i%16);
    } else {
        i = 0;
    }
    n += sprintf(line+n, " %*c%s", i, ' ', chr);
    console->AddLog("%s", line);
}

void Z2Edit::WriteBytes(DebugConsole* console, int argc, char **argv) {
    uint8_t bank = 0;
    int mode = argv[0][2];
    int index = 0;
    if (argc < 3) {
        console->AddLog("[error] %s: Wrong number of arguments.", argv[0]);
        if (mode) {
            console->AddLog("[error] %s [b=<bank>] <addr> <val> ...", argv[0]);
        } else {
            console->AddLog("[error] %s <addr> <val> ...", argv[0]);
        }
        return;
    }

    if (mode && !strncmp(argv[1], "b=", 2)) {
        bank = strtoul(argv[1]+2, 0, 0);
        index++;
    }
    uint32_t addr = strtoul(argv[index+1], 0, 0);

    for(int i=2+index; i<argc; i++) {
        uint8_t val = strtoul(argv[i], 0, 0);
        if (mode == 'p') {
            mapper_->WritePrgBank(bank, addr++, val);
        } else if (mode == 'c') {
            mapper_->WriteChrBank(bank, addr++, val);
        } else {
            mapper_->Write(addr++, val);
        }
    }
}

void Z2Edit::HexdumpWords(DebugConsole* console, int argc, char **argv) {
    uint8_t bank = 0;
    int mode = argv[0][2];
    int index = 0;
    if (argc < 2) {
        console->AddLog("[error] %s: Wrong number of arguments.", argv[0]);
        if (mode) {
            console->AddLog("[error] %s [b=<bank>] <addr> <length>", argv[0]);
        } else {
            console->AddLog("[error] %s <addr> <length>", argv[0]);
        }
        return;
    }

    if (mode && !strncmp(argv[1], "b=", 2)) {
        bank = strtoul(argv[1]+2, 0, 0);
        index++;
    }
    uint32_t addr = strtoul(argv[index+1], 0, 0);
    int len = (argc == 3 + index) ? strtoul(argv[index+2], 0, 0) : 64;

    char line[128], chr[17];
    int i, n;
    uint16_t val;

    for(i=n=0; i < len; i+=2) {
        if (i % 16 == 0) {
            if (i) {
                n += sprintf(line+n, "  %s", chr);
                console->AddLog("%s", line);
            }
            n = sprintf(line, "%04x: ", addr+i);
            memset(chr, 0, sizeof(chr));
        }
        if (mode == 'p') {
            val = uint16_t(mapper_->ReadPrgBank(bank, addr+i+1)) << 8 |
                  uint16_t(mapper_->ReadPrgBank(bank, addr+i));
        } else if (mode == 'c') {
            val = uint16_t(mapper_->ReadChrBank(bank, addr+i+1)) << 8 |
                  uint16_t(mapper_->ReadChrBank(bank, addr+i));
        } else {
            val = uint16_t(mapper_->Read(addr+i+1)) << 8 |
                  uint16_t(mapper_->Read(addr+i));
        }
        n += sprintf(line+n, " %04x", val);
        chr[i%16] = (uint8_t(val)>=32 && uint8_t(val)<127) ? uint8_t(val) : '.';
        val >>= 8;
        chr[(i+1)%16] = (val>=32 && val<127) ? val : '.';
    }
    if (i % 16) {
        i = 3*(16 - i%16);
    } else {
        i = 0;
    }
    n += sprintf(line+n, " %*c%s", i, ' ', chr);
    console->AddLog("%s", line);
}

void Z2Edit::WriteWords(DebugConsole* console, int argc, char **argv) {
    uint8_t bank = 0;
    int mode = argv[0][2];
    int index = 0;
    if (argc < 3) {
        console->AddLog("[error] %s: Wrong number of arguments.", argv[0]);
        if (mode) {
            console->AddLog("[error] %s [b=<bank>] <addr> <val> ...", argv[0]);
        } else {
            console->AddLog("[error] %s <addr> <val> ...", argv[0]);
        }
        return;
    }

    if (mode && !strncmp(argv[1], "b=", 2)) {
        bank = strtoul(argv[1]+2, 0, 0);
        index++;
    }
    uint32_t addr = strtoul(argv[index+1], 0, 0);

    for(int i=2+index; i<argc; i++) {
        uint16_t val = strtoul(argv[i], 0, 0);
        if (mode == 'p') {
            mapper_->WritePrgBank(bank, addr++, val);
            mapper_->WritePrgBank(bank, addr++, val>>8);
        } else if (mode == 'c') {
            mapper_->WriteChrBank(bank, addr++, val);
            mapper_->WriteChrBank(bank, addr++, val>>8);
        } else {
            mapper_->Write(addr++, val);
            mapper_->Write(addr++, val>>8);
        }
    }
}

void Z2Edit::MemMove(DebugConsole* console, int argc, char **argv) {
    uint8_t bank = -1;
    if (argc < 5 || strncmp(argv[1], "b=", 2) != 0) {
        console->AddLog("[error] %s: Wrong number of arguments.", argv[0]);
        console->AddLog("[error] %s b=<bank> <dst> <src> <len>",  argv[0]);
        return;
    }

    bank = strtoul(argv[1]+2, 0, 0);
    int32_t dst = strtoul(argv[2], 0, 0);
    int32_t src = strtoul(argv[3], 0, 0);
    int32_t len = strtoul(argv[4], 0, 0);

    if (dst < src) {
        for(int i=0; i<len; i++, dst++, src++) {
            mapper_->WritePrgBank(bank, dst, mapper_->ReadPrgBank(bank, src));
        }
    } else if (dst > src) {
        dst += len-1; src += len-1;
        for(int i=0; i<len; i++, dst--, src--) {
            mapper_->WritePrgBank(bank, dst, mapper_->ReadPrgBank(bank, src));
        }
    } else {
        console->AddLog("[error] dst and src are the same!");
    }
}

void Z2Edit::EnemyList(DebugConsole* console, int argc, char **argv) {
    char buf[1024];
    uint8_t bank = 0;
    int mode = argv[0][2];
    int index = 0;
    if (argc < 3) {
        console->AddLog("[error] %s: Wrong number of arguments.", argv[0]);
        if (mode) {
            console->AddLog("[error] %s [b=<bank>] <addr> <val> ...", argv[0]);
        } else {
            console->AddLog("[error] %s <addr> <val> ...", argv[0]);
        }
        return;
    }

    if (mode && !strncmp(argv[1], "b=", 2)) {
        bank = strtoul(argv[1]+2, 0, 0);
        index++;
    }
    uint32_t addr = strtoul(argv[index+1], 0, 0);
    int len = (argc == 3 + index) ? strtoul(argv[index+2], 0, 0) : 64;

    while(len > 0) {
        int listlen = mapper_->ReadPrgBank(bank, addr);
        buf[0] = buf[1] = 0;
        console->AddLog("%04x: %02x (copied to %04x)", addr, listlen, addr-0x18a0);

        addr++; len--;
        int j=0;
        for(int i=1; i<listlen && len; i++, len--) {
            j += sprintf(buf+j, " %02x", mapper_->ReadPrgBank(bank, addr++));
        }
        console->AddLog("    [%s]", buf+1);
    }
}

void Z2Edit::Unassemble(DebugConsole* console, int argc, char **argv) {
    static uint8_t bank;
    static uint16_t addr;

    int index = 0;
    if (argc < 2) {
        console->AddLog("[error] %s: Wrong number of arguments.", argv[0]);
        console->AddLog("[error] %s [b=<bank>] <addr> <length>", argv[0]);
        return;
    }

    if (!strncmp(argv[1], "b=", 2)) {
        bank = strtoul(argv[1]+2, 0, 0);
        index++;
    }
    addr = strtoul(argv[index+1], 0, 0);
    int len = (argc == 3 + index) ? strtoul(argv[index+2], 0, 0) : 10;

    Cpu cpu(mapper_.get());
    cpu.set_bank(bank);
    for(int i=0; i<len; i++) {
        std::string instruction = cpu.Disassemble(&addr);
        console->AddLog("%s", instruction.c_str());
    }
}

void Z2Edit::InsertPrg(DebugConsole* console, int argc, char **argv) {
    uint8_t bank = 0;
    if (argc < 2) {
        console->AddLog("[error] %s: Wrong number of arguments.", argv[0]);
        console->AddLog("[error] %s <bank>", argv[0]);
        return;
    }

    bank = strtoul(argv[1], 0, 0);
    cartridge_.InsertPrg(bank, nullptr);
    console->AddLog("#{0f0}Added bank %d", bank);
}

void Z2Edit::CopyPrg(DebugConsole* console, int argc, char **argv) {
    uint8_t src, dst;
    if (argc < 3) {
        console->AddLog("[error] %s: Wrong number of arguments.", argv[0]);
        console->AddLog("[error] %s <src> <dst>", argv[0]);
        return;
    }

    src = strtoul(argv[1], 0, 0);
    dst = strtoul(argv[2], 0, 0);
    for(int i=0; i<16384; i++) {
        mapper_->WritePrgBank(dst, i, mapper_->ReadPrgBank(src, i));
    }
    console->AddLog("#{0f0}Copied bank %d to %d", src, dst);
}




void Z2Edit::SpawnEmulator(const std::string& romfile) {
    std::string cmdline = FLAGS_emulator + " " + romfile + " &";
    system(cmdline.c_str());
}

void Z2Edit::ProcessEvent(SDL_Event* event) {
    editor_->ProcessEvent(event);
}

void Z2Edit::Draw() {
    ImGui::SetNextWindowSize(ImVec2(500,300), ImGuiSetCond_FirstUseEver);
    if (ImGui::BeginMainMenuBar()) {
        if (ImGui::BeginMenu("File")) {
            if (ImGui::MenuItem("Emulate", "Ctrl+E")) {
                cartridge_.SaveFile(FLAGS_romtmp);
                SpawnEmulator(FLAGS_romtmp);
            }
            ImGui::Separator();
#ifdef HAVE_NFD
            if (ImGui::MenuItem("Open", "Ctrl+O")) {
                char *filename = nullptr;
                auto result = NFD_OpenDialog(nullptr, nullptr, &filename);
                if (result == NFD_OKAY) {
                    Load(filename);
                    save_filename_.assign(filename);
                }
                free(filename);
            }
            if (ImGui::MenuItem("Save", "Ctrl+S")) {
                if (save_filename_.empty())
                    goto save_as;
                cartridge_.SaveFile(save_filename_);
            }
            if (ImGui::MenuItem("Save As")) {
save_as:
                char *filename = nullptr;
                auto result = NFD_SaveDialog(nullptr, nullptr, &filename);
                if (result == NFD_OKAY) {
                    save_filename_.assign(filename);
                    cartridge_.SaveFile(save_filename_);
                }
                free(filename);
            }
#endif
            ImGui::Separator();
            if (!FLAGS_config.empty()) {
                if (ImGui::MenuItem("Reload Config")) {
                    auto* config = ConfigLoader<z2util::RomInfo>::Get();
                    config->Reload();
                }
            }
            if (ImGui::MenuItem("Quit")) {
                running_ = false;
            }
            ImGui::EndMenu();
        }
        if (ImGui::BeginMenu("Edit")) {
            ImGui::MenuItem("Debug Console", nullptr,
                            &console_.visible());
            ImGui::MenuItem("Enemy Attributes", nullptr,
                            &enemy_editor_->visible());
            ImGui::MenuItem("Overworld Editor", nullptr,
                            &editor_->visible());
            ImGui::MenuItem("Sideview Editor", nullptr,
                            &simplemap_->visible());
            ImGui::MenuItem("Miscellaneous Hacks", nullptr,
                            &misc_hacks_->visible());
            ImGui::MenuItem("Palace Graphics", nullptr,
                            &palace_gfx_->visible());
            ImGui::MenuItem("Palette Editor", nullptr,
                            &palette_editor_->visible());
            ImGui::MenuItem("Start Values", nullptr,
                            &start_values_->visible());
            ImGui::EndMenu();
        }
        if (ImGui::BeginMenu("View")) {
            ImGui::MenuItem("Hardware Palette", nullptr,
                            &hwpal_->visible());
            ImGui::MenuItem("CHR Viewer", nullptr,
                            &chrview_->visible());
            ImGui::MenuItem("Object Table", nullptr,
                            &object_table_->visible());
            ImGui::EndMenu();
        }
        if (ImGui::BeginMenu("Help")) {
            if (ImGui::MenuItem("Online Help")) {
                Help("root");
            }
            ImGui::EndMenu();
        }
        ImGui::EndMainMenuBar();
    }

    start_values_->Draw();
    misc_hacks_->Draw();
    palace_gfx_->Draw();
    palette_editor_->Draw();
    hwpal_->Draw();
    chrview_->Draw();
    simplemap_->Draw();
    editor_->Draw();
    object_table_->Draw();
    enemy_editor_->Draw();

    if (!loaded_) {
        char *filename = nullptr;
        auto result = NFD_OpenDialog(nullptr, nullptr, &filename);
        if (result == NFD_OKAY) {
            Load(filename);
            save_filename_.assign(filename);
        }
        free(filename);
    }
}

void Z2Edit::Help(const std::string& topickey) {
    const auto& ri = ConfigLoader<RomInfo>::GetConfig();
    const auto& it = ri.help().url().find(topickey);
    if (it == ri.help().url().end()) {
        LOG(ERROR, "No help for topickey=", topickey);
        if (topickey != "root") {
            Help("root");
        }
        return;
    }
    Browser::Open(it->second);
}

}  // namespace z2util
