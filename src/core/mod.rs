//! E31 core peripherals

pub mod clint;
pub mod counters;

/// Platform-Level Interrupt Controller
pub use e310x::PLIC;

/// Core peripherals
pub struct CorePeripherals {
    /// Core-Local Interruptor
    pub clint: clint::Clint,

    /// Platform-Level Interrupt Controller
    pub plic: PLIC,

    /// Performance counters
    pub counters: counters::PerformanceCounters,
}

impl CorePeripherals {
    /// Creates a new instance of CorePeripherals
    pub(crate) fn new(clint: e310x::CLINT, plic: e310x::PLIC) -> Self {
        Self {
            clint: clint.into(),
            plic,
            counters: counters::PerformanceCounters::new(),
        }
    }

    /// Steal the peripherals
    pub unsafe fn steal() -> Self {
        let p = e310x::Peripherals::steal();
        Self::new(p.CLINT, p.PLIC)
    }
}
