#ifndef Z2HD_UTIL_STRUTIL_H
#define Z2HD_UTIL_STRUTIL_H
#include <algorithm>
#include <string>

// Grab useful utility functions from the protobuf library and put them in
// the root namespace.
#include "google/protobuf/stubs/strutil.h"

using google::protobuf::ascii_isalnum;
using google::protobuf::ascii_isdigit;
using google::protobuf::ascii_isspace;
using google::protobuf::ascii_isupper;
using google::protobuf::ascii_islower;
using google::protobuf::ascii_toupper;
using google::protobuf::ascii_tolower;
using google::protobuf::hex_digit_to_int;
;
using google::protobuf::HasPrefixString;
using google::protobuf::StripPrefixString;
using google::protobuf::HasSuffixString;
using google::protobuf::StripSuffixString;
using google::protobuf::StripWhitespace;

using google::protobuf::LowerString;
using google::protobuf::UpperString;
using google::protobuf::ToUpper;

using google::protobuf::StringReplace;
using google::protobuf::SplitStringUsing;
using google::protobuf::Split;
using google::protobuf::SplitStringAllowEmpty;
using google::protobuf::JoinStrings;

using google::protobuf::UnescapeCEscapeSequences;
using google::protobuf::UnescapeCEscapeString;

using google::protobuf::CEscape;
using google::protobuf::CEscapeAndAppend;

namespace strings {
using google::protobuf::strings::Utf8SafeCEscape;
using google::protobuf::strings::CHexEscape;
}

using google::protobuf::strto32_adaptor;
using google::protobuf::strtou32_adaptor;
using google::protobuf::strto32;
using google::protobuf::strtou32;
using google::protobuf::strto64;
using google::protobuf::strtou64;

using google::protobuf::safe_strtob;
using google::protobuf::safe_strto32;
using google::protobuf::safe_strtou32;
using google::protobuf::safe_strto64;
using google::protobuf::safe_strtou64;
using google::protobuf::safe_strtof;
using google::protobuf::safe_strtod;

using google::protobuf::FastInt32ToBuffer;
using google::protobuf::FastInt64ToBuffer;
using google::protobuf::FastHexToBuffer;
using google::protobuf::FastHex32ToBuffer;
using google::protobuf::FastHex64ToBuffer;
using google::protobuf::FastIntToBuffer;
using google::protobuf::FastUIntToBuffer;
using google::protobuf::FastLongToBuffer;
using google::protobuf::FastULongToBuffer;

using google::protobuf::FastInt32ToBufferLeft;
using google::protobuf::FastUInt32ToBufferLeft;
using google::protobuf::FastInt64ToBufferLeft;
using google::protobuf::FastUInt64ToBufferLeft;

using google::protobuf::SimpleBtoa;
using google::protobuf::SimpleItoa;
using google::protobuf::SimpleDtoa;
using google::protobuf::SimpleFtoa;
using google::protobuf::DoubleToBuffer;
using google::protobuf::FloatToBuffer;

using google::protobuf::StrCat;
using google::protobuf::StrAppend;
using google::protobuf::Join;
using google::protobuf::ToHex;
using google::protobuf::GlobalReplaceSubstring;
using google::protobuf::Base64Unescape;
using google::protobuf::WebSafeBase64Unescape;
using google::protobuf::CalculateBase64EscapedLen;
using google::protobuf::Base64Escape;
using google::protobuf::WebSafeBase64Escape;
using google::protobuf::EncodeAsUTF8Char;
using google::protobuf::UTF8FirstLetterNumBytes;

inline bool ends_with(const std::string& value, const std::string& ending) {
    if (ending.size() > value.size()) return false;
    return std::equal(ending.rbegin(), ending.rend(), value.rbegin());
}

#endif // Z2HD_UTIL_STRUTIL_H
