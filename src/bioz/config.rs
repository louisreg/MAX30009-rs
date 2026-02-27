//! BioZ high-level configuration builders

use crate::bioz::registers::*;
use crate::bioz::types::*;
use crate::device::Max30009;
use crate::register_interface::RegisterInterface;
use crate::traits::{Apply, DebugDump, Merge, Readback};
use crate::{debug, warn};
#[cfg(feature = "rpc-types")]
use postcard_schema::Schema;
#[cfg(feature = "rpc-types")]
use serde::{Deserialize, Serialize};

//
// HIGH LEVEL STRUCTS
//

#[cfg_attr(feature = "rpc-types", derive(Serialize, Deserialize, Schema))]
#[derive(Default, Copy, Clone, Debug)]
pub struct BiozConfig1 {
    pub dac_osr: Option<BiozDacOsr>,
    pub adc_osr: Option<BiozAdcOsr>,
    pub bg_en: Option<bool>,
    pub i_en: Option<bool>,
    pub q_en: Option<bool>,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct BiozConfig2 {
    pub dhpf: Option<BiozDhpf>,
    pub dlpf: Option<BiozDlpf>,
    pub cmp: Option<BiozCmp>,
    pub thresh: Option<bool>,
}

//
// ENCODE
//

impl BiozConfig1 {
    pub fn encode(self) -> Option<RegBiozConfig1> {
        let mut reg = RegBiozConfig1::empty();
        let mut changed = false;

        if let Some(v) = self.dac_osr {
            reg = reg.set_dac_osr(v);
            changed = true;
        }

        if let Some(v) = self.adc_osr {
            reg = reg.set_adc_osr(v);
            changed = true;
        }

        if let Some(v) = self.bg_en {
            reg = reg.enable_bg(v);
            changed = true;
        }

        if let Some(v) = self.i_en {
            reg = reg.enable_i(v);
            changed = true;
        }

        if let Some(v) = self.q_en {
            reg = reg.enable_q(v);
            changed = true;
        }

        if changed {
            Some(reg)
        } else {
            None
        }
    }

    pub fn debug_dump(&self) {
        debug!("╔══════════════════════════════════════╗");
        debug!("║             BIOZ CONFIG1             ║");
        debug!("╠══════════════════════════════════════╣");

        if let Some(r) = self.encode() {
            let _v = r.bits();
            debug!(" BIOZ_CONFIG1 = 0x{:02X} | b{:08b}", _v, _v);

            debug!("   dac_osr : {:?}", self.dac_osr);
            debug!("   adc_osr : {:?}", self.adc_osr);
            debug!("   bg_en   : {:?}", self.bg_en);
            debug!("   i_en    : {:?}", self.i_en);
            debug!("   q_en    : {:?}", self.q_en);
        } else {
            warn!(" BIOZ_CONFIG1 : <unchanged>");
        }

        debug!("╚══════════════════════════════════════╝");
    }
}

impl From<RegBiozConfig1> for BiozConfig1 {
    fn from(r: RegBiozConfig1) -> Self {
        let bits = r.bits();

        Self {
            dac_osr: Some(match (bits >> 6) & 0x03 {
                0 => BiozDacOsr::Osr32,
                1 => BiozDacOsr::Osr64,
                2 => BiozDacOsr::Osr128,
                _ => BiozDacOsr::Osr256,
            }),

            adc_osr: Some(match (bits >> 3) & 0x07 {
                0 => BiozAdcOsr::Osr8,
                1 => BiozAdcOsr::Osr16,
                2 => BiozAdcOsr::Osr32,
                3 => BiozAdcOsr::Osr64,
                4 => BiozAdcOsr::Osr128,
                5 => BiozAdcOsr::Osr256,
                6 => BiozAdcOsr::Osr512,
                _ => BiozAdcOsr::Osr1024,
            }),

            bg_en: Some((bits & (1 << 2)) != 0),
            q_en: Some((bits & (1 << 1)) != 0),
            i_en: Some((bits & (1 << 0)) != 0),
        }
    }
}

impl BiozConfig2 {
    pub fn encode(self) -> Option<RegBiozConfig2> {
        let mut reg = RegBiozConfig2::empty();
        let mut changed = false;

        if let Some(v) = self.dhpf {
            reg = reg.set_dhpf(v);
            changed = true;
        }

        if let Some(v) = self.dlpf {
            reg = reg.set_dlpf(v);
            changed = true;
        }

        if let Some(v) = self.cmp {
            reg = reg.set_cmp(v);
            changed = true;
        }

        if let Some(v) = self.thresh {
            reg = reg.enable_threshold(v);
            changed = true;
        }

        if changed {
            Some(reg)
        } else {
            None
        }
    }

