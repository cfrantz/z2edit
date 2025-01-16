use super::NesError;
use anyhow::{ensure, Result};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[pyclass]
pub enum Address {
    File(usize),
    Prg(i16, u16),
    Prg8k(i16, u16),
    Chr(i16, u16),
    Chr1k(i16, u16),
    Cpu(u16),
}

impl Default for Address {
    fn default() -> Self {
        Address::Cpu(0)
    }
}

#[pymethods]
impl Address {
    pub fn offset(&self) -> usize {
        match self {
            Address::File(x) => *x,
            Address::Prg(_, x) => *x as usize,
            Address::Prg8k(_, x) => *x as usize,
            Address::Chr(_, x) => *x as usize,
            Address::Chr1k(_, x) => *x as usize,
            Address::Cpu(x) => *x as usize,
        }
    }

    pub fn bank(&self) -> Option<i16> {
        match self {
            Address::File(_) => None,
            Address::Prg(x, _) => Some(*x),
            Address::Prg8k(x, _) => Some(*x),
            Address::Chr(x, _) => Some(*x),
            Address::Chr1k(x, _) => Some(*x),
            Address::Cpu(_) => None,
        }
    }

    pub fn with_offset(&self, offset: usize) -> Self {
        match self {
            Address::File(_) => Address::File(offset),
            Address::Prg(b, _) => Address::Prg(*b, offset as u16),
            Address::Prg8k(b, _) => Address::Prg8k(*b, offset as u16),
            Address::Chr(b, _) => Address::Chr(*b, offset as u16),
            Address::Chr1k(b, _) => Address::Chr1k(*b, offset as u16),
            Address::Cpu(_) => Address::Cpu(offset as u16),
        }
    }

    pub fn norm_offset(&self) -> Result<usize> {
        match self {
            Address::File(x) => Ok(*x),
            Address::Prg(b, x) => {
                ensure!(*b >= 0, NesError::NegativeBank);
                Ok((*b as usize) * 16384 + (*x as usize))
            }
            Address::Prg8k(b, x) => {
                ensure!(*b >= 0, NesError::NegativeBank);
                Ok((*b as usize) * 8192 + (*x as usize))
            }
            Address::Chr(b, x) => {
                ensure!(*b >= 0, NesError::NegativeBank);
                Ok((*b as usize) * 4096 + (*x as usize))
            }
            Address::Chr1k(b, x) => {
                ensure!(*b >= 0, NesError::NegativeBank);
                Ok((*b as usize) * 1024 + (*x as usize))
            }
            Address::Cpu(x) => Ok(*x as usize),
        }
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }

    fn __add__(&self, rhs: isize) -> Self {
        *self + rhs
    }
    fn __sub__(&self, rhs: isize) -> Self {
        *self - rhs
    }
}

macro_rules! address_math {
    ($t:ty) => {
        impl std::ops::Add<$t> for Address {
            type Output = Address;
            fn add(self, rhs: $t) -> Address {
                match self {
                    Address::File(x) => Address::File(x.wrapping_add(rhs as usize)),
                    Address::Prg(b, x) => Address::Prg(b, x.wrapping_add(rhs as u16)),
                    Address::Prg8k(b, x) => Address::Prg8k(b, x.wrapping_add(rhs as u16)),
                    Address::Chr(b, x) => Address::Chr(b, x.wrapping_add(rhs as u16)),
                    Address::Chr1k(b, x) => Address::Chr1k(b, x.wrapping_add(rhs as u16)),
                    Address::Cpu(x) => Address::Cpu(x.wrapping_add(rhs as u16)),
                }
            }
        }
        impl std::ops::Sub<$t> for Address {
            type Output = Address;
            fn sub(self, rhs: $t) -> Address {
                match self {
                    Address::File(x) => Address::File(x.wrapping_sub(rhs as usize)),
                    Address::Prg(b, x) => Address::Prg(b, x.wrapping_sub(rhs as u16)),
                    Address::Prg8k(b, x) => Address::Prg8k(b, x.wrapping_sub(rhs as u16)),
                    Address::Chr(b, x) => Address::Chr(b, x.wrapping_sub(rhs as u16)),
                    Address::Chr1k(b, x) => Address::Chr1k(b, x.wrapping_sub(rhs as u16)),
                    Address::Cpu(x) => Address::Cpu(x.wrapping_sub(rhs as u16)),
                }
            }
        }
    };

    ($t:ty, $($rest:ty),*) => {
        address_math!($t);
        address_math!($($rest),*);
    };
}

address_math!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);
