//! BioZ RAW register definitions

use crate::bioz::types::*;
use crate::register_map::Register;
use crate::traits::{ReadableRegister, WritableRegister};
use bitflags::bitflags;

//
// ─────────────────────────────────────────
// BIOZ CONFIG 1 (0x20)
// ─────────────────────────────────────────
//

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct RegBiozConfig1: u8 {
        const DAC_OSR_MASK = 0b11 << 6;
        const ADC_OSR_MASK = 0b111 << 3;
        const BG_EN = 1 << 2;
        const Q_EN  = 1 << 1;
        const I_EN  = 1 << 0;
    }
}

impl WritableRegister for RegBiozConfig1 {
    const ADDR: u8 = Register::BiozConfig1 as u8;

    fn value(&self) -> u8 {
        self.bits()
    }

    fn name() -> &'static str {
        "BIOZ_CONFIG1"
    }
}

impl ReadableRegister for RegBiozConfig1 {
    const ADDR: u8 = Register::BiozConfig1 as u8;

    fn from_raw(v: u8) -> Self {
        RegBiozConfig1::from_bits_truncate(v)
    }

    fn name() -> &'static str {
        "BIOZ_CONFIG1"
    }
}

impl ReadableRegister for RegBiozConfig2 {
    const ADDR: u8 = Register::BiozConfig2 as u8;

    fn from_raw(v: u8) -> Self {
        RegBiozConfig2::from_bits_truncate(v)
    }

    fn name() -> &'static str {
        "BIOZ_CONFIG2"
    }
}

impl RegBiozConfig1 {
    pub fn set_dac_osr(mut self, osr: BiozDacOsr) -> Self {
        self.remove(Self::DAC_OSR_MASK);
        self.insert(Self::from_bits_truncate((osr as u8) << 6));
        self
    }

    pub fn set_adc_osr(mut self, osr: BiozAdcOsr) -> Self {
        self.remove(Self::ADC_OSR_MASK);
        self.insert(Self::from_bits_truncate((osr as u8) << 3));
        self
    }

    pub fn enable_bg(mut self, en: bool) -> Self {
        self.set(Self::BG_EN, en);
        self
    }

    pub fn enable_i(mut self, en: bool) -> Self {
        self.set(Self::I_EN, en);
        self
    }

    pub fn enable_q(mut self, en: bool) -> Self {
        self.set(Self::Q_EN, en);
        self
    }
}

//
// ─────────────────────────────────────────
// BIOZ CONFIG 2 (0x21)
// ─────────────────────────────────────────
//

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct RegBiozConfig2: u8 {
        const DHPF_MASK = 0b11 << 6;
        const DLPF_MASK = 0b111 << 3;
        const CMP_MASK  = 0b11 << 1;
        const EN_THRESH = 1 << 0;
    }
}

impl WritableRegister for RegBiozConfig2 {
    const ADDR: u8 = Register::BiozConfig2 as u8;

    fn value(&self) -> u8 {
        self.bits()
    }

    fn name() -> &'static str {
        "BIOZ_CONFIG2"
    }
}

impl RegBiozConfig2 {
    pub fn set_dhpf(mut self, v: BiozDhpf) -> Self {
        self.remove(Self::DHPF_MASK);
        self.insert(Self::from_bits_truncate((v as u8) << 6));
        self
    }

    pub fn set_dlpf(mut self, v: BiozDlpf) -> Self {
        self.remove(Self::DLPF_MASK);
        self.insert(Self::from_bits_truncate((v as u8) << 3));
        self
    }

    pub fn set_cmp(mut self, v: BiozCmp) -> Self {
        self.remove(Self::CMP_MASK);
        self.insert(Self::from_bits_truncate((v as u8) << 1));
        self
    }

    pub fn enable_threshold(mut self, en: bool) -> Self {
        self.set(Self::EN_THRESH, en);
        self
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct RegBiozConfig3: u8 {
        const EXT_RES     = 1 << 7;
        const LOFF_RAPID  = 1 << 6;
        const VDRV_MASK   = 0b11 << 4;
        const IDRV_MASK   = 0b11 << 2;
        const DRV_MODE_MASK = 0b11;
    }
}

impl WritableRegister for RegBiozConfig3 {
    const ADDR: u8 = Register::BiozConfig3 as u8;

    fn value(&self) -> u8 {
        self.bits()
    }
    fn name() -> &'static str {
        "BIOZ_CONFIG3"
    }
}

impl ReadableRegister for RegBiozConfig3 {
    const ADDR: u8 = Register::BiozConfig3 as u8;

    fn from_raw(v: u8) -> Self {
        Self::from_bits_truncate(v)
    }

    fn name() -> &'static str {
        "BIOZ_CONFIG3"
    }
}

impl RegBiozConfig3 {
    pub fn set_vdrv(mut self, v: BiozVdrvMag) -> Self {
        self.remove(Self::VDRV_MASK);
        self.insert(Self::from_bits_truncate((v as u8) << 4));
        self
    }

    pub fn set_idrv(mut self, v: BiozIdrvRge) -> Self {
        self.remove(Self::IDRV_MASK);
        self.insert(Self::from_bits_truncate((v as u8) << 2));
        self
    }

    pub fn set_mode(mut self, v: BiozDrvMode) -> Self {
        self.remove(Self::DRV_MODE_MASK);
        self.insert(Self::from_bits_truncate(v as u8));
        self
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct RegBiozConfig4: u8 {
        const FAST_MANUAL   = 1 << 1;
        const FAST_START_EN = 1 << 0;
    }
}

impl WritableRegister for RegBiozConfig4 {
    const ADDR: u8 = Register::BiozConfig4 as u8;

