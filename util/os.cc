#include <cstdlib>
#include <time.h>
#include <unistd.h>
#include <limits.h>
#include <sched.h>
#ifdef _WIN32
#include <windows.h>
#endif

#include "util/os.h"
#include "util/strutil.h"

namespace os {
string GetCWD() {
    char path[PATH_MAX];
    string result;

    if (getcwd(path, PATH_MAX)) {
        result = string(path);
    }
    return result;
}

void SchedulerYield() {
#ifndef _WIN32
    sched_yield();
#else
    SwitchToThread();
#endif
}

int64_t utime_now() {
    int64_t now;
#ifndef _WIN32
    struct timespec tm;
    clock_gettime(CLOCK_REALTIME, &tm);
    now = tm.tv_sec * 1000000LL + tm.tv_nsec/1000;
#else
    FILETIME ft;
    GetSystemTimeAsFileTime(&ft);
    now = int64_t(ft.dwLowDateTime) | int64_t(ft.dwHighDateTime) << 32;
#endif
    return now;
}

std::string CTime(int64_t time_us) {
    time_t t = time_us / 1000000;
    char buf[32];
#ifndef _WIN32
    ctime_r(&t, buf);
#else
    ctime_s(buf, sizeof(buf), &t);
#endif
    if (buf[24] == '\n') buf[24] = 0;
    return buf;
}

std::string TempFilename(const std::string& filename) {
    const char *temp = getenv("TEMP");
#ifdef _WIN32
    std::string tmp = temp ? temp : "c:/temp";
#else
    std::string tmp = temp ? temp : "/tmp";
#endif
    return path::Join({tmp, filename});
}

int System(const std::string& cmd, bool background) {
    std::string header, trailer;
#ifdef _WIN32
    header = "cmd.exe /c start ";
#else
    trailer = " &";
#endif
    return system(StrCat(header, cmd, trailer).c_str());
}

namespace path {
char kPathSep = '/';

string Join(const std::vector<string>& components) {
    string result;

    for(const auto& p : components) {
        if (!result.empty()) {
            result.push_back(kPathSep);
        }
        if (p.front() == kPathSep) {
            result.clear();
        }
        if (p.back() == kPathSep) {
            result.append(p, 0, p.length() - 1);
        } else {
            result.append(p);
        }
    }
    return result;
}

} // namespace path
} // namespace os
