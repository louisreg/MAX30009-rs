use crate::register_map::Register;
use crate::traits::{ReadableRegister, WritableRegister};
use bitflags::bitflags;

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct RegDcLeadsConfig: u8 {
        const EN_LON_DET  = 1<<7;
        const EN_LOFF_DET = 1<<6;
        const EN_EXT_LOFF = 1<<5;
        const EN_DRV_OOR  = 1<<4;
        const LOFF_IPOL   = 1<<3;
        const LOFF_IMAG_MASK = 0b111;
    }
}

impl WritableRegister for RegDcLeadsConfig {
    const ADDR: u8 = Register::DcLeadsConfig as u8;
    fn value(&self) -> u8 {
        self.bits()
    }
    fn name() -> &'static str {
        "DC_LEADS_CONFIG"
    }
}

impl ReadableRegister for RegDcLeadsConfig {
    const ADDR: u8 = Register::DcLeadsConfig as u8;
    fn from_raw(v: u8) -> Self {
        Self::from_bits_truncate(v)
    }
    fn name() -> &'static str {
        "DC_LEADS_CONFIG"
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct RegDcLeadThresh: u8 {
        const LOFF_THRESH_MASK = 0x0F;
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct RegLeadBiasConfig1: u8 {
        const RBIAS_VALUE_MASK = 0b11<<1;
        const EN_RBIAS_BIP = 1<<1;
        const EN_RBIAS_BIN = 1<<0;
    }
}

impl WritableRegister for RegDcLeadThresh {
    const ADDR: u8 = Register::DcLeadDetectThresh as u8;

    fn value(&self) -> u8 {
        self.bits()
    }

    fn name() -> &'static str {
        "DC_LEAD_THRESH"
    }
}

impl ReadableRegister for RegDcLeadThresh {
    const ADDR: u8 = Register::DcLeadDetectThresh as u8;

    fn from_raw(v: u8) -> Self {
        Self::from_bits_truncate(v)
    }

    fn name() -> &'static str {
        "DC_LEAD_THRESH"
    }
}

impl WritableRegister for RegLeadBiasConfig1 {
    const ADDR: u8 = Register::LeadBiasConfig1 as u8;

    fn value(&self) -> u8 {
        self.bits()
    }

    fn name() -> &'static str {
        "LEAD_BIAS_CONFIG1"
    }
}

impl ReadableRegister for RegLeadBiasConfig1 {
    const ADDR: u8 = Register::LeadBiasConfig1 as u8;

    fn from_raw(v: u8) -> Self {
        Self::from_bits_truncate(v)
    }

    fn name() -> &'static str {
        "LEAD_BIAS_CONFIG1"
    }
}
