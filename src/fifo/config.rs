use crate::device::Max30009;
use crate::fifo::registers::*;
use crate::register_interface::RegisterInterface;
use crate::traits::{Apply, DebugDump, Merge, Readback};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Default, Copy, Clone, Debug)]
pub struct FifoConfig {
    pub a_full_threshold: Option<u8>,
}

impl FifoConfig {
    pub fn encode(self) -> Option<RegFifoCfg1> {
        self.a_full_threshold.map(RegFifoCfg1)
    }
}

impl<I: RegisterInterface> Apply<I> for FifoConfig {
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
pub struct FifoCtrl {
    pub stat_clr: Option<bool>,
    pub a_full_type: Option<bool>,
    pub fifo_ro: Option<bool>,
    pub flush: Option<bool>,
    pub mark: Option<bool>,
}

impl FifoCtrl {
    pub fn encode(self) -> Option<RegFifoCfg2> {
        let mut reg = RegFifoCfg2::empty();
        let mut changed = false;

        if let Some(v) = self.stat_clr {
            reg.set(RegFifoCfg2::FIFO_STAT_CLR, v);
            changed = true;
        }

        if let Some(v) = self.a_full_type {
            reg.set(RegFifoCfg2::A_FULL_TYPE, v);
            changed = true;
        }

        if let Some(v) = self.fifo_ro {
            reg.set(RegFifoCfg2::FIFO_RO, v);
            changed = true;
        }

        if let Some(v) = self.flush {
            reg.set(RegFifoCfg2::FLUSH_FIFO, v);
            changed = true;
        }

        if let Some(v) = self.mark {
            reg.set(RegFifoCfg2::FIFO_MARK, v);
            changed = true;
        }

        if changed {
            Some(reg)
        } else {
            None
        }
    }
}

impl<I: RegisterInterface> Apply<I> for FifoCtrl {
    fn apply(self, dev: &mut Max30009<I>) -> Result<(), I::Error> {
        self.debug_dump();
        if let Some(r) = self.encode() {
            dev.write(r)?;
        }
        Ok(())
    }
}

impl From<RegFifoCfg1> for FifoConfig {
    fn from(r: RegFifoCfg1) -> Self {
        Self {
            a_full_threshold: Some(r.0),
        }
    }
}

impl From<RegFifoCfg2> for FifoCtrl {
    fn from(r: RegFifoCfg2) -> Self {
        Self {
            stat_clr: Some(r.contains(RegFifoCfg2::FIFO_STAT_CLR)),
            a_full_type: Some(r.contains(RegFifoCfg2::A_FULL_TYPE)),
            fifo_ro: Some(r.contains(RegFifoCfg2::FIFO_RO)),
            flush: Some(r.contains(RegFifoCfg2::FLUSH_FIFO)),
            mark: Some(r.contains(RegFifoCfg2::FIFO_MARK)),
        }
    }
}

impl<I: RegisterInterface> Readback<I> for FifoConfig {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from(dev.read_reg::<RegFifoCfg1>()?);
        cfg.debug_dump();
        Ok(cfg)
    }
}

impl<I: RegisterInterface> Readback<I> for FifoCtrl {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from(dev.read_reg::<RegFifoCfg2>()?);
        cfg.debug_dump();
        Ok(cfg)
    }
}

impl<I: RegisterInterface> Readback<I> for FifoStatus {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from((
            dev.read_reg::<RegFifoCnt1>()?,
            dev.read_reg::<RegFifoCnt2>()?,
            dev.read_reg::<RegFifoWrPtr>()?,
            dev.read_reg::<RegFifoRdPtr>()?,
        ));
        cfg.debug_dump();
        Ok(cfg)
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub struct FifoStatus {
    pub wr_ptr: u8,
    pub rd_ptr: u8,
    pub data_count: u16,
    pub overflow: u8,
}

impl From<(RegFifoCnt1, RegFifoCnt2, RegFifoWrPtr, RegFifoRdPtr)> for FifoStatus {
    fn from(v: (RegFifoCnt1, RegFifoCnt2, RegFifoWrPtr, RegFifoRdPtr)) -> Self {
        let cnt1 = v.0 .0;
        let cnt2 = v.1 .0;

        let data_count = (((cnt1 >> 7) as u16) << 8) | (cnt2 as u16);

        let ovf = cnt1 & 0x7F;

        Self {
            wr_ptr: v.2 .0,
            rd_ptr: v.3 .0,
            data_count,
            overflow: ovf,
        }
    }
}

use crate::{debug, info, warn};

impl FifoStatus {
    pub fn debug_dump(&self) {
        debug!("╔══════════════════════════════╗");
        debug!("║         FIFO STATUS          ║");
        debug!("╠══════════════════════════════╣");

        debug!(" WR_PTR      = 0x{:02X}", self.wr_ptr);
        debug!(" RD_PTR      = 0x{:02X}", self.rd_ptr);

        debug!(" DATA_COUNT  = {}", self.data_count);
        debug!(" OVERFLOW    = {}", self.overflow);

        debug!("╚══════════════════════════════╝");
    }
}

impl DebugDump for FifoConfig {
    fn debug_dump(&self) {
        debug!("╔══════════════════════════════╗");
        debug!("║          FIFO CFG1           ║");
        debug!("╠══════════════════════════════╣");

        if let Some(_v) = self.a_full_threshold {
            debug!(" A_FULL_TH     = 0x{:02X} | b{:08b}", _v, _v);
        } else {
            warn!(" A_FULL_TH     : <unchanged>");
        }

        debug!("╚══════════════════════════════╝");
    }
}

impl DebugDump for FifoCtrl {
    fn debug_dump(&self) {
        debug!("╔══════════════════════════════╗");
        debug!("║          FIFO CFG2           ║");
        debug!("╠══════════════════════════════╣");

        if let Some(r) = self.encode() {
            let _v = r.bits();
            debug!(" FIFO_CFG2     = 0x{:02X} | b{:08b}", _v, _v);
            debug!("   stat_clr    : {:?}", self.stat_clr);
            debug!("   a_full_type : {:?}", self.a_full_type);
            debug!("   fifo_ro     : {:?}", self.fifo_ro);
            debug!("   flush       : {:?}", self.flush);
            debug!("   mark        : {:?}", self.mark);
        } else {
            warn!(" FIFO_CFG2     : <unchanged>");
        }

        info!("╚══════════════════════════════╝");
    }
}

impl DebugDump for FifoStatus {
    fn debug_dump(&self) {
        FifoStatus::debug_dump(self);
    }
}

impl Merge for FifoConfig {
    fn merge(self, current: Self) -> Self {
        Self {
            a_full_threshold: self.a_full_threshold.or(current.a_full_threshold),
        }
    }
}

impl Merge for FifoCtrl {
    fn merge(self, current: Self) -> Self {
        Self {
            stat_clr: self.stat_clr.or(current.stat_clr),
            a_full_type: self.a_full_type.or(current.a_full_type),
            fifo_ro: self.fifo_ro.or(current.fifo_ro),
            flush: self.flush.or(current.flush),
            mark: self.mark.or(current.mark),
        }
    }
}
