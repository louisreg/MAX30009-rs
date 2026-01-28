//! PLL configuration and control (MAX30009)
//!
//! Implements the Timing Subsystem / PLL configuration exactly as
//! described in the MAX30009 datasheet.
//!
//! Datasheet reference:
//! - Section: Timing Subsystem
//! - Registers: PLL Configuration 1–4 (0x17–0x1A)
//! - PLL output frequency range: 14 MHz – 28 MHz
//!
//! Architectural rules:
//! - Register addresses come from `registers::addresses`
//! - Bitfields come from `registers::pll`
//! - This module only orchestrates sequencing and validation

use crate::bioz::BiozSynthesis;
use crate::register_interface::RegisterInterface;
use crate::registers::addresses::Register as Reg;
use crate::registers::pll::{
    ClockFreqSel, KDiv, NDiv, PllConfig1, PllConfig2, PllConfig3, PllConfig4, RefClockSel,
};
use crate::registers::system::{SystemConfig, SystemSync};
include!(concat!(env!("OUT_DIR"), "/bioz_tables.rs"));

//
// ─────────────────────────────────────────────────────────────────────────────
// Errors
// ─────────────────────────────────────────────────────────────────────────────
//
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PllError<E> {
    /// PLL frequency outside datasheet limits
    FrequencyOutOfRange,

    /// Underlying bus / device error
    Bus(E),
}

/// Errors related to PLL synthesis from BioZ constraints
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PllSynthesisError {
    FrequencyNotAchievable,
}

/// Canonical BioZ timing solution
///
/// This structure represents **one unique BioZ stimulus frequency**
/// achievable by the MAX30009 timing subsystem.
///
/// Properties:
/// - One entry per unique `f_bioz`
/// - Sorted by `f_bioz` in generated tables
/// - Optimized for flash size and binary search
///
/// All other clocks (PLL_CLK, ADC_CLK, SR_BIOZ, etc.)
/// can be recomputed on demand.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PllBioZ {
    /// BioZ stimulus frequency in Hz
    ///
    /// Used as the lookup key (binary search).
    pub f_bioz: u32,

    /// PLL multiplier (MDIV)
    ///
    /// PLL_CLK = REF_CLK × (mdiv + 1)
    pub mdiv: u16,

    /// BioZ ADC clock divider
    ///
    /// PLL_CLK / NDIV
    pub ndiv: NDiv,

    /// DDS synthesis divider
    ///
    /// PLL_CLK / KDIV
    pub kdiv: KDiv,

    /// BioZ ADC oversampling ratio
    ///
    /// BIOZ_ADC_CLK / BIOZ_ADC_OSR = SR_BIOZ
    pub adc_osr: u16,

    /// BioZ DAC oversampling ratio
    ///
    /// BIOZ_SYNTH_CLK / BIOZ_DAC_OSR = F_BIOZ
    pub dac_osr: u16,
}

//Policy used for choosing adequate PLL configuration
#[derive(Copy, Clone)]
pub enum Policy {
    /// Favor minimal latency and datasheet-like behavior
    ///
    /// - smallest ADC_OSR
    /// - lowest MDIV
    /// - higher clocks
    MinLatency,

    /// Favor maximum SNR
    ///
    /// - largest ADC_OSR
    /// - more integration cycles
    /// - lower clocks when possible
    MaxSNR,
}

//
// ─────────────────────────────────────────────────────────────────────────────
// PLL configuration builder
// ─────────────────────────────────────────────────────────────────────────────
//
#[derive(Debug, Clone)]
pub struct PllConfig {
    ref_clk: RefClockSel,
    clk_freq: ClockFreqSel,
    pub mdiv: u16,
    pub ndiv: NDiv,
    pub kdiv: KDiv,
    lock_window_2clk: bool,
    enable: bool,
}

impl PllConfig {
    /// Create a new PLL configuration with datasheet-safe defaults
    pub fn new() -> Self {
        Self {
            ref_clk: RefClockSel::Internal,
            clk_freq: ClockFreqSel::Khz32768,
            mdiv: 426, // minimum valid MDIV for 32.768 kHz
            ndiv: NDiv::Div512,
            kdiv: KDiv::Div1,
            lock_window_2clk: false,
            enable: false,
        }
    }

