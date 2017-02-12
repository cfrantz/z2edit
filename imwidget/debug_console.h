#ifndef SYNTHY_IMWIDGET_DEBUG_CONSOLE_H
#define SYNTHY_IMWIDGET_DEBUG_CONSOLE_H
#include <string>
#include <map>
#include <functional>
#include "imwidget/imwidget.h"
#include "imgui.h"


class DebugConsole: public ImWindowBase {
  public:
    DebugConsole(const char* name);
    DebugConsole(): DebugConsole("DebugConsole") {}
    ~DebugConsole() override;

    void RegisterCommand(const char* cmd, const char* shorthelp,
                         std::function<void(DebugConsole* console,
                                            int argc, char **argv)> fn);

    template<typename T>
    inline void RegisterCommand(const char* cmd, const char* shorthelp,
                         T* that, void (T::*fn)(DebugConsole* console,
                                                int argc, char **argv)) {
        RegisterCommand(cmd, shorthelp, [=](DebugConsole* console,
                                            int argc, char **argv) {
            (that->*fn)(console, argc, argv);
        });
    }

    void ClearLog();
    void AddLog(const char* fmt, ...) IM_PRINTFARGS(2);
    bool Draw() override;
  private:
    void ExecCommand(const char* command_line);
    int TextEditCallback(ImGuiTextEditCallbackData* data);

    static int TextEditCallbackStub(ImGuiTextEditCallbackData* data);

    const char* name_;
    char inputbuf_[256];
    ImVector<char*> items_;
    bool scroll_to_bottom_;
    ImVector<char*> history_;
    // -1: new line, 0..history_.Size-1 browsing history.
    int history_pos_;
    std::map<const char*, const char*> shorthelp_;
    std::map<const char*, std::function<void(DebugConsole* console,
                                             int argc, char **argv)>> commands_;
};

#endif // SYNTHY_IMWIDGET_DEBUG_CONSOLE_H
