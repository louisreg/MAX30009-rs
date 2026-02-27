use crate::register_map::Register;
use crate::traits::{ReadableRegister, WritableRegister};
use bitflags::bitflags;

//
// ─────────────────────────────────────────
// BIOZ MUX CONFIG 1 (0x41)
// ─────────────────────────────────────────
//

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct RegBiozMuxConfig1: u8 {
        const RSEL_MASK        = 0b11 << 6;
        const BIST_EN          = 1 << 5;
        const CONNECT_CAL_ONLY = 1 << 2;
        const MUX_EN           = 1 << 1;
        const CAL_EN           = 1 << 0;
    }
}

impl WritableRegister for RegBiozMuxConfig1 {
    const ADDR: u8 = Register::BiozMuxConfig1 as u8;

    fn value(&self) -> u8 {
        self.bits()
    }

    fn name() -> &'static str {
        "BIOZ_MUX_CONFIG1"
    }
}

impl ReadableRegister for RegBiozMuxConfig1 {
    const ADDR: u8 = Register::BiozMuxConfig1 as u8;

    fn from_raw(v: u8) -> Self {
        Self::from_bits_truncate(v)
    }

    fn name() -> &'static str {
        "BIOZ_MUX_CONFIG1"
    }
}

//
// ─────────────────────────────────────────
// BIOZ MUX CONFIG 2 (0x42)
// ─────────────────────────────────────────
//

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct RegBiozMuxConfig2: u8 {
        const GSR_RSEL_MASK = 0b11 << 6;
        const GSR_LOAD_EN   = 1 << 5;
        const EN_EXT_INLOAD = 1 << 1;
        const EN_INT_INLOAD = 1 << 0;
    }
}

impl WritableRegister for RegBiozMuxConfig2 {
    const ADDR: u8 = Register::BiozMuxConfig2 as u8;

    fn value(&self) -> u8 {
        self.bits()
    }

    fn name() -> &'static str {
        "BIOZ_MUX_CONFIG2"
    }
}

impl ReadableRegister for RegBiozMuxConfig2 {
    const ADDR: u8 = Register::BiozMuxConfig2 as u8;

    fn from_raw(v: u8) -> Self {
        Self::from_bits_truncate(v)
    }

    fn name() -> &'static str {
        "BIOZ_MUX_CONFIG2"
    }
}

//
// ─────────────────────────────────────────
// BIOZ MUX CONFIG 3 (0x43)
// ─────────────────────────────────────────
//

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct RegBiozMuxConfig3: u8 {
        const BIP_MASK  = 0b11 << 6;
        const BIN_MASK  = 0b11 << 4;
        const DRVP_MASK = 0b11 << 2;
        const DRVN_MASK = 0b11;
    }
}

impl WritableRegister for RegBiozMuxConfig3 {
    const ADDR: u8 = Register::BiozMuxConfig3 as u8;

    fn value(&self) -> u8 {
        self.bits()
    }

    fn name() -> &'static str {
        "BIOZ_MUX_CONFIG3"
    }
}

impl ReadableRegister for RegBiozMuxConfig3 {
    const ADDR: u8 = Register::BiozMuxConfig3 as u8;

    fn from_raw(v: u8) -> Self {
        Self::from_bits_truncate(v)
    }

    fn name() -> &'static str {
        "BIOZ_MUX_CONFIG3"
    }
}

//
// ─────────────────────────────────────────
// BIOZ MUX CONFIG 4 (0x44) — READ ONLY
// ─────────────────────────────────────────
//

#[derive(Copy, Clone, Debug)]
pub struct RegBiozMuxConfig4(pub u8);

impl ReadableRegister for RegBiozMuxConfig4 {
    const ADDR: u8 = Register::BiozMuxConfig4 as u8;

    fn from_raw(v: u8) -> Self {
        Self(v)
    }

    fn name() -> &'static str {
        "BIOZ_MUX_CONFIG4"
    }
}
