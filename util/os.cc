#include <cstdio>
#include <cstdlib>
#include <time.h>
#include <unistd.h>
#include <limits.h>
#include <sched.h>
#ifdef _WIN32
#include <windows.h>
#endif

#include "util/os.h"

#include "absl/log/log.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_split.h"

namespace os {
namespace {
char ApplicationName[256];
}  // namespace

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

void SetApplicationName(const char *name) {
    strncpy(ApplicationName, name, sizeof(ApplicationName) - 1);
}

std::string GetApplicationName() {
    std::string name = ApplicationName;
    if (name == "") {
        name = path::Split(path::Executable()).back();
    }
    return name;
}

namespace path {
char kPathSep = '/';

std::string Executable() {
    char exepath[PATH_MAX] = {0, };
#ifdef _WIN32
    GetModuleFileName(nullptr, exepath, PATH_MAX);
#else
    char self[PATH_MAX];
#ifdef __linux__
    snprintf(self, sizeof(self), "/proc/self/exe");
#elif defined(__FreeBSD__)
    snprintf(self, sizeof(self), "/proc/%d/file", getpid());
#else
#error "Don't know how to get Executable"
#endif
    if (!realpath(self, exepath)) {
        perror("realpath");
    }
#endif
    return std::string(exepath);
}

std::string ResourceDir(const std::string& name) {
    std::string exe = Executable();
    if (exe.find("_bazel_") != std::string::npos) {
        LOG(INFO) << "Executable contains '_bazel_'. Assuming ResourceDir is CWD.";
        return GetCWD();
    }
    std::vector<std::string> path = Split(exe);
    path.pop_back(); // Remove exe filename

#ifdef _WIN32
    LOG(INFO) << "Windows: assuming ResourceDir is" << Join(path);
#else
    if (path.back() == "bin") {
        path.pop_back();
        path.push_back("share");
        path.push_back(name);
    } else {
        LOG(INFO) << "Unix: Did not find 'bin' in executable path. "
                  "Assuming ResourcDir is " << Join(path);
    }
#endif
    return Join(path);
}

std::string DataPath(const std::vector<std::string>& components) {
    std::vector<std::string> p;
#ifdef _WIN32
    const char *data = getenv("LOCALAPPDATA");
    const char *home = getenv("USERPROFILE");
#else
    const char *data = getenv("XDG_DATA_HOME");
    const char *home = getenv("HOME");
#endif

    if (data) {
        p = Split(data);
        p.push_back(absl::StrCat(GetApplicationName()));
    } else if (home) {
        p = Split(home);
        p.push_back(absl::StrCat(".", GetApplicationName()));
    } else {
        LOG(ERROR) << "ConfigPath is unknown.";
    }
    p.insert(p.end(), components.begin(), components.end());
    return Join(p);
}

std::string Join(const std::vector<std::string>& components) {
    std::string result;

    for(const auto& p : components) {
        if (!result.empty() && result.back() != kPathSep) {
            result.push_back(kPathSep);
        }
        if (p.front() == kPathSep) {
            result.clear();
        }
        if (p.back() == kPathSep) {
            size_t len = p.length() - 1;
            //  In case we're adding a single "/" component.
            if (len == 0) len = 1;
            result.append(p, 0, len);
        } else {
            result.append(p);
        }
    }
    return result;
}

std::vector<std::string> Split(const std::string& path) {
    std::vector<std::string> p = absl::StrSplit(path, absl::ByAnyChar("\\/"));
    if (p[0] == "") {
        p[0] = "/";
    }
    return p;
}

} // namespace path
} // namespace os