    pub fn debug_dump(&self) {
        debug!("╔══════════════════════════════════════╗");
        debug!("║             BIOZ CONFIG2             ║");
        debug!("╠══════════════════════════════════════╣");

        if let Some(r) = self.encode() {
            let _v = r.bits();
            debug!(" BIOZ_CONFIG2 = 0x{:02X} | b{:08b}", _v, _v);

            debug!("   dhpf   : {:?}", self.dhpf);
            debug!("   dlpf   : {:?}", self.dlpf);
            debug!("   cmp    : {:?}", self.cmp);
            debug!("   thresh : {:?}", self.thresh);
        } else {
            warn!(" BIOZ_CONFIG2 : <unchanged>");
        }

        debug!("╚══════════════════════════════════════╝");
    }
}

impl From<RegBiozConfig2> for BiozConfig2 {
    fn from(r: RegBiozConfig2) -> Self {
        let bits = r.bits();

        Self {
            dhpf: Some(match (bits >> 6) & 0x03 {
                0 => BiozDhpf::Bypass,
                1 => BiozDhpf::Hp00025,
                _ => BiozDhpf::Hp002,
            }),

            dlpf: Some(match (bits >> 3) & 0x07 {
                0 => BiozDlpf::Bypass,
                1 => BiozDlpf::Lp0005,
                2 => BiozDlpf::Lp002,
                3 => BiozDlpf::Lp008,
                _ => BiozDlpf::Lp025,
            }),

            cmp: Some(match (bits >> 1) & 0x03 {
                0 => BiozCmp::I,
                1 => BiozCmp::Q,
                _ => BiozCmp::Z,
            }),

            thresh: Some((bits & 1) != 0),
        }
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Default, Copy, Clone, Debug)]
pub struct BiozConfig3 {
    pub ext_res: Option<bool>,
    pub loff_rapid: Option<bool>,
    pub vdrv: Option<BiozVdrvMag>,
    pub idrv: Option<BiozIdrvRge>,
    pub mode: Option<BiozDrvMode>,
}

impl From<RegBiozConfig3> for BiozConfig3 {
    fn from(r: RegBiozConfig3) -> Self {
        let bits = r.bits();

        Self {
            ext_res: Some((bits & (1 << 7)) != 0),
            loff_rapid: Some((bits & (1 << 6)) != 0),

            vdrv: Some(match (bits >> 4) & 0x03 {
                0 => BiozVdrvMag::M50,
                1 => BiozVdrvMag::M100,
                2 => BiozVdrvMag::M250,
                _ => BiozVdrvMag::M500,
            }),

            idrv: Some(match (bits >> 2) & 0x03 {
                0 => BiozIdrvRge::R552k5,
                1 => BiozIdrvRge::R110k5,
                2 => BiozIdrvRge::R5k525,
                _ => BiozIdrvRge::R276R25,
            }),

            mode: Some(match bits & 0x03 {
                0 => BiozDrvMode::Current,
                1 => BiozDrvMode::Voltage,
                2 => BiozDrvMode::HBridge,
                _ => BiozDrvMode::Standby,
            }),
        }
    }
}

impl BiozConfig3 {
    pub fn encode(self) -> Option<RegBiozConfig3> {
        let mut reg = RegBiozConfig3::empty();
        let mut changed = false;

        // ───────── EXT RES ─────────
        if let Some(v) = self.ext_res {
            reg.set(RegBiozConfig3::EXT_RES, v);
            changed = true;
        }

        // ───────── LOFF RAPID ─────────
        if let Some(v) = self.loff_rapid {
            reg.set(RegBiozConfig3::LOFF_RAPID, v);
            changed = true;
        }

        // ───────── VDRV MAG ─────────
        if let Some(v) = self.vdrv {
            reg = reg.set_vdrv(v);
            changed = true;
        }

        // ───────── IDRV RANGE ─────────
        if let Some(v) = self.idrv {
            reg = reg.set_idrv(v);
            changed = true;
        }

        // ───────── DRIVE MODE ─────────
        if let Some(v) = self.mode {
            reg = reg.set_mode(v);
            changed = true;
        }

        if changed {
            Some(reg)
        } else {
            None
        }
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Default, Copy, Clone, Debug)]
pub struct BiozConfig4 {
    pub fast_manual: Option<bool>,
    pub fast_start_en: Option<bool>,
}

impl From<RegBiozConfig4> for BiozConfig4 {
    fn from(r: RegBiozConfig4) -> Self {
        let bits = r.bits();

        Self {
            fast_manual: Some((bits & (1 << 1)) != 0),
            fast_start_en: Some((bits & (1 << 0)) != 0),
        }
    }
}

impl BiozConfig4 {
    pub fn encode(self) -> Option<RegBiozConfig4> {
        let mut reg = RegBiozConfig4::empty();
        let mut changed = false;

        if let Some(v) = self.fast_manual {
            reg.set(RegBiozConfig4::FAST_MANUAL, v);
            changed = true;
        }

        if let Some(v) = self.fast_start_en {
            reg.set(RegBiozConfig4::FAST_START_EN, v);
            changed = true;
        }

        if changed {
            Some(reg)
        } else {
            None
        }
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Default, Copy, Clone, Debug)]
pub struct BiozConfig5 {
    pub ahpf: Option<BiozAhpf>,
    pub ina_mode: Option<bool>,
    pub dm_dis: Option<bool>,
    pub gain: Option<BiozGain>,
}

impl From<RegBiozConfig5> for BiozConfig5 {
    fn from(r: RegBiozConfig5) -> Self {
        let bits = r.bits();

        Self {
            ahpf: Some(unsafe { core::mem::transmute((bits >> 4) & 0x0F) }),
            ina_mode: Some((bits & (1 << 3)) != 0),
            dm_dis: Some((bits & (1 << 2)) != 0),

            gain: Some(match bits & 0x03 {
                0 => BiozGain::Gain1VV,
                1 => BiozGain::Gain2VV,
                2 => BiozGain::Gain5VV,
                _ => BiozGain::Gain10VV,
            }),
        }
    }
}

impl BiozConfig5 {
    pub fn encode(self) -> Option<RegBiozConfig5> {
        let mut reg = RegBiozConfig5::empty();
        let mut changed = false;

        if let Some(v) = self.ahpf {
            reg = reg.set_ahpf(v);
            changed = true;
        }

        if let Some(v) = self.ina_mode {
            reg.set(RegBiozConfig5::INA_MODE, v);
            changed = true;
        }

        if let Some(v) = self.dm_dis {
            reg.set(RegBiozConfig5::DM_DIS, v);
            changed = true;
        }

        if let Some(v) = self.gain {
            reg = reg.set_gain(v);
            changed = true;
        }

        if changed {
            Some(reg)
        } else {
            None
        }
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Default, Copy, Clone, Debug)]
pub struct BiozConfig6 {
    pub ext_cap: Option<bool>,
    pub dc_restore: Option<bool>,
    pub drv_reset: Option<bool>,
    pub dac_reset: Option<bool>,
    pub amp_rge: Option<BiozAmpRange>,
    pub amp_bw: Option<BiozAmpBw>,
}

impl From<RegBiozConfig6> for BiozConfig6 {
    fn from(r: RegBiozConfig6) -> Self {
        let bits = r.bits();

        Self {
            ext_cap: Some((bits & (1 << 7)) != 0),
            dc_restore: Some((bits & (1 << 6)) != 0),
            drv_reset: Some((bits & (1 << 5)) != 0),
            dac_reset: Some((bits & (1 << 4)) != 0),

            amp_rge: Some(match (bits >> 2) & 0x03 {
                0 => BiozAmpRange::Low,
                1 => BiozAmpRange::MedLow,
                2 => BiozAmpRange::MedHigh,
                _ => BiozAmpRange::High,
            }),

            amp_bw: Some(match bits & 0x03 {
                0 => BiozAmpBw::Low,
                1 => BiozAmpBw::MedLow,
                2 => BiozAmpBw::MedHigh,
                _ => BiozAmpBw::High,
            }),
        }
    }
}

impl BiozConfig6 {
    pub fn encode(self) -> Option<RegBiozConfig6> {
        let mut reg = RegBiozConfig6::empty();
        let mut changed = false;

        if let Some(v) = self.ext_cap {
            reg.set(RegBiozConfig6::EXT_CAP, v);
            changed = true;
        }

        if let Some(v) = self.dc_restore {
            reg.set(RegBiozConfig6::DC_RESTORE, v);
            changed = true;
        }

        if let Some(v) = self.drv_reset {
            reg.set(RegBiozConfig6::DRV_RESET, v);
            changed = true;
        }

        if let Some(v) = self.dac_reset {
            reg.set(RegBiozConfig6::DAC_RESET, v);
            changed = true;
        }

        if let Some(v) = self.amp_rge {
            reg = reg.set_amp_rge(v);
            changed = true;
        }

        if let Some(v) = self.amp_bw {
            reg = reg.set_amp_bw(v);
            changed = true;
        }

        if changed {
            Some(reg)
        } else {
            None
        }
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Default, Copy, Clone, Debug)]
pub struct BiozConfig7 {
    pub stbyon: Option<bool>,
    pub q_clk_phase: Option<bool>,
    pub i_clk_phase: Option<bool>,
    pub ina_chop_en: Option<bool>,
    pub ch_fsel: Option<bool>,
}

impl From<RegBiozConfig7> for BiozConfig7 {
    fn from(r: RegBiozConfig7) -> Self {
        let bits = r.bits();

        Self {
            stbyon: Some((bits & (1 << 4)) != 0),
            q_clk_phase: Some((bits & (1 << 3)) != 0),
            i_clk_phase: Some((bits & (1 << 2)) != 0),
            ina_chop_en: Some((bits & (1 << 1)) != 0),
            ch_fsel: Some((bits & (1 << 0)) != 0),
        }
    }
}

impl BiozConfig7 {
    pub fn encode(self) -> Option<RegBiozConfig7> {
        let mut reg = RegBiozConfig7::empty();
        let mut changed = false;

        if let Some(v) = self.stbyon {
            reg.set(RegBiozConfig7::STBYON, v);
            changed = true;
        }

        if let Some(v) = self.q_clk_phase {
            reg.set(RegBiozConfig7::Q_CLK_PHASE, v);
            changed = true;
        }

        if let Some(v) = self.i_clk_phase {
            reg.set(RegBiozConfig7::I_CLK_PHASE, v);
            changed = true;
        }

        if let Some(v) = self.ina_chop_en {
            reg.set(RegBiozConfig7::INA_CHOP_EN, v);
            changed = true;
        }

        if let Some(v) = self.ch_fsel {
            reg.set(RegBiozConfig7::CH_FSEL, v);
            changed = true;
        }

        if changed {
            Some(reg)
        } else {
            None
        }
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug)]
pub struct BiozThresholds {
    pub lo: Option<u8>,
    pub hi: Option<u8>,
}

impl BiozThresholds {
    pub fn lo_reg(self) -> Option<RegBiozLoThresh> {
        self.lo.map(RegBiozLoThresh)
    }

