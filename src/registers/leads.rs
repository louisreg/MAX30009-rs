//! DC Lead Detection & Bias Configuration (MAX30009)
//!
//! This module groups all **DC lead-on / lead-off detection** logic
//! and **input biasing** related registers.
//!
//! ⚠️ IMPORTANT DATASHEET NOTES
//! ----------------------------
//! - Ultra-Low-Power Lead-On detection (`EN_LON_DET`) only works when
//!   **BioZ is disabled** (BIOZ_I_EN = BIOZ_Q_EN = 0)
//! - DC Lead-Off detection (`EN_LOFF_DET`) only works when
//!   **BioZ is enabled**
//! - These modes are mutually exclusive in practice

// ─────────────────────────────────────────────────────────────────────────────
// DC Leads Configuration Register (0x50)
// ─────────────────────────────────────────────────────────────────────────────

/// DC Leads Configuration Register
///
/// Address: `0x50`
///
/// Controls:
/// - Ultra-low-power lead-on detection
/// - DC lead-off detection
/// - Drive electrode out-of-range detection
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DcLeadsConfig {
    /// EN_LON_DET – Ultra-Low-Power Lead-On Detect Enable
    ///
    /// ⚠️ Only functional when:
    /// - BIOZ_I_EN = 0
    /// - BIOZ_Q_EN = 0
    pub lon_det_en: bool,

    /// EN_LOFF_DET – DC Lead-Off Detect Enable
    ///
    /// ⚠️ Requires BioZ to be enabled
    pub loff_det_en: bool,

    /// EN_EXT_LOFF – External Lead-Off Detection Enable
    ///
    /// Applies DC lead-off detection via EL2B / EL3B
    /// when AC-coupled BioZ inputs are used.
    pub ext_loff_en: bool,

    /// EN_DRV_OOR – Drive Electrode Out-of-Range Detect Enable
    ///
    /// Detects disconnected or high-impedance DRVP / DRVN electrodes.
    pub drv_oor_en: bool,

    /// LOFF_IPOL – DC Lead-Off Current Polarity
    ///
    /// | Value | Polarity |
    /// |------|----------|
    /// | `0` | BIP sources current, BIN sinks |
    /// | `1` | BIP sinks current, BIN sources |
    pub loff_ipol: bool,

    /// LOFF_IMAG[2:0] – DC Lead-Off Current Magnitude
    ///
    /// | Value | Current |
    /// |------|---------|
    /// | 0x0 | Disabled |
    /// | 0x1 | 50 nA |
    /// | 0x2 | 100 nA |
    /// | 0x3 | 200 nA |
    /// | 0x4 | 500 nA |
    /// | 0x5 | 1 µA |
    pub loff_imag: u8,
}

// ─────────────────────────────────────────────────────────────────────────────
// DC Lead Detect Threshold Register (0x51)
// ─────────────────────────────────────────────────────────────────────────────

/// DC Lead-Off Threshold Configuration
///
/// Address: `0x51`
///
/// Selects the voltage threshold for DC lead-off window comparators.
/// Thresholds are centered around `VMID_RX`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DcLeadThreshold {
    /// LOFF_THRESH[3:0] – DC Lead-Off Threshold Selection
    pub threshold: u8,
}

// ─────────────────────────────────────────────────────────────────────────────
// Lead Bias Configuration Register (0x58)
// ─────────────────────────────────────────────────────────────────────────────

/// Input Lead Bias Configuration
///
/// Address: `0x58`
///
/// Controls resistive biasing of BioZ input leads (BIP / BIN)
/// to `VMID_RX`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LeadBiasConfig {
    /// RBIAS_VALUE[1:0] – Bias Resistance Selection
    ///
    /// | Value | Resistance |
    /// |------|------------|
    /// | 0x0 | 500 MΩ |
    /// | 0x1 | 1 GΩ |
    /// | 0x2 | 2 GΩ |
    pub rbias_value: u8,

    /// EN_RBIAS_BIP – Enable Resistive Bias on BIP
    pub bias_bip_en: bool,

    /// EN_RBIAS_BIN – Enable Resistive Bias on BIN
    pub bias_bin_en: bool,
}
