#ifndef PROTONES_UTIL_CRC_H
#define PROTONES_UTIL_CRC_H
#include <cstdint>

uint32_t Crc32(uint32_t crc, const void* buf, size_t length);


#endif // PROTONES_UTIL_CRC_H
