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
