#ifndef Z2HD_UTIL_OS_H
#define Z2HD_UTIL_OS_H
#include <vector>

#include "util/string.h"

namespace os {
string GetCWD();

#ifndef _WIN32
void Yield();
#endif

namespace path {
string Join(const std::vector<string>& components);
}
}

#endif // Z2HD_UTIL_OS_H
