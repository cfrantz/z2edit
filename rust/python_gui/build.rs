// Compile gui.cpp against the imgui sources

use std::path::Path;

fn main() {
    let python3 = pkg_config::probe_library("python3").expect("You need python3");

    let cimgui_include_path =
        std::env::var_os("DEP_IMGUI_THIRD_PARTY").expect("DEP_IMGUI_THIRD_PARTY not defined");
    let imgui_include_path = Path::new(&cimgui_include_path).join("imgui");

    cc::Build::new()
        .file("gui.cpp")
        .cpp(true)
        .include(&cimgui_include_path)
        .include(&imgui_include_path)
        .include("../pybind11/include")
        .includes(python3.include_paths)
        .compile("gui");

    println!("cargo::rustc-link-lib=static=gui");
    println!("cargo::rerun-if-changed=gui.cpp");
}
