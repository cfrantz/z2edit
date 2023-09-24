#ifndef PROJECT_UTIL_COMPRESS_H
#define PROJECT_UTIL_COMPRESS_H
#include <string>

#include "absl/status/statusor.h"

class ZLib {
  public:
    static std::string Compress(const std::string& data);
    static absl::StatusOr<std::string> Uncompress(const std::string& data, int size=0);
};

#endif // PROJECT_UTIL_COMPRESS_H
