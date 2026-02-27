use crate::bioz_mux::registers::*;
use crate::bioz_mux::types::*;
use crate::device::Max30009;
use crate::register_interface::RegisterInterface;
use crate::traits::{Apply, DebugDump, Merge, Readback};
use crate::{debug, warn};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Default, Copy, Clone, Debug)]
pub struct BiozMuxConfig1 {
    pub rsel: Option<BmuxRsel>,
    pub bist_en: Option<bool>,
    pub connect_cal_only: Option<bool>,
    pub mux_en: Option<bool>,
    pub cal_en: Option<bool>,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Default, Copy, Clone, Debug)]
pub struct BiozMuxConfig2 {
    pub gsr_rsel: Option<BmuxGsrRsel>,
    pub gsr_load_en: Option<bool>,
    pub en_ext_inload: Option<bool>,
    pub en_int_inload: Option<bool>,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Default, Copy, Clone, Debug)]
pub struct BiozMuxConfig3 {
    pub bip: Option<BipDrvpSel>,
    pub bin: Option<BinDrvnSel>,
    pub drvp: Option<BipDrvpSel>,
    pub drvn: Option<BinDrvnSel>,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub struct BiozMuxBist {
    pub bist_r_err: i8,
}

impl From<RegBiozMuxConfig1> for BiozMuxConfig1 {
    fn from(r: RegBiozMuxConfig1) -> Self {
        let b = r.bits();

        Self {
            rsel: Some(match (b >> 6) & 3 {
                0 => BmuxRsel::R5100,
                1 => BmuxRsel::R900,
                2 => BmuxRsel::R600,
                _ => BmuxRsel::R280,
            }),
            bist_en: Some((b & (1 << 5)) != 0),
            connect_cal_only: Some((b & (1 << 2)) != 0),
            mux_en: Some((b & (1 << 1)) != 0),
            cal_en: Some((b & 1) != 0),
        }
    }
}

impl BiozMuxConfig1 {
    pub fn encode(self) -> Option<RegBiozMuxConfig1> {
        let mut reg = RegBiozMuxConfig1::empty();
        let mut changed = false;

        if let Some(v) = self.rsel {
            reg.remove(RegBiozMuxConfig1::RSEL_MASK);
            reg.insert(RegBiozMuxConfig1::from_bits_truncate((v as u8) << 6));
            changed = true;
        }

        if let Some(v) = self.bist_en {
            reg.set(RegBiozMuxConfig1::BIST_EN, v);
            changed = true;
        }

        if let Some(v) = self.connect_cal_only {
            reg.set(RegBiozMuxConfig1::CONNECT_CAL_ONLY, v);
            changed = true;
        }

        if let Some(v) = self.mux_en {
            reg.set(RegBiozMuxConfig1::MUX_EN, v);
            changed = true;
        }

        if let Some(v) = self.cal_en {
            reg.set(RegBiozMuxConfig1::CAL_EN, v);
            changed = true;
        }

        if changed {
            Some(reg)
        } else {
            None
        }
    }
}

impl From<RegBiozMuxConfig2> for BiozMuxConfig2 {
    fn from(r: RegBiozMuxConfig2) -> Self {
        let b = r.bits();

        Self {
            gsr_rsel: Some(match (b >> 6) & 3 {
                0 => BmuxGsrRsel::R25k7,
                1 => BmuxGsrRsel::R101,
                2 => BmuxGsrRsel::R505,
                _ => BmuxGsrRsel::R1000,
            }),
            gsr_load_en: Some((b & (1 << 5)) != 0),
            en_ext_inload: Some((b & (1 << 1)) != 0),
            en_int_inload: Some((b & 1) != 0),
        }
    }
}

impl BiozMuxConfig2 {
    pub fn encode(self) -> Option<RegBiozMuxConfig2> {
        let mut reg = RegBiozMuxConfig2::empty();
        let mut changed = false;

        if let Some(v) = self.gsr_rsel {
            reg.remove(RegBiozMuxConfig2::GSR_RSEL_MASK);
            reg.insert(RegBiozMuxConfig2::from_bits_truncate((v as u8) << 6));
            changed = true;
        }

        if let Some(v) = self.gsr_load_en {
            reg.set(RegBiozMuxConfig2::GSR_LOAD_EN, v);
            changed = true;
        }

        if let Some(v) = self.en_ext_inload {
            reg.set(RegBiozMuxConfig2::EN_EXT_INLOAD, v);
            changed = true;
        }

        if let Some(v) = self.en_int_inload {
            reg.set(RegBiozMuxConfig2::EN_INT_INLOAD, v);
            changed = true;
        }

        if changed {
            Some(reg)
        } else {
            None
        }
    }
}

impl From<RegBiozMuxConfig3> for BiozMuxConfig3 {
    fn from(r: RegBiozMuxConfig3) -> Self {
        let b = r.bits();

        let bip = match (b >> 6) & 0x03 {
            0 => Some(BipDrvpSel::El1),
            1 => Some(BipDrvpSel::El2A),
            2 => Some(BipDrvpSel::El2B),
            _ => None, // 0x3 = Do not use (datasheet)
        };

        let bin = match (b >> 4) & 0x03 {
            0 => Some(BinDrvnSel::El4),
            1 => Some(BinDrvnSel::El3A),
            2 => Some(BinDrvnSel::El3B),
            _ => None,
        };

        let drvp = match (b >> 2) & 0x03 {
            0 => Some(BipDrvpSel::El1),
            1 => Some(BipDrvpSel::El2A),
            2 => Some(BipDrvpSel::El2B),
            _ => None,
        };

        let drvn = match b & 0x03 {
            0 => Some(BinDrvnSel::El4),
            1 => Some(BinDrvnSel::El3A),
            2 => Some(BinDrvnSel::El3B),
            _ => None,
        };

        Self {
            bip,
            bin,
            drvp,
            drvn,
        }
    }
}

impl BiozMuxConfig3 {
    pub fn encode(self) -> Option<RegBiozMuxConfig3> {
        let mut reg = RegBiozMuxConfig3::empty();
        let mut changed = false;

        if let Some(v) = self.bip {
            reg.remove(RegBiozMuxConfig3::BIP_MASK);
            reg.insert(RegBiozMuxConfig3::from_bits_truncate((v as u8) << 6));
            changed = true;
        }

        if let Some(v) = self.bin {
            reg.remove(RegBiozMuxConfig3::BIN_MASK);
            reg.insert(RegBiozMuxConfig3::from_bits_truncate((v as u8) << 4));
            changed = true;
        }

        if let Some(v) = self.drvp {
            reg.remove(RegBiozMuxConfig3::DRVP_MASK);
            reg.insert(RegBiozMuxConfig3::from_bits_truncate((v as u8) << 2));
            changed = true;
        }

        if let Some(v) = self.drvn {
            reg.remove(RegBiozMuxConfig3::DRVN_MASK);
            reg.insert(RegBiozMuxConfig3::from_bits_truncate(v as u8));
            changed = true;
        }

        if changed {
            Some(reg)
        } else {
            None
        }
    }
}

impl From<RegBiozMuxConfig4> for BiozMuxBist {
    fn from(r: RegBiozMuxConfig4) -> Self {
        Self {
            bist_r_err: r.0 as i8,
        }
    }
}

impl<I: RegisterInterface> Apply<I> for BiozMuxConfig1 {
    fn apply(self, dev: &mut Max30009<I>) -> Result<(), I::Error> {
        self.debug_dump();
        if let Some(r) = self.encode() {
            dev.write(r)?;
        }
        Ok(())
    }
}

impl<I: RegisterInterface> Apply<I> for BiozMuxConfig2 {
    fn apply(self, dev: &mut Max30009<I>) -> Result<(), I::Error> {
        self.debug_dump();
        if let Some(r) = self.encode() {
            dev.write(r)?;
        }
        Ok(())
    }
}

impl<I: RegisterInterface> Apply<I> for BiozMuxConfig3 {
    fn apply(self, dev: &mut Max30009<I>) -> Result<(), I::Error> {
        self.debug_dump();
        if let Some(r) = self.encode() {
            dev.write(r)?;
        }
        Ok(())
    }
}

impl<I: RegisterInterface> Readback<I> for BiozMuxConfig1 {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from(dev.read_reg::<RegBiozMuxConfig1>()?);
        cfg.debug_dump();
        Ok(cfg)
    }
}

