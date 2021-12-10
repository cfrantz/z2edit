use serde::{Deserialize, Serialize};

use super::{Address, Layout};
use crate::errors::*;

pub mod config {
    use super::*;
    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct AddressRange(pub Address, pub usize);

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Config {
        pub freespace: Vec<AddressRange>,
        pub keepout: Vec<AddressRange>,
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct FreeSpaceRange {
    bank: isize,
    address: u16,
    length: u16,
}

impl FreeSpaceRange {
    fn contains(&self, address: Address) -> bool {
        match address {
            Address::Prg(bank, addr) => {
                bank == self.bank && addr >= self.address && addr < self.address + self.length
            }
            _ => false,
        }
    }

    fn adjacent(&self, other: &FreeSpaceRange) -> bool {
        self.bank == other.bank && self.address + self.length == other.address
    }
    fn overlaps(&self, other: &FreeSpaceRange) -> bool {
        if self.bank == other.bank {
            if other.address >= self.address && other.address < self.address + self.length {
                return true;
            }
        }
        false
    }

    fn extend(&mut self, other: &FreeSpaceRange) -> bool {
        if self.adjacent(other) {
            self.length += other.length;
            true
        } else if self.overlaps(other) {
            let end = other.address + other.length;
            if end > self.address + self.length {
                self.length = end - self.address;
            }
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct FreeSpace {
    freelist: Vec<FreeSpaceRange>,
    banks: isize,
}

impl FreeSpace {
    pub fn new(config: &config::Config, layout: &Layout) -> Result<Self> {
        let mut space = FreeSpace::default();
        for item in config.freespace.iter() {
            FreeSpace::check_keepout_overlap(item.0, item.1, config);
            space.register(item.0, item.1 as u16, false)?;
        }
        space.banks = layout.segment("prg")?.banks() as isize;
        Ok(space)
    }

    pub fn adjust_layout(&mut self, layout: &Layout) -> Result<()> {
        let mut i = 0;
        let len = self.freelist.len();
        let oldtop = self.banks - 1;
        self.banks = layout.segment("prg")?.banks() as isize;
        let newtop = self.banks - 1;

        while i < len {
            if self.freelist[i].bank == oldtop {
                self.freelist[i].bank = newtop;
            }
            i += 1;
        }
        Ok(())
    }

    fn check_keepout_overlap(addr: Address, length: usize, config: &config::Config) {
        for config::AddressRange(koaddr, kolen) in config.keepout.iter() {
            if addr.in_range(*koaddr, *kolen) {
                panic!(
                    "Start of frespace range ({:x?}, {}) overlaps keepout region ({:x?}, {})",
                    addr, length, koaddr, kolen
                );
            }
            if (addr + (length - 1)).in_range(*koaddr, *kolen) {
                panic!(
                    "End of frespace range ({:x?}, {}) overlaps keepout region ({:x?}, {})",
                    addr, length, koaddr, kolen
                );
            }
            if koaddr.in_range(addr, length) {
                panic!(
                    "Start of keepout range ({:x?}, {}) overlaps freespace region ({:x?}, {})",
                    koaddr, kolen, addr, length
                );
            }
            if (*koaddr + (*kolen - 1)).in_range(addr, length) {
                panic!(
                    "Start of keepout range ({:x?}, {}) overlaps freespace region ({:x?}, {})",
                    koaddr, kolen, addr, length
                );
            }
        }
    }

    fn contains(&self, address: Address) -> bool {
        for f in self.freelist.iter() {
            if f.contains(address) {
                return true;
            }
        }
        false
    }

    fn coalesce(&mut self) {
        let mut i = 0;
        while i < self.freelist.len() - 1 {
            if self.freelist[i].length == 0 {
                self.freelist.remove(i);
                continue;
            }
            if self.freelist[i].adjacent(&self.freelist[i + 1]) {
                let next = self.freelist.remove(i + 1);
                self.freelist[i].extend(&next);
                continue;
            }
            if self.freelist[i].overlaps(&self.freelist[i + 1]) {
                let next = self.freelist.remove(i + 1);
                warn!(
                    "Coalescing overlapping ranges: {:x?} {:x?}",
                    self.freelist[i], next
                );
                self.freelist[i].extend(&next);
                continue;
            }
            i += 1;
        }
    }

    fn normalize_address(&self, address: Address) -> Result<Address> {
        match address {
            Address::Prg(bank, offset) => {
                let bank = if bank < 0 { bank + self.banks } else { bank };
                Ok(Address::Prg(bank, offset))
            }
            _ => Err(ErrorKind::FreeSpaceError(format!(
                "Address must by of type Prg: {:x?}",
                address
            ))
            .into()),
        }
    }

    fn get_bank(&self, address: Address) -> Result<isize> {
        let address = self.normalize_address(address)?;
        match address {
            Address::Prg(bank, _) => Ok(bank),
            _ => Err(ErrorKind::FreeSpaceError(format!(
                "Address must by of type Prg: {:x?}",
                address
            ))
            .into()),
        }
    }

    pub fn copy_bank(&mut self, frombank: isize, tobank: isize) {
        let mut newbank = Vec::new();
        for f in self.freelist.iter() {
            if f.bank == frombank {
                let mut new = f.clone();
                new.bank = tobank;
                newbank.push(new);
            }
        }
        self.freelist.extend(newbank);
    }

    pub fn register(&mut self, address: Address, length: u16, allow_overlap: bool) -> Result<()> {
        let address = self.normalize_address(address)?;
        if !allow_overlap {
            if self.contains(address) {
                return Err(ErrorKind::FreeSpaceError(format!(
                    "Address {:x?} already in freespace",
                    address
                ))
                .into());
            }

            if self.contains(address + length - 1) {
                return Err(ErrorKind::FreeSpaceError(format!(
                    "Address {:x?}+{} already in freespace",
                    address, length
                ))
                .into());
            }
        }
        match address {
            Address::Prg(bank, addr) => {
                info!("FreeSpace::register: {:x?} {:?}", address, length);
                self.freelist.push(FreeSpaceRange {
                    bank: bank,
                    address: addr,
                    length: length,
                });
                self.freelist.sort();
                self.coalesce();
                Ok(())
            }
            _ => Err(ErrorKind::FreeSpaceError(format!(
                "Address must by of type Prg: {:x?}",
                address
            ))
            .into()),
        }
    }

    pub fn alloc_exact_fit(&mut self, bank: isize, length: u16) -> Result<Address> {
        let mut i = 0;
        let len = self.freelist.len();

        while i < len {
            if self.freelist[i].bank == bank && self.freelist[i].length == length {
                let space = self.freelist.remove(i);
                return Ok(Address::Prg(space.bank, space.address));
            }
            i += 1;
        }
        Err(ErrorKind::OutOfMemory(bank).into())
    }

    pub fn alloc_first_fit(&mut self, bank: isize, length: u16) -> Result<Address> {
        for f in self.freelist.iter_mut() {
            if f.bank == bank && f.length >= length {
                f.length -= length;
                return Ok(Address::Prg(f.bank, f.address + f.length));
            }
        }
        Err(ErrorKind::OutOfMemory(bank).into())
    }

    fn alloc_near_helper(&mut self, address: Address, length: u16, exact: bool) -> Result<Address> {
        let bank = self.get_bank(address)?;
        // Generate (delta from requested address, freespace index) tuples.
        let mut nearness = Vec::new();
        for (i, f) in self.freelist.iter().enumerate() {
            if f.bank == bank && f.length >= length {
                let delta = f.address as isize - address.raw() as isize;
                nearness.push((delta.abs(), i));
            }
        }
        if nearness.len() > 0 {
            // Order the tuples by nearness and allocate.
            nearness.sort();
            let index = nearness[0].1;
            let result = self.freelist[index].address;
            if exact {
                if (result as i64) - address.raw() != 0 {
                    return Err(ErrorKind::OutOfMemory(bank).into());
                }
            }
            self.freelist[index].address += length;
            self.freelist[index].length -= length;
            Ok(Address::Prg(bank, result))
        } else {
            Err(ErrorKind::OutOfMemory(bank).into())
        }
    }

    pub fn alloc_near(&mut self, address: Address, length: u16) -> Result<Address> {
        self.alloc_near_helper(address, length, false)
    }

    pub fn alloc_at(&mut self, address: Address, length: u16) -> Result<Address> {
        self.alloc_near_helper(address, length, true)
    }

    pub fn alloc(&mut self, address: Address, length: u16) -> Result<Address> {
        let bank = self.get_bank(address)?;
        let mut result = self.alloc_exact_fit(bank, length);
        if result.is_err() {
            result = self.alloc_first_fit(bank, length);
        }
        result
    }

    pub fn free(&mut self, address: Address, length: u16) {
        match self.register(address, length, false) {
            Err(e) => panic!("{:?}", e),
            _ => {}
        };
    }
    pub fn report(&self, address: Address) -> Result<(usize, u16)> {
        let bank = self.get_bank(address)?;
        let mut chunks = 0;
        let mut total = 0;
        for f in self.freelist.iter() {
            if f.bank == bank && f.length > 0 {
                info!(
                    "Chunk {}: start=0x{:x?} length={}",
                    chunks, f.address, f.length
                );
                chunks += 1;
                total += f.length;
            }
        }
        Ok((chunks, total))
    }
}
