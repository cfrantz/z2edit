error_chain! {
    foreign_links {
        WindowBuildError(::sdl2::video::WindowBuildError);
        Io(::std::io::Error);
    }
    errors {
        SdlError(e: String) {
            description("SDL Error"),
            display("SDL Error: {}", e)
        }
    }
}
