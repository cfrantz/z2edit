use pyo3::class::{PyObjectProtocol, PyNumberProtocol};
use pyo3::prelude::*;

use crate::nes::Address;

#[pyclass]
#[derive(Debug, Copy, Clone)]
pub struct PyAddress {
    pub address: Address,
}

#[pymethods]
impl PyAddress {
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
}

#[pyproto]
impl PyObjectProtocol for PyAddress {
    fn __repr__(&self) -> PyResult<String> {
        Ok(match self.address {
            Address::File(x) => format!("PyAddress.file({})", x),
            Address::Seg(n, x) => format!("PyAddress.segment({}, {})", n.as_str(), x),
            Address::Cpu(x) => format!("PyAddress.cpu({})", x),
            Address::Bank(n, b, x) => format!("PyAddress.banked({}, {}, {})", n.as_str(), b, x),
            Address::Prg(b, x) => format!("PyAddress.prg({}, {})", b, x),
            Address::Chr(b, x) => format!("PyAddress.chr({}, {})", b, x),
        })
    }
}

#[pyproto]
impl PyNumberProtocol for PyAddress {
    fn __add__(lhs: PyAddress, rhs: isize) -> Self {
        PyAddress { address: lhs.address + rhs }
    }
    fn __sub__(lhs: PyAddress, rhs: isize) -> Self {
        PyAddress { address: lhs.address - rhs }
    }
    fn __iadd__(&mut self, rhs: isize) {
        self.address = self.address + rhs;
    }
    fn __isub__(&mut self, rhs: isize) {
        self.address = self.address - rhs;
    }
}
