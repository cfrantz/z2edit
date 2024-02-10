#ifndef Z2E3_NES_NESFILE_H
#define Z2E3_NES_NESFILE_H
#include <pybind11/pybind11.h>
#include <pybind11/stl.h>

#include <cstdint>

#include "absl/status/status.h"
#include "ajson/reflect.h"
#include "nes/address.h"

namespace nes {
struct Header : public ::ajson::Reflection {
    uint32_t header = 0;
    uint8_t prg_len = 0;
    uint8_t chr_len = 0;
    uint16_t mapper = 0;
    bool mirror = false;
    bool battery = false;
    bool trainer = false;
    bool four_screen = false;
    // clang-format off
    OBJECT_CONFIG(
            /*typename=*/ Header,
            /*features=*/ (reflection, pybind11),
            /*field_list ... */
            header,
            prg_len,
            chr_len,
            mapper,
            mirror,
            battery,
            trainer,
            four_screen
    )
    // clang-format on
};

class NesFile {
  public:
    NesFile(Layout& layout) : layout_(&layout) {}
    absl::Status load(const std::string& filename);
    absl::Status save(const std::string& filename);

    uint8_t read(const Address& address) const;
    uint16_t read_word(const Address& address) const;
    void write(const Address& address, uint8_t val);
    void write_word(const Address& address, uint16_t val);

    absl::Status insert_bank(const Address& address, size_t banks = 1);

    static void pybind11_bind(::pybind11::module_& pybind11_module_);

  private:
    Layout* layout_;
    Header header_;
    std::vector<uint8_t> data_;
};

}  // namespace nes
DEFINE_TYPE(::nes::Header);

#endif  // Z2E3_NES_NESFILE_H
