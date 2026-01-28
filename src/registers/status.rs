//! Status register definitions for MAX30009
//!
//! This module documents the **Status registers**:
//! - STATUS 1 (0x00)
//! - STATUS 2 (0x01)
//! - PART_ID  (0xFF)
//!
//! Datasheet reference:
//! - MAX30009 – Status Registers section
//!
//! Common behavior:
//! - All status bits are **read-only**
//! - Reading a status register clears all asserted flags
//!   (unless explicitly noted otherwise)

use bitflags::bitflags;

// ─────────────────────────────────────────────────────────────────────────────
// STATUS 1 (0x00)
// ─────────────────────────────────────────────────────────────────────────────

bitflags! {
    /// STATUS 1 Register
    ///
    /// Address: `0x00`
    ///
    /// ```text
    /// BIT:  7        6      5             4            3             2               1              0
    ///       ─────────────────────────────────────────────────────────────────────────────────────────
    ///       A_FULL   —   FIFO_DATA_RDY  FREQ_UNLOCK  FREQ_LOCK   PHASE_UNLOCK   PHASE_LOCK     PWR_RDY
    /// ```
    ///
    /// Reading this register clears all flags.
    #[derive(Default)]
    pub struct Status1: u8 {

        /// FIFO almost full
        ///
        /// Asserted when FIFO reaches the threshold programmed in
        /// `FIFO_A_FULL[7:0]` (0x0D).
        const A_FULL = 1 << 7;

        /// New FIFO data available
        const FIFO_DATA_RDY = 1 << 5;

        /// PLL frequency unlock detected
        ///
        /// Set when the PLL loses frequency lock.
        const FREQ_UNLOCK = 1 << 4;

        /// PLL frequency lock achieved
        const FREQ_LOCK = 1 << 3;

        /// PLL phase unlock detected
        ///
        /// May be unreliable when using:
        /// - internal oscillator
        /// - high-jitter external clock
        const PHASE_UNLOCK = 1 << 2;

        /// PLL phase lock achieved
        const PHASE_LOCK = 1 << 1;

        /// Power ready / UVLO detected
        ///
        /// Asserted when VDVDD falls below the UVLO threshold (~1.3 V).
        ///
        /// Effects:
        /// - All registers reset to POR
        /// - Non-maskable interrupt (always asserted on INT)
        const PWR_RDY = 1 << 0;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// STATUS 2 (0x01)
// ─────────────────────────────────────────────────────────────────────────────

bitflags! {
    /// STATUS 2 Register
    ///
    /// Address: `0x01`
    ///
    /// ```text
    /// BIT:  7        6          5           4        3            2            1            0
    ///       ─────────────────────────────────────────────────────────────────────────────────
    ///       LON   BIOZ_OVER  BIOZ_UNDR   DRV_OOR  DC_LOFF_PH   DC_LOFF_PL   DC_LOFF_NH   DC_LOFF_NL
    /// ```
    ///
    /// Reading this register clears all flags.
    #[derive(Default)]
    pub struct Status2: u8 {

        /// DC lead-on detected
        const LON = 1 << 7;

        /// BioZ ADC over-range
        ///
        /// Absolute BioZ value exceeded `BIOZ_HI_THRESH`.
        const BIOZ_OVER = 1 << 6;

        /// BioZ ADC under-range
        ///
        /// Absolute BioZ value below `BIOZ_LO_THRESH`.
        const BIOZ_UNDR = 1 << 5;

        /// Drive electrode voltage out of range
        ///
        /// Indicates possible drive electrode lead-off.
        const DRV_OOR = 1 << 4;

        /// BIP above high DC lead-off threshold
        const DC_LOFF_PH = 1 << 3;

        /// BIP below low DC lead-off threshold
        const DC_LOFF_PL = 1 << 2;

        /// BIN above high DC lead-off threshold
        const DC_LOFF_NH = 1 << 1;

        /// BIN below low DC lead-off threshold
        const DC_LOFF_NL = 1 << 0;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PART ID (0xFF)
// ─────────────────────────────────────────────────────────────────────────────

/// Part ID Register
///
/// Address: `0xFF`
///
/// Read-only register identifying the silicon revision.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PartId {
    /// PART_ID[7:0]
    ///
    /// Default value for MAX30009: `0x42`
    pub id: u8,
}
