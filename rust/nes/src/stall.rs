use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

#[derive(Default, Serialize, Deserialize)]
#[pyclass]
pub struct Stall {
    pub(crate) cycles: AtomicU64,
    pub(crate) odd_cycle: AtomicBool,
}

impl Clone for Stall {
    fn clone(&self) -> Self {
        Stall {
            cycles: self.cycles.load(Ordering::SeqCst).into(),
            odd_cycle: self.odd_cycle.load(Ordering::SeqCst).into(),
        }
    }
}

#[pymethods]
impl Stall {
    pub fn clear(&self, is_odd: bool) -> u64 {
        let cycles = self.cycles.load(Ordering::SeqCst);
        let odd_cycle = self.odd_cycle.load(Ordering::SeqCst);
        self.cycles.store(0, Ordering::Relaxed);
        self.odd_cycle.store(false, Ordering::Relaxed);
        if odd_cycle && is_odd {
            cycles + 1
        } else {
            cycles
        }
    }

    pub fn stall(&self, cycles: u64, odd_cycle: bool) {
        self.cycles.store(cycles, Ordering::Relaxed);
        self.odd_cycle.store(odd_cycle, Ordering::Relaxed);
    }

    pub fn stalling(&self) -> bool {
        self.cycles.load(Ordering::SeqCst) != 0
    }

    pub fn clone_from(&self, source: &Stall) {
        self.cycles
            .store(source.cycles.load(Ordering::SeqCst), Ordering::Relaxed);
        self.odd_cycle
            .store(source.odd_cycle.load(Ordering::SeqCst), Ordering::Relaxed);
    }
}
