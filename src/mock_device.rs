//! Mock MAX30009 device for host-side testing
//!
//! This mock simulates register-level access to the MAX30009.
//! It is intended for:
//! - unit tests
//! - integration tests
//! - validating configuration sequences
//!
//! ⚠️ This module uses `std` and must only be compiled for tests.

#![cfg(test)]

extern crate std;

use std::collections::HashMap;
use std::vec::Vec;

use crate::register_interface::RegisterInterface;

/// Mock MAX30009 device
///
/// Features:
/// - Records all register writes
/// - Stores register values
/// - Simulates delays
/// - Simulates PLL lock behavior
#[derive(Debug, Default)]
pub struct MockDevice {
    /// Register storage (addr → value)
    registers: HashMap<u8, u8>,

    /// Log of all register writes (addr, value)
    pub writes: Vec<(u8, u8)>,

    /// Requested delays in milliseconds
    pub delays_ms: Vec<u32>,

    /// Whether PLL lock was waited for
    pub waited_for_pll_lock: bool,

    /// Whether PLL is considered locked
    pll_locked: bool,
}

impl MockDevice {
    /// Create a new mock device
    pub fn new() -> Self {
        Self::default()
    }

    /// Manually set a register value
    pub fn set_reg(&mut self, addr: u8, value: u8) {
        self.registers.insert(addr, value);
    }

    /// Read back a register value
    pub fn get_reg(&self, addr: u8) -> Option<u8> {
        self.registers.get(&addr).copied()
    }

    /// Enable or disable PLL lock simulation
    pub fn set_pll_locked(&mut self, locked: bool) {
        self.pll_locked = locked;
    }

    /// Clear all logs (writes, delays, flags)
    pub fn clear_logs(&mut self) {
        self.writes.clear();
        self.delays_ms.clear();
        self.waited_for_pll_lock = false;
    }
}

impl RegisterInterface for MockDevice {
    type Error = ();

    fn write_reg(&mut self, addr: u8, value: u8) -> Result<(), Self::Error> {
        self.writes.push((addr, value));
        Ok(())
    }

    fn delay_ms(&mut self, ms: u32) {
        self.delays_ms.push(ms);
    }

    fn wait_for_pll_lock(&mut self) -> Result<(), Self::Error> {
        self.waited_for_pll_lock = true;
        Ok(())
    }
}