    fn value(&self) -> u8 {
        self.bits()
    }
    fn name() -> &'static str {
        "BIOZ_CONFIG4"
    }
}

impl ReadableRegister for RegBiozConfig4 {
    const ADDR: u8 = Register::BiozConfig4 as u8;

    fn from_raw(v: u8) -> Self {
        Self::from_bits_truncate(v)
    }

    fn name() -> &'static str {
        "BIOZ_CONFIG4"
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct RegBiozConfig5: u8 {
        const AHPF_MASK = 0b1111 << 4;
        const INA_MODE  = 1 << 3;
        const DM_DIS    = 1 << 2;
        const GAIN_MASK = 0b11;
    }
}

impl WritableRegister for RegBiozConfig5 {
    const ADDR: u8 = Register::BiozConfig5 as u8;
    fn value(&self) -> u8 {
        self.bits()
    }
    fn name() -> &'static str {
        "BIOZ_CONFIG5"
    }
}

impl ReadableRegister for RegBiozConfig5 {
    const ADDR: u8 = Register::BiozConfig5 as u8;
    fn from_raw(v: u8) -> Self {
        Self::from_bits_truncate(v)
    }
    fn name() -> &'static str {
        "BIOZ_CONFIG5"
    }
}

impl RegBiozConfig5 {
    #[inline]
    pub fn set_ahpf(mut self, v: BiozAhpf) -> Self {
        self.remove(Self::AHPF_MASK);
        self.insert(Self::from_bits_truncate((v as u8) << 4));
        self
    }

    #[inline]
    pub fn set_gain(mut self, v: BiozGain) -> Self {
        self.remove(Self::GAIN_MASK);
        self.insert(Self::from_bits_truncate(v as u8));
        self
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct RegBiozConfig6: u8 {
        const EXT_CAP     = 1 << 7;
        const DC_RESTORE  = 1 << 6;
        const DRV_RESET   = 1 << 5;
        const DAC_RESET   = 1 << 4;
        const AMP_RGE_MASK = 0b11 << 2;
        const AMP_BW_MASK  = 0b11;
    }
}

impl WritableRegister for RegBiozConfig6 {
    const ADDR: u8 = Register::BiozConfig6 as u8;
    fn value(&self) -> u8 {
        self.bits()
    }
    fn name() -> &'static str {
        "BIOZ_CONFIG6"
    }
}

impl ReadableRegister for RegBiozConfig6 {
    const ADDR: u8 = Register::BiozConfig6 as u8;
    fn from_raw(v: u8) -> Self {
        Self::from_bits_truncate(v)
    }
    fn name() -> &'static str {
        "BIOZ_CONFIG6"
    }
}

impl RegBiozConfig6 {
    #[inline]
    pub fn set_amp_rge(mut self, v: BiozAmpRange) -> Self {
        // clear bits [3:2]
        self.remove(Self::AMP_RGE_MASK);

        // insert new value
        self.insert(Self::from_bits_truncate((v as u8) << 2));

        self
    }

    #[inline]
    pub fn set_amp_bw(mut self, v: BiozAmpBw) -> Self {
        // clear bits [1:0]
        self.remove(Self::AMP_BW_MASK);

        // insert new value
        self.insert(Self::from_bits_truncate(v as u8));

        self
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct RegBiozConfig7: u8 {

        /// BIT4 — BIOZ_STBYON
        const STBYON        = 1 << 4;

        /// BIT3 — BIOZ_Q_CLK_PHASE
        const Q_CLK_PHASE   = 1 << 3;

        /// BIT2 — BIOZ_I_CLK_PHASE
        const I_CLK_PHASE   = 1 << 2;

        /// BIT1 — BIOZ_INA_CHOP_EN
        const INA_CHOP_EN   = 1 << 1;

        /// BIT0 — BIOZ_CH_FSEL
        const CH_FSEL       = 1 << 0;
    }
}

impl WritableRegister for RegBiozConfig7 {
    const ADDR: u8 = Register::BiozConfig7 as u8;

    fn value(&self) -> u8 {
        self.bits()
    }

    fn name() -> &'static str {
        "BIOZ_CONFIG7"
    }
}

impl ReadableRegister for RegBiozConfig7 {
    const ADDR: u8 = Register::BiozConfig7 as u8;

    fn from_raw(v: u8) -> Self {
        Self::from_bits_truncate(v)
    }

    fn name() -> &'static str {
        "BIOZ_CONFIG7"
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RegBiozLoThresh(pub u8);

impl WritableRegister for RegBiozLoThresh {
    const ADDR: u8 = Register::BiozLowThresh as u8;

    fn value(&self) -> u8 {
        self.0
    }

    fn name() -> &'static str {
        "BIOZ_LO_THRESH"
    }
}

impl ReadableRegister for RegBiozLoThresh {
    const ADDR: u8 = Register::BiozLowThresh as u8;

    fn from_raw(v: u8) -> Self {
        Self(v)
    }

    fn name() -> &'static str {
        "BIOZ_LO_THRESH"
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RegBiozHiThresh(pub u8);

impl WritableRegister for RegBiozHiThresh {
    const ADDR: u8 = Register::BiozHighThresh as u8;

    fn value(&self) -> u8 {
        self.0
    }

    fn name() -> &'static str {
        "BIOZ_HI_THRESH"
    }
}

impl ReadableRegister for RegBiozHiThresh {
    const ADDR: u8 = Register::BiozHighThresh as u8;

    fn from_raw(v: u8) -> Self {
        Self(v)
    }

    fn name() -> &'static str {
        "BIOZ_HI_THRESH"
    }
}
