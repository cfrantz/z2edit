#include <gflags/gflags.h>
#include "imapp.h"
#include "imgui.h"
#include "nes/cpu6502.h"
#include "util/config.h"
#include "proto/rominfo.pb.h"
#include "util/os.h"
#include "util/logging.h"
#include "util/imgui_impl_sdl.h"

#ifdef HAVE_NFD
#include "nfd.h"
#endif


DEFINE_string(emulator, "fceux", "Emulator to run for testing");
DEFINE_string(romtmp, "/tmp/zelda2-test.nes", "Temporary filename for running under test");
DEFINE_int32(audio_frequency, 48000, "Audio sample frequency");
//DEFINE_int32(audio_bufsize, 256, "Audio buffer size");
DEFINE_int32(audio_bufsize, 2048, "Audio buffer size");
DEFINE_double(hidpi, 1.0, "HiDPI scaling factor");

ImApp* ImApp::singleton_;

ImApp::ImApp(const std::string& name, int width, int height)
  : name_(name),
    width_(width),
    height_(height),
    audio_producer_(0),
    audio_consumer_(0)
{
    singleton_ = this;
    SDL_Init(SDL_INIT_VIDEO |
             SDL_INIT_AUDIO |
             SDL_INIT_TIMER |
             SDL_INIT_JOYSTICK |
             SDL_INIT_GAMECONTROLLER);

    SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1);
    SDL_GL_SetAttribute(SDL_GL_DEPTH_SIZE, 24);
    SDL_GL_SetAttribute(SDL_GL_STENCIL_SIZE, 8);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 2);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 2);

    window_ = SDL_CreateWindow(name.c_str(),
                               SDL_WINDOWPOS_UNDEFINED,
                               SDL_WINDOWPOS_UNDEFINED,
                               width_, height_,
                               SDL_WINDOW_OPENGL | SDL_WINDOW_RESIZABLE);

    glcontext_ = SDL_GL_CreateContext(window_);
    ImGui_ImplSdl_SetHiDPIScale(FLAGS_hidpi);
    ImGui_ImplSdl_Init(window_);
    fpsmgr_.SetRate(60);

    ///audiobufsz_ = FLAGS_audio_bufsize * 2;
    ///audiobuf_.reset(new float[audiobufsz_]);
    ///InitAudio(FLAGS_audio_frequency, 1, FLAGS_audio_bufsize, AUDIO_F32);
}

ImApp::~ImApp() {
    ImGui_ImplSdl_Shutdown();
    SDL_GL_DeleteContext(glcontext_);
    SDL_DestroyWindow(window_);
    SDL_Quit();
}

void ImApp::Init() {
    RegisterCommand("quit", "Quit the application.", this, &ImApp::Quit);
    RegisterCommand("load", "Load a NES ROM.", this, &ImApp::LoadFile);
    RegisterCommand("save", "Save a NES ROM.", this, &ImApp::SaveFile);
    RegisterCommand("wm", "Write mapper register.", this, &ImApp::WriteMapper);
    RegisterCommand("header", "Print iNES header.",
            &cartridge_, &Cartridge::PrintHeader);

    RegisterCommand("db", "Hexdump bytes (via mapper).", this, &ImApp::HexdumpBytes);
    RegisterCommand("dbp", "Hexdump PRG bytes.", this, &ImApp::HexdumpBytes);
    RegisterCommand("dbc", "Hexdump CHR bytes.", this, &ImApp::HexdumpBytes);
    RegisterCommand("dw", "Hexdump words (via mapper).", this, &ImApp::HexdumpWords);
    RegisterCommand("dwp", "Hexdump PRG words.", this, &ImApp::HexdumpWords);
    RegisterCommand("dwc", "Hexdump CHR words.", this, &ImApp::HexdumpWords);
    RegisterCommand("wb", "Write bytes (via mapper).", this, &ImApp::WriteBytes);
    RegisterCommand("wbp", "Write PRG bytes.", this, &ImApp::WriteBytes);
    RegisterCommand("wbc", "Write CHR bytes.", this, &ImApp::WriteBytes);
    RegisterCommand("ww", "Write words (via mapper).", this, &ImApp::WriteWords);
    RegisterCommand("wwp", "Write PRG words.", this, &ImApp::WriteWords);
    RegisterCommand("wwc", "Write CHR words.", this, &ImApp::WriteWords);
    RegisterCommand("elist", "Dump Enemy List.", this, &ImApp::EnemyList);
    RegisterCommand("u", "Disassemble Code.", this, &ImApp::Unassemble);

    hwpal_ = NesHardwarePalette::Get();
    chrview_.reset(new NesChrView);
    simplemap_.reset(new z2util::SimpleMap);
    misc_hacks_.reset(new z2util::MiscellaneousHacks);
    start_values_.reset(new z2util::StartValues);
    object_table_.reset(new z2util::ObjectTable);
    editor_.reset(z2util::Editor::New());
}

