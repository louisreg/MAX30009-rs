//! BioZ high-level public types

#[cfg(feature = "rpc-types")]
use postcard_schema::Schema;
#[cfg(feature = "rpc-types")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "rpc-types", derive(Serialize, Deserialize, Schema))]
#[derive(Copy, Clone, Debug)]
pub enum BiozDacOsr {
    Osr32 = 0,
    Osr64 = 1,
    Osr128 = 2,
    Osr256 = 3,
}
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "rpc-types", derive(Serialize, Deserialize, Schema))]
#[derive(Copy, Clone, Debug)]
pub enum BiozAdcOsr {
    Osr8 = 0,
    Osr16 = 1,
    Osr32 = 2,
    Osr64 = 3,
    Osr128 = 4,
    Osr256 = 5,
    Osr512 = 6,
    Osr1024 = 7,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub enum BiozDhpf {
    Bypass = 0,
    Hp00025 = 1,
    Hp002 = 2,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub enum BiozDlpf {
    Bypass = 0,
    Lp0005 = 1,
    Lp002 = 2,
    Lp008 = 3,
    Lp025 = 4,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub enum BiozCmp {
    I = 0,
    Q = 1,
    Z = 2,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub enum BiozDrvMode {
    Current = 0,
    Voltage = 1,
    HBridge = 2,
    Standby = 3,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub enum BiozIdrvRge {
    R552k5 = 0,
    R110k5 = 1,
    R5k525 = 2,
    R276R25 = 3,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub enum BiozVdrvMag {
    M50 = 0,
    M100 = 1,
    M250 = 2,
    M500 = 3,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub enum BiozAhpf {
    Hz100 = 0x0,
    Hz200 = 0x1,
    Hz500 = 0x2,
    Hz1000 = 0x3,
    Hz2000 = 0x4,
    Hz5000 = 0x5,
    Hz10000 = 0x6,
    Bypass = 0x7,
    R42M4 = 0x8,
    R21M2 = 0x9,
    R8M4 = 0xA,
    R4M2 = 0xB,
    R2M2 = 0xC,
    R848k = 0xD,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub enum BiozGain {
    Gain1VV = 0,
    Gain2VV = 1,
    Gain5VV = 2,
    Gain10VV = 3,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub enum BiozAmpRange {
    Low = 0,
    MedLow = 1,
    MedHigh = 2,
    High = 3,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub enum BiozAmpBw {
    Low = 0,
    MedLow = 1,
    MedHigh = 2,
    High = 3,
}
