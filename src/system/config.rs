//! System configuration builder + debug helpers

use crate::{debug, warn};

use crate::system::registers::*;
use crate::system::types::*;

use crate::device::Max30009;
use crate::register_interface::RegisterInterface;
use crate::traits::{DebugDump, Merge, Readback};

// ─────────────────────────────────────────
// High-level System Update object
// ─────────────────────────────────────────

#[derive(Copy, Clone, Debug, Default)]
pub struct SystemUpdate {
    pub sys: Option<SystemConfig>,
    pub pin_func: Option<PinFuncConfig>,
    pub pin_output: Option<OutputPinConfig>,
    pub i2c: Option<I2cBroadcast>,
}

// ─────────────────────────────────────────
// High level settings → register mapping
// ─────────────────────────────────────────

impl From<SystemSettings> for SystemConfig {
    fn from(s: SystemSettings) -> Self {
        let mut reg = SystemConfig::empty();

        if s.master {
            reg |= SystemConfig::MASTER;
        }
        if s.spi_only {
            reg |= SystemConfig::DISABLE_I2C;
        }

        reg
    }
}

// ─────────────────────────────────────────
// GENERIC APPLY (configure support)
// ─────────────────────────────────────────

impl<I: RegisterInterface> crate::traits::Apply<I> for SystemUpdate {
    fn apply(self, dev: &mut Max30009<I>) -> Result<(), I::Error> {
        self.debug_dump();

        if let Some(x) = self.sys {
            dev.write(x)?;
        }

        if let Some(x) = self.pin_func {
            dev.write(x)?;
        }

        if let Some(x) = self.pin_output {
            dev.write(x)?;
        }

        if let Some(x) = self.i2c {
            dev.write(x)?;
        }

        Ok(())
    }
}

impl<I: RegisterInterface> Readback<I> for SystemUpdate {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error> {
        let cfg = Self {
            sys: Some(dev.read_reg::<SystemConfig>()?),
            pin_func: Some(dev.read_reg::<PinFuncConfig>()?),
            pin_output: Some(dev.read_reg::<OutputPinConfig>()?),
            i2c: Some(dev.read_reg::<I2cBroadcast>()?),
        };

        cfg.debug_dump();
        Ok(cfg)
    }
}

// ─────────────────────────────────────────
// 🌟 PRETTY DEBUG LOGGER
// ─────────────────────────────────────────

impl SystemUpdate {
    /// Ultra readable system configuration dump.
    ///
    /// Designed for:
    /// - bring-up
    /// - register validation
    /// - SPI trace correlation
    pub fn debug_dump(&self) {
        #[inline]
        fn raw_fmt(v: u8) -> (u8, u8) {
            (v, v)
        }

        debug!("╔══════════════════════════════════════╗");
        debug!("║        MAX30009 SYSTEM CONFIG        ║");
        debug!("╠══════════════════════════════════════╣");

        // ───────── SYSTEM CONFIG ─────────
        if let Some(sys) = self.sys {
            let (_hex, _bin) = raw_fmt(sys.bits());

            debug!(" SYSTEM_CONFIG   = 0x{:02X} | b{:08b}", _hex, _bin);

            debug!("   master        : {}", sys.contains(SystemConfig::MASTER));
            debug!(
                "   spi_only      : {}",
                sys.contains(SystemConfig::DISABLE_I2C)
            );
            debug!("   shutdown      : {}", sys.contains(SystemConfig::SHDN));
            debug!("   reset         : {}", sys.contains(SystemConfig::RESET));
        } else {
            warn!(" SYSTEM_CONFIG   : <unchanged>");
        }

        // ───────── PIN FUNC ─────────
        if let Some(p) = self.pin_func {
            let v = p.encode();
            let (_hex, _bin) = raw_fmt(v);

            debug!(" PIN_FUNC_CONFIG = 0x{:02X} | b{:08b}", _hex, _bin);

            debug!("   int_mode      : {:?}", p.int_fcfg);
            debug!("   trig_edge     : {:?}", p.trig_edge);
        } else {
            warn!(" PIN_FUNC_CONFIG : <unchanged>");
        }

        // ───────── OUTPUT PIN ─────────
        if let Some(p) = self.pin_output {
            let v = p.encode();
            let (_hex, _bin) = raw_fmt(v);

            debug!(" OUTPUT_CONFIG   = 0x{:02X} | b{:08b}", _hex, _bin);

            debug!("   int_drive     : {:?}", p.int_drive);
            debug!("   trig_drive    : {:?}", p.trig_drive);
        } else {
            warn!(" OUTPUT_CONFIG   : <unchanged>");
        }

        // ───────── I2C BROADCAST ─────────
        if let Some(i2c) = self.i2c {
            let v = i2c.encode();
            let (_hex, _bin) = raw_fmt(v);

            debug!(" I2C_BROADCAST   = 0x{:02X} | b{:08b}", _hex, _bin);

            debug!("   address       : {}", i2c.address);
            debug!("   enable        : {}", i2c.enable);
        } else {
            warn!(" I2C_BROADCAST   : <unchanged>");
        }

        debug!("╚══════════════════════════════════════╝");
    }
}

impl DebugDump for SystemUpdate {
    fn debug_dump(&self) {
        SystemUpdate::debug_dump(self);
    }
}

impl Merge for SystemUpdate {
    fn merge(self, current: Self) -> Self {
        Self {
            sys: self.sys.or(current.sys),
            pin_func: self.pin_func.or(current.pin_func),
            pin_output: self.pin_output.or(current.pin_output),
            i2c: self.i2c.or(current.i2c),
        }
    }
}
