//! Leads high-level configuration builders
//! Compatible with generic Apply<I> configuration system.

use crate::leads::registers::*;
use crate::leads::types::*;

use crate::device::Max30009;
use crate::register_interface::RegisterInterface;
use crate::traits::{Apply, DebugDump, Merge, Readback};
use crate::{debug, warn};

//
// ─────────────────────────────────────────────
// DC LEADS CONFIG
// ─────────────────────────────────────────────
//

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Default, Copy, Clone, Debug)]
pub struct DcLeadsConfig {
    pub en_lon_det: Option<bool>,
    pub en_loff_det: Option<bool>,
    pub en_ext_loff: Option<bool>,
    pub en_drv_oor: Option<bool>,
    pub loff_ipol: Option<bool>,
    pub loff_imag: Option<LoffImag>,
}

impl DcLeadsConfig {
    pub fn encode(self) -> Option<RegDcLeadsConfig> {
        let mut reg = RegDcLeadsConfig::empty();
        let mut changed = false;

        if let Some(v) = self.en_lon_det {
            reg.set(RegDcLeadsConfig::EN_LON_DET, v);
            changed = true;
        }

        if let Some(v) = self.en_loff_det {
            reg.set(RegDcLeadsConfig::EN_LOFF_DET, v);
            changed = true;
        }

        if let Some(v) = self.en_ext_loff {
            reg.set(RegDcLeadsConfig::EN_EXT_LOFF, v);
            changed = true;
        }

        if let Some(v) = self.en_drv_oor {
            reg.set(RegDcLeadsConfig::EN_DRV_OOR, v);
            changed = true;
        }

        if let Some(v) = self.loff_ipol {
            reg.set(RegDcLeadsConfig::LOFF_IPOL, v);
            changed = true;
        }

        if let Some(v) = self.loff_imag {
            reg.remove(RegDcLeadsConfig::LOFF_IMAG_MASK);
            reg.insert(RegDcLeadsConfig::from_bits_truncate(v as u8));
            changed = true;
        }

        if changed {
            Some(reg)
        } else {
            None
        }
    }
}

impl<I: RegisterInterface> Apply<I> for DcLeadsConfig {
    fn apply(self, dev: &mut Max30009<I>) -> Result<(), I::Error> {
        self.debug_dump();
        if let Some(r) = self.encode() {
            dev.write(r)?;
        }
        Ok(())
    }
}

impl From<RegDcLeadsConfig> for DcLeadsConfig {
    fn from(r: RegDcLeadsConfig) -> Self {
        let b = r.bits();

        Self {
            en_lon_det: Some((b & (1 << 7)) != 0),
            en_loff_det: Some((b & (1 << 6)) != 0),
            en_ext_loff: Some((b & (1 << 5)) != 0),
            en_drv_oor: Some((b & (1 << 4)) != 0),
            loff_ipol: Some((b & (1 << 3)) != 0),
            loff_imag: Some(unsafe { core::mem::transmute(b & 0x07) }),
        }
    }
}

//
// ─────────────────────────────────────────────
// DC LEAD THRESHOLD
// ─────────────────────────────────────────────
//

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Default, Copy, Clone, Debug)]
pub struct DcLeadThresh {
    pub thresh: Option<LoffThresh>,
}

impl DcLeadThresh {
    pub fn encode(self) -> Option<RegDcLeadThresh> {
        self.thresh
            .map(|t| RegDcLeadThresh::from_bits_truncate((t as u8) & 0x0F))
    }
}

impl<I: RegisterInterface> Apply<I> for DcLeadThresh {
    fn apply(self, dev: &mut Max30009<I>) -> Result<(), I::Error> {
        self.debug_dump();
        if let Some(r) = self.encode() {
            dev.write(r)?;
        }
        Ok(())
    }
}

//
// ─────────────────────────────────────────────
// LEAD BIAS CONFIG 1
// ─────────────────────────────────────────────
//

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Default, Copy, Clone, Debug)]
pub struct LeadBiasConfig1 {
    pub rbias: Option<RbiasValue>,
    pub en_bip: Option<bool>,
    pub en_bin: Option<bool>,
}

impl LeadBiasConfig1 {
    pub fn encode(self) -> Option<RegLeadBiasConfig1> {
        let mut reg = RegLeadBiasConfig1::empty();
        let mut changed = false;

        if let Some(v) = self.rbias {
            reg.remove(RegLeadBiasConfig1::RBIAS_VALUE_MASK);
            reg.insert(RegLeadBiasConfig1::from_bits_truncate((v as u8) << 1));
            changed = true;
        }

        if let Some(v) = self.en_bip {
            reg.set(RegLeadBiasConfig1::EN_RBIAS_BIP, v);
            changed = true;
        }

        if let Some(v) = self.en_bin {
            reg.set(RegLeadBiasConfig1::EN_RBIAS_BIN, v);
            changed = true;
        }

        if changed {
            Some(reg)
        } else {
            None
        }
    }
}

impl<I: RegisterInterface> Apply<I> for LeadBiasConfig1 {
    fn apply(self, dev: &mut Max30009<I>) -> Result<(), I::Error> {
        self.debug_dump();
        if let Some(r) = self.encode() {
            dev.write(r)?;
        }
        Ok(())
    }
}

