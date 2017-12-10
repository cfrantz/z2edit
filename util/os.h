#ifndef Z2HD_UTIL_OS_H
#define Z2HD_UTIL_OS_H
#include <vector>
#include <string>

namespace os {

std::string GetCWD();
void SchedulerYield();

int64_t utime_now();
std::string CTime(int64_t time_us);

std::string TempFilename(const std::string& filename);
int System(const std::string& cmd, bool background=false);

namespace path {
std::string Join(const std::vector<std::string>& components);
}
}

#endif // Z2HD_UTIL_OS_H
