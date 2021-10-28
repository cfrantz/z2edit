#include <gflags/gflags.h>
#include "imapp.h"
#include "imgui.h"
#include "util/os.h"
#include "util/gamecontrollerdb.h"
#include "util/logging.h"
#include "util/imgui_impl_sdl.h"
#include "absl/strings/str_cat.h"

DEFINE_double(hidpi, 1.0, "HiDPI scaling factor");
DEFINE_string(controller_db, "", "Path to the SDL gamecontrollerdb.txt file");


ImApp* ImApp::singleton_;

ImApp::ImApp(const std::string& name, int width, int height)
  : name_(name),
    width_(width),
    height_(height),
    running_(true)
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
    // Setup ImGui binding
    ImGui::CreateContext();
    ImGuiIO& io = ImGui::GetIO(); (void)io;
    ImGui_ImplSdl_SetHiDPIScale(FLAGS_hidpi);
    ImGui_ImplSdlGL2_Init(window_);
    clear_color_ = ImColor(0, 16, 64);
    //fpsmgr_.SetRate(60);

    RegisterCommand("quit", "Quit the application.", this, &ImApp::Quit);
}

ImApp::~ImApp() {
    ImGui_ImplSdlGL2_Shutdown();
    ImGui::DestroyContext();
    SDL_GL_DeleteContext(glcontext_);
    SDL_DestroyWindow(window_);
    SDL_Quit();
}

void ImApp::InitControllers() {
    if (FLAGS_controller_db.empty()) {
        SDL_RWops* f = SDL_RWFromConstMem(kGameControllerDB,
                                          kGameControllerDB_len);
        SDL_GameControllerAddMappingsFromRW(f, 1);
    } else {
        SDL_GameControllerAddMappingsFromFile(FLAGS_controller_db.c_str());
    }

    int controllers = 0;
    char guid[64];
    for(int i=0; i<SDL_NumJoysticks(); ++i) {
        const char *name, *desc;
        SDL_JoystickGetGUIDString(SDL_JoystickGetDeviceGUID(i), guid, sizeof(guid));
        if (SDL_IsGameController(i)) {
            controllers++;
            name = SDL_GameControllerNameForIndex(i);
            desc = "Controller";
        } else {
            name = SDL_JoystickNameForIndex(i);
            desc = "Joystick";
        }
        printf("%s %d: %s (guid: %s)\n", desc, i,
               name ? name : "Unknown", guid);
    }
    for(int i=0; i<controllers; i++) {
        printf("Opening controller %d\n", i);
        SDL_GameControllerOpen(i);
    }
}

void ImApp::Quit(DebugConsole* console, int argc, char **argv) {
    running_ = false;
}

void ImApp::SetTitle(const std::string& title, bool with_appname) {
    std::string val;
    if (with_appname) {
        val = title.empty() ? name_ : absl::StrCat(name_, ": ", title);
    } else {
        val = name_;
    }
    SDL_SetWindowTitle(window_, val.c_str());
}

void ImApp::Run() {
    while(running_) {
        if (!ProcessEvents()) {
            running_ = false;
            break;
        }
        BaseDraw();
        //fpsmgr_.Delay();
    }
}

bool ImApp::ProcessEvents() {
    SDL_Event event;
    bool done = false;
    while (SDL_PollEvent(&event)) {
        ImGui_ImplSdlGL2_ProcessEvent(&event);
        if (event.type == SDL_QUIT)
            done = true;
        ProcessEvent(&event);
    }
    return !done;
}

void ImApp::BaseDraw() {
    if (!PreDraw()) {
        glViewport(0, 0,
                   (int)ImGui::GetIO().DisplaySize.x,
                   (int)ImGui::GetIO().DisplaySize.y);
        glClearColor(clear_color_.x, clear_color_.y, clear_color_.z, clear_color_.w);
        glClear(GL_COLOR_BUFFER_BIT);
    }

    ImGui_ImplSdlGL2_NewFrame(window_);
    console_.Draw();
    for(auto it=draw_callback_.begin(); it != draw_callback_.end();) {
        if ((*it)->visible()) {
            (*it)->Draw();
        } else if ((*it)->want_dispose()) {
            it = draw_callback_.erase(it);
            continue;
        }
        ++it;
    }

    Draw();
    ImGui::Render();
    ImGui_ImplSdlGL2_RenderDrawData(ImGui::GetDrawData());
    SDL_GL_SwapWindow(window_);
    for(auto& widget : draw_added_) {
        draw_callback_.emplace_back(std::move(widget));
    }
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

void ImApp::HelpButton(const std::string& topickey, bool right_justify) {
    if (right_justify) {
        ImGui::SameLine(ImGui::GetWindowWidth() - 50);
    }
    if (ImGui::Button("Help")) {
        Help(topickey);
    }
}

void ImApp::AudioCallback(void* stream, int len) {
    memset(stream, 0, len);
}

void ImApp::AudioCallback_(void* userdata, uint8_t* stream, int len) {
    ImApp* instance = (ImApp*)userdata;
    instance->AudioCallback(stream, len);
}

void ImApp::AddDrawCallback(ImWindowBase* window) {
    draw_added_.emplace_back(window);
}