    pub fn hi_reg(self) -> Option<RegBiozHiThresh> {
        self.hi.map(RegBiozHiThresh)
    }

    pub fn debug_dump(&self) {
        debug!("╔══════════════════════════════════════╗");
        debug!("║           BIOZ THRESHOLDS            ║");
        debug!("╠══════════════════════════════════════╣");

        if let Some(_lo) = self.lo {
            debug!(" LO_THRESH = 0x{:02X} | b{:08b}", _lo, _lo);
            debug!("   under-range window = ±{}", (_lo as u32) * 32);
        } else {
            debug!(" LO_THRESH : <unchanged>");
        }

        if let Some(_hi) = self.hi {
            debug!(" HI_THRESH = 0x{:02X} | b{:08b}", _hi, _hi);
            debug!("   over-range window = ±{}", (_hi as u32) * 2048);
        } else {
            debug!(" HI_THRESH : <unchanged>");
        }

        debug!("╚══════════════════════════════════════╝");
    }
}

impl From<(RegBiozLoThresh, RegBiozHiThresh)> for BiozThresholds {
    fn from((lo, hi): (RegBiozLoThresh, RegBiozHiThresh)) -> Self {
        Self {
            lo: Some(lo.0),
            hi: Some(hi.0),
        }
    }
}

impl<I: RegisterInterface> Apply<I> for BiozConfig1 {
    fn apply(self, dev: &mut Max30009<I>) -> Result<(), I::Error> {
        self.debug_dump();
        if let Some(r) = self.encode() {
            dev.write(r)?;
        }
        Ok(())
    }
}

impl<I: RegisterInterface> Apply<I> for BiozConfig2 {
    fn apply(self, dev: &mut Max30009<I>) -> Result<(), I::Error> {
        self.debug_dump();
        if let Some(r) = self.encode() {
            dev.write(r)?;
        }
        Ok(())
    }
}

impl<I: RegisterInterface> Apply<I> for BiozConfig3 {
    fn apply(self, dev: &mut Max30009<I>) -> Result<(), I::Error> {
        self.debug_dump();
        if let Some(r) = self.encode() {
            dev.write(r)?;
        }
        Ok(())
    }
}

impl<I: RegisterInterface> Apply<I> for BiozConfig4 {
    fn apply(self, dev: &mut Max30009<I>) -> Result<(), I::Error> {
        self.debug_dump();
        if let Some(r) = self.encode() {
            dev.write(r)?;
        }
        Ok(())
    }
}

impl<I: RegisterInterface> Apply<I> for BiozConfig5 {
    fn apply(self, dev: &mut Max30009<I>) -> Result<(), I::Error> {
        self.debug_dump();
        if let Some(r) = self.encode() {
            dev.write(r)?;
        }
        Ok(())
    }
}

impl<I: RegisterInterface> Apply<I> for BiozConfig6 {
    fn apply(self, dev: &mut Max30009<I>) -> Result<(), I::Error> {
        self.debug_dump();
        if let Some(r) = self.encode() {
            dev.write(r)?;
        }
        Ok(())
    }
}

impl<I: RegisterInterface> Apply<I> for BiozConfig7 {
    fn apply(self, dev: &mut Max30009<I>) -> Result<(), I::Error> {
        self.debug_dump();
        if let Some(r) = self.encode() {
            dev.write(r)?;
        }
        Ok(())
    }
}

impl<I: RegisterInterface> Apply<I> for BiozThresholds {
    fn apply(self, dev: &mut Max30009<I>) -> Result<(), I::Error> {
        self.debug_dump();
        if let Some(lo) = self.lo_reg() {
            dev.write(lo)?;
        }

        if let Some(hi) = self.hi_reg() {
            dev.write(hi)?;
        }

        Ok(())
    }
}

impl<I: RegisterInterface> Readback<I> for BiozConfig1 {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from(dev.read_reg::<RegBiozConfig1>()?);
        cfg.debug_dump();
        Ok(cfg)
    }
}

