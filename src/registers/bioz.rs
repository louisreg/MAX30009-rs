//! BioZ subsystem – MAX30009
//!
//! This file groups **all BioZ-related registers** in a single place:
//! - Configuration 1–7
//! - Thresholds
//! - MUX & calibration
//!
//! Datasheet sections:
//! - BioZ Setup
//! - BioZ Calibration
//! - AC/DC Lead-Off Detection

// =============================================================================
// BioZ Configuration 1 (0x20)
// =============================================================================

/// BioZ Configuration 1
///
/// Address: `0x20`
///
/// Controls DAC/ADC oversampling and enables BioZ subsystems.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BiozConfig1 {
    /// BIOZ_DAC_OSR[1:0] – DAC Oversampling Ratio
    ///
    /// | Value | OSR |
    /// |------:|----:|
    /// | 0x0   | 32  |
    /// | 0x1   | 64  |
    /// | 0x2   | 128 |
    /// | 0x3   | 256 |
    pub dac_osr: u8,

    /// BIOZ_ADC_OSR[2:0] – ADC Oversampling Ratio
    ///
    /// | Value | OSR  |
    /// |------:|-----:|
    /// | 0x0   | 8    |
    /// | 0x1   | 16   |
    /// | 0x2   | 32   |
    /// | 0x3   | 64   |
    /// | 0x4   | 128  |
    /// | 0x5   | 256  |
    /// | 0x6   | 512  |
    /// | 0x7   | 1024 |
    pub adc_osr: u8,

    /// BIOZ_BG_EN – Bandgap Enable
    ///
    /// Enables the BioZ bandgap bias.
    ///
    /// ⚠️ Power-up time ≈ 200 ms.
    /// Should be kept ON between measurements.
    pub bg_enable: bool,

    /// BIOZ_Q_EN – Enable Q-channel
    ///
    /// Enables quadrature-phase BioZ measurement.
    ///
    /// Notes:
    /// - Automatically enables PLL and bandgap
    /// - Recommended to enable PLL and BG explicitly first
    pub q_enable: bool,

    /// BIOZ_I_EN – Enable I-channel
    ///
    /// Enables in-phase BioZ measurement.
    ///
    /// Notes:
    /// - Automatically enables PLL and bandgap
    /// - Recommended to enable PLL and BG explicitly first
    pub i_enable: bool,
}

// =============================================================================
// BioZ Configuration 2 (0x21)
// =============================================================================

/// BioZ Configuration 2
///
/// Controls digital filters and threshold comparison source.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BiozConfig2 {
    /// BIOZ_DHPF[1:0] – Digital High-Pass Filter
    ///
    /// | Value | Cutoff |
    /// |------:|--------|
    /// | 0x0   | Bypass |
    /// | 0x1   | 0.00025 × SR_BIOZ |
    /// | 0x2   | 0.002 × SR_BIOZ |
    /// | 0x3   | 0.002 × SR_BIOZ |
    pub dhpf: u8,

    /// BIOZ_DLPF[2:0] – Digital Low-Pass Filter
    ///
    /// | Value | Cutoff |
    /// |------:|--------|
    /// | 0x0   | Bypass |
    /// | 0x1   | 0.005 × SR_BIOZ |
    /// | 0x2   | 0.02 × SR_BIOZ |
    /// | 0x3   | 0.08 × SR_BIOZ |
    /// | 0x4–7 | 0.25 × SR_BIOZ |
    pub dlpf: u8,

    /// BIOZ_CMP[1:0] – Threshold Compare Source
    ///
    /// | Value | Component |
    /// |------:|-----------|
    /// | 0x0   | |I| |
    /// | 0x1   | |Q| |
    /// | 0x2   | |Z| = sqrt(I² + Q²) |
    /// | 0x3   | Reserved |
    pub cmp_sel: u8,

    /// EN_BIOZ_THRESH – Enable AC Lead-Off Detection
    ///
    /// Enables comparison against BIOZ_LO_THRESH / BIOZ_HI_THRESH.
    pub thresh_enable: bool,
}

// =============================================================================
// BioZ Configuration 3 (0x22)
// =============================================================================

