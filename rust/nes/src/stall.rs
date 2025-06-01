use pyo3::prelude::*;

#[derive(Default)]
#[pyclass]
pub struct Stall {
    #[pyo3(get, set)]
    pub(crate) cycles: u64,
    pub(crate) odd_cycle: bool,
}

#[pymethods]
impl Stall {
    pub fn clear(&mut self) -> u64 {
        let cycles = self.cycles;
        self.cycles = 0;
        self.odd_cycle = false;
        cycles
    }

    pub fn stall(&mut self, cycles: u64, odd_cycle: bool) {
        self.cycles = cycles;
        self.odd_cycle = odd_cycle;
    }

    pub fn stalling(&self) -> bool {
        self.cycles != 0
    }
}
