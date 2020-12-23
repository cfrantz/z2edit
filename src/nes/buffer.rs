use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::vec::Vec;

use super::address::{Address, MemoryAccess};
use super::layout::*;
use crate::errors::*;

use sha2::{Digest, Sha256};

#[derive(Default, Debug, Clone)]
pub struct Buffer {
    layout: Layout,
    data: Vec<u8>,
}

impl Buffer {
    pub fn new(size: usize) -> Self {
        Buffer {
            layout: Layout(vec![
                Segment::Raw {
                    name: "".to_string(),
                    offset: 0,
                    length: size,
                    fill: 0xff,
                };
                1
            ]),
            data: vec![0xffu8; size],
        }
    }

    pub fn from_layout(layout: Layout) -> Self {
        let mut data = Vec::<u8>::new();
        for segment in &layout.0 {
            match segment {
                Segment::Raw {
                    offset: o,
                    length: l,
                    fill: f,
                    ..
                } => data.resize(*o + *l, *f),
                Segment::Banked {
                    offset: o,
                    length: l,
                    fill: f,
                    ..
                } => data.resize(*o + *l, *f),
            }
        }
        Buffer {
            layout: layout,
            data: data,
        }
    }

    fn check_layout(layout: &Layout, data: &[u8]) -> Result<()> {
        if let Some(segment) = layout.0.last() {
            let length = match segment {
                Segment::Raw {
                    offset: o,
                    length: l,
                    ..
                } => o + l,
                Segment::Banked {
                    offset: o,
                    length: l,
                    ..
                } => o + l,
            };
            if length != data.len() {
                bail!(ErrorKind::LayoutError(
                    "Layout length doesn't match data length".into()
                ));
            }
        } else {
            bail!(ErrorKind::LayoutError("Layout is empty".into()));
        }
        Ok(())
    }

    pub fn from_reader(mut r: impl Read, layout: Option<Layout>) -> Result<Self> {
        let mut data = Vec::<u8>::new();
        r.read_to_end(&mut data)?;
        let layout = if let Some(lo) = layout {
            Buffer::check_layout(&lo, &data)?;
            lo
        } else {
            Layout(vec![
                Segment::Raw {
                    name: "".to_string(),
                    offset: 0,
                    length: data.len(),
                    fill: 0,
                };
                1
            ])
        };
        Ok(Buffer {
            layout: layout,
            data: data,
        })
    }

    pub fn to_writer(&self, w: &mut impl Write) -> Result<()> {
        w.write_all(&self.data)?;
        Ok(())
    }

    pub fn from_file(filepath: &Path, layout: Option<Layout>) -> Result<Self> {
        let file = File::open(filepath)?;
        Buffer::from_reader(file, layout)
    }

    pub fn save(&self, filepath: &Path) -> Result<()> {
        let mut file = File::create(filepath)?;
        self.to_writer(&mut file)
    }

    pub fn sha256(&self) -> String {
        let hash = Sha256::digest(&self.data);
        format!("{:x}", hash)
    }
}

impl MemoryAccess for Buffer {
    fn read_bytes(&self, address: Address, length: usize) -> Result<&[u8]> {
        let offset = address.offset(length, &self.layout)?;
        Ok(&self.data[offset..offset + length])
    }

    fn write_bytes(&mut self, address: Address, value: &[u8]) -> Result<()> {
        let offset = address.offset(value.len(), &self.layout)?;
        for (i, v) in value.iter().enumerate() {
            self.data[offset + i] = *v;
        }
        Ok(())
    }

    fn apply_layout(&mut self, layout: Layout) -> Result<()> {
        if self.data.len() == 0 {
            *self = Buffer::from_layout(layout);
        } else {
            Buffer::check_layout(&layout, &self.data)?;
            self.layout = layout;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer() {
        let layout = Layout(vec![
            Segment::Raw {
                name: "header".into(),
                offset: 0,
                length: 16,
                fill: 0,
            },
            Segment::Banked {
                name: "prg".into(),
                offset: 16,
                length: 4096,
                banksize: 1024,
                mask: 0x3ff,
                fill: 0xff,
            },
            Segment::Banked {
                name: "chr".into(),
                offset: 16 + 4096,
                length: 1024,
                banksize: 256,
                mask: 0xff,
                fill: 0x55,
            },
        ]);

        let buffer = Buffer::from_layout(layout);

        let b = buffer.read(Address::new_seg("header", 0)).unwrap();
        assert_eq!(0, b);

        let b = buffer.read(Address::Prg(0, 0)).unwrap();
        assert_eq!(0xff, b);

        let b = buffer.read(Address::Chr(0, 0)).unwrap();
        assert_eq!(0x55, b);
    }
}
