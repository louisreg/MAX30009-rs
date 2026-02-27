use crate::register_map::Register;
use crate::traits::{ReadableRegister, WritableRegister};
use bitflags::bitflags;

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct RegInterruptEnable1: u8 {
        const A_FULL_EN        = 1<<7;
        const FIFO_DATA_RDY_EN = 1<<5;
        const FREQ_UNLOCK_EN   = 1<<4;
        const FREQ_LOCK_EN     = 1<<3;
        const PHASE_UNLOCK_EN  = 1<<2;
        const PHASE_LOCK_EN    = 1<<1;
    }
}

impl WritableRegister for RegInterruptEnable1 {
    const ADDR: u8 = Register::InterruptEnable1 as u8;

    fn value(&self) -> u8 {
        self.bits()
    }

    fn name() -> &'static str {
        "INTERRUPT_ENABLE1"
    }
}

impl ReadableRegister for RegInterruptEnable1 {
    const ADDR: u8 = Register::InterruptEnable1 as u8;

    fn from_raw(v: u8) -> Self {
        Self::from_bits_truncate(v)
    }

    fn name() -> &'static str {
        "INTERRUPT_ENABLE1"
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct RegInterruptEnable2: u8 {
        const LON_EN        = 1<<7;
        const BIOZ_OVER_EN  = 1<<6;
        const BIOZ_UNDR_EN  = 1<<5;
        const DRV_OOR_EN    = 1<<4;
        const DC_LOFF_PH_EN = 1<<3;
        const DC_LOFF_PL_EN = 1<<2;
        const DC_LOFF_NH_EN = 1<<1;
        const DC_LOFF_NL_EN = 1<<0;
    }
}

impl WritableRegister for RegInterruptEnable2 {
    const ADDR: u8 = Register::InterruptEnable2 as u8;

    fn value(&self) -> u8 {
        self.bits()
    }

    fn name() -> &'static str {
        "INTERRUPT_ENABLE2"
    }
}

impl ReadableRegister for RegInterruptEnable2 {
    const ADDR: u8 = Register::InterruptEnable2 as u8;

    fn from_raw(v: u8) -> Self {
        Self::from_bits_truncate(v)
    }

    fn name() -> &'static str {
        "INTERRUPT_ENABLE2"
    }
}
