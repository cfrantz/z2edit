#include <pybind11/pybind11.h>

#include "app.h"

namespace py = pybind11;
namespace project {

extern "C" PyObject* PyInit_gui();

PYBIND11_MODULE(application, m) {
    m.add_object("gui", PyInit_gui());

    py::class_<App, PyApp>(m, "App")
        .def(py::init<>())
        .def("init", &App::Init)
        .def("menu_bar_hook", &App::MenuBarHook)
        .def("menu_hook", &App::MenuHook)
        .def("run", &App::Run, py::call_guard<py::gil_scoped_release>())
        .def_readwrite("running", &App::running_)
        .def_readwrite("clear_color", &App::clear_color_);
}

}  // namespace project
