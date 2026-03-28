# About `rust/python_gui`

This crate provides a general-purpose, high-performance framework for building hybrid Rust and Python applications using the `Dear ImGui` toolkit. It is designed to be a reusable foundation for any project requiring a seamless GUI bridge between the two languages.

### Architecture
The framework avoids "double-binding" ImGUI by providing a unified bridge:
1. **Rust Side**: Provides the application lifecycle (SDL2), audio/font support, and `pyo3` bindings for core utilities.
2. **Python Side**: Provides direct access to ImGUI via `pybind11` (see `gui.cpp`), allowing Python components to participate in the rendering loop alongside Rust logic.

### Core Modules
- `audio.rs`: SDL2-based audio framework.
- `dirs.rs`: Cross-platform directory management.
- `docking.rs`: ImGUI docking layout helpers.
- `font_awesome_5.rs`: FontAwesome icon constants.
- `framework.rs`: The main application loop and window management.
- `image.rs`: SDL2 image loading support.
- `lib.rs`: The crate root and `pyo3` entry point.
- `style.rs`: ImGUI style customization.

The `gui.cpp` file and `generate_python_bindings.py` script manage the direct C++/Python ImGUI bindings that enable the framework's hybrid capabilities.
