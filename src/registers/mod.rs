//! MAX30009 Register Map
//!
//! This module contains a structured, strongly-typed representation
//! of the MAX30009 user register map.
//!
//! Registers are grouped by functional blocks, following the datasheet:
//!
//! - Status
//! - FIFO
//! - System Control
//! - PLL
//! - BioZ Setup & Calibration
//! - DC Leads & Lead Bias
//! - Interrupt Enables
//! - Part Identification
//!
//! All register addresses and bit definitions are derived from the
//! MAX30009 datasheet and reference headers provided by Analog Devices.
//!
//! ⚠️ All status registers are **read-to-clear**.

pub mod addresses;

pub mod status;

pub mod fifo;
pub mod pll;
pub mod system;

pub mod bioz;

pub mod leads;

pub mod interrupt;
pub mod part_id;