impl<I: RegisterInterface> Readback<I> for BiozConfig2 {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from(dev.read_reg::<RegBiozConfig2>()?);
        cfg.debug_dump();
        Ok(cfg)
    }
}

impl<I: RegisterInterface> Readback<I> for BiozConfig3 {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from(dev.read_reg::<RegBiozConfig3>()?);
        cfg.debug_dump();
        Ok(cfg)
    }
}

impl<I: RegisterInterface> Readback<I> for BiozConfig4 {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from(dev.read_reg::<RegBiozConfig4>()?);
        cfg.debug_dump();
        Ok(cfg)
    }
}

impl<I: RegisterInterface> Readback<I> for BiozConfig5 {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from(dev.read_reg::<RegBiozConfig5>()?);
        cfg.debug_dump();
        Ok(cfg)
    }
}

impl<I: RegisterInterface> Readback<I> for BiozConfig6 {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from(dev.read_reg::<RegBiozConfig6>()?);
        cfg.debug_dump();
        Ok(cfg)
    }
}

impl<I: RegisterInterface> Readback<I> for BiozConfig7 {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from(dev.read_reg::<RegBiozConfig7>()?);
        cfg.debug_dump();
        Ok(cfg)
    }
}

impl<I: RegisterInterface> Readback<I> for BiozThresholds {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from((
            dev.read_reg::<RegBiozLoThresh>()?,
            dev.read_reg::<RegBiozHiThresh>()?,
        ));
        cfg.debug_dump();
        Ok(cfg)
    }
}

