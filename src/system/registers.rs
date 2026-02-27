//! System register definitions for MAX30009

use crate::register_map::Register;
use crate::system::types::*;
use crate::traits::{ReadableRegister, WritableRegister};
use bitflags::bitflags;

// ─────────────────────────────────────────
// System Sync (0x10)
// ─────────────────────────────────────────

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct SystemSync: u8 {
        const TIMING_SYS_RESET = 1 << 7;
    }
}

impl WritableRegister for SystemSync {
    const ADDR: u8 = 0x10;
    fn value(&self) -> u8 {
        self.bits()
    }
    fn name() -> &'static str {
        "SYSTEM_SYNC"
    }
}

// ─────────────────────────────────────────
// System Config (0x11)
// ─────────────────────────────────────────

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct SystemConfig: u8 {
        const MASTER      = 1 << 7;
        const DISABLE_I2C = 1 << 6;
        const SHDN        = 1 << 1;
        const RESET       = 1 << 0;
    }
}

impl WritableRegister for SystemConfig {
    const ADDR: u8 = Register::SystemConfig as u8;

    fn value(&self) -> u8 {
        self.bits()
    }
    fn name() -> &'static str {
        "SYSTEM_CONFIG"
    }
}

impl ReadableRegister for SystemConfig {
    const ADDR: u8 = Register::SystemConfig as u8;

    fn from_raw(v: u8) -> Self {
        SystemConfig::from_bits_truncate(v)
    }

    fn name() -> &'static str {
        "SYSTEM_CONFIG"
    }
}

// ─────────────────────────────────────────
// PinFuncConfig (0x12)
// ─────────────────────────────────────────

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PinFuncConfig {
    pub int_fcfg: IntFunc,
    pub trig_edge: TrigEdge,
}

impl PinFuncConfig {
    pub fn encode(self) -> u8 {
        let mut v = (self.int_fcfg as u8) & 0x03;

        if matches!(self.trig_edge, TrigEdge::Rising) {
            v |= 1 << 2;
        }

        v
    }
}

impl WritableRegister for PinFuncConfig {
    const ADDR: u8 = Register::PinFuncConfig as u8;

    fn value(&self) -> u8 {
        self.encode()
    }
    fn name() -> &'static str {
        "PIN_FUNC_CONFIG"
    }
}

impl ReadableRegister for PinFuncConfig {
    const ADDR: u8 = Register::PinFuncConfig as u8;

    fn from_raw(v: u8) -> Self {
        Self {
            int_fcfg: match v & 0x03 {
                1 => IntFunc::ClearOnRead,
                2 => IntFunc::AutoClearShort,
                3 => IntFunc::AutoClearLong,
                _ => IntFunc::Disabled,
            },
            trig_edge: if (v & (1 << 2)) != 0 {
                TrigEdge::Rising
            } else {
                TrigEdge::Falling
            },
        }
    }

    fn name() -> &'static str {
        "PIN_FUNC_CONFIG"
    }
}

// ─────────────────────────────────────────
// OutputPinConfig (0x13)
// ─────────────────────────────────────────

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OutputPinConfig {
    pub int_drive: OutputDrive,
    pub trig_drive: OutputDrive,
}

impl OutputPinConfig {
    pub fn encode(self) -> u8 {
        ((self.int_drive as u8) << 0) | ((self.trig_drive as u8) << 2)
    }
}

impl WritableRegister for OutputPinConfig {
    const ADDR: u8 = Register::OutputPinConfig as u8;

    fn value(&self) -> u8 {
        self.encode()
    }
    fn name() -> &'static str {
        "OUTPUT_PIN_CONFIG"
    }
}

impl ReadableRegister for OutputPinConfig {
    const ADDR: u8 = Register::OutputPinConfig as u8;

    fn from_raw(v: u8) -> Self {
        let decode = |x| match x {
            1 => OutputDrive::PushPullHigh,
            2 => OutputDrive::PushPullLow,
            _ => OutputDrive::OpenDrainLow,
        };

        Self {
            int_drive: decode((v >> 0) & 0x03),
            trig_drive: decode((v >> 2) & 0x03),
        }
    }

    fn name() -> &'static str {
        "OUTPUT_PIN_CONFIG"
    }
}

// ─────────────────────────────────────────
// I2C Broadcast (0x14)
// ─────────────────────────────────────────

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct I2cBroadcast {
    pub address: u8,
    pub enable: bool,
}

impl I2cBroadcast {
    pub fn encode(self) -> u8 {
        let mut v = (self.address & 0x7F) << 1;
        if self.enable {
            v |= 1;
        }
        v
    }
}

impl WritableRegister for I2cBroadcast {
    const ADDR: u8 = Register::I2cBroadcast as u8;

    fn value(&self) -> u8 {
        self.encode()
    }
    fn name() -> &'static str {
        "I2C_BROADCAST"
    }
}

impl ReadableRegister for I2cBroadcast {
    const ADDR: u8 = Register::I2cBroadcast as u8;

    fn from_raw(v: u8) -> Self {
        Self {
            address: (v >> 1) & 0x7F,
            enable: (v & 1) != 0,
        }
    }

    fn name() -> &'static str {
        "I2C_BROADCAST"
    }
}
