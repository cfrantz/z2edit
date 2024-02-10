#include <pybind11/pybind11.h>
#include <pybind11/stl.h>

#include "nes/address.h"
#include "nes/nesfile.h"

namespace nes {
namespace py = pybind11;

// clang-format off
void nes_submodule(py::module_& parent) {
    auto m = parent.def_submodule("nes");
    auto am = m.def_submodule("address");

    address::File::pybind11_bind(am)
        .def(py::init<uint32_t>())
        ;
    address::Prg::pybind11_bind(am)
        .def(py::init<int, uint16_t>())
        ;
    address::Chr::pybind11_bind(am)
        .def(py::init<int, uint16_t>())
        ;

    Bank::pybind11_bind(m);
    Layout::pybind11_bind(m);
    Address::pybind11_bind(m)
        .def(py::init<address::File>())
        .def(py::init<address::Prg>())
        .def(py::init<address::Chr>())
        .def("File", &Address::File)
        .def("Prg", &Address::Prg)
        .def("Chr", &Address::Chr)
        .def("offset", &Address::offset)
        .def("__str__", &Address::to_string)
        .def("__add__", [](Address* self, int v) { return *self + v; })
        .def("__sub__", [](Address* self, int v) { return *self - v; })
        .def("__iadd__", &Address::operator+=)
        .def("__isub__", &Address::operator-=);

    Header::pybind11_bind(m);
    NesFile::pybind11_bind(m);
}
// clang-format on
}  // namespace nes
