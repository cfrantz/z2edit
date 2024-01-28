#include <pybind11/pybind11.h>
#include <pybind11/stl.h>

#include "ajson/test_lib.h"

namespace py = pybind11;
namespace ajson {

PYBIND11_MODULE(python_test_lib, m) {
    Foo::pybind11_bind(m);
    Bar::pybind11_bind(m);
    Baz::pybind11_bind(m);
}

}  // namespace ajson
