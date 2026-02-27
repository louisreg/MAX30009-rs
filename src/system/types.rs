//! System high-level types (user-facing API)

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum IntFunc {
    Disabled = 0,
    ClearOnRead = 1,
    AutoClearShort = 2,
    AutoClearLong = 3,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TrigEdge {
    Falling,
    Rising,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum OutputDrive {
    OpenDrainLow = 0,
    PushPullHigh = 1,
    PushPullLow = 2,
}

/// High-level system settings
#[derive(Copy, Clone, Debug)]
pub struct SystemSettings {
    pub master: bool,
    pub spi_only: bool,
}
