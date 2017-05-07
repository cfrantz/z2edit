#ifndef Z2UTIL_UTIL_COMPRESS_H
#define Z2UTIL_UTIL_COMPRESS_H
#include <string>

#include "util/statusor.h"

class ZLib {
  public:
    static std::string Compress(const std::string& data);
    static StatusOr<std::string> Uncompress(const std::string& data, int size=0);
};

#endif // Z2UTIL_UTIL_COMPRESS_H
