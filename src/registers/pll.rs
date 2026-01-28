//! PLL register definitions for the MAX30009
//!
//! This module defines **only** the raw PLL-related registers and
//! their bit-level encoding, exactly as described in the datasheet.
//!
//! Covered functionality:
//! - PLL multiplication (MDIV)
//! - BioZ ADC clock divider (NDIV)
//! - DDS DAC synthesis divider (KDIV)
//! - PLL enable and lock detection
//! - Reference clock selection and fine tuning
//!
//! ⚠️ IMPORTANT DESIGN RULE
//! ------------------------
//! - This module performs **no validation**
//! - It does **not** enforce sequencing
//! - It does **not** touch hardware
//!
//! All validation and sequencing logic lives in `pll.rs`.
//!
//! Source: MAX30009 datasheet – Timing Subsystem / PLL section

use bitflags::bitflags;

//
// ─────────────────────────────────────────────────────────────────────────────
// PLL Configuration 1 (0x17)
// ─────────────────────────────────────────────────────────────────────────────
//

bitflags! {
    /// PLL Configuration 1 Register
    ///
    /// Address: `0x17`
    ///
    /// Access: **Read / Write**
    ///
    /// Bit layout:
    ///
    /// ```text
    /// BIT:  7      6      5      4    3    2    1    0
    ///       ─────────────────────────────────────────
    ///       MDIV9  MDIV8  NDIV   KDIV[3:0]        PLL_EN
    /// ```
    pub struct PllConfig1: u8 {
        /// MDIV[9:8] – Most significant bits of PLL multiplier
        const MDIV_MSB_MASK = 0b11 << 6;

        /// NDIV – BioZ ADC clock divider
        /// 0 → divide by 512
        /// 1 → divide by 1024
        const NDIV          = 1 << 5;

        /// KDIV[3:0] – DDS synthesis divider
        const KDIV_MASK     = 0b1111 << 1;

        /// PLL_EN – Enable PLL
        const PLL_EN        = 1 << 0;
    }
}

impl PllConfig1 {
    /// Set MDIV[9:8] from a full 10-bit MDIV value
    ///
    /// Datasheet:
    /// ```text
    /// PLL_CLK = REF_CLK × (MDIV + 1)
    /// ```
    ///
    /// ⚠️ No range checking is performed here.
    #[inline]
    pub fn set_mdiv_msb(mut self, mdiv: u16) -> Self {
        let msb = ((mdiv >> 8) & 0b11) as u8;
        self.remove(Self::MDIV_MSB_MASK);
        self.insert(PllConfig1::from_bits_truncate(msb << 6));
        self
    }

    /// Set NDIV (BioZ ADC clock divider)
    #[inline]
    pub fn set_ndiv(mut self, ndiv: NDiv) -> Self {
        self.remove(Self::NDIV);
        if matches!(ndiv, NDiv::Div1024) {
            self.insert(Self::NDIV);
        }
        self
    }

    /// Set KDIV (DDS synthesis divider)
    #[inline]
    pub fn set_kdiv(mut self, kdiv: KDiv) -> Self {
        self.remove(Self::KDIV_MASK);
        self.insert(PllConfig1::from_bits_truncate((kdiv as u8) << 1));
        self
    }

    /// Enable or disable PLL
    #[inline]
    pub fn set_pll_enable(mut self, enable: bool) -> Self {
        self.remove(Self::PLL_EN);
        if enable {
            self.insert(Self::PLL_EN);
        }
        self
    }
}

//
// ─────────────────────────────────────────────────────────────────────────────
// PLL Configuration 2 (0x18)
// ─────────────────────────────────────────────────────────────────────────────
//

/// PLL Configuration 2 Register
///
/// Address: `0x18`
///
/// Contains MDIV[7:0] (lower byte of PLL multiplier).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PllConfig2 {
    mdiv_lsb: u8,
}

impl PllConfig2 {
    /// Create PLL Config 2 from a full 10-bit MDIV value
    ///
    /// Datasheet mapping:
    /// ```text
    /// MDIV[9:8] → PLL_CONFIG_1 (0x17)
    /// MDIV[7:0] → PLL_CONFIG_2 (0x18)
    /// ```
    #[inline]
    pub const fn from_mdiv(mdiv: u16) -> Self {
        Self {
            mdiv_lsb: (mdiv & 0x00FF) as u8,
        }
    }
}

