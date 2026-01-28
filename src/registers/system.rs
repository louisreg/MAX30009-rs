//! System Control register definitions for the MAX30009
//!
//! This module documents all system-level control registers:
//!
//! - Timing synchronization
//! - Soft reset and shutdown
//! - Interface selection (SPI / I2C)
//! - INT / TRIG pin configuration
//! - I2C broadcast addressing
//!
//! Source: MAX30009 datasheet – System Control section

use bitflags::bitflags;

//
// ─────────────────────────────────────────────────────────────────────────────
// System Sync Register (0x10)
// ─────────────────────────────────────────────────────────────────────────────
//

bitflags! {
    /// System Sync Register
    ///
    /// Address: `0x10`
    ///
    /// Access: **Read / Write**
    /// Reset value: `0x00`
    ///
    /// Bit layout:
    ///
    /// ```text
    /// BIT:  7                  6 ... 0
    ///       ───────────────────────────
    ///       TIMING_SYS_RESET      —
    /// ```
    ///
    /// This register is used to synchronize the **timing subsystems**
    /// of multiple MAX30009 devices.
    pub struct SystemSync: u8 {

        /// BIT 7 – TIMING_SYS_RESET: Generate Timing Reset Signal
        ///
        /// Writing `1` resets the **NDIV divider** used by the PLL timing system.
        ///
        /// Usage constraints:
        /// - Must only be asserted when **all** of the following are `0`:
        ///   - `BIOZ_BG_EN`  (BioZ Configuration 1, bit 2, 0x20)
        ///   - `BIOZ_I_EN`   (BioZ Configuration 1, bit 0, 0x20)
        ///   - `BIOZ_Q_EN`   (BioZ Configuration 1, bit 1, 0x20)
        ///
        /// If any of the above bits are `1`, TIMING_SYS_RESET is **ignored**.
        ///
        /// MASTER interaction:
        /// - If `MASTER = 1`:
        ///   - A synchronization pulse is also output on the **TRIG pin**
        /// - If `MASTER = 0`:
        ///   - Writing `1` has no effect
        ///   - A sync pulse received on the TRIG pin resets NDIV instead
        ///
        /// Behavior:
        /// - Self-clearing bit
        ///
        /// Decode:
        /// - `0`: Normal operation
        /// - `1`: Timing reset requested
        const TIMING_SYS_RESET = 1 << 7;
    }
}

impl SystemSync {
    /// Convenience constructor for asserting timing reset
    #[inline]
    pub fn timing_reset() -> Self {
        Self::TIMING_SYS_RESET
    }
}

//
// ─────────────────────────────────────────────────────────────────────────────
// System Configuration 1 (0x11)
// ─────────────────────────────────────────────────────────────────────────────
//

bitflags! {
    /// System Configuration 1 Register
    ///
    /// Address: `0x11`
    ///
    /// Access: **Read / Write**
    /// Reset value: `0x00`
    ///
    /// Controls master/target role, interface selection,
    /// shutdown mode, and soft reset.
    pub struct SystemConfig: u8 {

        /// BIT 7 – MASTER: Master for Timing System Reset
        ///
        /// Selects whether the device behaves as a **controller (master)**
        /// or **target (slave)** for timing synchronization.
        ///
        /// Interaction with TIMING_SYS_RESET:
        /// - `MASTER = 1`:
        ///   - Device drives synchronization pulses on TRIG
        /// - `MASTER = 0`:
        ///   - Device listens for sync pulses on TRIG
        ///
        /// Decode:
        /// - `0`: Target mode (TRIG pin configured as input)
        /// - `1`: Controller mode (TRIG pin configured as output)
        const MASTER = 1 << 7;

        /// BIT 6 – DISABLE_I2C: Disable I2C Interface
        ///
        /// Selects the digital interface used by the device.
        ///
        /// Decode:
        /// - `0` (default):
        ///   - Interface selected by CSB/I2C_SEL pin
        /// - `1`:
        ///   - SPI interface only
        ///
        /// Notes:
        /// - For SPI operation, this bit should be set to `1`
        ///   during initialization after power-up
        const DISABLE_I2C = 1 << 6;

        /// BIT 1 – SHDN: Shutdown Control
        ///
        /// Puts the device into **shutdown mode**.
        ///
        /// Behavior in shutdown:
        /// - All configuration registers retain their values
        /// - Read/write operations still function
        /// - All interrupts are cleared
        /// - Oscillator is shut down
        /// - Device draws minimum current
        ///
        /// If asserted during an active conversion:
        /// - The conversion is aborted
        ///
        /// Decode:
        /// - `0`: Normal mode
        /// - `1`: Shutdown mode
        const SHDN = 1 << 1;

        /// BIT 0 – RESET: Soft Reset
        ///
        /// Forces a **power-on-reset (POR) sequence**.
        ///
        /// Important:
        /// - The **Soft-Reset Sequence** described in the datasheet
        ///   must be followed
        /// - Incorrect sequencing may cause registers to become
        ///   unresponsive until a hard power cycle
        ///
        /// Behavior:
        /// - Self-clearing bit
        /// - Resets all configuration, threshold, and data registers
        ///   to their power-on default values
        ///
        /// Decode:
        /// - `0`: Normal operation
        /// - `1`: Force POR sequence
        const RESET = 1 << 0;
    }
}

