#ifndef Z2HD_UTIL_STATUS_H
#define Z2HD_UTIL_STATUS_H
#include "google/protobuf/stubs/status.h"
#include "util/string.h"

namespace util {
namespace error {
using google::protobuf::util::error::Code;
}

using google::protobuf::util::Status;

string StrError(int error);
Status PosixStatus(int error);

}

#endif // Z2HD_UTIL_STATUS_H
