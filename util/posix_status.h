#ifndef PROTONES_UTIL_POSIX_STATUS_H
#define PROTONES_UTIL_POSIX_STATUS_H
#include "absl/status/status.h"

namespace util {
std::string StrError(int error);
absl::Status PosixStatus(int error);
} // namespace util

#endif // PROTONES_UTIL_POSIX_STATUS_H