impl DebugDump for BiozConfig1 {
    fn debug_dump(&self) {
        BiozConfig1::debug_dump(self);
    }
}

impl DebugDump for BiozConfig2 {
    fn debug_dump(&self) {
        BiozConfig2::debug_dump(self);
    }
}

impl DebugDump for BiozConfig3 {
    fn debug_dump(&self) {
        debug!("╔══════════════════════════════════════╗");
        debug!("║             BIOZ CONFIG3             ║");
        debug!("╠══════════════════════════════════════╣");
        if let Some(r) = self.encode() {
            let _v = r.bits();
            debug!(" BIOZ_CONFIG3 = 0x{:02X} | b{:08b}", _v, _v);
            debug!("   ext_res    : {:?}", self.ext_res);
            debug!("   loff_rapid : {:?}", self.loff_rapid);
            debug!("   vdrv       : {:?}", self.vdrv);
            debug!("   idrv       : {:?}", self.idrv);
            debug!("   mode       : {:?}", self.mode);
        } else {
            warn!(" BIOZ_CONFIG3 : <unchanged>");
        }
        debug!("╚══════════════════════════════════════╝");
    }
}

impl DebugDump for BiozConfig4 {
    fn debug_dump(&self) {
        debug!("╔══════════════════════════════════════╗");
        debug!("║             BIOZ CONFIG4             ║");
        debug!("╠══════════════════════════════════════╣");
        if let Some(r) = self.encode() {
            let _v = r.bits();
            debug!(" BIOZ_CONFIG4 = 0x{:02X} | b{:08b}", _v, _v);
            debug!("   fast_manual   : {:?}", self.fast_manual);
            debug!("   fast_start_en : {:?}", self.fast_start_en);
        } else {
            warn!(" BIOZ_CONFIG4 : <unchanged>");
        }
        debug!("╚══════════════════════════════════════╝");
    }
}

