use indexmap::{IndexMap, IndexSet};
use pyo3::prelude::*;

/// A Code/Data Logger.
///
/// The CDL records, for each byte of PRG ROM, how the CPU accessed that
/// byte.  This information is used to separate code from data when
/// disassembling a ROM.
#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct Cdl {
    /// One flags byte per byte of PRG ROM.
    #[pyo3(get)]
    pub prg: Vec<u8>,
    /// Indirect control transfers observed: (kind, source PRG offset,
    /// target CPU address, target PRG offset).
    pub jumps: IndexSet<(JumpKind, usize, u16, Option<usize>)>,
    /// PRG banks referenced by each instruction: (instruction PRG offset,
    /// referenced 16K bank).  Used to resolve which bank an address in a
    /// switchable window refers to.
    pub refs: IndexSet<(usize, usize)>,
    /// RAM and SRAM accesses: (instruction PRG offset, CPU address) ->
    /// RAM_* flags.  Internal RAM mirrors are folded to $0000-$07FF.
    pub ram: IndexMap<(usize, u16), u8>,
    /// PRG offset of the instruction currently executing.
    pub(crate) current: Option<usize>,
}

#[pyclass(eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JumpKind {
    /// A `JMP (indirect)` instruction.
    Indirect = 0,
    /// An `RTS` whose return address does not follow a `JSR`
    /// (e.g. a jump table implemented by pushing an address and returning).
    RtsDispatch = 1,
    /// An NMI, IRQ or RESET vector.
    Vector = 2,
}

impl Cdl {
    /// Byte was fetched as an instruction opcode.
    pub const OPCODE: u8 = 0x01;
    /// Byte was fetched as an instruction operand.
    pub const OPERAND: u8 = 0x02;
    /// Byte was read as data by an instruction.
    pub const DATA: u8 = 0x04;
    /// Byte was read as data via an indirect addressing mode.
    pub const DATA_INDIRECT: u8 = 0x08;
    /// Byte was the target of an indirect control transfer.
    pub const INDIRECT_TARGET: u8 = 0x10;
    /// Byte was the target of a `JSR`.
    pub const SUB_ENTRY: u8 = 0x20;
    /// Byte was accessed while mapped into the $8000-$BFFF window.
    pub const WINDOW_8000: u8 = 0x40;
    /// Byte was accessed while mapped into the $C000-$FFFF window.
    pub const WINDOW_C000: u8 = 0x80;

    /// RAM byte was read as data.
    pub const RAM_READ: u8 = 0x01;
    /// RAM byte was written.
    pub const RAM_WRITE: u8 = 0x02;
    /// RAM byte was read as part of a pointer by an indirect addressing mode.
    pub const RAM_POINTER: u8 = 0x04;

    /// Record a RAM or SRAM access by the current instruction.  Other
    /// addresses (registers, ROM) are ignored.
    pub fn mark_ram(&mut self, cpu: u16, flags: u8) {
        let cpu = match cpu {
            0x0000..=0x1FFF => cpu & 0x07FF,
            0x6000..=0x7FFF => cpu,
            _ => return,
        };
        if let Some(src) = self.current {
            *self.ram.entry((src, cpu)).or_insert(0) |= flags;
        }
    }

    pub fn new(prg_size: usize) -> Self {
        Cdl {
            prg: vec![0; prg_size],
            jumps: IndexSet::new(),
            refs: IndexSet::new(),
            ram: IndexMap::new(),
            current: None,
        }
    }

    pub fn mark(&mut self, offset: usize, cpu: u16, flags: u8) {
        if flags & Self::OPCODE != 0 {
            self.current = Some(offset);
        } else if flags & (Self::OPCODE | Self::OPERAND) == 0 {
            // A data access or control transfer target: remember which bank
            // the current instruction referenced.
            if let Some(src) = self.current {
                self.refs.insert((src, offset / 16384));
            }
        }
        if let Some(b) = self.prg.get_mut(offset) {
            let window = if cpu >= 0xC000 {
                Self::WINDOW_C000
            } else {
                Self::WINDOW_8000
            };
            *b |= flags | window;
        }
    }
}

#[pymethods]
impl Cdl {
    /// Return the observed indirect control transfers as a list of
    /// `(kind, source_prg_offset, target_cpu_address, target_prg_offset)`.
    #[getter]
    pub fn get_jumps(&self) -> Vec<(JumpKind, usize, u16, Option<usize>)> {
        self.jumps.iter().cloned().collect()
    }

    /// Return the banks referenced by instructions as a list of
    /// `(instruction_prg_offset, bank)`.
    #[getter]
    pub fn get_refs(&self) -> Vec<(usize, usize)> {
        self.refs.iter().cloned().collect()
    }

    /// Return the RAM and SRAM accesses as a list of
    /// `(instruction_prg_offset, cpu_address, flags)`.
    #[getter]
    pub fn get_ram(&self) -> Vec<(usize, u16, u8)> {
        self.ram.iter().map(|(&(o, a), &f)| (o, a, f)).collect()
    }
}
