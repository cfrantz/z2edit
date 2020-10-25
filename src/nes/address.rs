use std::ops;
use serde::{Serialize, Deserialize};

use crate::errors::*;
use super::layout::*;
use super::intern::Istr;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Address {
    File(usize),
    Seg(Istr, usize),
    Cpu(u16),
    Bank(Istr, isize, usize),
    Prg(isize, u16),
    Chr(isize, u16),
}

impl Default for Address {
    fn default() -> Self {
        Address::File(0)
    }
}

impl Address {
    pub fn new_seg(name: &str, offset: usize) -> Address {
        Address::Seg(Istr::new(name), offset)
    }
    pub fn new_bank(name: &str, bank: isize, offset: usize) -> Address {
        Address::Bank(Istr::new(name), bank, offset)
    }


    // Simplify the helper variants into the Bank variant.
    pub fn simplify(self) -> Self {
        match self {
            Address::Prg(bank, ofs) => Address::Bank(Istr::new("prg"), bank, ofs as usize),
            Address::Chr(bank, ofs) => Address::Bank(Istr::new("chr"), bank, ofs as usize),
            _ => self,
        }
    }

    // Get the layout bounds which apply to this address.
    fn bound(&self, layout: &Layout) -> Result<(usize, usize)> {
        match self {
            Address::File(_) => layout.bound("__all__"),
            Address::Seg(n, _) => layout.bound(n.as_str()),
            Address::Bank(n, _, _) => layout.bound(n.as_str()),
            _ => Err(ErrorKind::AddressTypeError(self.clone()).into()),
        }
    }

    // Check this address+length against a Layout and convert it into
    // an absolute offset wrt the layout.
    pub fn offset(self, length: usize, layout: &Layout) -> Result<usize> {
        let address = self.simplify();
        let (start, end) = address.bound(layout)?;
        match address {
            Address::File(o) => {
                if start+o+length <= end {
                    Ok(start + o)
                } else {
                    Err(ErrorKind::AddressBoundError(self.clone()).into())
                }
            },
            Address::Seg(_, o) => {
                if start+o+length <= end {
                    Ok(start + o)
                } else {
                    Err(ErrorKind::AddressBoundError(self.clone()).into())
                }
            }
            Address::Bank(n, b, o) => {
                if let Segment::Banked{length: len, banksize, mask, ..} = layout.segment(n.as_str())? {
                    let banks = len / banksize;
                    let b = if b >= 0 { b as usize } else { banks - (-b) as usize };
                    let so = b*banksize + (o & mask);
                    let eo = (o & mask) + length - 1;
                    let s = start + so;
                    let e = so + length - 1;
                    if e < end  && eo & !mask == 0 {
                        Ok(s)
                    } else {
                        Err(ErrorKind::AddressBoundError(self.clone()).into())
                    }
                } else {
                    Err(ErrorKind::LayoutError(format!("Address::Bank in non Banked segment {}", n.as_str())).into())
                }
            }

            _ => Err(ErrorKind::AddressTypeError(self.clone()).into()),
        }
    }

    pub fn bank(&self) -> Option<(&'static str, isize)> {
        match self {
            Address::File(_) => None,
            Address::Seg(n, _) => Some((n.as_str(), std::isize::MAX)),
            Address::Cpu(_) => None,
            Address::Bank(n, b, _) => Some((n.as_str(), *b)),
            Address::Prg(b, _) => Some(("prg", *b)),
            Address::Chr(b, _) => Some(("chr", *b)),
        }
    }

    pub fn raw(&self) -> i64 {
        match self {
            Address::File(x) => *x as i64,
            Address::Seg(_, x) => *x as i64,
            Address::Cpu(x) => *x as i64,
            Address::Bank(_, _, x) => *x as i64,
            Address::Prg(_, x) => *x as i64,
            Address::Chr(_, x) => *x as i64,
        }
    }

    pub fn set_seg(&self, segment: &str) -> Address {
        let segment = Istr::new(segment);
        match self {
            Address::File(x) => Address::Seg(segment, *x),
            Address::Seg(_, x) => Address::Seg(segment, *x),
            Address::Cpu(x) => Address::Seg(segment, *x as usize),
            Address::Bank(_, b, x) => Address::Bank(segment, *b, *x),
            Address::Prg(b, x) => Address::Bank(segment, *b, *x as usize),
            Address::Chr(b, x) => Address::Bank(segment, *b, *x as usize),
        }
    }

    pub fn set_bank(&self, bank: isize) -> Address {
        match self {
            Address::File(x) => Address::Bank(Istr::new(""), bank, *x),
            Address::Seg(s, x) => Address::Bank(*s, bank, *x),
            Address::Cpu(x) => Address::Bank(Istr::new(""), bank, *x as usize),
            Address::Bank(s, _, x) => Address::Bank(*s, bank, *x),
            Address::Prg(_, x) => Address::Prg(bank, *x),
            Address::Chr(_, x) => Address::Chr(bank, *x),
        }
    }

