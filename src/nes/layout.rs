use std::vec::Vec;
use serde::{Serialize, Deserialize};

use crate::errors::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Segment {
    Raw {
        name: String,
        offset: usize,
        length: usize,
        fill: u8,
    },
    Banked {
        name: String,
        offset: usize,
        length: usize,
        banksize: usize,
        mask: usize,
        fill: u8,
    }
}

impl Segment {
    pub fn name(&self) -> String {
        match self {
            Segment::Raw{name: n, ..} => n.clone(),
            Segment::Banked{name: n, ..} => n.clone(),
        }
    }
    pub fn offset(&self) -> usize {
        match self {
            Segment::Raw{offset: n, ..} => *n,
            Segment::Banked{offset: n, ..} => *n,
        }
    }
    pub fn length(&self) -> usize {
        match self {
            Segment::Raw{length: n, ..} => *n,
            Segment::Banked{length: n, ..} => *n,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Layout(pub Vec<Segment>);

impl Layout {
    // Find a layout segment by name.
    pub fn segment(&self, name: &str) -> Result<&Segment> {
        for segment in self.0.iter() {
            if name == segment.name() {
                return Ok(segment)
            }
        }
        bail!(ErrorKind::UnknownSegmentError(name.into()));
    }

    // Get the layout bounds for the named segment.
    pub fn bound(&self, name: &str) -> Result<(usize, usize)> {
        if name == "__all__" {
            let segment = self.0.last().ok_or(ErrorKind::LayoutError("Layout is empty".into()))?;
            Ok((0, segment.offset() + segment.length()))
        } else {
            let segment = self.segment(name)?;
            Ok((segment.offset(), segment.offset() + segment.length()))
        }
    }
}
