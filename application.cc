#include <memory>

#include "absl/flags/commandlineflag.h"
#include "absl/flags/reflection.h"
#include <pybind11/pybind11.h>
#include <pybind11/stl.h>

#include "app.h"

namespace py = pybind11;
namespace project {

extern "C" PyObject* PyInit_gui();

PYBIND11_MODULE(application, m) {
    // Bring in the ImGui bindings as a submodule of the application module.
    m.add_object("gui", PyInit_gui());

    // Export the main application class.
    py::class_<App, PyApp>(m, "App")
        .def(py::init<>())
        .def("init", &App::Init)
        .def("menu_bar_hook", &App::MenuBarHook)
        .def("menu_hook", &App::MenuHook)
        .def("run", &App::Run, py::call_guard<py::gil_scoped_release>())
        .def_readwrite("running", &App::running_)
        .def_readwrite("clear_color", &App::clear_color_)
        ;

    // Export some absl CommandLineFlags stuff so we can integrate C++ flags
    // into the python flags parser.
    py::class_<absl::CommandLineFlag, std::unique_ptr<absl::CommandLineFlag, py::nodelete>>(m, "CommandLineFlag")
        .def_property_readonly("name", &absl::CommandLineFlag::Name)
        .def_property_readonly("filename", &absl::CommandLineFlag::Filename)
        .def_property_readonly("help", &absl::CommandLineFlag::Help)
        .def_property_readonly("is_retired", &absl::CommandLineFlag::IsRetired)
        .def_property_readonly("default", &absl::CommandLineFlag::DefaultValue)
        .def_property_readonly("value", &absl::CommandLineFlag::CurrentValue)
        .def("parse", [](absl::CommandLineFlag* self, std::string_view flag) {
                std::string error;
                if (!self->ParseFrom(flag, &error)) {
                    throw std::invalid_argument(error);
                }
        })
        ;

    m.def("find_command_line_flag"
        , &absl::FindCommandLineFlag
        , py::arg("name"));
    m.def("get_all_flags", []() {
            auto all = absl::GetAllFlags();
            std::map flags(all.cbegin(), all.cend());
            return flags;
    });
}

}  // namespace project
