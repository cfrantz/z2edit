#ifndef Z2UTIL_IPS_IPS_H
#define Z2UTIL_IPS_IPS_H

#include <string>
#include "util/statusor.h"

namespace ips {

std::string CreatePatch(const std::string& original, const std::string& modified);
StatusOr<std::string> ApplyPatch(const std::string& original, const std::string& patch);

}  // namespace

#endif // Z2UTIL_IPS_IPS_H
