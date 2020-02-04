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

void SetApplicationName(const char *name);
std::string GetApplicationName();

namespace path {
std::string Executable();

std::string ResourceDir(const std::string& name);
inline std::string ResourceDir() { return ResourceDir(GetApplicationName()); }

std::string DataPath(const std::vector<std::string>& components);
inline std::string DataPath() { return DataPath({}); }

std::string Join(const std::vector<std::string>& components);
std::vector<std::string> Split(const std::string& path);
}
}

#endif // Z2HD_UTIL_OS_H
