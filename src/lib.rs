#![no_std]
//! MAX30009 Bioimpedance AFE driver
//!
//! This crate provides a **register-accurate**, **datasheet-driven**
//! driver for the MAX30009 BioZ AFE.
//!
//! Design goals:
//! - No HAL lock-in (SPI / I2C abstracted)
//! - 1:1 mapping with datasheet registers
//! - Safe high-level builders (PLL, BioZ, Leads, FIFO, Interrupts)
//! - Suitable for GUI / block-diagram configuration tools
//!
//! Datasheet: MAX30009 Rev. 1+

// =======================
// Public modules
// =======================
pub mod register_interface;

/// Register addresses and bitfield definitions
///
/// This module contains **ONLY**:
/// - register addresses
/// - bitfield structs
/// - enums mapping datasheet values
///
/// No logic, no delays, no sequencing.
pub mod registers;

/// PLL / timing subsystem
///
/// Implements:
/// - MDIV / NDIV / KDIV configuration
/// - reference clock selection
/// - PLL enable & lock sequencing
pub mod pll;

/// BioZ stimulus and receive chain configuration
///
/// Covers registers:
/// - 0x20–0x28 (BioZ config)
/// - 0x41–0x44 (MUX & calibration)
pub mod bioz;

/// DC lead-on / lead-off detection and biasing
///
/// Covers registers:
/// - 0x50–0x51 (DC leads)
/// - 0x58 (lead bias)
pub mod leads;

/// Interrupt routing to INT pin
///
/// Covers registers:
/// - 0x80–0x81 (interrupt enable)
pub mod interrupts;

/// FIFO configuration and data handling
///
/// Covers registers:
/// - 0x08–0x0E
/// - FIFO read / flush / markers
pub mod fifo;

// =======================
// Re-exports (ergonomics)
// =======================

// High-level configs
pub use bioz::*;
pub use interrupts::*;
pub use leads::*;
pub use pll::PllConfig;

// Common enums users will want
pub use registers::bioz::*;
pub use registers::leads::*;
pub use registers::pll::{ClockFreqSel, KDiv, NDiv};

// -----------------------------------------------------------------------------
// Test-only modules
// -----------------------------------------------------------------------------
#[cfg(test)]
extern crate std;

#[cfg(test)]
pub mod mock_device;