void ImApp::Quit(DebugConsole* console, int argc, char **argv) {
    running_ = false;
}

void ImApp::Load(const std::string& filename) {
    cartridge_.LoadFile(filename);
    mapper_.reset(MapperRegistry::New(&cartridge_, cartridge_.mapper()));
    chrview_->set_mapper(mapper_.get());

    simplemap_->set_mapper(mapper_.get());
    for(const auto& m : rominfo_.map()) {
        if (m.name().find("North Palace") != std::string::npos) {
            simplemap_->SetMap(m);
            break;
        }
    }

    editor_->set_mapper(mapper_.get());
    editor_->ConvertFromMap(rominfo_.mutable_map(0));

    misc_hacks_->set_mapper(mapper_.get());
    start_values_->set_mapper(mapper_.get());

    object_table_->set_mapper(mapper_.get());
    object_table_->Init();
}

void ImApp::LoadFile(DebugConsole* console, int argc, char **argv) {
    if (argc != 2) {
        console->AddLog("[error] Usage: %s [filename]", argv[0]);
        return;
    }
    Load(argv[1]);
}

void ImApp::WriteMapper(DebugConsole* console, int argc, char **argv) {
    mapper_->DebugWriteReg(console, argc, argv);
}

void ImApp::SaveFile(DebugConsole* console, int argc, char **argv) {
    if (argc != 2) {
        console->AddLog("[error] Usage: %s [filename]", argv[0]);
        return;
    }
    cartridge_.SaveFile(argv[1]);
}

