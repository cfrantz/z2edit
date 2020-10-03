error_chain! {
    foreign_links {
        WindowBuildError(::sdl2::video::WindowBuildError);
        Io(::std::io::Error);
        DecodeError(::ron::error::Error);
        PyErr(::pyo3::PyErr);
    }
    errors {
        SdlError(e: String) {
            description("SDL Error"),
            display("SDL Error: {}", e)
        }
    }
}
