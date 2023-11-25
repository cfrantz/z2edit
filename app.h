#ifndef PROJECT_APP_H
#define PROJECT_APP_H
#include <SDL2/SDL.h>
#include <google/protobuf/message.h>
#include <pybind11/pybind11.h>

#include <memory>
#include <string>

#include "imwidget/imapp.h"

namespace project {

class App : public ImApp {
  public:
    App(const std::string& name);
    App() : App("") {}
    ~App() override;

    void Init() override;
    void ProcessEvent(SDL_Event* event) override;
    void ProcessMessage(const std::string& msg, const void* extra) override;
    void Draw() override;

    void SetMessage(google::protobuf::Message* msg) { msg_ = msg; }
    void Help(const std::string& topickey);

    virtual void MenuBarHook() {}
    virtual void MenuHook(const std::string& name) {}

  private:
    std::string save_filename_;
    bool plot_demo_ = false;
    google::protobuf::Message* msg_ = nullptr;
};

class PyApp : public App {
  public:
    using App::App;

    void MenuBarHook() override {
        pybind11::gil_scoped_acquire gil;
        PYBIND11_OVERRIDE_NAME(void, App, "menu_bar_hook", MenuBarHook);
    }
    void MenuHook(const std::string& name) override {
        pybind11::gil_scoped_acquire gil;
        PYBIND11_OVERRIDE_NAME(void, App, "menu_hook", MenuHook, name);
    }
};

}  // namespace project
#endif  // PROJECT_APP_H
