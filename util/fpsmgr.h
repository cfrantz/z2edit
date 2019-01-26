#ifndef PROJECT_UTIL_FPSMGR_H
#define PROJECT_UTIL_FPSMGR_H
#include <SDL2/SDL2_framerate.h>

class FPSManager {
  public:
    FPSManager() {
        SDL_initFramerate(&mgr_);
    }

    int SetRate(int rate) {
        return SDL_setFramerate(&mgr_, rate);
    }
    int GetRate(int rate) {
        return SDL_getFramerate(&mgr_);
    }
    int Count() {
        return SDL_getFramecount(&mgr_);
    }
    int Delay() {
        return SDL_framerateDelay(&mgr_);
    }
  private:
    FPSmanager mgr_;
};

#endif // PROJECT_UTIL_FPSMGR_H