/// BioZ Configuration 3
///
/// Controls stimulus type and amplitude.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BiozConfig3 {
    /// BIOZ_EXT_RES – External Resistor Select
    ///
    /// | Value | Mode |
    /// |------:|------|
    /// | 0 | Internal range resistors |
    /// | 1 | External REXT resistor |
    pub ext_res: bool,

    /// LOFF_RAPID – DC Lead-Off Immediate Detection
    ///
    /// | 0 | ~128 ms qualification |
    /// | 1 | Immediate detection |
    pub loff_rapid: bool,

    /// BIOZ_VDRV_MAG[1:0] – Drive Voltage Magnitude
    ///
    /// Used in voltage drive or combined with IDRV_RGE in current drive.
    pub vdrv_mag: u8,

    /// BIOZ_IDRV_RGE[1:0] – Current Drive Range
    ///
    /// Selects internal current range resistor.
    pub idrv_rge: u8,

    /// BIOZ_DRV_MODE[1:0] – Drive Mode
    ///
    /// | Value | Mode |
    /// |------:|------|
    /// | 0x0 | Current drive |
    /// | 0x1 | Voltage drive |
    /// | 0x2 | H-bridge drive |
    /// | 0x3 | Standby |
    pub drv_mode: u8,
}

// =============================================================================
// BioZ Configuration 4 (0x23)
// =============================================================================

/// BioZ Fast-Start Control
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BiozConfig4 {
    /// BIOZ_FAST_MANUAL – Manual Fast Start Hold
    pub fast_manual: bool,

    /// BIOZ_FAST_START_EN – Enable Fast Start
    ///
    /// See datasheet for the 3 operating modes.
    pub fast_start_en: bool,
}

// =============================================================================
// BioZ Configuration 5 (0x24)
// =============================================================================

/// BioZ Receive Channel Configuration
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BiozConfig5 {
    /// BIOZ_AHPF[3:0] – Analog High-Pass Filter / Resistance
    pub ahpf: u8,

    /// BIOZ_INA_MODE – INA Power Mode
    ///
    /// | 0 | High power / low noise |
    /// | 1 | Low power |
    pub ina_low_power: bool,

    /// BIOZ_DM_DIS – Demodulator Disable
    pub demod_disable: bool,

    /// BIOZ_GAIN[1:0] – Total Gain
    ///
    /// | Value | Gain |
    /// |------:|------|
    /// | 0 | 1× |
    /// | 1 | 2× |
    /// | 2 | 5× |
    /// | 3 | 10× |
    pub gain: u8,
}

// =============================================================================
// BioZ Configuration 6 (0x25)
// =============================================================================

/// BioZ Transmit Amplifier Control
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BiozConfig6 {
    /// BIOZ_EXT_CAP – External AC Coupling Capacitor
    pub ext_cap: bool,

    /// BIOZ_DC_RESTORE – DC Restore Enable
    pub dc_restore: bool,

    /// BIOZ_DRV_RESET – Drive Amplifier Reset
    pub drv_reset: bool,

    /// BIOZ_DAC_RESET – DDS DAC Reset
    pub dac_reset: bool,

    /// BIOZ_AMP_RGE[1:0] – Amplifier Strength
    pub amp_rge: u8,

    /// BIOZ_AMP_BW[1:0] – Amplifier Bandwidth
    pub amp_bw: u8,
}

// =============================================================================
// BioZ Thresholds (0x26, 0x27)
// =============================================================================

/// BioZ Threshold Registers
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BiozThresholds {
    /// BIOZ_LO_THRESH (0x26)
    pub low: u8,

    /// BIOZ_HI_THRESH (0x27)
    pub high: u8,
}

// =============================================================================
// BioZ Configuration 7 (0x28)
// =============================================================================

/// BioZ Advanced Receive Channel Control
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BiozConfig7 {
    /// BIOZ_STBYON – RX behavior in standby
    pub standby_rx_on: bool,

    /// BIOZ_Q_CLK_PHASE – Q demodulator phase
    pub q_clk_phase: bool,

    /// BIOZ_I_CLK_PHASE – I demodulator phase
    pub i_clk_phase: bool,

    /// BIOZ_INA_CHOP_EN – INA chopping enable
    pub ina_chop: bool,

    /// BIOZ_CH_FSEL – PGA chopping frequency select
    pub ch_fsel: bool,
}

// =============================================================================
// BioZ MUX & Calibration (0x41 – 0x44)
// =============================================================================

/// BioZ MUX Configuration 1 (0x41)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BiozMuxConfig1 {
    pub rsel: u8,
    pub bist_en: bool,
    pub connect_cal_only: bool,
    pub mux_en: bool,
    pub cal_en: bool,
}

/// BioZ MUX Configuration 2 (0x42)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BiozMuxConfig2 {
    pub gsr_rsel: u8,
    pub gsr_load_en: bool,
    pub en_ext_inload: bool,
    pub en_int_inload: bool,
}

/// BioZ MUX Configuration 3 (0x43)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BiozMuxConfig3 {
    pub bip_assign: u8,
    pub bin_assign: u8,
    pub drvp_assign: u8,
    pub drvn_assign: u8,
}

/// BioZ MUX Configuration 4 (0x44)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BiozMuxConfig4 {
    /// Factory-measured resistor error (2’s complement)
    pub bist_r_err: i8,
}
