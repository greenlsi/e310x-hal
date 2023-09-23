//! E31 core peripherals

pub mod counters;

pub use e310x::{CLINT, PLIC};

/// Core peripherals
pub struct CorePeripherals {
    /// Performance counters
    pub counters: counters::PerformanceCounters,
}

impl CorePeripherals {
    /// Creates a new instance of CorePeripherals
    pub(crate) fn new() -> Self {
        Self {
            counters: counters::PerformanceCounters::new(),
        }
    }

    /// Steal the peripherals
    pub unsafe fn steal() -> Self {
        Self::new()
    }
}
