//! Register addresses for the MAX30009
//!
//! This enum lists **all user-accessible registers**, grouped
//! by functional block. The values match the register addresses
//! defined in the datasheet.

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Register {
    // ─────────────────────────────────────────────────────────────
    // Status Registers
    // ─────────────────────────────────────────────────────────────
    /// Status 1 Register
    ///
    /// Address: 0x00
    ///
    /// Contains FIFO, PLL, and power-related status flags.
    /// Read-only, read-to-clear.
    Status1 = 0x00,

    /// Status 2 Register
    ///
    /// Address: 0x01
    ///
    /// Contains BioZ lead-on/lead-off and range status flags.
    /// Read-only, read-to-clear.
    Status2 = 0x01,

    // ─────────────────────────────────────────────────────────────
    // FIFO Registers
    // ─────────────────────────────────────────────────────────────
    /// FIFO Write Pointer
    ///
    /// Address: 0x08
    FifoWritePtr = 0x08,

    /// FIFO Read Pointer
    ///
    /// Address: 0x09
    FifoReadPtr = 0x09,

    /// FIFO Counter 1 (MSB)
    ///
    /// Address: 0x0A
    ///
    /// - FIFO_DATA_COUNT[8]
    /// - OVF_COUNTER[6:0]
    FifoCounter1 = 0x0A,

    /// FIFO Counter 2 (LSB)
    ///
    /// Address: 0x0B
    ///
    /// - FIFO_DATA_COUNT[7:0]
    FifoCounter2 = 0x0B,

    /// FIFO Data Register
    ///
    /// Address: 0x0C
    ///
    /// Reading this register pops data from the FIFO.
    FifoData = 0x0C,

    /// FIFO Configuration 1
    ///
    /// Address: 0x0D
    ///
    /// - FIFO_A_FULL[7:0]
    FifoConfig1 = 0x0D,

    /// FIFO Configuration 2
    ///
    /// Address: 0x0E
    ///
    /// Contains FIFO control and status clear bits.
    FifoConfig2 = 0x0E,

    // ─────────────────────────────────────────────────────────────
    // System Control
    // ─────────────────────────────────────────────────────────────
    /// System Sync Register
    ///
    /// Address: 0x10
    ///
    /// - TIMING_SYS_RESET
    SystemSync = 0x10,

    /// System Configuration 1
    ///
    /// Address: 0x11
    ///
    /// Contains master mode, shutdown, and reset bits.
    SystemConfig = 0x11,

    /// Pin Functional Configuration
    ///
    /// Address: 0x12
    PinFuncConfig = 0x12,

    /// Output Pin Configuration
    ///
    /// Address: 0x13
    OutputPinConfig = 0x13,

    /// I2C Broadcast Address
    ///
    /// Address: 0x14
    I2cBroadcastAddr = 0x14,

    // ─────────────────────────────────────────────────────────────
    // PLL Configuration
    // ─────────────────────────────────────────────────────────────
    /// PLL Configuration 1
    ///
    /// Address: 0x17
    ///
    /// Contains MDIV[9:8], NDIV, KDIV, PLL_EN.
    PllConfig1 = 0x17,

    /// PLL Configuration 2
    ///
    /// Address: 0x18
    ///
    /// Contains MDIV[7:0].
    PllConfig2 = 0x18,

    /// PLL Configuration 3
    ///
    /// Address: 0x19
    ///
    /// Contains PLL_LOCK_WNDW.
    PllConfig3 = 0x19,

    /// PLL Configuration 4
    ///
    /// Address: 0x1A
    ///
    /// Contains REF_CLK_SEL, CLK_FREQ_SEL, CLK_FINE_TUNE.
    PllConfig4 = 0x1A,

    // ─────────────────────────────────────────────────────────────
    // BioZ Setup
    // ─────────────────────────────────────────────────────────────
    BiozConfig1 = 0x20,
    BiozConfig2 = 0x21,
    BiozConfig3 = 0x22,
    BiozConfig4 = 0x23,
    BiozConfig5 = 0x24,
    BiozConfig6 = 0x25,

    /// BioZ Low Threshold
    BiozLowThresh = 0x26,

    /// BioZ High Threshold
    BiozHighThresh = 0x27,

    BiozConfig7 = 0x28,

    // ─────────────────────────────────────────────────────────────
    // BioZ Calibration / Mux
    // ─────────────────────────────────────────────────────────────
    BiozMuxConfig1 = 0x41,
    BiozMuxConfig2 = 0x42,
    BiozMuxConfig3 = 0x43,
    BiozMuxConfig4 = 0x44,

    // ─────────────────────────────────────────────────────────────
    // DC Leads Setup
    // ─────────────────────────────────────────────────────────────
    DcLeadsConfig = 0x50,
    DcLeadDetectThresh = 0x51,

    // ─────────────────────────────────────────────────────────────
    // Lead Bias
    // ─────────────────────────────────────────────────────────────
    LeadBiasConfig1 = 0x58,

    // ─────────────────────────────────────────────────────────────
    // Interrupt Enables
    // ─────────────────────────────────────────────────────────────
    InterruptEnable1 = 0x80,
    InterruptEnable2 = 0x81,

    // ─────────────────────────────────────────────────────────────
    // Part Identification
    // ─────────────────────────────────────────────────────────────
    /// Part ID Register
    ///
    /// Address: 0xFF
    PartId = 0xFF,
}