impl DebugDump for BiozConfig5 {
    fn debug_dump(&self) {
        debug!("╔══════════════════════════════════════╗");
        debug!("║             BIOZ CONFIG5             ║");
        debug!("╠══════════════════════════════════════╣");
        if let Some(r) = self.encode() {
            let _v = r.bits();
            debug!(" BIOZ_CONFIG5 = 0x{:02X} | b{:08b}", _v, _v);
            debug!("   ahpf     : {:?}", self.ahpf);
            debug!("   ina_mode : {:?}", self.ina_mode);
            debug!("   dm_dis   : {:?}", self.dm_dis);
            debug!("   gain     : {:?}", self.gain);
        } else {
            warn!(" BIOZ_CONFIG5 : <unchanged>");
        }
        debug!("╚══════════════════════════════════════╝");
    }
}

impl DebugDump for BiozConfig6 {
    fn debug_dump(&self) {
        debug!("╔══════════════════════════════════════╗");
        debug!("║             BIOZ CONFIG6             ║");
        debug!("╠══════════════════════════════════════╣");
        if let Some(r) = self.encode() {
            let _v = r.bits();
            debug!(" BIOZ_CONFIG6 = 0x{:02X} | b{:08b}", _v, _v);
            debug!("   ext_cap    : {:?}", self.ext_cap);
            debug!("   dc_restore : {:?}", self.dc_restore);
            debug!("   drv_reset  : {:?}", self.drv_reset);
            debug!("   dac_reset  : {:?}", self.dac_reset);
            debug!("   amp_rge    : {:?}", self.amp_rge);
            debug!("   amp_bw     : {:?}", self.amp_bw);
        } else {
            warn!(" BIOZ_CONFIG6 : <unchanged>");
        }
        debug!("╚══════════════════════════════════════╝");
    }
}

