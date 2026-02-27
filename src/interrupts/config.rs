use crate::device::Max30009;
use crate::interrupts::registers::*;
use crate::register_interface::RegisterInterface;
use crate::traits::{Apply, DebugDump, Merge, Readback};
use crate::{debug, info, warn};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Default, Copy, Clone, Debug)]
pub struct InterruptEnable1 {
    pub a_full: Option<bool>,
    pub fifo_data_rdy: Option<bool>,
    pub freq_unlock: Option<bool>,
    pub freq_lock: Option<bool>,
    pub phase_unlock: Option<bool>,
    pub phase_lock: Option<bool>,
}

impl InterruptEnable1 {
    pub fn encode(self) -> Option<RegInterruptEnable1> {
        let mut reg = RegInterruptEnable1::empty();
        let mut changed = false;

        if let Some(v) = self.a_full {
            reg.set(RegInterruptEnable1::A_FULL_EN, v);
            changed = true;
        }
        if let Some(v) = self.fifo_data_rdy {
            reg.set(RegInterruptEnable1::FIFO_DATA_RDY_EN, v);
            changed = true;
        }
        if let Some(v) = self.freq_unlock {
            reg.set(RegInterruptEnable1::FREQ_UNLOCK_EN, v);
            changed = true;
        }
        if let Some(v) = self.freq_lock {
            reg.set(RegInterruptEnable1::FREQ_LOCK_EN, v);
            changed = true;
        }
        if let Some(v) = self.phase_unlock {
            reg.set(RegInterruptEnable1::PHASE_UNLOCK_EN, v);
            changed = true;
        }
        if let Some(v) = self.phase_lock {
            reg.set(RegInterruptEnable1::PHASE_LOCK_EN, v);
            changed = true;
        }

        if changed {
            Some(reg)
        } else {
            None
        }
    }
}

impl<I: RegisterInterface> Apply<I> for InterruptEnable1 {
    fn apply(self, dev: &mut Max30009<I>) -> Result<(), I::Error> {
        self.debug_dump();
        if let Some(r) = self.encode() {
            dev.write(r)?;
        }
        Ok(())
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Default, Copy, Clone, Debug)]
pub struct InterruptEnable2 {
    pub lon: Option<bool>,
    pub bioz_over: Option<bool>,
    pub bioz_undr: Option<bool>,
    pub drv_oor: Option<bool>,
    pub dc_loff_ph: Option<bool>,
    pub dc_loff_pl: Option<bool>,
    pub dc_loff_nh: Option<bool>,
    pub dc_loff_nl: Option<bool>,
}

impl InterruptEnable2 {
    pub fn encode(self) -> Option<RegInterruptEnable2> {
        let mut reg = RegInterruptEnable2::empty();
        let mut changed = false;

        if let Some(v) = self.lon {
            reg.set(RegInterruptEnable2::LON_EN, v);
            changed = true;
        }
        if let Some(v) = self.bioz_over {
            reg.set(RegInterruptEnable2::BIOZ_OVER_EN, v);
            changed = true;
        }
        if let Some(v) = self.bioz_undr {
            reg.set(RegInterruptEnable2::BIOZ_UNDR_EN, v);
            changed = true;
        }
        if let Some(v) = self.drv_oor {
            reg.set(RegInterruptEnable2::DRV_OOR_EN, v);
            changed = true;
        }
        if let Some(v) = self.dc_loff_ph {
            reg.set(RegInterruptEnable2::DC_LOFF_PH_EN, v);
            changed = true;
        }
        if let Some(v) = self.dc_loff_pl {
            reg.set(RegInterruptEnable2::DC_LOFF_PL_EN, v);
            changed = true;
        }
        if let Some(v) = self.dc_loff_nh {
            reg.set(RegInterruptEnable2::DC_LOFF_NH_EN, v);
            changed = true;
        }
        if let Some(v) = self.dc_loff_nl {
            reg.set(RegInterruptEnable2::DC_LOFF_NL_EN, v);
            changed = true;
        }

        if changed {
            Some(reg)
        } else {
            None
        }
    }
}

impl<I: RegisterInterface> Apply<I> for InterruptEnable2 {
    fn apply(self, dev: &mut Max30009<I>) -> Result<(), I::Error> {
        self.debug_dump();
        if let Some(r) = self.encode() {
            dev.write(r)?;
        }
        Ok(())
    }
}

impl<I: RegisterInterface> Readback<I> for InterruptEnable1 {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from(dev.read_reg::<RegInterruptEnable1>()?);
        cfg.debug_dump();
        Ok(cfg)
    }
}

impl<I: RegisterInterface> Readback<I> for InterruptEnable2 {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from(dev.read_reg::<RegInterruptEnable2>()?);
        cfg.debug_dump();
        Ok(cfg)
    }
}

