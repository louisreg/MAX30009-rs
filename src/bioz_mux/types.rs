#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub enum BmuxRsel {
    R5100 = 0,
    R900 = 1,
    R600 = 2,
    R280 = 3,
}

#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub enum BmuxGsrRsel {
    R25k7 = 0,
    R101 = 1,
    R505 = 2,
    R1000 = 3,
}

#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub enum BipDrvpSel {
    El1 = 0,
    El2A = 1,
    El2B = 2,
}

#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub enum BinDrvnSel {
    El4 = 0,
    El3A = 1,
    El3B = 2,
}
