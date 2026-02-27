//! PLL high-level configuration builders
//!
//! Pure encoding layer.
//! No hardware access.

use crate::{debug, warn};

use crate::device::Max30009;
use crate::pll::registers::*;
use crate::pll::types::*;
use crate::register_interface::RegisterInterface;
use crate::traits::{Apply, DebugDump, Merge, Readback, WritableRegister};
#[cfg(feature = "rpc-types")]
use postcard_schema::Schema;
#[cfg(feature = "rpc-types")]
use serde::{Deserialize, Serialize};

//
// ─────────────────────────────────────────────
// STATIC PLL CONFIG (0x19 / 0x1A)
// ─────────────────────────────────────────────
//

#[derive(Copy, Clone, Debug, Default)]
pub struct PllConfig {
    pub lock_window_2clk: Option<bool>,
    pub ref_clock: Option<RefClockSel>,
    pub ref_freq: Option<ClockFreqSel>,
    pub fine_tune: Option<i8>,
}

impl PllConfig {
    // --------------------------------------------------
    // 🌟 NEW GENERIC ENCODER
    // --------------------------------------------------

    pub fn encode(self) -> (Option<PllConfig3>, Option<PllConfig4>) {
        (self.pll_config3(), self.pll_config4())
    }

    /// PLL_CONFIG_3
    pub fn pll_config3(self) -> Option<PllConfig3> {
        self.lock_window_2clk.map(|lock2| {
            if lock2 {
                PllConfig3::lock_window_2clk()
            } else {
                PllConfig3::lock_window_1clk()
            }
        })
    }

    /// PLL_CONFIG_4
    pub fn pll_config4(self) -> Option<PllConfig4> {
        if self.ref_clock.is_none() && self.ref_freq.is_none() && self.fine_tune.is_none() {
            return None;
        }

        Some(PllConfig4 {
            ref_clk_sel: self.ref_clock.map(|c| c.bit()).unwrap_or(false),
            clk_freq_sel: self.ref_freq.map(|f| f.bit()).unwrap_or(false),
            clk_fine_tune: self.fine_tune.unwrap_or(0),
        })
    }

    // --------------------------------------------------
    // DEBUG
    // --------------------------------------------------

    pub fn debug_dump(&self) {
        debug!("╔══════════════════════════════════════╗");
        debug!("║           PLL STATIC CONFIG          ║");
        debug!("╠══════════════════════════════════════╣");

        let (r3, r4) = self.encode();

        if let Some(r) = r3 {
            let _v = r.bits();
            debug!(" PLL_CONFIG3 = 0x{:02X} | b{:08b}", _v, _v);
            debug!("   lock_window_2clk : {}", (_v & 1) != 0);
        } else {
            warn!(" PLL_CONFIG3 : <unchanged>");
        }

        if let Some(r) = r4 {
            let _v = r.encode();
            debug!(" PLL_CONFIG4 = 0x{:02X} | b{:08b}", _v, _v);

            debug!("   ref_clk_sel : {}", r.ref_clk_sel);
            debug!("   clk_freq    : {}", r.clk_freq_sel);
            debug!("   fine_tune   : {}", r.clk_fine_tune);
        } else {
            warn!(" PLL_CONFIG4 : <unchanged>");
        }

        debug!("╚══════════════════════════════════════╝");
    }
}

// register → high level

impl From<(PllConfig3, PllConfig4)> for PllConfig {
    fn from((r3, r4): (PllConfig3, PllConfig4)) -> Self {
        use crate::pll::types::*;

        Self {
            lock_window_2clk: Some(r3.contains(PllConfig3::PLL_LOCK_WNDW)),

            ref_clock: Some(if r4.ref_clk_sel {
                RefClockSel::External
            } else {
                RefClockSel::Internal
            }),

            ref_freq: Some(if r4.clk_freq_sel {
                ClockFreqSel::Khz32768
            } else {
                ClockFreqSel::Khz32000
            }),

            fine_tune: Some(r4.clk_fine_tune),
        }
    }
}

//
// ─────────────────────────────────────────────
// DYNAMIC PLL FREQUENCY CONFIG (0x17 / 0x18)
// ─────────────────────────────────────────────
//

#[cfg_attr(feature = "rpc-types", derive(Serialize, Deserialize, Schema))]
#[derive(Copy, Clone, Debug, Default)]
pub struct PllFrequency {
    pub enable: Option<bool>,
    pub mdiv: Option<u16>,
    pub ndiv: Option<NDiv>,
    pub kdiv: Option<KDiv>,
}

impl PllFrequency {
    // --------------------------------------------------
    // 🌟 NEW GENERIC ENCODER
    // --------------------------------------------------

    pub fn encode(self) -> (Option<PllConfig1>, Option<PllConfig2>) {
        (self.pll_config1(), self.pll_config2())
    }

    pub fn pll_config1(self) -> Option<PllConfig1> {
        let mut reg = PllConfig1::default();
        let mut changed = false;

        if let Some(mdiv) = self.mdiv {
            reg = reg.set_mdiv_msb(mdiv);
            changed = true;
        }

        if let Some(ndiv) = self.ndiv {
            reg = reg.set_ndiv(ndiv);
            changed = true;
        }

        if let Some(kdiv) = self.kdiv {
            reg = reg.set_kdiv(kdiv);
            changed = true;
        }

        if let Some(en) = self.enable {
            reg = reg.set_pll_enable(en);
            changed = true;
        }

        if changed {
            Some(reg)
        } else {
            None
        }
    }

