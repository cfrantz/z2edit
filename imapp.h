#ifndef Z2UTIL_IMAPP_H
#define Z2UTIL_IMAPP_H
#include <memory>

#include <string>
#include <SDL2/SDL.h>
#include <SDL2/SDL_opengl.h>

#include "proto/rominfo.pb.h"
#include "util/fpsmgr.h"
#include "imwidget/debug_console.h"
#include "imwidget/editor.h"
#include "imwidget/hwpalette.h"
#include "imwidget/misc_hacks.h"
#include "imwidget/neschrview.h"
#include "imwidget/simplemap.h"
#include "imwidget/start_values.h"
#include "imwidget/object_table.h"
#include "nes/cartridge.h"
#include "nes/mapper.h"
#include "nes/memory.h"

class ImApp {
  public:
    static ImApp* Get() { return singleton_; }
    ImApp(const std::string& name, int width, int height);
    ImApp(const std::string& name) : ImApp(name, 1280, 720) {}
    ~ImApp();

    virtual void Init();
    virtual bool ProcessEvents();
    virtual void Draw();
    virtual void Run();

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
    inline void set_rominfo(const z2util::RomInfo& ri) { rominfo_ = ri; }
    inline z2util::RomInfo& rominfo() { return rominfo_; }

    void PlayAudio(float* data, int len);
    void Load(const std::string& filename);
    void Help(const std::string& topickey);
    void AddDrawCallback(std::function<bool()> cb);
  private:
    void Quit(DebugConsole* console, int argc, char **argv);
    void LoadFile(DebugConsole* console, int argc, char **argv);
    void SaveFile(DebugConsole* console, int argc, char **argv);
    void HexdumpBytes(DebugConsole* console, int argc, char **argv);
    void WriteBytes(DebugConsole* console, int argc, char **argv);
    void WriteMapper(DebugConsole* console, int argc, char **argv);
    void HexdumpWords(DebugConsole* console, int argc, char **argv);
    void WriteWords(DebugConsole* console, int argc, char **argv);
    void Unassemble(DebugConsole* console, int argc, char **argv);
    void EnemyList(DebugConsole* console, int argc, char **argv);
    void InsertPrg(DebugConsole* console, int argc, char **argv);
    void CopyPrg(DebugConsole* console, int argc, char **argv);

    void SpawnEmulator(const std::string& romfile);

    void InitAudio(int freq, int chan, int bufsz, SDL_AudioFormat fmt);
    void AudioCallback(float* stream, int len);

    static ImApp* singleton_;
    std::string name_;
    int width_;
    int height_;
    bool running_;

    std::unique_ptr<float[]> audiobuf_;
    int audiobufsz_;
    int audio_producer_, audio_consumer_;

    SDL_Window *window_;
    SDL_Renderer *renderer_;
    SDL_Texture *texture_;
    SDL_PixelFormat *format_;
    SDL_GLContext glcontext_;
    FPSManager fpsmgr_;
    z2util::RomInfo rominfo_;
    DebugConsole console_;

    std::string save_filename_;
    NesHardwarePalette* hwpal_;
    std::unique_ptr<NesChrView> chrview_;
    std::unique_ptr<z2util::SimpleMap> simplemap_;
    std::unique_ptr<z2util::Editor> editor_;
    std::unique_ptr<z2util::MiscellaneousHacks> misc_hacks_;
    std::unique_ptr<z2util::StartValues> start_values_;
    std::unique_ptr<z2util::ObjectTable> object_table_;
    std::vector<std::function<bool()>> draw_callback_;
    std::vector<std::function<bool()>> draw_added_;

    Cartridge cartridge_;
    z2util::Memory memory_;
    std::unique_ptr<Mapper> mapper_;

    static void AudioCallback_(void* userdata, uint8_t* stream, int len);
};

#endif // Z2UTIL_IMAPP_H