impl DebugDump for BiozConfig7 {
    fn debug_dump(&self) {
        debug!("╔══════════════════════════════════════╗");
        debug!("║             BIOZ CONFIG7             ║");
        debug!("╠══════════════════════════════════════╣");
        if let Some(r) = self.encode() {
            let _v = r.bits();
            debug!(" BIOZ_CONFIG7 = 0x{:02X} | b{:08b}", _v, _v);
            debug!("   stbyon      : {:?}", self.stbyon);
            debug!("   q_clk_phase : {:?}", self.q_clk_phase);
            debug!("   i_clk_phase : {:?}", self.i_clk_phase);
            debug!("   ina_chop_en : {:?}", self.ina_chop_en);
            debug!("   ch_fsel     : {:?}", self.ch_fsel);
        } else {
            warn!(" BIOZ_CONFIG7 : <unchanged>");
        }
        debug!("╚══════════════════════════════════════╝");
    }
}

impl DebugDump for BiozThresholds {
    fn debug_dump(&self) {
        BiozThresholds::debug_dump(self);
    }
}

impl Merge for BiozConfig1 {
    fn merge(self, current: Self) -> Self {
        Self {
            dac_osr: self.dac_osr.or(current.dac_osr),
            adc_osr: self.adc_osr.or(current.adc_osr),
            bg_en: self.bg_en.or(current.bg_en),
            i_en: self.i_en.or(current.i_en),
            q_en: self.q_en.or(current.q_en),
        }
    }
}

impl Merge for BiozConfig2 {
    fn merge(self, current: Self) -> Self {
        Self {
            dhpf: self.dhpf.or(current.dhpf),
            dlpf: self.dlpf.or(current.dlpf),
            cmp: self.cmp.or(current.cmp),
            thresh: self.thresh.or(current.thresh),
        }
    }
}

impl Merge for BiozConfig3 {
    fn merge(self, current: Self) -> Self {
        Self {
            ext_res: self.ext_res.or(current.ext_res),
            loff_rapid: self.loff_rapid.or(current.loff_rapid),
            vdrv: self.vdrv.or(current.vdrv),
            idrv: self.idrv.or(current.idrv),
            mode: self.mode.or(current.mode),
        }
    }
}

impl Merge for BiozConfig4 {
    fn merge(self, current: Self) -> Self {
        Self {
            fast_manual: self.fast_manual.or(current.fast_manual),
            fast_start_en: self.fast_start_en.or(current.fast_start_en),
        }
    }
}

impl Merge for BiozConfig5 {
    fn merge(self, current: Self) -> Self {
        Self {
            ahpf: self.ahpf.or(current.ahpf),
            ina_mode: self.ina_mode.or(current.ina_mode),
            dm_dis: self.dm_dis.or(current.dm_dis),
            gain: self.gain.or(current.gain),
        }
    }
}

impl Merge for BiozConfig6 {
    fn merge(self, current: Self) -> Self {
        Self {
            ext_cap: self.ext_cap.or(current.ext_cap),
            dc_restore: self.dc_restore.or(current.dc_restore),
            drv_reset: self.drv_reset.or(current.drv_reset),
            dac_reset: self.dac_reset.or(current.dac_reset),
            amp_rge: self.amp_rge.or(current.amp_rge),
            amp_bw: self.amp_bw.or(current.amp_bw),
        }
    }
}

impl Merge for BiozConfig7 {
    fn merge(self, current: Self) -> Self {
        Self {
            stbyon: self.stbyon.or(current.stbyon),
            q_clk_phase: self.q_clk_phase.or(current.q_clk_phase),
            i_clk_phase: self.i_clk_phase.or(current.i_clk_phase),
            ina_chop_en: self.ina_chop_en.or(current.ina_chop_en),
            ch_fsel: self.ch_fsel.or(current.ch_fsel),
        }
    }
}

impl Merge for BiozThresholds {
    fn merge(self, current: Self) -> Self {
        Self {
            lo: self.lo.or(current.lo),
            hi: self.hi.or(current.hi),
        }
    }
}