    // ---------------------------------------------------------------------
    // Clock source selection
    // ---------------------------------------------------------------------

    pub fn internal_32k768(mut self) -> Self {
        self.ref_clk = RefClockSel::Internal;
        self.clk_freq = ClockFreqSel::Khz32768;
        self
    }

    pub fn internal_32k(mut self) -> Self {
        self.ref_clk = RefClockSel::Internal;
        self.clk_freq = ClockFreqSel::Khz32000;
        self
    }

    pub fn external_clock(mut self, freq: ClockFreqSel) -> Self {
        self.ref_clk = RefClockSel::External;
        self.clk_freq = freq;
        self
    }

    // ---------------------------------------------------------------------
    // Divider configuration
    // ---------------------------------------------------------------------

    pub fn mdiv(mut self, mdiv: u16) -> Self {
        self.mdiv = mdiv;
        self
    }

    pub fn ndiv(mut self, ndiv: NDiv) -> Self {
        self.ndiv = ndiv;
        self
    }

    pub fn kdiv(mut self, kdiv: KDiv) -> Self {
        self.kdiv = kdiv;
        self
    }

    // ---------------------------------------------------------------------
    // Lock detection
    // ---------------------------------------------------------------------

    pub fn lock_window_2clk(mut self) -> Self {
        self.lock_window_2clk = true;
        self
    }

    pub fn enable(mut self) -> Self {
        self.enable = true;
        self
    }

    // ---------------------------------------------------------------------
    // Validation
    // ---------------------------------------------------------------------

    /// Compute PLL output frequency in Hz
    ///
    /// PLL_CLK = REF_CLK × (MDIV + 1)
    pub fn pll_clk_hz(&self) -> Result<u32, &'static str> {
        let ref_clk = self.clk_freq.hz();
        let pll_clk = ref_clk * (self.mdiv as u32 + 1);

        if !(14_000_000..=28_000_000).contains(&pll_clk) {
            return Err("PLL_CLK out of datasheet range (14–28 MHz)");
        }

