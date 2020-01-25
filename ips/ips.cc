#include <cstdint>
#include <string>

#include "ips/ips.h"
#include "util/status.h"
#include "util/statusor.h"

namespace ips {
namespace {
void write_uint3(std::string *p, uint32_t val) {
    p->append(1, (val >> 16) & 0xFF);
    p->append(1, (val >> 8) & 0xFF);
    p->append(1, (val >> 0) & 0xFF);
}

void write_uint2(std::string *p, uint16_t val) {
    p->append(1, (val >> 8) & 0xFF);
    p->append(1, (val >> 0) & 0xFF);
}

int32_t read_uint(const std::string& p, size_t offset, size_t len) {
    int32_t ret = 0;
    for(size_t i=0; i<len; i++) {
        size_t k = i+offset;
        if (k < p.size()) {
            ret <<= 8;
            ret |= (uint8_t)p.at(k);
        } else {
            return -1;
        }
    }
    return ret;
}
}  // namespace

std::string CreatePatch(const string& original, const string& modified) {
    std::string patch = "PATCH";
    size_t len = original.size();
    size_t i;
    for(i=0; i<len; i++) {
        if (i >= modified.size())
            break;
        if (original.at(i) == modified.at(i))
            continue;

        size_t plen = 0;
        while(plen < 0xFFFF && plen+i < len &&
              original.at(i+plen) != modified.at(i+plen)) {
            plen += 1;
        }

        write_uint3(&patch, i);
        write_uint2(&patch, plen);
        patch.append(modified.substr(i, plen));
        i += plen;
    }

    while(i < modified.size()) {
        size_t chunk = modified.size() - i;
        if (chunk > 0xFFFF) chunk = 0xFFFF;

        write_uint3(&patch, i);
        write_uint2(&patch, chunk);
        patch.append(modified.substr(i, chunk));
        i += chunk;
    }
    patch.append("EOF");
    return patch;
}

StatusOr<std::string> ApplyPatch(const string& original,
                                 const string& patch) {
    size_t i = 0;
    int32_t offset, len;
    bool rle_chunk;

    if (patch.substr(i, 5) != "PATCH") {
        return util::Status(util::error::Code::INVALID_ARGUMENT,
                            "Bad IPS header");
    }
    i += 5;
    std::string modified = original;
    while(i < patch.size()) {
        if (patch.substr(i, 3) == "EOF")
            break;
        if ((offset = read_uint(patch, i, 3)) < 0) {
            return util::Status(util::error::Code::INVALID_ARGUMENT,
                                "Premature end of patch reading offset");
        }
        i += 3;
        if ((len = read_uint(patch, i, 2)) < 0) {
            return util::Status(util::error::Code::INVALID_ARGUMENT,
                                "Premature end of patch reading length");
        }
        i += 2;
        rle_chunk = len == 0;
        if (rle_chunk) {
            // An RLE chunk specifies a number of repeated bytes
            if ((len = read_uint(patch, i, 2)) < 0) {
                return util::Status(util::error::Code::INVALID_ARGUMENT,
                                    "Premature end of patch reading RLE size");
            }
            i += 2;
        }

        if (unsigned(offset+len) > modified.size()) {
            modified.resize(offset+len, '\xff');
        }

#ifdef DEBUG_PRINT
        // TODO: distinguish between PRG and CHR banks.
        int addr = 0x8000 | ((offset - 0x10) & 0x3FFF);
        int bank = (offset - 0x10) / 16384;
        if (bank == 7) addr |= 0xC000;
        printf("Applying patch at bank=%d addr=0x%x for 0x%x bytes\n", bank, addr, len);
        printf("   ");
#endif
        for(int32_t j=0; j<len; j++) {
            if (i >= patch.size()) {
                return util::Status(util::error::Code::INVALID_ARGUMENT,
                                "Premature end of patch reading data");
            }
            modified[offset+j] = patch.at(i);
#ifdef DEBUG_PRINT
            printf(" %02x", patch.at(i) & 0xff);
#endif

            // An RLE chunk is "len" of the same byte
            // In non-RLE mode, we increment the patch offset in the loop
            if (!rle_chunk) i++;
        }
#ifdef DEBUG_PRINT
        printf("\n");
#endif
        // In RLE mode, we increment the patch offset once outside the loop
        if (rle_chunk) i++;
    }
    return modified;
}

}  // namespace ips
