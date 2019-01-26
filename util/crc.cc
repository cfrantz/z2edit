#include <zlib.h>

#include "util/crc.h"

uint32_t Crc32(uint32_t crc, const void* buf, size_t length) {
    // Use zlib's implementation.
    return crc32(crc, static_cast<const Bytef*>(buf), length);
}
