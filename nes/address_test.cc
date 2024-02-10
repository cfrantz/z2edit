#include "nes/address.h"

#include "gmock/gmock.h"
#include "gtest/gtest.h"

namespace nes {
namespace {

using ::testing::ElementsAre;
using ::testing::Key;

TEST(AddressTest, FileOffset) {
    Layout layout{{16, 16384, 8}, {16 + 131072, 4096, 8}};
    Address addr{address::File{8}};
    EXPECT_EQ(addr.offset(layout), 8);
    Address addr2 = addr + 8;
    EXPECT_EQ(addr2.offset(layout), 16);
}

TEST(AddressTest, PrgOffset) {
    Layout layout{{16, 16384, 8}, {16 + 131072, 4096, 8}};
    Address addr{address::Prg{7, 0xC000}};
    EXPECT_EQ(addr.offset(layout), 16 + 16384 * 7 + (0xC000 & 0x3FFF));
    Address addr2 = addr - 1;
    EXPECT_EQ(addr2.offset(layout), 16 + 16384 * 7 + (0xFFFF & 0x3FFF));
}

TEST(AddressTest, ChrOffset) {
    Layout layout{{16, 16384, 8}, {16 + 131072, 4096, 8}};
    Address addr{address::Chr{2, 0x100}};
    EXPECT_EQ(addr.offset(layout), 139536);
    EXPECT_EQ(addr.offset(layout), 131088 + 2 * 4096 + (0x100 & 0xFFF));
    Address addr2 = addr - 1;
    EXPECT_EQ(addr2.offset(layout), 131088 + 2 * 4096 + (0x0FF & 0xFFF));
}

}  // namespace
}  // namespace nes
