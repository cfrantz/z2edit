use super::NesError;
use anyhow::{ensure, Result};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
#[pyclass]
pub enum Address {
    File(usize),
    Prg(i16, u16),
    Prg8k(i16, u16),
    Chr(i16, u16),
    Chr1k(i16, u16),
    Cpu(u16),
    NullPtr(),
}

impl Default for Address {
    fn default() -> Self {
        Address::NullPtr()
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
            Address::NullPtr() => 0,
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
            Address::NullPtr() => None,
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
            Address::NullPtr() => Address::NullPtr(),
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
            Address::NullPtr() => Err(NesError::InvalidAddress.into()),
        }
    }

    pub fn is_chr(&self) -> bool {
        match self {
            Address::Chr(_, _) => true,
            _ => false,
        }
    }

    pub fn is_prg(&self) -> bool {
        match self {
            Address::Prg(_, _) => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            Address::NullPtr() => true,
            _ => false,
        }
    }

    pub fn is_valid(&self) -> bool {
        match self {
            Address::NullPtr() => false,
            Address::Prg(_, x) => *x != 0 && *x != 0xFFFF,
            Address::Prg8k(_, x) => *x != 0 && *x != 0xFFFF,
            _ => true,
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
                    Address::NullPtr() => Address::NullPtr(),
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
                    Address::NullPtr() => Address::NullPtr(),
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

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct AddressRange {
    pub address: Address,
    pub length: u16,
}

impl AddressRange {
    fn same_bank(&self, address: Address) -> bool {
        if std::mem::discriminant(&self.address) != std::mem::discriminant(&address) {
            return false;
        }
        if self.address.bank() != address.bank() {
            return false;
        }
        true
    }

    pub fn contains(&self, other: &AddressRange) -> bool {
        if !self.same_bank(other.address) {
            return false;
        }
        let start = self.address.offset();
        let end = start + self.length as usize;
        let other_start = other.address.offset();
        let other_end = other_start + other.length as usize;
        start <= other_start && other_end <= end
    }

    pub fn adjacent(&self, other: &AddressRange) -> bool {
        if !self.same_bank(other.address) {
            return false;
        }
        let end = self.address.offset() + self.length as usize;
        let addr = other.address.offset();
        end == addr
    }

    pub fn overlaps(&self, other: &AddressRange) -> bool {
        if !self.same_bank(other.address) {
            return false;
        }
        let start = self.address.offset();
        let end = start + self.length as usize;
        let other_start = other.address.offset();
        let other_end = other_start + other.length as usize;
        (start <= other_start && other_start < end) || (start < other_end && other_end <= end)
    }

    pub fn cut(&mut self, length: u16) -> Result<AddressRange> {
        if length <= self.length {
            let result = AddressRange {
                address: self.address,
                length,
            };
            self.address = self.address + length;
            self.length -= length;
            Ok(result)
        } else {
            Err(NesError::RangeTooSmall(*self, length).into())
        }
    }

    pub fn extend(&mut self, other: &AddressRange) -> bool {
        if self.adjacent(other) {
            self.length += other.length;
            true
        } else {
            false
        }
    }
}
