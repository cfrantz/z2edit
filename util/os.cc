#include <cstdlib>
#include <time.h>
#include <unistd.h>
#include <limits.h>
#include <sched.h>
#ifdef _WIN32
#include <windows.h>
#endif

#include "absl/strings/str_cat.h"
#include "util/os.h"

namespace os {
std::string GetCWD() {
    char path[PATH_MAX];
    std::string result;

    if (getcwd(path, PATH_MAX)) {
        result = std::string(path);
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
    // FileTime is in 100ns increments.  Convert to microseconds.
    now = (int64_t(ft.dwLowDateTime) | int64_t(ft.dwHighDateTime) << 32) / 10;
    // Windows time is since Jan 1, 1601
    // Unix time is since Jan 1, 1970
#define SEC_TO_UNIX_EPOCH 11644473600LL
    now -= SEC_TO_UNIX_EPOCH * 1000000LL;
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
    return system(absl::StrCat(header, cmd, trailer).c_str());
}

namespace path {
char kPathSep = '/';

std::string Join(const std::vector<std::string>& components) {
    std::string result;

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
