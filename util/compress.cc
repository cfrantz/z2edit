#include <string>
#include <memory>
#include <zlib.h>

#include "util/compress.h"
#include "util/logging.h"
#include "util/status.h"


std::string ZLib::Compress(const std::string& data) {
    unsigned long size = compressBound(data.size());
    std::string buf;
    buf.resize(size);
    if (compress2((unsigned char*)buf.data(), &size,
                  (const unsigned char*)data.data(), data.size(), -1) != Z_OK) {
        LOG(FATAL, "Failed to compress rom buffer.");
    }
    buf.resize(size);
    return buf;
}

StatusOr<std::string> ZLib::Uncompress(const std::string& data, int size) {
    std::string result;
    if (size == 0)
        size = data.size() * 2;
    int zret;
    for(;;) {
        result.resize(size);
        unsigned long len = size;
        zret = uncompress((unsigned char*)result.data(), &len,
                          (const unsigned char*)data.data(), data.size());
        if (zret == Z_OK) {
            result.resize(len);
            break;
        }
        if (zret == Z_DATA_ERROR) {
            return util::Status(util::error::Code::INVALID_ARGUMENT,
                          "Compressed data corrputed");
        }

        size *= 2;
        if (zret == Z_MEM_ERROR || size < 0) {
            return util::Status(util::error::Code::ABORTED, "Out of memory");
        }
    }
    return result;
}
