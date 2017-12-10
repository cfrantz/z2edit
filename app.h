#ifndef Z2UTIL_APP_H
#define Z2UTIL_APP_H
#include <memory>
#include <string>
#include <SDL2/SDL.h>

#include "imwidget/imapp.h"

namespace z2util {

class App: public ImApp {
  public:
    App(const std::string& name) : ImApp(name, 1280, 720, false) {}
    ~App() override {}

    void Init() override;
    void ProcessEvent(SDL_Event* event) override;
    void ProcessMessage(const std::string& msg, const void* extra) override;
    void Draw() override;

    void Help(const std::string& topickey);
  private:
    std::string save_filename_;
};

}  // namespace z2util
#endif // Z2UTIL_APP_H