impl<I: RegisterInterface> Readback<I> for BiozMuxConfig2 {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from(dev.read_reg::<RegBiozMuxConfig2>()?);
        cfg.debug_dump();
        Ok(cfg)
    }
}

impl<I: RegisterInterface> Readback<I> for BiozMuxConfig3 {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from(dev.read_reg::<RegBiozMuxConfig3>()?);
        cfg.debug_dump();
        Ok(cfg)
    }
}

impl<I: RegisterInterface> Readback<I> for BiozMuxBist {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from(dev.read_reg::<RegBiozMuxConfig4>()?);
        cfg.debug_dump();
        Ok(cfg)
    }
}

impl DebugDump for BiozMuxConfig1 {
    fn debug_dump(&self) {
        debug!("╔══════════════════════════════════════╗");
        debug!("║            BIOZ MUX CFG1             ║");
        debug!("╠══════════════════════════════════════╣");
        if let Some(r) = self.encode() {
            let _v = r.bits();
            debug!(" BIOZ_MUX_CFG1 = 0x{:02X} | b{:08b}", _v, _v);
            debug!("   rsel             : {:?}", self.rsel);
            debug!("   bist_en          : {:?}", self.bist_en);
            debug!("   connect_cal_only : {:?}", self.connect_cal_only);
            debug!("   mux_en           : {:?}", self.mux_en);
            debug!("   cal_en           : {:?}", self.cal_en);
        } else {
            warn!(" BIOZ_MUX_CFG1 : <unchanged>");
        }
        debug!("╚══════════════════════════════════════╝");
    }
}

