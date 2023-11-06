#ifndef PROJECT_APP_H
#define PROJECT_APP_H
#include <memory>
#include <string>
#include <SDL2/SDL.h>

#include "imwidget/imapp.h"
#include <google/protobuf/message.h>

namespace project {

class App: public ImApp {
  public:
    App(const std::string& name);
    ~App() override;

    void Init() override;
    void ProcessEvent(SDL_Event* event) override;
    void ProcessMessage(const std::string& msg, const void* extra) override;
    void Draw() override;

    void SetMessage(google::protobuf::Message* msg) { msg_ = msg; }
    void Help(const std::string& topickey);
  private:
    std::string save_filename_;
    bool plot_demo_ = false;
    google::protobuf::Message* msg_ = nullptr;
};

}  // namespace project
#endif // PROJECT_APP_H
