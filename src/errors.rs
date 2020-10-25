use crate::nes::Address;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

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
        UnknownSegmentError(segment: String) {
            description("Unknown segment name"),
            display("Unknown segment name: {}", segment),
        }
        AddressBoundError(address: Address) {
            description("Address bound error"),
            display("Address out of bounds: {:?}", address),
        }
        AddressTypeError(address: Address) {
            description("Address type error"),
            display("Address type not supported in this context: {:?}", address),
        }
        LayoutError(message: String) {
            description("Layout error"),
            display("Layout error: {}", message),
        }
        ConfigNotFound(which: String) {
            description("Config not found"),
            display("Config not found: {}", which),
        }
        IdPathNotFound(path: String) {
            description("IdPath not found"),
            display("IdPath not found: {}", path),
        }
        IdPathBadLength(category: String, len: usize) {
            description("Bad IdPath length"),
            display("Bad IdPath length: {} expecting length {}", category, len),
        }
        CommitIndexError(index: isize) {
            description("Bad commit index"),
            display("Bad commit index {}", index),
        }
        NoGuiImplemented(name: String) {
            description("No GUI Implemented"),
            display("No GUI Implemented for {}", name),
        }
    }
}

// Hack: wrap our errors in a generic Python Exception.
impl std::convert::From<Error> for PyErr {
    fn from(err: Error) -> PyErr {
        PyException::new_err(err.to_string())
    }
}
