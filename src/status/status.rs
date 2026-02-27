use crate::debug;
use crate::device::Max30009;
use crate::register_interface::RegisterInterface;
use crate::status::registers::*;
use crate::status::types::*;
use crate::traits::{DebugDump, Readback};

impl From<RegStatus1> for Status1 {
    fn from(r: RegStatus1) -> Self {
        Self {
            a_full: r.contains(RegStatus1::A_FULL),
            fifo_data_rdy: r.contains(RegStatus1::FIFO_DATA_RDY),
            freq_unlock: r.contains(RegStatus1::FREQ_UNLOCK),
            freq_lock: r.contains(RegStatus1::FREQ_LOCK),
            phase_unlock: r.contains(RegStatus1::PHASE_UNLOCK),
            phase_lock: r.contains(RegStatus1::PHASE_LOCK),
            pwr_rdy: r.contains(RegStatus1::PWR_RDY),
        }
    }
}

impl From<RegStatus2> for Status2 {
    fn from(r: RegStatus2) -> Self {
        Self {
            lon: r.contains(RegStatus2::LON),
            bioz_over: r.contains(RegStatus2::BIOZ_OVER),
            bioz_undr: r.contains(RegStatus2::BIOZ_UNDR),
            drv_oor: r.contains(RegStatus2::DRV_OOR),
            dc_loff_ph: r.contains(RegStatus2::DC_LOFF_PH),
            dc_loff_pl: r.contains(RegStatus2::DC_LOFF_PL),
            dc_loff_nh: r.contains(RegStatus2::DC_LOFF_NH),
            dc_loff_nl: r.contains(RegStatus2::DC_LOFF_NL),
        }
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub struct Status {
    pub s1: Status1,
    pub s2: Status2,
}

impl Status {
    pub fn debug_dump(&self) {
        debug!("╔══════════════════════════════╗");
        debug!("║       MAX30009 STATUS        ║");
        debug!("╠══════════════════════════════╣");

        debug!(" STATUS1:");
        debug!("   A_FULL        : {}", self.s1.a_full);
        debug!("   FIFO_DATA_RDY : {}", self.s1.fifo_data_rdy);
        debug!("   FREQ_UNLOCK   : {}", self.s1.freq_unlock);
        debug!("   FREQ_LOCK     : {}", self.s1.freq_lock);
        debug!("   PHASE_UNLOCK  : {}", self.s1.phase_unlock);
        debug!("   PHASE_LOCK    : {}", self.s1.phase_lock);
        debug!("   PWR_RDY       : {}", self.s1.pwr_rdy);

        debug!(" STATUS2:");
        debug!("   LON           : {}", self.s2.lon);
        debug!("   BIOZ_OVER     : {}", self.s2.bioz_over);
        debug!("   BIOZ_UNDR     : {}", self.s2.bioz_undr);
        debug!("   DRV_OOR       : {}", self.s2.drv_oor);
        debug!("   DC_LOFF_PH    : {}", self.s2.dc_loff_ph);
        debug!("   DC_LOFF_PL    : {}", self.s2.dc_loff_pl);
        debug!("   DC_LOFF_NH    : {}", self.s2.dc_loff_nh);
        debug!("   DC_LOFF_NL    : {}", self.s2.dc_loff_nl);

        debug!("╚══════════════════════════════╝");
    }
}

impl<I: RegisterInterface> Readback<I> for Status {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let status = Self {
            s1: Status1::from(dev.read_reg::<RegStatus1>()?),
            s2: Status2::from(dev.read_reg::<RegStatus2>()?),
        };
        status.debug_dump();
        Ok(status)
    }
}

impl DebugDump for Status {
    fn debug_dump(&self) {
        Status::debug_dump(self);
    }
}