impl DebugDump for BiozMuxConfig2 {
    fn debug_dump(&self) {
        debug!("╔══════════════════════════════════════╗");
        debug!("║            BIOZ MUX CFG2             ║");
        debug!("╠══════════════════════════════════════╣");
        if let Some(r) = self.encode() {
            let _v = r.bits();
            debug!(" BIOZ_MUX_CFG2 = 0x{:02X} | b{:08b}", _v, _v);
            debug!("   gsr_rsel      : {:?}", self.gsr_rsel);
            debug!("   gsr_load_en   : {:?}", self.gsr_load_en);
            debug!("   en_ext_inload : {:?}", self.en_ext_inload);
            debug!("   en_int_inload : {:?}", self.en_int_inload);
        } else {
            warn!(" BIOZ_MUX_CFG2 : <unchanged>");
        }
        debug!("╚══════════════════════════════════════╝");
    }
}

impl DebugDump for BiozMuxConfig3 {
    fn debug_dump(&self) {
        debug!("╔══════════════════════════════════════╗");
        debug!("║            BIOZ MUX CFG3             ║");
        debug!("╠══════════════════════════════════════╣");
        if let Some(r) = self.encode() {
            let _v = r.bits();
            debug!(" BIOZ_MUX_CFG3 = 0x{:02X} | b{:08b}", _v, _v);
            debug!("   bip  : {:?}", self.bip);
            debug!("   bin  : {:?}", self.bin);
            debug!("   drvp : {:?}", self.drvp);
            debug!("   drvn : {:?}", self.drvn);
        } else {
            warn!(" BIOZ_MUX_CFG3 : <unchanged>");
        }
        debug!("╚══════════════════════════════════════╝");
    }
}

impl DebugDump for BiozMuxBist {
    fn debug_dump(&self) {
        debug!("╔══════════════════════════════════════╗");
        debug!("║              BIOZ MUX BIST           ║");
        debug!("╠══════════════════════════════════════╣");
        debug!(" BIST_R_ERR = {}", self.bist_r_err);
        debug!("╚══════════════════════════════════════╝");
    }
}

impl Merge for BiozMuxConfig1 {
    fn merge(self, current: Self) -> Self {
        Self {
            rsel: self.rsel.or(current.rsel),
            bist_en: self.bist_en.or(current.bist_en),
            connect_cal_only: self.connect_cal_only.or(current.connect_cal_only),
            mux_en: self.mux_en.or(current.mux_en),
            cal_en: self.cal_en.or(current.cal_en),
        }
    }
}

impl Merge for BiozMuxConfig2 {
    fn merge(self, current: Self) -> Self {
        Self {
            gsr_rsel: self.gsr_rsel.or(current.gsr_rsel),
            gsr_load_en: self.gsr_load_en.or(current.gsr_load_en),
            en_ext_inload: self.en_ext_inload.or(current.en_ext_inload),
            en_int_inload: self.en_int_inload.or(current.en_int_inload),
        }
    }
}

impl Merge for BiozMuxConfig3 {
    fn merge(self, current: Self) -> Self {
        Self {
            bip: self.bip.or(current.bip),
            bin: self.bin.or(current.bin),
            drvp: self.drvp.or(current.drvp),
            drvn: self.drvn.or(current.drvn),
        }
    }
}
