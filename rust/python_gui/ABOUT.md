# About `rust/python_gui`

This rust crate is a `pyo3` binding to create a generic application framework for rust+python programs using the `Dear ImGui` gui toolkit.

In the `src` subdirectory, you'll find the following modules:
- audio.rs: A basic audio framework based on SDL2 audio support.
- dirs.rs: A basic wrapper around the `directories` crate.
- docking.rs: Helper functions for manipulating ImGUI docking support.
- font_awesome_5.rs: `const` definitions for FontAwesome.
- framework.rs: A basic application framework for creating a window and running the main loop of an SDL2-based ImGUI application.
- image.rs: A basic wrapper around SDL2 BMP image support.
- lib.rs: The `python_gui` crate root.
- style.rs: A basic wrapper around ImGUI's `Style` structure.

In addition to binding some basic application building blocks from rust to Python, this module includes the file `gui.cpp`.  This file is a `pybind11` binding from ImGUI to python.  This is necessary to provide the Python components of an application access to ImGUI without having to "double-bind" ImGUI (e.g. ImGUI C++ bound to rust via the `imgui` crate then bound to python via `pyo3`).  This binding was created with the script `generate_python_bindings.py` and a bit of hand tweaking.
