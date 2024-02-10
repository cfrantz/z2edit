#ifndef Z2E3_NES_ADDRESS_H
#define Z2E3_NES_ADDRESS_H
#include <pybind11/pybind11.h>
#include <pybind11/stl.h>

#include <cstdint>
#include <variant>

#include "ajson/reflect.h"

namespace nes {
namespace address {
struct File : public ::ajson::Reflection {
    File() = default;
    File(const File&) = default;
    File(uint32_t offset) : offset(offset) {}
    uint32_t offset = 0;
    // clang-format off
    OBJECT_CONFIG(
            /*typename=*/ File,
            /*features=*/ (reflection, pybind11),
            /*field_list ... */
            offset
    )
    // clang-format on
};

struct Prg : public ::ajson::Reflection {
    Prg() = default;
    Prg(const Prg&) = default;
    Prg(int bank, uint16_t offset) : bank(bank), offset(offset) {}
    int bank = 0;
    uint16_t offset = 0;
    // clang-format off
    OBJECT_CONFIG(
            /*typename=*/ Prg,
            /*features=*/ (reflection, pybind11),
            /*field_list ... */
            bank,
            offset
    )
    // clang-format on
};

struct Chr : public ::ajson::Reflection {
    Chr() = default;
    Chr(const Chr&) = default;
    Chr(int bank, uint16_t offset) : bank(bank), offset(offset) {}
    int bank = 0;
    uint16_t offset = 0;
    // clang-format off
    OBJECT_CONFIG(
            /*typename=*/ Chr,
            /*features=*/ (reflection, pybind11),
            /*field_list ... */
            bank,
            offset
    )
    // clang-format on
};
}  // namespace address

struct Bank : public ::ajson::Reflection {
    Bank() = default;
    Bank(const Bank&) = default;
    Bank(uint32_t start, uint32_t bank_size, uint32_t len)
        : start(start), bank_size(bank_size), len(len) {}
    uint32_t start;
    uint32_t bank_size;
    uint32_t len;
    // clang-format off
    OBJECT_CONFIG(
            /*typename=*/ Bank,
            /*features=*/ (reflection, pybind11),
            /*field_list ... */
            start,
            bank_size,
            len
    )
    // clang-format on
};

struct Layout : public ::ajson::Reflection {
    Layout() = default;
    Layout(const Layout&) = default;
    Layout(const Bank& prg, const Bank& chr) : prg(prg), chr(chr) {}
    Bank prg;
    Bank chr;
    // clang-format off
    OBJECT_CONFIG(
            /*typename=*/ Layout,
            /*features=*/ (reflection, pybind11),
            /*field_list ... */
            prg,
            chr
    )
    // clang-format on
};

class Address : public ::ajson::Reflection {
  public:
    Address() = default;
    Address(const Address&) = default;
    Address(std::variant<address::File, address::Prg, address::Chr> a)
        : addr(a) {}
    static Address File(uint32_t offset);
    static Address Prg(int bank, uint16_t offset);
    static Address Chr(int bank, uint16_t offset);

    std::variant<address::File, address::Prg, address::Chr> addr;
    uint32_t offset(const Layout& layout) const;
    std::string to_string() const;
    Address& operator+=(int v);
    Address& operator-=(int v);
    // clang-format off
    OBJECT_CONFIG(
            /*typename=*/ Address,
            /*features=*/ (reflection, pybind11),
            /*field_list ... */
            (addr, .metadata="variant:file,prg,chr")
    )
    // clang-format on
};

inline Address operator+(Address lhs, int rhs) { return lhs += rhs; }
inline Address operator-(Address lhs, int rhs) { return lhs -= rhs; }

}  // namespace nes

DEFINE_TYPE(::nes::address::File);
DEFINE_TYPE(::nes::address::Prg);
DEFINE_TYPE(::nes::address::Chr);
DEFINE_TYPE(::nes::Bank);
DEFINE_TYPE(::nes::Layout);
DEFINE_TYPE(::nes::Address);

#endif  // Z2E3_NES_ADDRESS_H
