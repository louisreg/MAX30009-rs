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
//!
//! Source: MAX30009 datasheet – Timing Subsystem / PLL section

use crate::pll::types::*;
use crate::register_map::Register;
use crate::traits::{ReadableRegister, WritableRegister};
use bitflags::bitflags;

//
// PLL CONFIG 1
//

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct PllConfig1: u8 {
        const MDIV_MSB_MASK = 0b11 << 6;
        const NDIV          = 1 << 5;
        const KDIV_MASK     = 0b1111 << 1;
        const PLL_EN        = 1 << 0;
    }
}

impl Default for PllConfig1 {
    fn default() -> Self {
        Self::empty()
    }
}

impl WritableRegister for PllConfig1 {
    const ADDR: u8 = Register::PllConfig1 as u8;

    fn value(&self) -> u8 {
        self.bits()
    }

    fn name() -> &'static str {
        "PLL_CONFIG1"
    }
}

impl ReadableRegister for PllConfig1 {
    const ADDR: u8 = Register::PllConfig1 as u8;

    fn from_raw(v: u8) -> Self {
        PllConfig1::from_bits_truncate(v)
    }

    fn name() -> &'static str {
        "PLL_CONFIG1"
    }
}

impl PllConfig1 {
    pub fn set_mdiv_msb(mut self, mdiv: u16) -> Self {
        let msb = ((mdiv >> 8) & 0b11) as u8;
        self.remove(Self::MDIV_MSB_MASK);
        self.insert(PllConfig1::from_bits_truncate(msb << 6));
        self
    }

    pub fn set_ndiv(mut self, ndiv: NDiv) -> Self {
        self.remove(Self::NDIV);
        if matches!(ndiv, NDiv::Div1024) {
            self.insert(Self::NDIV);
        }
        self
    }

    pub fn set_kdiv(mut self, kdiv: KDiv) -> Self {
        self.remove(Self::KDIV_MASK);
        self.insert(PllConfig1::from_bits_truncate((kdiv as u8) << 1));
        self
    }

    pub fn set_pll_enable(mut self, enable: bool) -> Self {
        self.remove(Self::PLL_EN);
        if enable {
            self.insert(Self::PLL_EN);
        }
        self
    }
}

//
// PLL CONFIG 2
//

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PllConfig2 {
    pub(crate) mdiv_lsb: u8,
}

impl From<u16> for PllConfig2 {
    fn from(mdiv: u16) -> Self {
        Self {
            mdiv_lsb: (mdiv & 0xFF) as u8,
        }
    }
}

impl ReadableRegister for PllConfig2 {
    const ADDR: u8 = Register::PllConfig2 as u8;

    fn from_raw(v: u8) -> Self {
        Self { mdiv_lsb: v }
    }

    fn name() -> &'static str {
        "PLL_CONFIG2"
    }
}

impl WritableRegister for PllConfig2 {
    const ADDR: u8 = Register::PllConfig2 as u8;

    fn value(&self) -> u8 {
        self.mdiv_lsb
    }

    fn name() -> &'static str {
        "PLL_CONFIG2"
    }
}

//
// PLL CONFIG 3
//

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct PllConfig3: u8 {
        const PLL_LOCK_WNDW = 1 << 0;
    }
}

impl WritableRegister for PllConfig3 {
    const ADDR: u8 = Register::PllConfig3 as u8;

    fn value(&self) -> u8 {
        self.bits()
    }

    fn name() -> &'static str {
        "PLL_CONFIG3"
    }
}

impl ReadableRegister for PllConfig3 {
    const ADDR: u8 = Register::PllConfig3 as u8;

    fn from_raw(v: u8) -> Self {
        PllConfig3::from_bits_truncate(v)
    }

    fn name() -> &'static str {
        "PLL_CONFIG3"
    }
}

impl PllConfig3 {
    pub fn lock_window_1clk() -> Self {
        Self::empty()
    }

    pub fn lock_window_2clk() -> Self {
        Self::PLL_LOCK_WNDW
    }
}

//
// PLL CONFIG 4
//

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PllConfig4 {
    pub ref_clk_sel: bool,
    pub clk_freq_sel: bool,
    pub clk_fine_tune: i8,
}

impl WritableRegister for PllConfig4 {
    const ADDR: u8 = Register::PllConfig4 as u8;

    fn value(&self) -> u8 {
        self.encode()
    }

    fn name() -> &'static str {
        "PLL_CONFIG4"
    }
}

impl ReadableRegister for PllConfig4 {
    const ADDR: u8 = Register::PllConfig4 as u8;

    fn from_raw(v: u8) -> Self {
        Self {
            ref_clk_sel: (v & (1 << 6)) != 0,
            clk_freq_sel: (v & (1 << 5)) != 0,
            clk_fine_tune: (v & 0x1F) as i8,
        }
    }

    fn name() -> &'static str {
        "PLL_CONFIG4"
    }
}

impl PllConfig4 {
    pub fn encode(self) -> u8 {
        let mut reg = 0u8;

        if self.ref_clk_sel {
            reg |= 1 << 6;
        }
        if self.clk_freq_sel {
            reg |= 1 << 5;
        }

        reg |= (self.clk_fine_tune & 0x1F) as u8;
        reg
    }
}
