use crate::nes::Address;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::ops::Range;

error_chain! {
    foreign_links {
        DecodeError(::ron::error::Error);
        Io(::std::io::Error);
        Convert(::core::num::ParseIntError);
        Json(::serde_json::error::Error);
        PyErr(::pyo3::PyErr);
        WindowBuildError(::sdl2::video::WindowBuildError);
        MismatchedQuotes(::shellwords::MismatchedQuotes);
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
        ConfigError(message: String) {
            description("Config error"),
            display("Config error: {}", message),
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
        IdPathBadRange(category: String, range: Range<usize>) {
            description("Bad IdPath length"),
            display("Bad IdPath length: {} expecting length in range {}..{}",
                    category, range.start, range.end),
        }
        CommitIndexError(index: isize) {
            description("Bad commit index"),
            display("Bad commit index {}", index),
        }
        NotImplemented(name: String) {
            description("Not Implemented"),
            display("Not Implemented: {}", name),
        }
        FreeSpaceError(error: String) {
            description("FreeSpace Error"),
            display("FreeSpace Error: {}", error),
        }
        OutOfMemory(bank: isize) {
            description("Out of Memory"),
            display("Out of memory in bank {}", bank),
        }
        TransformationError(error: String) {
            description("Transformation Error"),
            display("Transformation Error: {}", error),
        }
        LengthError(error: String) {
            description("Length Error"),
            display("Length Error: {}", error),
        }
    }
}

// Hack: wrap our errors in a generic Python Exception.
impl std::convert::From<Error> for PyErr {
    fn from(err: Error) -> PyErr {
        PyException::new_err(err.to_string())
    }
}
