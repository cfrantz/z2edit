#include <time.h>
#include <unistd.h>
#include <limits.h>
#include <sched.h>

#include "util/os.h"

namespace os {
string GetCWD() {
    char path[PATH_MAX];
    string result;

    if (getcwd(path, PATH_MAX)) {
        result = string(path);
    }
    return result;
}

void Yield() {
    sched_yield();
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
    now = int64_t(ft.dwLowDateTime) | int64_t(ft.HighDateTime) << 32;
#endif
    return now;
}

std::string CTime(int64_t time_us) {
    time_t t = time_us / 1000000;
    char buf[32];
    ctime_r(&t, buf);
    if (buf[24] == '\n') buf[24] = 0;
    return buf;
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
