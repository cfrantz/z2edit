use pyo3::class::basic::CompareOp;
use pyo3::class::{PyNumberProtocol, PyObjectProtocol};
use pyo3::exceptions::{PyException, PyNotImplementedError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::convert::From;

use crate::nes::Address;

#[pyclass]
#[derive(Debug, Copy, Clone)]
pub struct PyAddress {
    pub address: Address,
}

impl From<Address> for PyAddress {
    fn from(a: Address) -> Self {
        PyAddress { address: a }
    }
}

impl From<PyAddress> for Address {
    fn from(a: PyAddress) -> Self {
        a.address
    }
}

#[pymethods]
impl PyAddress {
    #[new]
    fn new(init: &PyDict) -> PyResult<PyAddress> {
        let keys = init.keys();
        if keys.len() == 1 {
            let key = keys.get_item(0).extract::<String>()?;
            let keylower = key.to_lowercase();
            let val = init.get_item(&key).expect("PyAddress dict value");
            let val = val.downcast::<PyList>()?;
            match keylower.as_str() {
                "prg" => Ok(PyAddress {
                    address: Address::Prg(
                        val.get_item(0).extract::<isize>()?,
                        val.get_item(1).extract::<u16>()?,
                    ),
                }),
                "chr" => Ok(PyAddress {
                    address: Address::Chr(
                        val.get_item(0).extract::<isize>()?,
                        val.get_item(1).extract::<u16>()?,
                    ),
                }),

                _ => Err(PyNotImplementedError::new_err(format!(
                    "Cannot create PyAddress from {:?}",
                    key
                ))),
            }
        } else {
            Err(PyException::new_err("expected exactly one key in dict"))
        }
    }

    #[staticmethod]
    fn file(addr: usize) -> PyAddress {
        PyAddress {
            address: Address::File(addr),
        }
    }

    #[staticmethod]
    fn segment(segment: &str, addr: usize) -> PyAddress {
        PyAddress {
            address: Address::new_seg(segment, addr),
        }
    }

    #[staticmethod]
    fn banked(bankname: &str, bank: isize, addr: usize) -> PyAddress {
        PyAddress {
            address: Address::new_bank(bankname, bank, addr),
        }
    }

    #[staticmethod]
    fn prg(bank: isize, addr: u16) -> PyAddress {
        PyAddress {
            address: Address::Prg(bank, addr),
        }
    }

    #[staticmethod]
    fn chr(bank: isize, addr: u16) -> PyAddress {
        PyAddress {
            address: Address::Chr(bank, addr),
        }
    }

    fn bank(&self) -> Option<(String, isize)> {
        if let Some((name, offset)) = self.address.bank() {
            Some((name.to_owned(), offset))
        } else {
            None
        }
    }

    fn addr(&self) -> i64 {
        self.address.raw()
    }

    fn in_range(&self, addr: &PyAddress, length: usize) -> bool {
        self.address.in_range(addr.address, length)
    }
}

#[pyproto]
impl PyObjectProtocol for PyAddress {
    fn __repr__(&self) -> PyResult<String> {
        Ok(match self.address {
            Address::File(x) => format!("PyAddress.file(0x{:x})", x),
            Address::Seg(n, x) => format!("PyAddress.segment({}, 0x{:x})", n.as_str(), x),
            Address::Cpu(x) => format!("PyAddress.cpu(0x{:x})", x),
            Address::Bank(n, b, x) => format!("PyAddress.banked({}, {}, 0x{:x})", n.as_str(), b, x),
            Address::Prg(b, x) => format!("PyAddress.prg({}, 0x{:x})", b, x),
            Address::Chr(b, x) => format!("PyAddress.chr({}, 0x{:x})", b, x),
        })
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(match self.address {
            Address::File(x) => format!("File(0x{:x})", x),
            Address::Seg(n, x) => format!("Segment({}, 0x{:x})", n.as_str(), x),
            Address::Cpu(x) => format!("Cpu(0x{:x})", x),
            Address::Bank(n, b, x) => format!("Banked({}, {}, 0x{:x})", n.as_str(), b, x),
            Address::Prg(b, x) => format!("Prg({}, 0x{:x})", b, x),
            Address::Chr(b, x) => format!("Chr({}, 0x{:x})", b, x),
        })
    }

    fn __hash__(&self) -> isize {
        match self.address {
            Address::File(a) => 0x0100_0000_0000_0000 | a as isize,
            Address::Seg(s, a) => 0x0200_0000_0000_0000 | (s.ival() << 48) as isize | a as isize,
            Address::Cpu(a) => a as isize,
            Address::Bank(s, b, a) => {
                0x0300_0000_0000_0000 | (s.ival() << 48) as isize | (b & 0xffff) << 32 | a as isize
            }
            Address::Prg(b, a) => 0x0400_0000_0000_0000 | (b & 0xffff) << 32 | a as isize,
            Address::Chr(b, a) => 0x0500_0000_0000_0000 | (b & 0xffff) << 32 | a as isize,
        }
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<bool> {
        let other = other.extract::<PyAddress>().unwrap_or(PyAddress {
            // Hack: a bullshit address.
            address: Address::File(usize::MAX),
        });

        match op {
            CompareOp::Eq => Ok(self.address == other.address),
            CompareOp::Ne => Ok(self.address != other.address),
            _ => Err(PyNotImplementedError::new_err(format!(
                "CompareOp::{:?} not implemented for PyAddress",
                op
            ))),
        }
    }
}

#[pyproto]
impl PyNumberProtocol for PyAddress {
    fn __add__(lhs: PyAddress, rhs: isize) -> Self {
        PyAddress {
            address: lhs.address + rhs,
        }
    }
    fn __sub__(lhs: PyAddress, rhs: isize) -> Self {
        PyAddress {
            address: lhs.address - rhs,
        }
    }
    fn __iadd__(&mut self, rhs: isize) {
        self.address = self.address + rhs;
    }
    fn __isub__(&mut self, rhs: isize) {
        self.address = self.address - rhs;
    }
}