impl From<RegLeadBiasConfig1> for LeadBiasConfig1 {
    fn from(r: RegLeadBiasConfig1) -> Self {
        let b = r.bits();

        Self {
            rbias: Some(match (b >> 1) & 3 {
                0 => RbiasValue::R500M,
                1 => RbiasValue::R1G,
                _ => RbiasValue::R2G,
            }),
            en_bip: Some((b & (1 << 1)) != 0),
            en_bin: Some((b & 1) != 0),
        }
    }
}

impl From<RegDcLeadThresh> for DcLeadThresh {
    fn from(r: RegDcLeadThresh) -> Self {
        let b = r.bits() & 0x0F;
        let thresh = match b {
            0 => LoffThresh::Th215,
            1 => LoffThresh::Th245,
            2 => LoffThresh::Th275,
            3 => LoffThresh::Th305,
            4 => LoffThresh::Th335,
            5 => LoffThresh::Th365,
            6 => LoffThresh::Th395,
            7 => LoffThresh::Th425,
            8 => LoffThresh::Th455,
            9 => LoffThresh::Th485,
            10 => LoffThresh::Th515,
            11 => LoffThresh::Th545,
            12 => LoffThresh::Th575,
            13 => LoffThresh::Th605,
            14 => LoffThresh::Th635,
            _ => LoffThresh::Th665,
        };

        Self {
            thresh: Some(thresh),
        }
    }
}

impl<I: RegisterInterface> Readback<I> for DcLeadsConfig {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from(dev.read_reg::<RegDcLeadsConfig>()?);
        cfg.debug_dump();
        Ok(cfg)
    }
}

impl<I: RegisterInterface> Readback<I> for DcLeadThresh {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from(dev.read_reg::<RegDcLeadThresh>()?);
        cfg.debug_dump();
        Ok(cfg)
    }
}

impl<I: RegisterInterface> Readback<I> for LeadBiasConfig1 {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from(dev.read_reg::<RegLeadBiasConfig1>()?);
        cfg.debug_dump();
        Ok(cfg)
    }
}

impl DebugDump for DcLeadsConfig {
    fn debug_dump(&self) {
        debug!("╔══════════════════════════════════════╗");
        debug!("║            DC LEADS CONFIG           ║");
        debug!("╠══════════════════════════════════════╣");

        if let Some(r) = self.encode() {
            let _v = r.bits();
            debug!(" DC_LEADS_CFG  = 0x{:02X} | b{:08b}", _v, _v);
            debug!("   en_lon_det  : {:?}", self.en_lon_det);
            debug!("   en_loff_det : {:?}", self.en_loff_det);
            debug!("   en_ext_loff : {:?}", self.en_ext_loff);
            debug!("   en_drv_oor  : {:?}", self.en_drv_oor);
            debug!("   loff_ipol   : {:?}", self.loff_ipol);
            debug!("   loff_imag   : {:?}", self.loff_imag);
        } else {
            warn!(" DC_LEADS_CFG  : <unchanged>");
        }

        debug!("╚══════════════════════════════════════╝");
    }
}

impl DebugDump for DcLeadThresh {
    fn debug_dump(&self) {
        debug!("╔══════════════════════════════════════╗");
        debug!("║            DC LEAD THRESH            ║");
        debug!("╠══════════════════════════════════════╣");

        if let Some(r) = self.encode() {
            let _v = r.bits();
            debug!(" DC_LEAD_TH    = 0x{:02X} | b{:08b}", _v, _v);
            debug!("   thresh      : {:?}", self.thresh);
        } else {
            warn!(" DC_LEAD_TH    : <unchanged>");
        }

        debug!("╚══════════════════════════════════════╝");
    }
}

impl DebugDump for LeadBiasConfig1 {
    fn debug_dump(&self) {
        debug!("╔══════════════════════════════════════╗");
        debug!("║            LEAD BIAS CFG1            ║");
        debug!("╠══════════════════════════════════════╣");

        if let Some(r) = self.encode() {
            let _v = r.bits();
            debug!(" LEAD_BIAS1    = 0x{:02X} | b{:08b}", _v, _v);
            debug!("   rbias       : {:?}", self.rbias);
            debug!("   en_bip      : {:?}", self.en_bip);
            debug!("   en_bin      : {:?}", self.en_bin);
        } else {
            warn!(" LEAD_BIAS1    : <unchanged>");
        }

        debug!("╚══════════════════════════════════════╝");
    }
}

impl Merge for DcLeadsConfig {
    fn merge(self, current: Self) -> Self {
        Self {
            en_lon_det: self.en_lon_det.or(current.en_lon_det),
            en_loff_det: self.en_loff_det.or(current.en_loff_det),
            en_ext_loff: self.en_ext_loff.or(current.en_ext_loff),
            en_drv_oor: self.en_drv_oor.or(current.en_drv_oor),
            loff_ipol: self.loff_ipol.or(current.loff_ipol),
            loff_imag: self.loff_imag.or(current.loff_imag),
        }
    }
}

impl Merge for DcLeadThresh {
    fn merge(self, current: Self) -> Self {
        Self {
            thresh: self.thresh.or(current.thresh),
        }
    }
}

impl Merge for LeadBiasConfig1 {
    fn merge(self, current: Self) -> Self {
        Self {
            rbias: self.rbias.or(current.rbias),
            en_bip: self.en_bip.or(current.en_bip),
            en_bin: self.en_bin.or(current.en_bin),
        }
    }
}