        Ok(pll_clk)
    }

    /// Derive PLL configuration from the *nearest achievable*
    /// BioZ stimulus frequency.
    ///
    /// - Uses precomputed build-time tables
    /// - Selects table based on REF_CLK and policy
    /// - Uses binary search (O(log N))
    /// - Updates PLL dividers (MDIV / NDIV / KDIV)
    /// - Returns BioZ OSR parameters and actual frequency
    /// - Does **NOT** touch hardware
    pub fn from_f_bioz_nearest(
        &mut self,
        f_target: u32,
        policy: Policy,
    ) -> Result<BiozSynthesis, PllSynthesisError> {
        // -------------------------------------------------------------
        // Select the appropriate pre-generated table
        // -------------------------------------------------------------
        let table: &[PllBioZ] = match (self.clk_freq, policy) {
            (ClockFreqSel::Khz32768, Policy::MinLatency) => BIOZ_TABLE_32K768_MIN_LATENCY,
            (ClockFreqSel::Khz32768, Policy::MaxSNR) => BIOZ_TABLE_32K768_MAX_SNR,
            (ClockFreqSel::Khz32000, Policy::MinLatency) => BIOZ_TABLE_32K_MIN_LATENCY,
            (ClockFreqSel::Khz32000, Policy::MaxSNR) => BIOZ_TABLE_32K_MAX_SNR,
        };

        if table.is_empty() {
            return Err(PllSynthesisError::FrequencyNotAchievable);
        }

        // -------------------------------------------------------------
        // Binary search on sorted f_bioz
        // -------------------------------------------------------------
        let idx = match table.binary_search_by(|e| e.f_bioz.cmp(&f_target)) {
            Ok(i) => i, // exact match
            Err(i) => {
                // i is insertion point → check nearest neighbor(s)
                if i == 0 {
                    0
                } else if i >= table.len() {
                    table.len() - 1
                } else {
                    let prev = &table[i - 1];
                    let next = &table[i];

                    let err_prev = f_target.abs_diff(prev.f_bioz);
                    let err_next = f_target.abs_diff(next.f_bioz);

                    if err_prev <= err_next {
                        i - 1
                    } else {
                        i
                    }
                }
            }
        };

        let entry = &table[idx];

        // -------------------------------------------------------------
        // Apply PLL configuration (pure logic)
        // -------------------------------------------------------------
        self.mdiv = entry.mdiv;
        self.ndiv = entry.ndiv;
        self.kdiv = entry.kdiv;

        Ok(BiozSynthesis {
            f_bioz_actual: entry.f_bioz,
            adc_osr: entry.adc_osr,
            dac_osr: entry.dac_osr,
        })
    }
    // ---------------------------------------------------------------------
    // Hardware application
    // ---------------------------------------------------------------------

    pub fn apply<I: RegisterInterface>(&self, iface: &mut I) -> Result<(), PllError<I::Error>> {
        self.pll_clk_hz()
            .map_err(|_| PllError::FrequencyOutOfRange)?;

        let cfg1 = PllConfig1::empty()
            .set_mdiv_msb(self.mdiv)
            .set_ndiv(self.ndiv)
            .set_kdiv(self.kdiv)
            .set_pll_enable(self.enable);

        let cfg2 = PllConfig2::from_mdiv(self.mdiv);

        let cfg3 = if self.lock_window_2clk {
            PllConfig3::lock_window_2clk()
        } else {
            PllConfig3::lock_window_1clk()
        };

        let cfg4 = PllConfig4 {
            ref_clk_sel: matches!(self.ref_clk, RefClockSel::External),
            clk_freq_sel: matches!(self.clk_freq, ClockFreqSel::Khz32768),
            clk_fine_tune: 0,
        };

        iface
            .write_reg(Reg::PllConfig1 as u8, cfg1.bits())
            .map_err(PllError::Bus)?;
        iface
            .write_reg(Reg::PllConfig2 as u8, cfg2.into())
            .map_err(PllError::Bus)?;
        iface
            .write_reg(Reg::PllConfig3 as u8, cfg3.bits())
            .map_err(PllError::Bus)?;
        iface
            .write_reg(Reg::PllConfig4 as u8, cfg4.into())
            .map_err(PllError::Bus)?;

        iface.delay_ms(6);

        if self.enable {
            iface.wait_for_pll_lock().map_err(PllError::Bus)?;
        }

        Ok(())
    }
}

