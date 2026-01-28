//! Interrupt Enable Registers (MAX30009)
//!
//! This module defines which **status events** are forwarded to the `INT` pin.
//!
//! Notes:
//! - Status bits are **always updated internally**
//! - These registers only control whether a given status bit
//!   generates an interrupt on the `INT` output

// ─────────────────────────────────────────────────────────────────────────────
// Interrupt Enable Register 1 (0x80)
// ─────────────────────────────────────────────────────────────────────────────

/// Interrupt Enable Register 1
///
/// Address: `0x80`
///
/// Controls interrupts related to:
/// - FIFO status
/// - PLL lock / unlock events
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct InterruptEnable1 {
    /// A_FULL_EN – FIFO Almost-Full Interrupt Enable
    pub a_full: bool,

    /// FIFO_DATA_RDY_EN – FIFO Data Ready Interrupt Enable
    pub fifo_data_rdy: bool,

    /// FREQ_UNLOCK_EN – PLL Frequency Unlock Interrupt Enable
    pub freq_unlock: bool,

    /// FREQ_LOCK_EN – PLL Frequency Lock Interrupt Enable
    pub freq_lock: bool,

    /// PHASE_UNLOCK_EN – PLL Phase Unlock Interrupt Enable
    pub phase_unlock: bool,

    /// PHASE_LOCK_EN – PLL Phase Lock Interrupt Enable
    pub phase_lock: bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// Interrupt Enable Register 2 (0x81)
// ─────────────────────────────────────────────────────────────────────────────

/// Interrupt Enable Register 2
///
/// Address: `0x81`
///
/// Controls interrupts related to:
/// - BioZ lead detection
/// - Drive electrode safety conditions
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct InterruptEnable2 {
    /// LON_EN – Lead-On Detection Interrupt Enable
    pub lon: bool,

    /// BIOZ_OVER_EN – BioZ Over-Range Interrupt Enable
    pub bioz_over: bool,

    /// BIOZ_UNDR_EN – BioZ Under-Range Interrupt Enable
    pub bioz_under: bool,

    /// DRV_OOR_EN – Drive Electrode Out-of-Range Interrupt Enable
    pub drv_oor: bool,

    /// DC_LOFF_PH_EN – DC Lead-Off Positive High Interrupt Enable
    pub dc_loff_ph: bool,

    /// DC_LOFF_PL_EN – DC Lead-Off Positive Low Interrupt Enable
    pub dc_loff_pl: bool,

    /// DC_LOFF_NH_EN – DC Lead-Off Negative High Interrupt Enable
    pub dc_loff_nh: bool,

    /// DC_LOFF_NL_EN – DC Lead-Off Negative Low Interrupt Enable
    pub dc_loff_nl: bool,
}
