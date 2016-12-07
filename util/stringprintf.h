#ifndef Z2HD_UTIL_STRINGPRINTF_H
#define Z2HD_UTIL_STRINGPRINTF_H

// Grab useful utility functions from the protobuf library and put them in
// the root namespace.
#include "google/protobuf/stubs/stringprintf.h"

using google::protobuf::StringPrintf;
using google::protobuf::StringAppendF;
using google::protobuf::StringAppendV;
using google::protobuf::kStringPrintfVectorMaxArgs;
using google::protobuf::StringPrintfVector;

#endif // Z2HD_UTIL_STRINGPRINTF_H