void ImApp::HexdumpBytes(DebugConsole* console, int argc, char **argv) {
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

void ImApp::WriteBytes(DebugConsole* console, int argc, char **argv) {
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

void ImApp::HexdumpWords(DebugConsole* console, int argc, char **argv) {
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

void ImApp::WriteWords(DebugConsole* console, int argc, char **argv) {
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

void ImApp::EnemyList(DebugConsole* console, int argc, char **argv) {
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

void ImApp::Unassemble(DebugConsole* console, int argc, char **argv) {
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

    LOG(VERBOSE, "u b=", bank, " ", HEX(addr), " ", len);
    Cpu cpu(mapper_.get());
    cpu.set_bank(bank);
    for(int i=0; i<len; i++) {
        std::string instruction = cpu.Disassemble(&addr);
        console->AddLog("%s", instruction.c_str());
        LOG(VERBOSE, instruction);
    }
}

void ImApp::Run() {
    running_ = true;
    while(running_) {
        if (!ProcessEvents())
            break;
        Draw();
        // Play audio here if you have audio
        // PlayAudio(...)
        fpsmgr_.Delay();
    }
}

void ImApp::SpawnEmulator(const std::string& romfile) {
    std::string cmdline = FLAGS_emulator + " " + romfile + " &";
    system(cmdline.c_str());
}

bool ImApp::ProcessEvents() {
    SDL_Event event;
    bool done = false;
    while (SDL_PollEvent(&event)) {
        ImGui_ImplSdl_ProcessEvent(&event);
        if (event.type == SDL_QUIT)
            done = true;
        editor_->ProcessEvent(&event);
    }
    return !done;
}

void ImApp::Draw() {
    ImVec4 clear_color = ImColor(114, 144, 154);
    ImGui_ImplSdl_NewFrame(window_);

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
            if (ImGui::MenuItem("Reload Config")) {
                auto* config = ConfigLoader<z2util::RomInfo>::Get();
                config->Reload();
            }
            if (ImGui::MenuItem("Quit")) {
                running_ = false;
            }
            ImGui::EndMenu();
        }
        if (ImGui::BeginMenu("Edit")) {
            ImGui::MenuItem("Debug Console", nullptr,
                            console_.visible());
            ImGui::MenuItem("Overworld Editor", nullptr,
                            editor_->visible());
            ImGui::MenuItem("Sideview Editor", nullptr,
                            simplemap_->visible());
            ImGui::MenuItem("Miscellaneous Hacks", nullptr,
                            misc_hacks_->visible());
            ImGui::MenuItem("Start Values", nullptr,
                            start_values_->visible());
            ImGui::EndMenu();
        }
        if (ImGui::BeginMenu("View")) {
            ImGui::MenuItem("Hardware Palette", nullptr,
                            hwpal_->visible());
            ImGui::MenuItem("CHR Viewer", nullptr,
                            chrview_->visible());
            ImGui::MenuItem("Object Table", nullptr,
                            object_table_->visible());
            ImGui::EndMenu();
        }
        ImGui::EndMainMenuBar();
    }

    console_.Draw();
    start_values_->Draw();
    misc_hacks_->Draw();
    hwpal_->Draw();
    chrview_->Draw();
    simplemap_->Draw();
    editor_->Draw();
    object_table_->Draw();

    for(auto it=draw_callback_.begin(); it != draw_callback_.end();) {
        if (!(*it)()) {
            draw_callback_.erase(it);
            continue;
        }
        ++it;
    }

    glViewport(0, 0,
               (int)ImGui::GetIO().DisplaySize.x,
               (int)ImGui::GetIO().DisplaySize.y);
    glClearColor(clear_color.x, clear_color.y, clear_color.z, clear_color.w);
    glClear(GL_COLOR_BUFFER_BIT);
    ImGui::Render();
    SDL_GL_SwapWindow(window_);
    draw_callback_.insert(draw_callback_.end(), draw_added_.begin(), draw_added_.end());
    draw_added_.clear();
}


void ImApp::InitAudio(int freq, int chan, int bufsz, SDL_AudioFormat fmt) {
    SDL_AudioSpec want, have;
    SDL_AudioDeviceID dev;

    SDL_memset(&want, 0, sizeof(want));
    want.freq = freq;
    want.channels = chan;
    want.samples = bufsz;
    want.format = fmt;
    want.callback = ImApp::AudioCallback_;
    want.userdata = (void*)this;

    dev = SDL_OpenAudioDevice(NULL, 0, &want, &have,
                              SDL_AUDIO_ALLOW_FORMAT_CHANGE);
    SDL_PauseAudioDevice(dev, 0);
}

void ImApp::PlayAudio(float* data, int len) {
    int producer = audio_producer_;
    while(len) {
        audiobuf_[producer] = *data++;
        len--;
        producer = (producer + 1) % audiobufsz_;
        while(producer == audio_consumer_) {
            // Audio overrun.
            // FIXME(cfrantz): This should use a condition variable, but this
            // program doesn't use audio anyway.
            // os::Yield();
        }
        audio_producer_ = producer;
    }
}

void ImApp::AudioCallback(float* stream, int len) {
    while(audio_consumer_ != audio_producer_ && len) {
        *stream++ = audiobuf_[audio_consumer_];
        len--;
        audio_consumer_ = (audio_consumer_ + 1) % audiobufsz_;
    }
    if (len) {
        fprintf(stderr, "Audio underrun!\n");
        while(len--) {
            *stream++ = 0;
        }
    }
}

void ImApp::AudioCallback_(void* userdata, uint8_t* stream, int len) {
    ImApp* instance = (ImApp*)userdata;
    instance->AudioCallback((float*)stream, len/sizeof(float));
}


void ImApp::AddDrawCallback(std::function<bool()> cb) {
    draw_added_.push_back(cb);
}


void AddDrawCallback(std::function<bool()> cb) {
    ImApp::Get()->AddDrawCallback(cb);
}