//
// ─────────────────────────────────────────────────────────────────────────────
// Pin Functional Configuration (0x12)
// ─────────────────────────────────────────────────────────────────────────────
//

/// Pin Functional Configuration Register
///
/// Address: `0x12`
///
/// Access: **Read / Write**
///
/// Controls the functional behavior of the **INT** and **TRIG** pins.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PinFuncConfig {
    /// INT_FCFG[1:0] – Functional Configuration for INT Pin
    ///
    /// Decode:
    /// - `0x0`: INT disabled
    /// - `0x1`: INT enabled, cleared when any status or FIFO register is read
    /// - `0x2`: INT enabled, self-clearing after ~30–60 µs (PLL dependent)
    /// - `0x3`: INT enabled, self-clearing after ~240–480 µs (PLL dependent)
    pub int_fcfg: u8,

    /// TRIG_ICFG – TRIG Input Pin Active Edge
    ///
    /// Decode:
    /// - `0`: Falling edge active
    /// - `1`: Rising edge active
    pub trig_icfg: bool,
}

//
// ─────────────────────────────────────────────────────────────────────────────
// Output Pin Configuration (0x13)
// ─────────────────────────────────────────────────────────────────────────────
//

/// Output Pin Configuration Register
///
/// Address: `0x13`
///
/// Access: **Read / Write**
///
/// Selects the output drive type for the INT and TRIG pins.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OutputPinConfig {
    /// INT_OCFG[1:0] – INT Output Drive Type
    ///
    /// Decode:
    /// - `0x0`: Open-drain, active-low
    /// - `0x1`: Push-pull, active-high
    /// - `0x2`: Push-pull, active-low
    /// - `0x3`: Reserved (do not use)
    pub int_ocfg: u8,

    /// TRIG_OCFG[1:0] – TRIG Output Drive Type
    ///
    /// Decode:
    /// - `0x0`: Open-drain, active-low
    /// - `0x1`: Push-pull, active-high
    /// - `0x2`: Push-pull, active-low
    /// - `0x3`: Reserved (do not use)
    pub trig_ocfg: u8,
}

//
// ─────────────────────────────────────────────────────────────────────────────
// I2C Broadcast Address (0x14)
// ─────────────────────────────────────────────────────────────────────────────
//

/// I2C Broadcast Address Register
///
/// Address: `0x14`
///
/// Access: **Read / Write**
///
/// Used to enable **broadcast write transactions**
/// to multiple devices over I2C.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct I2cBroadcast {
    /// I2C_BCAST_ADDR[6:0]
    ///
    /// Upper 7 bits of the broadcast I2C address.
    ///
    /// Notes:
    /// - Used only when `I2C_BCAST_EN = 1`
    /// - Ignored in SPI mode
    pub address: u8,

    /// I2C_BCAST_EN – Enable I2C Broadcast Mode
    ///
    /// Decode:
    /// - `0`: Normal I2C mode (single device)
    /// - `1`: Broadcast mode enabled (write-only)
    ///
    /// Notes:
    /// - Broadcast **read** transactions are not supported
    /// - For SPI systems, broadcast behavior can be achieved
    ///   by asserting CSB on multiple devices simultaneously
    pub enable: bool,
}
