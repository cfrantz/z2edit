use anyhow::Result;
use indexmap::map::{Entry, IndexMap};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use super::{Address, AddressRange, NesError};

pub mod config {
    use super::*;

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    #[serde(default)]
    pub struct FreeSpace {
        pub freespace: Vec<AddressRange>,
        pub keepout: Vec<AddressRange>,
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[pyclass(eq)]
pub enum Zone {
    Prg(i16),
    Cpu(i16),
}

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[pyclass(eq)]
pub enum Alloc {
    /// Choose the smallest free block that can fit the requested allocation.
    #[default]
    Best,
    /// Choose the free block nearest to the requested allocation.
    Near,
    /// Choose the free block that can provide the exact allocation.
    Exact,
}

impl TryFrom<Address> for Zone {
    type Error = NesError;
    fn try_from(a: Address) -> Result<Self, Self::Error> {
        match a {
            Address::Prg(bank, _) => Ok(Zone::Prg(bank)),
            Address::Cpu(_) => Ok(Zone::Cpu(0)),
            _ => Err(NesError::Convert(format!("{a:?} to Zone"))),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct FreeSpace {
    freelist: IndexMap<Zone, Vec<AddressRange>>,
}

impl FreeSpace {
    fn _free(list: &mut Vec<AddressRange>, range: AddressRange) -> Result<()> {
        for item in list.iter() {
            if item.overlaps(&range) {
                return Err(NesError::Overlaps(format!("{item:x?} overlaps {range:x?}")).into());
            }
        }
        list.push(range);
        Self::_normalize(list);
        Ok(())
    }

    // Sort the list, remove zero-length blocks and merge adjacent blocks.
    fn _normalize(list: &mut Vec<AddressRange>) {
        list.sort();
        let mut i = 0;
        while i < list.len() - 1 {
            if list[i].length == 0 {
                list.remove(i);
            } else if list[i + 1].length == 0 {
                list.remove(i + 1);
            } else if list[i].adjacent(&list[i + 1]) {
                list[i].length += list[i + 1].length;
                list.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    fn register_one(&mut self, range: AddressRange) -> Result<()> {
        let zone = Zone::try_from(range.address)?;
        match self.freelist.entry(zone) {
            Entry::Vacant(entry) => {
                entry.insert(vec![range]);
            }
            Entry::Occupied(mut entry) => {
                Self::_free(entry.get_mut(), range)?;
            }
        }
        Ok(())
    }

    pub fn register(&mut self, data: &config::FreeSpace) -> Result<()> {
        for item in data.freespace.iter() {
            for ko in data.keepout.iter() {
                if ko.overlaps(item) {
                    return Err(NesError::Overlaps(format!(
                        "freespace {item:x?} overlaps keepout {ko:x?}"
                    ))
                    .into());
                }
            }
            self.register_one(*item)?;
        }
        Ok(())
    }

    pub fn alloc(&mut self, address: Address, length: u16, policy: Alloc) -> Result<Address> {
        let zone = Zone::try_from(address)?;
        let request = AddressRange { address, length };
        if let Some(list) = self.freelist.get_mut(&zone) {
            let mut candidates = Vec::new();
            for (i, candidate) in list.iter().enumerate() {
                if policy == Alloc::Exact && candidate.contains(&request) {
                    candidates.push(i);
                } else if policy != Alloc::Exact && length <= candidate.length {
                    candidates.push(i);
                }
            }
            match policy {
                Alloc::Best => {
                    // Best fit chooses the shortest free block that can contain the
                    // allocation.
                    candidates.sort_by(|&a, &b| {
                        let a = list[a].length;
                        let b = list[b].length;
                        a.cmp(&b)
                    });
                    if let Some(&i) = candidates.get(0) {
                        let range = list[i].cut(length)?;
                        Self::_normalize(list);
                        Ok(range.address)
                    } else {
                        Err(NesError::NoMemory(format!("no candidates for {request:?}")).into())
                    }
                }
                Alloc::Near => {
                    // Nearest fit chooses the block with the address closest to the
                    // requested address.
                    let target = address.offset() as isize;
                    candidates.sort_by(|&a, &b| {
                        let a = (list[a].address.offset() as isize - target).abs();
                        let b = (list[b].address.offset() as isize - target).abs();
                        a.cmp(&b)
                    });
                    if let Some(&i) = candidates.get(0) {
                        let range = list[i].cut(length)?;
                        Self::_normalize(list);
                        Ok(range.address)
                    } else {
                        Err(NesError::NoMemory(format!("no candidates near {request:?}")).into())
                    }
                }
                Alloc::Exact => {
                    // Exact finds a block that has the exact requested address.
                    if let Some(&i) = candidates.get(0) {
                        if address.offset() > list[i].address.offset() {
                            // If the candidate start is lower than the requested address,
                            // cut the beginning of the range and append it to the freelist.
                            // After this cut, the candidate will start exactly at the
                            // requested address.
                            let front = address.offset() - list[i].address.offset();
                            let front = list[i].cut(front as u16)?;
                            list.push(front);
                        }
                        // Cut the address range, returning `length` bytes at the start
                        // of the range and modifying the range to cover the remainder.
                        let range = list[i].cut(length)?;
                        Self::_normalize(list);
                        Ok(range.address)
                    } else {
                        Err(NesError::NoMemory(format!("no exact fit for {request:?}")).into())
                    }
                }
            }
        } else {
            Err(NesError::NoMemory(format!("no freespace zone {zone:?}")).into())
        }
    }

    pub fn free(&mut self, address: Address, length: u16) -> Result<()> {
        let zone = Zone::try_from(address)?;
        let request = AddressRange { address, length };
        if let Some(list) = self.freelist.get_mut(&zone) {
            Self::_free(list, request)
        } else {
            Err(NesError::NoMemory(format!("no freespace zone {zone:?}")).into())
        }
    }

    /// Frees a large range of address space, consuming pre-existing overlapping
    /// regions into the large block.  Creates a new bank if one doesn't
    /// already exist.
    ///
    /// This call is used when performing major edits on a bank or registering
    /// freespace in a new bank.
    pub fn bulkfree(&mut self, address: Address, length: u16) -> Result<()> {
        let zone = Zone::try_from(address)?;
        let mut block = AddressRange { address, length };
        if let Some(list) = self.freelist.get_mut(&zone) {
            let mut i = 0;
            while i < list.len() {
                if block.extend(&list[i]) {
                    list.remove(i);
                } else {
                    i += 1;
                }
            }
            list.push(block);
            Self::_normalize(list);
            Ok(())
        } else {
            self.register_one(block)
        }
    }

    pub fn copy_zone(&mut self, oldbank: i16, newbank: i16) -> Result<()> {
        let oldzone = Zone::Prg(oldbank);
        let newzone = Zone::Prg(newbank);
        if let Some(mut list) = self.freelist.get(&oldzone).cloned() {
            for item in list.iter_mut() {
                item.address = item.address.with_bank(newbank);
            }
            self.freelist.insert(newzone, list);
            Ok(())
        } else {
            Err(NesError::NoMemory(format!("no freespace zone {oldzone:?}")).into())
        }
    }
}

impl std::fmt::Display for FreeSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "FreeSpace {{")?;
        for (bank, list) in self.freelist.iter() {
            writeln!(f, "  Bank {bank:?} {{")?;
            for item in list.iter() {
                writeln!(f, "    {item:x?}")?;
            }
            writeln!(f, "  }}")?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}
