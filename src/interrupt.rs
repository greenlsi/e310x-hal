//! Vectored machine external interrupt handler.
//!
//! # Notes
//!
//! - You must activate the `virq` feature to use this module.
//!
//! - The vectored handler automatically claims the PLIC interrupt source as complete.
//! Thus, users do not have to worry about this step.
//!
//! # Features
//!
//! This module provides:
//!
//! - A vectored implementation for handling each machine external interrupt source independently.
//!
//! - A linker script that provides weak symbols for all the interrupt sources of an E310X microcontroller.
//!   This file must be supplied using rustflag when compiling.
//!
//! # Implementation details
//!
//! You can define a custom handler for each interrupt source (see [`e310x::interrupt::Interrupt`]).
//! For instance, if you want to define a custom handler for interrupts triggered by
//!  the [`e310x::interrupt::Interrupt::GPIO0`] source, you must define the `GPIO0` function:
//!
//! ```ignore
//! #[no_mangle]
//! #[allow(non_snake_case)]
//! fn GPIO0() {
//!     // define the behavior of your custom handler
//! }
//! ```
//!
//! Note that the function must be marked as `no_mangle`.
//! You can also use the [`e310x::interrupt!`] macro.
//!
//! If a source without custom handler triggers an interruption, it executes the
//! `OtherMachineExternal` handler. This handler function is shared among all the
//! undefined interrupt sources. You can define this handler as follows:
//!
//! ```ignore,no_run
//! #[no_mangle]
//! #[allow(non_snake_case)]
//! fn OtherMachineExternal() {
//!     // define the behavior of this handler
//! }
//! ```
//!
//! By default, `OtherMachineExternal` executes the [`DefaultMachineExternal`] handler.
//! This handler is just an infinite loop.

pub use e310x::{PLIC, __EXTERNAL_INTERRUPTS};

#[no_mangle]
#[allow(non_snake_case)]
/// Default machine external interrupt handler. It is an infinite loop.
pub fn DefaultMachineExternal() {
    loop {
        // Prevent this from turning into a UDF instruction
        // see rust-lang/rust#28728 for details
        continue;
    }
}

/// Handler for vectored machine external interrupts (see the [`riscv-rt`] crate).
#[no_mangle]
#[allow(non_snake_case)]
unsafe fn MachineExternal() {
    if let Some(interrupt) = e310x::PLIC::claim() {
        (__EXTERNAL_INTERRUPTS[interrupt as usize - 1]._handler)();
        e310x::PLIC::complete(interrupt);
    }
}
