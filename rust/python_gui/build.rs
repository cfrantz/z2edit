// Compile gui.cpp against the imgui sources

use std::path::Path;

fn main() {
    let pythoninc =
        std::env::var("PYTHON_HEADERS").unwrap_or("/usr/include/python3.12".to_string());

    let cimgui_include_path =
        std::env::var_os("DEP_IMGUI_THIRD_PARTY").expect("DEP_IMGUI_THIRD_PARTY not defined");
    let imgui_include_path = Path::new(&cimgui_include_path).join("imgui");

    cc::Build::new()
        .file("gui.cpp")
        .cpp(true)
        .include(&cimgui_include_path)
        .include(&imgui_include_path)
        .include("../pybind11/include")
        .include(pythoninc)
        .compile("gui");

    println!("cargo::rustc-link-lib=static=gui");
    println!("cargo::rerun-if-changed=gui.cpp");
}
