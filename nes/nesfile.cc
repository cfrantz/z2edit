#include "nes/nesfile.h"

#include <cstdint>
#include <cstring>
#include <variant>

#include "util/file.h"
#include "util/status.h"

namespace nes {

absl::Status NesFile::load(const std::string& filename) {
    ASSIGN_OR_RETURN(auto data, File::GetContents(filename));
    data_.resize(data.size(), 0);
    memcpy(data_.data(), data.data(), data.size());
    header_.header = data_[0] << 24 | data_[1] << 16 | data_[2] << 8 | data_[3];
    header_.prg_len = data_[4];
    header_.chr_len = data_[5];
    header_.mapper = (data_[6] >> 4) | (data_[7] & 0xF0);
    header_.mirror = (data_[6] & 1) != 0;
    header_.battery = (data_[6] & 2) != 0;
    header_.trainer = (data_[6] & 4) != 0;
    header_.four_screen = (data_[6] & 8) != 0;

    layout_->prg.start = 16;
    layout_->prg.bank_size = 16384;
    layout_->prg.len = header_.prg_len;

    layout_->chr.start =
        layout_->prg.start + layout_->prg.bank_size * layout_->prg.len;
    // This should be 8192, but its easier to conceptualize each 8K CHR bank
    // as two 4K banks.
    layout_->chr.bank_size = 4096;
    layout_->chr.len = header_.chr_len;
    return absl::OkStatus();
}

absl::Status NesFile::save(const std::string& filename) {
    data_[0] = header_.header >> 24;
    data_[1] = header_.header >> 16;
    data_[2] = header_.header >> 8;
    data_[3] = header_.header >> 0;
    data_[4] = header_.prg_len;
    data_[5] = header_.chr_len;
    data_[6] = (header_.mapper & 0x0F) << 4 ;
    data_[6] |= (header_.mirror ? 1 : 0) ;
    data_[6] |= (header_.battery ? 2 : 0) ;
    data_[6] |= (header_.trainer ? 4 : 0) ;
    data_[6] |= (header_.four_screen ? 8 : 0);
    data_[7] = header_.mapper & 0xF0;
    ASSIGN_OR_RETURN(auto file, File::Open(filename, "wb"));
    return file->Write(data_.data(), data_.size());
}

absl::Status NesFile::insert_bank(const Address& address, size_t banks) {
    uint32_t offset = address.offset(*layout_);
    if (std::holds_alternative<address::Prg>(address.addr)) {
        size_t len = layout_->prg.bank_size * banks;
        if (len % 16384 != 0)
            return absl::InvalidArgumentError(
                "Must add PRG banks in multiples of 16384 bytes");
        std::vector<uint8_t> empty(len, 0xFF);
        data_.insert(data_.begin() + offset, empty.begin(), empty.end());
        layout_->prg.len += banks;
        layout_->chr.start += empty.size();
        header_.prg_len += len / 16384;
    } else if (std::holds_alternative<address::Chr>(address.addr)) {
        size_t len = layout_->chr.bank_size * banks;
        if (len % 8192 != 0)
            return absl::InvalidArgumentError(
                "Must add CHR banks in multiples of 8192 bytes");
        std::vector<uint8_t> empty(len, 0xFF);
        data_.insert(data_.begin() + offset, empty.begin(), empty.end());
        layout_->chr.len += banks;
        header_.chr_len += len / 8192;
    } else {
        return absl::InvalidArgumentError(
            "Cannot insert bank with non-banked address");
    }
    return absl::OkStatus();
}

uint8_t NesFile::read(const Address& address) const {
    uint32_t offset = address.offset(*layout_);
    return data_[offset];
}

uint16_t NesFile::read_word(const Address& address) const {
    return read(address) | read(address + 1) << 8;
}

void NesFile::write(const Address& address, uint8_t val) {
    uint32_t offset = address.offset(*layout_);
    data_[offset] = val;
}

void NesFile::write_word(const Address& address, uint16_t val) {
    write(address, val & 0xFF);
    write(address + 1, val >> 8);
}

void NesFile::pybind11_bind(::pybind11::module_& pybind11_module_) {
    ::pybind11::class_<NesFile>(pybind11_module_, "NesFile")
        .def(::pybind11::init<Layout&>())
        .def_readwrite("layout", &NesFile::layout_)
        .def_readwrite("header", &NesFile::header_)
        .def("load", [](NesFile* self,
                        const std::string& fn) { status_exc(self->load(fn)); })
        .def("save", [](NesFile* self,
                        const std::string& fn) { status_exc(self->save(fn)); })
        .def("insert_bank", &NesFile::insert_bank)
        .def("read", &NesFile::read)
        .def("read_word", &NesFile::read_word)
        .def("write", &NesFile::write)
        .def("write_word", &NesFile::write_word);
}

}  // namespace nes
