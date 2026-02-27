//! High-level PLL public types (user API)

#[cfg(feature = "rpc-types")]
use postcard_schema::Schema;
#[cfg(feature = "rpc-types")]
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RefClockSel {
    Internal,
    External,
}

impl RefClockSel {
    #[inline]
    pub const fn bit(self) -> bool {
        matches!(self, Self::External)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ClockFreqSel {
    Khz32000,
    Khz32768,
}

impl ClockFreqSel {
    #[inline]
    pub const fn bit(self) -> bool {
        matches!(self, Self::Khz32768)
    }
}

#[cfg_attr(feature = "rpc-types", derive(Serialize, Deserialize, Schema))]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NDiv {
    Div512,
    Div1024,
}

impl NDiv {
    #[inline]
    pub const fn divisor(self) -> u32 {
        match self {
            Self::Div512 => 512,
            Self::Div1024 => 1024,
        }
    }
}

#[cfg_attr(feature = "rpc-types", derive(Serialize, Deserialize, Schema))]
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
