use crate::register_map::Register;
use crate::traits::{ReadableRegister, WritableRegister};

#[derive(Copy, Clone, Debug)]
pub struct RegFifoWrPtr(pub u8);

impl ReadableRegister for RegFifoWrPtr {
    const ADDR: u8 = Register::FifoWritePtr as u8;

    fn from_raw(v: u8) -> Self {
        Self(v)
    }

    fn name() -> &'static str {
        "FIFO_WR_PTR"
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RegFifoRdPtr(pub u8);

impl ReadableRegister for RegFifoRdPtr {
    const ADDR: u8 = Register::FifoReadPtr as u8;

    fn from_raw(v: u8) -> Self {
        Self(v)
    }

    fn name() -> &'static str {
        "FIFO_RD_PTR"
    }
}

impl WritableRegister for RegFifoRdPtr {
    const ADDR: u8 = Register::FifoReadPtr as u8;

    fn value(&self) -> u8 {
        self.0
    }

    fn name() -> &'static str {
        "FIFO_RD_PTR"
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RegFifoCnt1(pub u8);

impl ReadableRegister for RegFifoCnt1 {
    const ADDR: u8 = Register::FifoCounter1 as u8;

    fn from_raw(v: u8) -> Self {
        Self(v)
    }

    fn name() -> &'static str {
        "FIFO_CNT1"
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RegFifoCnt2(pub u8);

impl ReadableRegister for RegFifoCnt2 {
    const ADDR: u8 = Register::FifoCounter2 as u8;

    fn from_raw(v: u8) -> Self {
        Self(v)
    }

    fn name() -> &'static str {
        "FIFO_CNT2"
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RegFifoCfg1(pub u8);

impl WritableRegister for RegFifoCfg1 {
    const ADDR: u8 = Register::FifoConfig1 as u8;

    fn value(&self) -> u8 {
        self.0
    }

    fn name() -> &'static str {
        "FIFO_CFG1"
    }
}

impl ReadableRegister for RegFifoCfg1 {
    const ADDR: u8 = Register::FifoConfig1 as u8;

    fn from_raw(v: u8) -> Self {
        Self(v)
    }

    fn name() -> &'static str {
        "FIFO_CFG1"
    }
}

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct RegFifoCfg2: u8 {
        const FIFO_MARK     = 1<<5;
        const FLUSH_FIFO    = 1<<4;
        const FIFO_STAT_CLR = 1<<3;
        const A_FULL_TYPE   = 1<<2;
        const FIFO_RO       = 1<<1;
    }
}

impl WritableRegister for RegFifoCfg2 {
    const ADDR: u8 = Register::FifoConfig2 as u8;

    fn value(&self) -> u8 {
        self.bits()
    }

    fn name() -> &'static str {
        "FIFO_CFG2"
    }
}

impl ReadableRegister for RegFifoCfg2 {
    const ADDR: u8 = Register::FifoConfig2 as u8;

    fn from_raw(v: u8) -> Self {
        Self::from_bits_truncate(v)
    }

    fn name() -> &'static str {
        "FIFO_CFG2"
    }
}