//
// ─────────────────────────────────────────────────────────────────────────────
// PLL Synchronization
// ─────────────────────────────────────────────────────────────────────────────
//

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PllSyncMethod {
    Trig,
    Broadcast,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PllSyncRole {
    Controller,
    Target,
}

pub struct PllSync {
    role: PllSyncRole,
    method: PllSyncMethod,
}

impl PllSync {
    pub fn trig_controller() -> Self {
        Self {
            role: PllSyncRole::Controller,
            method: PllSyncMethod::Trig,
        }
    }

    pub fn trig_target() -> Self {
        Self {
            role: PllSyncRole::Target,
            method: PllSyncMethod::Trig,
        }
    }

    pub fn broadcast() -> Self {
        Self {
            role: PllSyncRole::Controller,
            method: PllSyncMethod::Broadcast,
        }
    }

    pub fn synchronize<I: RegisterInterface>(&self, iface: &mut I) -> Result<(), I::Error> {
        match self.method {
            PllSyncMethod::Trig => match self.role {
                PllSyncRole::Controller => {
                    iface.write_reg(Reg::SystemConfig as u8, SystemConfig::MASTER.bits())?;
                    iface.write_reg(Reg::SystemSync as u8, SystemSync::timing_reset().bits())?;
                }
                PllSyncRole::Target => {
                    iface.write_reg(Reg::SystemConfig as u8, SystemConfig::empty().bits())?;
                }
            },
            PllSyncMethod::Broadcast => {
                iface.write_reg(Reg::SystemSync as u8, SystemSync::timing_reset().bits())?;
            }
        }
        Ok(())
    }
}

//
// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────
//

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use crate::mock_device::MockDevice;
    use crate::registers::addresses::Register;
    use crate::registers::pll::{KDiv, NDiv};

    /// Valid PLL configuration:
    /// - correct register order
    /// - delay applied
    /// - waits for lock
    #[test]
    fn pll_valid_configuration_writes_registers_in_order() {
        let mut iface = MockDevice::default();

        let pll = PllConfig::new()
            .internal_32k768()
            .mdiv(439)
            .ndiv(NDiv::Div512)
            .kdiv(KDiv::Div64)
            .lock_window_2clk()
            .enable();

        pll.apply(&mut iface).unwrap();

        assert_eq!(iface.writes.len(), 4);
        assert_eq!(iface.writes[0].0, Register::PllConfig1 as u8);
        assert_eq!(iface.writes[1].0, Register::PllConfig2 as u8);
        assert_eq!(iface.writes[2].0, Register::PllConfig3 as u8);
        assert_eq!(iface.writes[3].0, Register::PllConfig4 as u8);

        assert!(iface.delays_ms.contains(&6));
        assert!(iface.waited_for_pll_lock);
    }

    /// PLL frequency computation follows datasheet formula
    #[test]
    fn pll_frequency_computation_is_correct() {
        let pll = PllConfig::new().internal_32k768().mdiv(427);

        let freq = pll.pll_clk_hz().unwrap();
        assert_eq!(freq, 32_768 * (427 + 1));
    }

    #[test]
    fn pll_rejects_too_low_frequency() {
        let pll = PllConfig::new().internal_32k768().mdiv(100);
        assert!(pll.pll_clk_hz().is_err());
    }

    #[test]
    fn pll_rejects_too_high_frequency() {
        let pll = PllConfig::new().internal_32k768().mdiv(2000);
        assert!(pll.pll_clk_hz().is_err());
    }

    #[test]
    fn pll_disabled_does_not_wait_for_lock() {
        let mut iface = MockDevice::default();

        let pll = PllConfig::new().internal_32k().mdiv(437);
        pll.apply(&mut iface).unwrap();

        assert!(!iface.waited_for_pll_lock);
    }

    #[test]
    fn pll_external_clock_is_accepted() {
        let pll = PllConfig::new()
            .external_clock(ClockFreqSel::Khz32768)
            .mdiv(500);

        assert!(pll.pll_clk_hz().is_ok());
    }

    #[test]
    fn pll_sync_trig_controller_writes_master_and_reset() {
        let mut iface = MockDevice::default();
        let sync = PllSync::trig_controller();

        sync.synchronize(&mut iface).unwrap();

        assert_eq!(iface.writes.len(), 2);
        assert_eq!(iface.writes[0].0, Register::SystemConfig as u8);
        assert_eq!(iface.writes[1].0, Register::SystemSync as u8);
    }

    #[test]
    fn pll_sync_trig_target_only_sets_master_low() {
        let mut iface = MockDevice::default();
        let sync = PllSync::trig_target();

        sync.synchronize(&mut iface).unwrap();

        assert_eq!(iface.writes.len(), 1);
        assert_eq!(iface.writes[0].0, Register::SystemConfig as u8);
    }

    #[test]
    fn pll_sync_broadcast_only_writes_system_sync() {
        let mut iface = MockDevice::default();
        let sync = PllSync::broadcast();

        sync.synchronize(&mut iface).unwrap();

        assert_eq!(iface.writes.len(), 1);
        assert_eq!(iface.writes[0].0, Register::SystemSync as u8);
    }

    #[test]
    fn pll_from_f_bioz_produces_valid_pll() {
        let mut pll = PllConfig::new().internal_32k768().enable();

        pll.from_f_bioz_nearest(10_000, Policy::MaxSNR).unwrap();
        assert!(pll.pll_clk_hz().is_ok());
    }
}
