//! FIFO register definitions for the MAX30009
//!
//! This module documents all FIFO-related registers:
//!
//! - FIFO write/read pointers
//! - FIFO data counters
//! - FIFO data register
//! - FIFO configuration registers
//!
//! The FIFO is a circular buffer of 256 entries.
//! Each FIFO entry corresponds to **one sample**, which is composed
//! of **three bytes** read from the FIFO_DATA register (0x0C).
//!
//! Source: MAX30009 datasheet – FIFO section

use bitflags::bitflags;

//
// ─────────────────────────────────────────────────────────────────────────────
// FIFO Write Pointer (0x08)
// ─────────────────────────────────────────────────────────────────────────────
//

/// FIFO Write Pointer Register
///
/// Address: `0x08`
///
/// Access: **Read-only**
/// Reset value: `0x00`
///
/// `FIFO_WR_PTR[7:0]` points to the FIFO location where the **next sample
/// will be written** by the device.
///
/// Behavior:
/// - Increments automatically when a new sample is pushed to the FIFO
/// - Wraps around from `0xFF` back to `0x00` (circular FIFO)
///
/// Notes:
/// - This pointer is managed entirely by the device
/// - It cannot be written by the host
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FifoWritePtr(pub u8);

//
// ─────────────────────────────────────────────────────────────────────────────
// FIFO Read Pointer (0x09)
// ─────────────────────────────────────────────────────────────────────────────
//

/// FIFO Read Pointer Register
///
/// Address: `0x09`
///
/// Access:
/// - **Read**
/// - **Write** (only if PLL is enabled)
/// Reset value: `0x00`
///
/// `FIFO_RD_PTR[7:0]` points to the FIFO location from which the **next sample
/// will be read** through the serial interface.
///
/// Behavior:
/// - Increments automatically each time a **complete sample**
///   (3 bytes) is read from the FIFO_DATA register (0x0C)
/// - Wraps around from `0xFF` back to `0x00`
///
/// Write behavior:
/// - Allowed **only when PLL is enabled**
/// - Writing allows rereading or retrying FIFO samples
///
/// ⚠️ Warning:
/// Writing this register when the FIFO is almost full can cause
/// adverse effects (e.g. data loss or overflow).
///
/// If PLL is disabled:
/// - Writing to FIFO_RD_PTR is **not allowed**
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FifoReadPtr(pub u8);

//
// ─────────────────────────────────────────────────────────────────────────────
// FIFO Counter 1 (0x0A) – MSB
// ─────────────────────────────────────────────────────────────────────────────
//

/// FIFO Counter 1 Register (MSB)
///
/// Address: `0x0A`
///
/// Access: **Read-only**
/// Reset value:
/// - FIFO_DATA_COUNT[8] = 0
/// - OVF_COUNTER = 0x00
///
/// Bit layout:
///
/// ```text
/// BIT:  7                     6 ... 0
///       ─────────────────────────────────────
///       FIFO_DATA_COUNT[8]   OVF_COUNTER[6:0]
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FifoCounter1 {
    /// Most significant bit of FIFO_DATA_COUNT[8:0]
    pub data_count_msb: bool,

    /// FIFO overflow counter
    ///
    /// Logs the number of samples lost due to FIFO overflow.
    ///
    /// Behavior:
    /// - Increments when FIFO is full and new samples arrive
    /// - Behavior depends on FIFO_RO (0x0E)
    /// - Saturates at `0x7F`
    ///
    /// Reset:
    /// - Reset to zero when a **complete sample** is read
    ///   and the FIFO read pointer advances
    ///
    /// Usage recommendation:
    /// - Read **immediately before reading FIFO data**
    ///   to detect overflow conditions
    pub overflow_counter: u8,
}

//
// ─────────────────────────────────────────────────────────────────────────────
// FIFO Counter 2 (0x0B) – LSB
// ─────────────────────────────────────────────────────────────────────────────
//

/// FIFO Counter 2 Register (LSB)
///
/// Address: `0x0B`
///
/// Access: **Read-only**
/// Reset value: `0x00`
///
/// Contains the lower 8 bits of `FIFO_DATA_COUNT[8:0]`.
///
/// `FIFO_DATA_COUNT[8:0]`:
/// - Increments when a new sample is pushed to the FIFO
/// - Decrements when the host reads a sample
///
/// Useful mainly for debugging and monitoring FIFO fill level.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FifoCounter2(pub u8);

//
// ─────────────────────────────────────────────────────────────────────────────
// FIFO Data Register (0x0C)
// ─────────────────────────────────────────────────────────────────────────────
//

