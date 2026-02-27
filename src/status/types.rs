#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub struct Status1 {
    pub a_full: bool,
    pub fifo_data_rdy: bool,
    pub freq_unlock: bool,
    pub freq_lock: bool,
    pub phase_unlock: bool,
    pub phase_lock: bool,
    pub pwr_rdy: bool,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub struct Status2 {
    pub lon: bool,
    pub bioz_over: bool,
    pub bioz_undr: bool,
    pub drv_oor: bool,
    pub dc_loff_ph: bool,
    pub dc_loff_pl: bool,
    pub dc_loff_nh: bool,
    pub dc_loff_nl: bool,
}