impl From<PllConfig2> for u8 {
    #[inline]
    fn from(cfg: PllConfig2) -> u8 {
        cfg.mdiv_lsb
    }
}

//
// ─────────────────────────────────────────────────────────────────────────────
// PLL Configuration 3 (0x19)
// ─────────────────────────────────────────────────────────────────────────────
//

bitflags! {
    /// PLL Configuration 3 Register
    ///
    /// Controls PLL phase lock detection window.
    pub struct PllConfig3: u8 {
        /// PLL_LOCK_WNDW
        /// 0 → 1 PLL clock period
        /// 1 → 2 PLL clock periods
        const PLL_LOCK_WNDW = 1 << 0;
    }
}

impl PllConfig3 {
    /// Use 1 PLL clock period lock window
    #[inline]
    pub fn lock_window_1clk() -> Self {
        Self::empty()
    }

    /// Use 2 PLL clock period lock window
    ///
    /// Recommended for:
    /// - High-jitter external FCLK
    /// - Internal oscillator
    #[inline]
    pub fn lock_window_2clk() -> Self {
        Self::PLL_LOCK_WNDW
    }
}

//
// ─────────────────────────────────────────────────────────────────────────────
// PLL Configuration 4 (0x1A)
// ─────────────────────────────────────────────────────────────────────────────
//

/// PLL Configuration 4 Register
///
/// Controls reference clock source, frequency selection,
/// and fine tuning of the internal oscillator.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PllConfig4 {
    /// REF_CLK_SEL
    /// false → internal oscillator
    /// true  → external FCLK
    pub ref_clk_sel: bool,

    /// CLK_FREQ_SEL
    /// false → 32.0 kHz
    /// true  → 32.768 kHz
    pub clk_freq_sel: bool,

    /// CLK_FINE_TUNE[4:0]
    ///
    /// Signed 5-bit two’s complement value.
    /// Only applies to **internal oscillator**.
    pub clk_fine_tune: i8,
}

impl From<PllConfig4> for u8 {
    #[inline]
    fn from(cfg: PllConfig4) -> u8 {
        let mut reg = 0u8;

        if cfg.ref_clk_sel {
            reg |= 1 << 6;
        }
        if cfg.clk_freq_sel {
            reg |= 1 << 5;
        }

        // Signed 5-bit two’s complement
        reg |= (cfg.clk_fine_tune & 0x1F) as u8;
        reg
    }
}

//
// ─────────────────────────────────────────────────────────────────────────────
// Enums used by PLL API
// ─────────────────────────────────────────────────────────────────────────────
//

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RefClockSel {
    Internal,
    External,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ClockFreqSel {
    Khz32000,
    Khz32768,
}

impl ClockFreqSel {
    #[inline]
    pub const fn hz(self) -> u32 {
        match self {
            Self::Khz32000 => 32_000,
            Self::Khz32768 => 32_768,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NDiv {
    /// Divide PLL clock by 512
    Div512,

    /// Divide PLL clock by 1024
    Div1024,
}

impl NDiv {
    /// Return the numeric divider value
    #[inline]
    pub const fn divisor(self) -> u32 {
        match self {
            NDiv::Div512 => 512,
            NDiv::Div1024 => 1024,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum KDiv {
    Div1 = 0x0,
    Div2 = 0x1,
    Div4 = 0x2,
    Div8 = 0x3,
    Div16 = 0x4,
    Div32 = 0x5,
    Div64 = 0x6,
    Div128 = 0x7,
    Div256 = 0x8,
    Div512 = 0x9,
    Div1024 = 0xA,
    Div2048 = 0xB,
    Div4096 = 0xC,
    Div8192 = 0xD,
}

impl KDiv {
    #[inline]
    pub const fn divisor(self) -> u32 {
        match self {
            Self::Div1 => 1,
            Self::Div2 => 2,
            Self::Div4 => 4,
            Self::Div8 => 8,
            Self::Div16 => 16,
            Self::Div32 => 32,
            Self::Div64 => 64,
            Self::Div128 => 128,
            Self::Div256 => 256,
            Self::Div512 => 512,
            Self::Div1024 => 1024,
            Self::Div2048 => 2048,
            Self::Div4096 => 4096,
            Self::Div8192 => 8192,
        }
    }
}