/// FIFO Data Register
///
/// Address: `0x0C`
///
/// Access: **Read-only**
/// Reset value: `0xFF`
///
/// This register is used to read data from the FIFO using **burst reads only**.
///
/// Behavior:
/// - The register address does **not auto-increment**
/// - Each burst read of **three bytes** corresponds to **one FIFO sample**
/// - After reading 3 bytes:
///   - `FIFO_RD_PTR` increments by one
///   - The next sample becomes available
///
/// Notes:
/// - The format and meaning of FIFO data depends on the **tag**
///   embedded in the first byte of each sample
/// - See the FIFO Description section of the datasheet
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FifoData(pub u8);

//
// ─────────────────────────────────────────────────────────────────────────────
// FIFO Configuration 1 (0x0D)
// ─────────────────────────────────────────────────────────────────────────────
//

/// FIFO Configuration 1 Register
///
/// Address: `0x0D`
///
/// Access: **Read / Write**
/// Reset value: `0x7F`
///
/// `FIFO_A_FULL[7:0]` defines the **almost-full threshold** of the FIFO.
///
/// The A_FULL status bit (Status 1, bit 7) is asserted when:
///
/// ```text
/// FIFO contains (256 − FIFO_A_FULL) samples
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FifoConfig1 {
    /// Number of free FIFO spaces before A_FULL is asserted
    ///
    /// Examples:
    ///
    /// | FIFO_A_FULL | Free Spaces | Samples in FIFO |
    /// |--------------|-------------|-----------------|
    /// | 0x00         | 0           | 256             |
    /// | 0x01         | 1           | 255             |
    /// | 0x0F         | 15          | 241             |
    /// | 0xFF         | 255         | 1               |
    pub fifo_a_full: u8,
}

//
// ─────────────────────────────────────────────────────────────────────────────
// FIFO Configuration 2 (0x0E)
// ─────────────────────────────────────────────────────────────────────────────
//

bitflags! {
    /// FIFO Configuration 2 Register
    ///
    /// Address: `0x0E`
    ///
    /// Access: **Read / Write**
    ///
    /// Contains FIFO control bits and interrupt/status behavior configuration.
    pub struct FifoConfig2: u8 {

        /// BIT 5 – FIFO_MARK: Push Marker to FIFO
        ///
        /// When set to `1`, a **marker tag** is pushed into the FIFO.
        ///
        /// Behavior:
        /// - Self-clearing bit
        /// - Used to differentiate FIFO data before and after the marker
        const FIFO_MARK = 1 << 5;

        /// BIT 4 – FLUSH_FIFO: Manual FIFO Flush
        ///
        /// When set to `1`:
        /// - FIFO contents are discarded
        /// - The following registers are reset:
        ///   - FIFO_WR_PTR (0x08)
        ///   - FIFO_RD_PTR (0x09)
        ///   - FIFO_DATA_COUNT (0x0A, 0x0B)
        ///   - OVF_COUNTER (0x0A)
        ///
        /// Behavior:
        /// - Self-clearing bit
        const FLUSH_FIFO = 1 << 4;

        /// BIT 3 – FIFO_STAT_CLR: FIFO Status Clear Mode
        ///
        /// Controls how FIFO-related status bits are cleared.
        ///
        /// Decode:
        /// - `0`:
        ///   - A_FULL and FIFO_DATA_RDY are cleared
        ///     **only** by reading Status 1 (0x00)
        /// - `1`:
        ///   - A_FULL and FIFO_DATA_RDY are cleared by:
        ///     - Reading FIFO_DATA (0x0C), **or**
        ///     - Reading Status 1 (0x00)
        const FIFO_STAT_CLR = 1 << 3;

        /// BIT 2 – A_FULL_TYPE: A_FULL Interrupt Type
        ///
        /// Controls whether A_FULL reasserts continuously.
        ///
        /// Decode:
        /// - `0`:
        ///   - A_FULL reasserts for every sample
        ///     while FIFO remains almost full
        /// - `1`:
        ///   - A_FULL asserts only once per almost-full condition
        const A_FULL_TYPE = 1 << 2;

        /// BIT 1 – FIFO_RO: FIFO Roll-Over Enable
        ///
        /// Controls FIFO behavior when completely full.
        ///
        /// Decode:
        /// - `0`:
        ///   - FIFO stops on full
        ///   - New samples are lost
        /// - `1`:
        ///   - FIFO rolls over
        ///   - Old samples are overwritten
        ///   - Both read and write pointers increment
        const FIFO_RO = 1 << 1;
    }
}
