use pyo3::class::basic::CompareOp;
use pyo3::class::{PyNumberProtocol, PyObjectProtocol};
use pyo3::exceptions::{PyException, PyNotImplementedError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use std::convert::From;

use crate::nes::Address as RealAddress;

#[pyclass]
#[derive(Debug, Copy, Clone)]
pub struct Address {
    pub address: RealAddress,
}

impl From<RealAddress> for Address {
    fn from(a: RealAddress) -> Self {
        Address { address: a }
    }
}

impl From<Address> for RealAddress {
    fn from(a: Address) -> Self {
        a.address
    }
}

#[pymethods]
impl Address {
    #[new]
    fn new(value: &PyAny) -> PyResult<Address> {
        if let Ok(init) = value.extract::<Address>() {
            return Ok(init.clone());
        } else if let Ok(init) = value.downcast::<PyDict>() {
            let keys = init.keys();
            if keys.len() == 1 {
                let key = keys.get_item(0).extract::<String>()?;
                let keylower = key.to_lowercase();
                let val = init.get_item(&key).expect("Address dict value");
                let val = val.downcast::<PyList>()?;
                match keylower.as_str() {
                    "prg" => Ok(Address {
                        address: RealAddress::Prg(
                            val.get_item(0).extract::<isize>()?,
                            val.get_item(1).extract::<u16>()?,
                        ),
                    }),
                    "chr" => Ok(Address {
                        address: RealAddress::Chr(
                            val.get_item(0).extract::<isize>()?,
                            val.get_item(1).extract::<u16>()?,
                        ),
                    }),

                    _ => Err(PyNotImplementedError::new_err(format!(
                        "Cannot create Address from {:?}",
                        key
                    ))),
                }
            } else {
                Err(PyException::new_err("expected exactly one key in dict"))
            }
        } else {
            Err(PyException::new_err("Unknown type"))
        }
    }

    #[staticmethod]
    fn file(addr: usize) -> Address {
        Address {
            address: RealAddress::File(addr),
        }
    }

    #[staticmethod]
    fn segment(segment: &str, addr: usize) -> Address {
        Address {
            address: RealAddress::new_seg(segment, addr),
        }
    }

    #[staticmethod]
    fn banked(bankname: &str, bank: isize, addr: usize) -> Address {
        Address {
            address: RealAddress::new_bank(bankname, bank, addr),
        }
    }

    #[staticmethod]
    fn prg(bank: isize, addr: u16) -> Address {
        Address {
            address: RealAddress::Prg(bank, addr),
        }
    }

    #[staticmethod]
    fn chr(bank: isize, addr: u16) -> Address {
        Address {
            address: RealAddress::Chr(bank, addr),
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

    fn in_range(&self, addr: &Address, length: usize) -> bool {
        self.address.in_range(addr.address, length)
    }
}

#[pyproto]
impl PyObjectProtocol for Address {
    fn __repr__(&self) -> PyResult<String> {
        Ok(match self.address {
            RealAddress::File(x) => format!("Address.file(0x{:x})", x),
            RealAddress::Seg(n, x) => format!("Address.segment({}, 0x{:x})", n.as_str(), x),
            RealAddress::Cpu(x) => format!("Address.cpu(0x{:x})", x),
            RealAddress::Bank(n, b, x) => {
                format!("Address.banked({}, {}, 0x{:x})", n.as_str(), b, x)
            }
            RealAddress::Prg(b, x) => format!("Address.prg({}, 0x{:x})", b, x),
            RealAddress::Chr(b, x) => format!("Address.chr({}, 0x{:x})", b, x),
        })
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(match self.address {
            RealAddress::File(x) => format!("File(0x{:x})", x),
            RealAddress::Seg(n, x) => format!("Segment({}, 0x{:x})", n.as_str(), x),
            RealAddress::Cpu(x) => format!("Cpu(0x{:x})", x),
            RealAddress::Bank(n, b, x) => format!("Banked({}, {}, 0x{:x})", n.as_str(), b, x),
            RealAddress::Prg(b, x) => format!("Prg({}, 0x{:x})", b, x),
            RealAddress::Chr(b, x) => format!("Chr({}, 0x{:x})", b, x),
        })
    }

    fn __hash__(&self) -> isize {
        match self.address {
            RealAddress::File(a) => 0x0100_0000_0000_0000 | a as isize,
            RealAddress::Seg(s, a) => {
                0x0200_0000_0000_0000 | (s.ival() << 48) as isize | a as isize
            }
            RealAddress::Cpu(a) => a as isize,
            RealAddress::Bank(s, b, a) => {
                0x0300_0000_0000_0000 | (s.ival() << 48) as isize | (b & 0xffff) << 32 | a as isize
            }
            RealAddress::Prg(b, a) => 0x0400_0000_0000_0000 | (b & 0xffff) << 32 | a as isize,
            RealAddress::Chr(b, a) => 0x0500_0000_0000_0000 | (b & 0xffff) << 32 | a as isize,
        }
    }

    fn __richcmp__(&self, other: &PyAny, op: CompareOp) -> PyResult<bool> {
        let other = other.extract::<Address>().unwrap_or(Address {
            // Hack: a bullshit address.
            address: RealAddress::File(usize::MAX),
        });

        match op {
            CompareOp::Eq => Ok(self.address == other.address),
            CompareOp::Ne => Ok(self.address != other.address),
            _ => Err(PyNotImplementedError::new_err(format!(
                "CompareOp::{:?} not implemented for Address",
                op
            ))),
        }
    }
}

#[pyproto]
impl PyNumberProtocol for Address {
    fn __add__(lhs: Address, rhs: isize) -> Self {
        Address {
            address: lhs.address + rhs,
        }
    }
    fn __sub__(lhs: Address, rhs: isize) -> Self {
        Address {
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