impl DebugDump for InterruptEnable1 {
    fn debug_dump(&self) {
        debug!("╔══════════════════════════════════════╗");
        debug!("║           INTERRUPT ENABLE1          ║");
        debug!("╠══════════════════════════════════════╣");

        if let Some(r) = self.encode() {
            let _v = r.bits();
            debug!(" INTERRUPT_EN1 = 0x{:02X} | b{:08b}", _v, _v);
            debug!("   a_full        : {:?}", self.a_full);
            debug!("   fifo_data_rdy : {:?}", self.fifo_data_rdy);
            debug!("   freq_unlock   : {:?}", self.freq_unlock);
            debug!("   freq_lock     : {:?}", self.freq_lock);
            debug!("   phase_unlock  : {:?}", self.phase_unlock);
            debug!("   phase_lock    : {:?}", self.phase_lock);
        } else {
            warn!(" INTERRUPT_EN1 : <unchanged>");
        }

        debug!("╚══════════════════════════════════════╝");
    }
}

impl DebugDump for InterruptEnable2 {
    fn debug_dump(&self) {
        debug!("╔══════════════════════════════════════╗");
        debug!("║           INTERRUPT ENABLE2          ║");
        debug!("╠══════════════════════════════════════╣");

        if let Some(r) = self.encode() {
            let _v = r.bits();
            debug!(" INTERRUPT_EN2 = 0x{:02X} | b{:08b}", _v, _v);
            debug!("   lon        : {:?}", self.lon);
            debug!("   bioz_over  : {:?}", self.bioz_over);
            debug!("   bioz_undr  : {:?}", self.bioz_undr);
            debug!("   drv_oor    : {:?}", self.drv_oor);
            debug!("   dc_loff_ph : {:?}", self.dc_loff_ph);
            debug!("   dc_loff_pl : {:?}", self.dc_loff_pl);
            debug!("   dc_loff_nh : {:?}", self.dc_loff_nh);
            debug!("   dc_loff_nl : {:?}", self.dc_loff_nl);
        } else {
            warn!(" INTERRUPT_EN2 : <unchanged>");
        }

        info!("╚══════════════════════════════════════╝");
    }
}

impl Merge for InterruptEnable1 {
    fn merge(self, current: Self) -> Self {
        Self {
            a_full: self.a_full.or(current.a_full),
            fifo_data_rdy: self.fifo_data_rdy.or(current.fifo_data_rdy),
            freq_unlock: self.freq_unlock.or(current.freq_unlock),
            freq_lock: self.freq_lock.or(current.freq_lock),
            phase_unlock: self.phase_unlock.or(current.phase_unlock),
            phase_lock: self.phase_lock.or(current.phase_lock),
        }
    }
}

impl Merge for InterruptEnable2 {
    fn merge(self, current: Self) -> Self {
        Self {
            lon: self.lon.or(current.lon),
            bioz_over: self.bioz_over.or(current.bioz_over),
            bioz_undr: self.bioz_undr.or(current.bioz_undr),
            drv_oor: self.drv_oor.or(current.drv_oor),
            dc_loff_ph: self.dc_loff_ph.or(current.dc_loff_ph),
            dc_loff_pl: self.dc_loff_pl.or(current.dc_loff_pl),
            dc_loff_nh: self.dc_loff_nh.or(current.dc_loff_nh),
            dc_loff_nl: self.dc_loff_nl.or(current.dc_loff_nl),
        }
    }
}

impl From<RegInterruptEnable1> for InterruptEnable1 {
    fn from(r: RegInterruptEnable1) -> Self {
        Self {
            a_full: Some(r.contains(RegInterruptEnable1::A_FULL_EN)),
            fifo_data_rdy: Some(r.contains(RegInterruptEnable1::FIFO_DATA_RDY_EN)),
            freq_unlock: Some(r.contains(RegInterruptEnable1::FREQ_UNLOCK_EN)),
            freq_lock: Some(r.contains(RegInterruptEnable1::FREQ_LOCK_EN)),
            phase_unlock: Some(r.contains(RegInterruptEnable1::PHASE_UNLOCK_EN)),
            phase_lock: Some(r.contains(RegInterruptEnable1::PHASE_LOCK_EN)),
        }
    }
}

impl From<RegInterruptEnable2> for InterruptEnable2 {
    fn from(r: RegInterruptEnable2) -> Self {
        Self {
            lon: Some(r.contains(RegInterruptEnable2::LON_EN)),
            bioz_over: Some(r.contains(RegInterruptEnable2::BIOZ_OVER_EN)),
            bioz_undr: Some(r.contains(RegInterruptEnable2::BIOZ_UNDR_EN)),
            drv_oor: Some(r.contains(RegInterruptEnable2::DRV_OOR_EN)),
            dc_loff_ph: Some(r.contains(RegInterruptEnable2::DC_LOFF_PH_EN)),
            dc_loff_pl: Some(r.contains(RegInterruptEnable2::DC_LOFF_PL_EN)),
            dc_loff_nh: Some(r.contains(RegInterruptEnable2::DC_LOFF_NH_EN)),
            dc_loff_nl: Some(r.contains(RegInterruptEnable2::DC_LOFF_NL_EN)),
        }
    }
}
