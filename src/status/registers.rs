use crate::register_map::Register;
use crate::traits::ReadableRegister;
use bitflags::bitflags;

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct RegStatus1: u8 {
        const A_FULL        = 1<<7;
        const FIFO_DATA_RDY = 1<<5;
        const FREQ_UNLOCK   = 1<<4;
        const FREQ_LOCK     = 1<<3;
        const PHASE_UNLOCK  = 1<<2;
        const PHASE_LOCK    = 1<<1;
        const PWR_RDY       = 1<<0;
    }
}

impl ReadableRegister for RegStatus1 {
    const ADDR: u8 = Register::Status1 as u8;

    fn from_raw(v: u8) -> Self {
        Self::from_bits_truncate(v)
    }

    fn name() -> &'static str {
        "STATUS1"
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct RegStatus2: u8 {
        const LON         = 1<<7;
        const BIOZ_OVER   = 1<<6;
        const BIOZ_UNDR   = 1<<5;
        const DRV_OOR     = 1<<4;
        const DC_LOFF_PH  = 1<<3;
        const DC_LOFF_PL  = 1<<2;
        const DC_LOFF_NH  = 1<<1;
        const DC_LOFF_NL  = 1<<0;
    }
}

impl ReadableRegister for RegStatus2 {
    const ADDR: u8 = Register::Status2 as u8;

    fn from_raw(v: u8) -> Self {
        Self::from_bits_truncate(v)
    }

    fn name() -> &'static str {
        "STATUS2"
    }
}
