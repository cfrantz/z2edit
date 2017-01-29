#include <stdlib.h>
#include "util/browser.h"

void Browser::Open(const std::string& url) {
#ifdef _WIN32
    std::string cmd = "cmd.exe /c start";
#else
    std::string cmd = "xdg-open";
#endif

    cmd = cmd + " " + url;
    system(cmd.c_str());
}
