#ifndef Z2HD_UTIL_OS_H
#define Z2HD_UTIL_OS_H
#include <vector>

#include "util/string.h"

namespace os {
string GetCWD();
void Yield();

namespace path {
string Join(const std::vector<string>& components);
}
}

#endif // Z2HD_UTIL_OS_H
