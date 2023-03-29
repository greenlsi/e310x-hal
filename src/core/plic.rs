//! Platform-Level Interrupt Controller
use e310x::plic::PLIC;

/// Parts of `PLIC` peripheral for fine grained permissions.
pub struct Plic {
    /// TODO
    pub plic: PLIC,
}

impl From<PLIC> for Plic {
    fn from(plic: PLIC) -> Self {
        Plic { plic }
    }
}