    pub fn set_val(&self, addr: usize) -> Address {
        match self {
            Address::File(_) => Address::File(addr),
            Address::Seg(s, _) => Address::Seg(*s, addr),
            Address::Cpu(_) => Address::Cpu(addr as u16),
            Address::Bank(s, b, _) => Address::Bank(*s, *b, addr),
            Address::Prg(b, _) => Address::Prg(*b, addr as u16),
            Address::Chr(b, _) => Address::Chr(*b, addr as u16),
        }
    }
}

macro_rules! addsub {
    ($t:ty) => {
        impl ops::Add<$t> for Address {
            type Output = Address;
            fn add(self, rhs: $t) -> Address {
                match self {
                    Address::File(x) => Address::File(x.wrapping_add(rhs as usize)),
                    Address::Seg(n, x) => Address::Seg(n, x.wrapping_add(rhs as usize)),
                    Address::Cpu(x) => Address::Cpu(x.wrapping_add(rhs as u16)),
                    Address::Bank(n, b, x) => Address::Bank(n, b, x.wrapping_add(rhs as usize)),
                    Address::Prg(b, x) => Address::Prg(b, x.wrapping_add(rhs as u16)),
                    Address::Chr(b, x) => Address::Chr(b, x.wrapping_add(rhs as u16)),
                }

            }
        }

        impl ops::Sub<$t> for Address {
            type Output = Address;
            fn sub(self, rhs: $t) -> Address {
                match self {
                    Address::File(x) => Address::File(x.wrapping_sub(rhs as usize)),
                    Address::Seg(n, x) => Address::Seg(n, x.wrapping_sub(rhs as usize)),
                    Address::Cpu(x) => Address::Cpu(x.wrapping_sub(rhs as u16)),
                    Address::Bank(n, b, x) => Address::Bank(n, b, x.wrapping_sub(rhs as usize)),
                    Address::Prg(b, x) => Address::Prg(b, x.wrapping_sub(rhs as u16)),
                    Address::Chr(b, x) => Address::Chr(b, x.wrapping_sub(rhs as u16)),
                }
            }
        }
    }
}

addsub!(usize); addsub!(isize);
addsub!(u8); addsub!(i8);
addsub!(u16); addsub!(i16);
addsub!(u32); addsub!(i32);
addsub!(u64); addsub!(i64);

pub trait MemoryAccess {
    // Memory reading functions.
    fn read(&self, address: Address) -> Result<u8> {
        let byte = self.read_bytes(address, 1)?;
        Ok(byte[0])
    }

    fn read_word(&self, address: Address) -> Result<u16> {
        let byte = self.read_bytes(address, 2)?;
        Ok(byte[0] as u16 | (byte[1] as u16) << 8)
    }
    fn read_bytes(&self, address: Address, length: usize) -> Result<&[u8]>;

    // Memory writing functions.
    fn write(&mut self, address: Address, value: u8) -> Result<()> {
        self.write_bytes(address, &[value])
    }
    fn write_word(&mut self, address: Address, value: u16) -> Result<()> {
        let v = [value as u8, (value >> 8) as u8];
        self.write_bytes(address, &v)
    }
    fn write_bytes(&mut self, address: Address, value: &[u8]) -> Result<()>;

    // Apply a layout for memory access.
    fn apply_layout(&mut self, layout: Layout) -> Result<()>;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout() {
        let layout = Layout(vec![
            Segment::Raw { name: "header".into(), offset: 0, length: 16, fill: 0 },
            Segment::Banked { name: "prg".into(), offset: 16, length: 4096, banksize: 1024, mask: 0x3ff, fill: 0xff },
            Segment::Banked { name: "chr".into(), offset: 16+4096, length: 1024, banksize: 256, mask: 0xff, fill: 0x55 },
        ]);

        let a = Address::File(0).offset(1, &layout).unwrap();
        assert_eq!(0, a);

        let a = Address::File(6000).offset(1, &layout).unwrap_err();
        assert_eq!("address bound error", a.description());

        let a = Address::new_seg("header", 5).offset(1, &layout).unwrap();
        assert_eq!(5, a);

        let a = Address::new_seg("header", 16).offset(1, &layout).unwrap_err();
        assert_eq!("address bound error", a.description());

        let a = Address::Prg(0, 4).offset(1, &layout).unwrap();
        assert_eq!(16+4, a);

        let a = Address::Prg(1, 0x8000).offset(1, &layout).unwrap();
        assert_eq!(16+1024, a);

        let a = Address::Prg(-1, 0x8000).offset(1, &layout).unwrap();
        assert_eq!(16+3*1024, a);

        let a = Address::Prg(-1, 0xffff).offset(2, &layout).unwrap_err();
        assert_eq!("address bound error", a.description());
    }
}
