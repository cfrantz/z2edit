#ifndef Z2UTIL_IMAPP_H
#define Z2UTIL_IMAPP_H
#include <memory>

#include <string>
#include <SDL2/SDL.h>
#include <SDL2/SDL_opengl.h>

#include "util/fpsmgr.h"
#include "imwidget/debug_console.h"
#include "imwidget/imwidget.h"

class ImApp {
  public:
    static ImApp* Get() { return singleton_; }
    ImApp(const std::string& name, int width, int height);
    ImApp(const std::string& name) : ImApp(name, 1280, 720) {}
    virtual ~ImApp();

    void InitControllers();
    void InitAudio(int freq, int chan, int bufsz, SDL_AudioFormat fmt);
    virtual void Init() {}
    virtual bool PreDraw() { return false; }
    virtual void Draw() {}
    virtual void ProcessEvent(SDL_Event* event) {}
    virtual void Help(const std::string& topickey) {}

    void SetTitle(const std::string& title, bool with_appname=true);
    void Run();
    void BaseDraw();
    virtual bool ProcessEvents();

    virtual void ProcessMessage(const std::string& msg, const void *extra) {}
    void ProcessMessage(const std::string& msg) {
        ProcessMessage(msg, nullptr);
    }

    // Convenience helpers for registering debug console commands
    inline void RegisterCommand(const char* cmd, const char* shorthelp,
                                std::function<void(DebugConsole*,
                                                   int argc, char **argv)> fn) {
        console_.RegisterCommand(cmd, shorthelp, fn);
    }

    template<typename T>
    inline void RegisterCommand(const char* cmd, const char* shorthelp,
                         T* that, void (T::*fn)(DebugConsole*,
                                                int argc, char **argv)) {
        console_.RegisterCommand(cmd, shorthelp, that, fn);
    }

    void AddDrawCallback(ImWindowBase* window);
    void HelpButton(const std::string& topickey, bool right_justify=false);

    inline const ImVec4& clear_color() { return clear_color_; }
    inline void set_clear_color(const ImVec4& c) { clear_color_ = c; }

  protected:
    virtual void AudioCallback(void* stream, int len);
    std::string name_;
    int width_;
    int height_;
    bool running_;
    ImVec4 clear_color_;
    DebugConsole console_;
    std::vector<std::unique_ptr<ImWindowBase>> draw_callback_;

  private:
    void Quit(DebugConsole* console, int argc, char **argv);
    static void AudioCallback_(void* userdata, uint8_t* stream, int len);

    static ImApp* singleton_;

    SDL_Window *window_;
    SDL_Renderer *renderer_;
    SDL_Texture *texture_;
    SDL_PixelFormat *format_;
    SDL_GLContext glcontext_;
    FPSManager fpsmgr_;

    std::vector<std::unique_ptr<ImWindowBase>> draw_added_;
};

#endif // Z2UTIL_IMAPP_H
