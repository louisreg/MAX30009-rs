//! MAX30009 Bioimpedance AFE driver.
//!
//! The crate is split by register domains (PLL, BioZ, FIFO, leads, system...)
//! and provides:
//! - typed register wrappers,
//! - sparse read-modify-write updates,
//! - high-level runtime helpers on [`device::Max30009`].
//!
//! Datasheet reference: `max30009.pdf` at repository root.

#![cfg_attr(not(test), no_std)]

#[cfg(test)]
extern crate std;

pub mod log;
#[allow(unused_imports)]
pub use log::*;

pub mod bioz;
pub mod bioz_mux;
pub mod device;
pub mod fifo;
pub mod interrupts;
pub mod leads;
pub mod pll;
pub mod register_interface;
pub mod register_map;
pub mod status;
pub mod system;
pub mod traits;

// High-level configuration re-exports.
pub use bioz::config::{
    BiozConfig1, BiozConfig2, BiozConfig3, BiozConfig4, BiozConfig5, BiozConfig6, BiozConfig7,
    BiozThresholds,
};
pub use bioz_mux::config::{BiozMuxBist, BiozMuxConfig1, BiozMuxConfig2, BiozMuxConfig3};
pub use device::Max30009;
pub use fifo::config::{FifoConfig, FifoCtrl, FifoStatus};
pub use interrupts::config::{InterruptEnable1, InterruptEnable2};
pub use leads::config::{DcLeadThresh, DcLeadsConfig, LeadBiasConfig1};
pub use pll::config::{PllConfig, PllFrequency};
pub use status::status::Status;
pub use system::config::SystemUpdate;