    pub fn pll_config2(self) -> Option<PllConfig2> {
        self.mdiv.map(PllConfig2::from)
    }

    // --------------------------------------------------
    // DEBUG
    // --------------------------------------------------

    pub fn debug_dump(&self) {
        debug!("╔══════════════════════════════════════╗");
        debug!("║          PLL DYNAMIC CONFIG          ║");
        debug!("╠══════════════════════════════════════╣");

        let (r1, r2) = self.encode();

        if let Some(r) = r2 {
            let _v = r.value();
            debug!(" PLL_CONFIG2 = 0x{:02X} | b{:08b}", _v, _v);
            debug!("   mdiv_lsb : {}", _v);
        } else {
            warn!(" PLL_CONFIG2 : <unchanged>");
        }

        if let Some(r) = r1 {
            let _v = r.bits();
            debug!(" PLL_CONFIG1 = 0x{:02X} | b{:08b}", _v, _v);

            debug!("   pll_enable : {}", (_v & 1) != 0);
            debug!("   ndiv       : {}", (_v >> 5) & 1);
            debug!("   kdiv_bits  : {}", (_v >> 1) & 0x0F);
            debug!("   mdiv_msb   : {}", (_v >> 6) & 0x03);
        } else {
            warn!(" PLL_CONFIG1 : <unchanged>");
        }

        debug!("╚══════════════════════════════════════╝");
    }
}

impl From<(PllConfig1, PllConfig2)> for PllFrequency {
    fn from((r1, r2): (PllConfig1, PllConfig2)) -> Self {
        use crate::pll::types::*;

        let bits = r1.bits();

        let mdiv = {
            let msb = ((bits >> 6) & 0x03) as u16;
            let lsb = r2.value() as u16;
            (msb << 8) | lsb
        };

        Self {
            enable: Some((bits & 1) != 0),
            mdiv: Some(mdiv),

            ndiv: Some(if (bits & (1 << 5)) != 0 {
                NDiv::Div1024
            } else {
                NDiv::Div512
            }),

            kdiv: Some(match (bits >> 1) & 0x0F {
                0x1 => KDiv::Div2,
                0x2 => KDiv::Div4,
                0x3 => KDiv::Div8,
                0x4 => KDiv::Div16,
                0x5 => KDiv::Div32,
                0x6 => KDiv::Div64,
                0x7 => KDiv::Div128,
                0x8 => KDiv::Div256,
                0x9 => KDiv::Div512,
                0xA => KDiv::Div1024,
                0xB => KDiv::Div2048,
                0xC => KDiv::Div4096,
                0xD => KDiv::Div8192,
                _ => KDiv::Div1,
            }),
        }
    }
}

impl<I: RegisterInterface> Apply<I> for PllConfig {
    fn apply(self, dev: &mut Max30009<I>) -> Result<(), I::Error> {
        self.debug_dump();

        if let Some(r) = self.pll_config3() {
            dev.write(r)?;
        }

        if let Some(r) = self.pll_config4() {
            dev.write(r)?;
        }

        Ok(())
    }
}

impl<I: RegisterInterface> Apply<I> for PllFrequency {
    fn apply(self, dev: &mut Max30009<I>) -> Result<(), I::Error> {
        self.debug_dump();

        // IMPORTANT :
        // CONFIG2 avant CONFIG1 (comme ton ancien code)
        if let Some(r) = self.pll_config2() {
            dev.write(r)?;
        }

        if let Some(r) = self.pll_config1() {
            dev.write(r)?;
        }

        Ok(())
    }
}

impl<I: RegisterInterface> Readback<I> for PllConfig {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from((dev.read_reg::<PllConfig3>()?, dev.read_reg::<PllConfig4>()?));
        cfg.debug_dump();
        Ok(cfg)
    }
}

impl<I: RegisterInterface> Readback<I> for PllFrequency {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self::from((dev.read_reg::<PllConfig1>()?, dev.read_reg::<PllConfig2>()?));
        cfg.debug_dump();
        Ok(cfg)
    }
}

impl DebugDump for PllConfig {
    fn debug_dump(&self) {
        PllConfig::debug_dump(self);
    }
}

impl DebugDump for PllFrequency {
    fn debug_dump(&self) {
        PllFrequency::debug_dump(self);
    }
}

impl Merge for PllConfig {
    fn merge(self, current: Self) -> Self {
        Self {
            lock_window_2clk: self.lock_window_2clk.or(current.lock_window_2clk),
            ref_clock: self.ref_clock.or(current.ref_clock),
            ref_freq: self.ref_freq.or(current.ref_freq),
            fine_tune: self.fine_tune.or(current.fine_tune),
        }
    }
}

impl Merge for PllFrequency {
    fn merge(self, current: Self) -> Self {
        Self {
            enable: self.enable.or(current.enable),
            mdiv: self.mdiv.or(current.mdiv),
            ndiv: self.ndiv.or(current.ndiv),
            kdiv: self.kdiv.or(current.kdiv),
        }
    }
}
