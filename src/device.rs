use crate::bioz::config::{BiozConfig1, BiozConfig3, BiozConfig7};
use crate::bioz::types::{BiozDrvMode, BiozGain};
use crate::debug;
use crate::fifo::registers::RegFifoCfg2;
use crate::pll::registers::{PllConfig1, PllConfig4};
use crate::register_interface::RegisterInterface;
use crate::register_map::Register;
use crate::status::registers::RegStatus1;
use crate::system::registers::{SystemConfig, SystemSync};
use crate::traits::{Apply, DebugDump, Readback, Update};
use crate::traits::{ReadableRegister, WritableRegister};
use embedded_hal::delay::DelayNs;

pub struct Max30009<I: RegisterInterface> {
    iface: I,
}

impl<I: RegisterInterface> Max30009<I> {
    /// Create a new MAX30009 driver instance.
    ///
    /// The driver takes ownership of the register interface.
    /// This keeps the hardware layer fully abstracted from
    /// the application.
    pub fn new(iface: I) -> Self {
        Self { iface }
    }

    /// Read silicon PART_ID register.
    pub fn part_id(&mut self) -> Result<u8, I::Error> {
        self.iface.read_reg(Register::PartId as u8)
    }

    /// Apply a complete typed configuration payload.
    ///
    /// This is a direct write path: fields not present in `cfg` may remain
    /// unchanged only if the config type internally models sparse writes.
    pub fn configure<C>(&mut self, cfg: C) -> Result<(), I::Error>
    where
        C: Apply<I> + DebugDump,
    {
        cfg.apply(self)
    }

    /// Read a typed configuration/status payload from the device.
    pub fn read<C>(&mut self) -> Result<C, I::Error>
    where
        C: Readback<I> + DebugDump,
    {
        C::read_from(self)
    }

    /// Update one or multiple bits via read-modify-write semantics.
    ///
    /// This is the recommended API for sparse updates against live register
    /// state to avoid unintentionally clearing unrelated bits.
    pub fn update<C>(&mut self, cfg: C) -> Result<(), I::Error>
    where
        C: Update<I>,
    {
        cfg.update(self)
    }

    /// Refresh status registers and return true when FREQ_LOCK is set.
    pub fn is_freq_locked(&mut self) -> Result<bool, I::Error> {
        let reg = self.read_reg::<RegStatus1>()?;
        Ok(reg.contains(RegStatus1::FREQ_LOCK))
    }

    /// Refresh status registers and return true when PLL (PHASE_LOCK) is set.
    pub fn is_pll_locked(&mut self) -> Result<bool, I::Error> {
        let reg = self.read_reg::<RegStatus1>()?;
        Ok(reg.contains(RegStatus1::PHASE_LOCK))
    }

    /// Trigger timing system reset sync pulse.
    pub fn sync(&mut self) -> Result<(), I::Error> {
        self.write(SystemSync::TIMING_SYS_RESET)
    }

    /// Set FIFO_MARK bit in FIFO_CFG2 using read-modify-write.
    pub fn set_fifo_mark(&mut self) -> Result<(), I::Error> {
        let mut reg = self.read_reg::<RegFifoCfg2>()?;
        reg.insert(RegFifoCfg2::FIFO_MARK);
        self.write(reg)
    }

    /// Set FLUSH_FIFO bit in FIFO_CFG2 using read-modify-write.
    pub fn flush_fifo(&mut self) -> Result<(), I::Error> {
        let mut reg = self.read_reg::<RegFifoCfg2>()?;
        reg.insert(RegFifoCfg2::FLUSH_FIFO);
        self.write(reg)
    }

    /// Set FIFO_STAT_CLR bit in FIFO_CFG2 using read-modify-write.
    pub fn clear_fifo_status(&mut self) -> Result<(), I::Error> {
        let mut reg = self.read_reg::<RegFifoCfg2>()?;
        reg.insert(RegFifoCfg2::FIFO_STAT_CLR);
        self.write(reg)
    }

    /// Enable or disable PLL by updating PLL_EN in PLL_CONFIG1.
    pub fn set_pll_enabled(&mut self, enabled: bool) -> Result<(), I::Error> {
        let mut reg = self.read_reg::<PllConfig1>()?;
        reg.set(PllConfig1::PLL_EN, enabled);
        self.write(reg)
    }

    /// Wait until either FREQ_LOCK or PHASE_LOCK is asserted.
    pub fn wait_for_pll_lock<D: DelayNs>(
        &mut self,
        delay: &mut D,
        poll_interval_ms: u32,
        max_attempts: u32,
    ) -> Result<bool, I::Error> {
        for _ in 0..max_attempts {
            let status1 = self.read_reg::<RegStatus1>()?;
            if status1.intersects(RegStatus1::FREQ_LOCK | RegStatus1::PHASE_LOCK) {
                return Ok(true);
            }
            if poll_interval_ms > 0 {
                delay.delay_ms(poll_interval_ms);
            }
        }

        Ok(false)
    }

    /// Datasheet sequence: disable BioZ, enable PLL, then wait for lock.
    pub fn enable_pll_and_wait_lock<D: DelayNs>(
        &mut self,
        delay: &mut D,
        poll_interval_ms: u32,
        max_attempts: u32,
    ) -> Result<bool, I::Error> {
        self.bioz_shutdown()?;
        self.set_pll_enabled(true)?;
        self.wait_for_pll_lock(delay, poll_interval_ms, max_attempts)
    }

    /// Datasheet sequence for disabling PLL.
    pub fn disable_pll_sequence(&mut self) -> Result<(), I::Error> {
        self.bioz_shutdown()?;
        self.set_pll_enabled(false)
    }

    /// Enter shutdown sequence: disable BioZ, disable PLL, set SHDN=1.
    pub fn enter_shutdown_sequence(&mut self) -> Result<(), I::Error> {
        self.bioz_shutdown()?;
        self.set_pll_enabled(false)?;

        let mut sys = self.read_reg::<SystemConfig>()?;
        sys.set(SystemConfig::SHDN, true);
        self.write(sys)
    }

    /// Exit shutdown sequence: set SHDN=0, then enable PLL.
    pub fn exit_shutdown_sequence(&mut self) -> Result<(), I::Error> {
        let mut sys = self.read_reg::<SystemConfig>()?;
        sys.set(SystemConfig::SHDN, false);
        self.write(sys)?;
        self.set_pll_enabled(true)
    }

    /// Soft-reset sequence from datasheet.
    pub fn soft_reset_sequence<D: DelayNs>(&mut self, delay: &mut D) -> Result<(), I::Error> {
        self.update(BiozConfig1 {
            bg_en: Some(true),
            ..Default::default()
        })?;

        let mut sys = self.read_reg::<SystemConfig>()?;
        sys.set(SystemConfig::SHDN, false);
        self.write(sys)?;

        let mut pll_cfg4 = self.read_reg::<PllConfig4>()?;
        pll_cfg4.ref_clk_sel = false; // REF_CLK_SEL = 0 (internal)
        self.write(pll_cfg4)?;

        self.set_pll_enabled(false)?;

        delay.delay_ms(1);

        let mut sys = self.read_reg::<SystemConfig>()?;
        sys.set(SystemConfig::RESET, true);
        self.write(sys)?;

        self.set_pll_enabled(true)
    }

    /// Power-up BioZ path, wait bandgap startup, then enable I/Q as requested.
    pub fn bioz_startup<D: DelayNs>(
        &mut self,
        delay: &mut D,
        enable_i: bool,
        enable_q: bool,
    ) -> Result<(), I::Error> {
        self.update(BiozConfig1 {
            bg_en: Some(true),
            ..Default::default()
        })?;

        delay.delay_ms(200);

        self.update(BiozConfig1 {
            i_en: Some(enable_i),
            q_en: Some(enable_q),
            ..Default::default()
        })
    }

    /// Disable BioZ subsystem (BG, I, Q).
    pub fn bioz_shutdown(&mut self) -> Result<(), I::Error> {
        self.update(BiozConfig1 {
            bg_en: Some(false),
            i_en: Some(false),
            q_en: Some(false),
            ..Default::default()
        })
    }

    /// Put BioZ TX in standby mode and optionally keep RX active (BIOZ_STBYON).
    pub fn bioz_standby(&mut self, keep_rx_active: bool) -> Result<(), I::Error> {
        self.update(BiozConfig3 {
            mode: Some(BiozDrvMode::Standby),
            ..Default::default()
        })?;

        self.update(BiozConfig7 {
            stbyon: Some(keep_rx_active),
            ..Default::default()
        })
    }
    // ─────────────────────────────────────────────
    // Low level read helpers
    // ─────────────────────────────────────────────

    pub fn write<R: WritableRegister>(&mut self, reg: R) -> Result<(), I::Error> {
        use crate::trace;
        trace!("WRITE {} = 0x{:02X}", R::name(), reg.value());
        self.iface.write_reg(R::ADDR, reg.value())
    }

    pub(crate) fn read_reg<R: ReadableRegister>(&mut self) -> Result<R, I::Error> {
        use crate::trace;

        let raw = self.iface.read_reg(R::ADDR)?;
        trace!("READ {} = 0x{:02X}", R::name(), raw);

        Ok(R::from_raw(raw))
    }

    pub fn read_fifo_bytes(&mut self, buf: &mut [u8]) -> Result<usize, I::Error> {
        // ─────────────────────────────
        // 1️⃣ Read FIFO count
        // ─────────────────────────────

        let cnt1 = self.iface.read_reg(0x0A)?;
        let cnt2 = self.iface.read_reg(0x0B)?;

        // FIFO_DATA_COUNT[8] = bit7 of 0x0A
        let mut fifo_count: u16 = (cnt1 as u16 & 0x80) << 1;
        fifo_count |= cnt2 as u16;

        // Each sample = 3 bytes
        let bytes_to_read = (fifo_count as usize) * 3;

        /*
        trace!(
            "FIFO COUNT={} → BYTES_TO_READ={}",
            fifo_count,
            bytes_to_read
        );
         */

        if bytes_to_read == 0 {
            return Ok(0);
        }

        let n = bytes_to_read.min(buf.len());

        //debug!("FIFO BURST READ {} bytes", n);

        // ─────────────────────────────
        // 2️⃣ Burst read FIFO (0x0C)
        // ─────────────────────────────
        self.iface.read_reg_burst(0x0C, &mut buf[..n])?;

        Ok(n)
    }

    pub fn debug_decode_fifo(data: &[u8]) {
        let mut i = 0;

        while i + 2 < data.len() {
            let b0 = data[i];
            let b1 = data[i + 1];
            let b2 = data[i + 2];

            // MAX30009 FIFO format:
            // [TAG(4bit) | DATA[19:16]]
            // [DATA[15:8]]
            // [DATA[7:0]]

            let _tag = (b0 >> 4) & 0x0F;

            let raw = (((b0 as u32) & 0x0F) << 16) | ((b1 as u32) << 8) | (b2 as u32);

            // sign extend 20 bits
            let _value = ((raw as i32) << 12) >> 12;

            debug!("FIFO TAG={} VAL={}", _tag, _value);

            i += 3;
        }
    }

    pub fn read_fifo_impedance(
        &mut self,
        out: &mut [f32],
        gain: BiozGain,
        imag_amp: f32,
    ) -> Result<usize, I::Error> {
        // Buffer brut FIFO
        let mut fifo_buf = [0u8; 192];

        let nbytes = self.read_fifo_bytes(&mut fifo_buf)?;

        if nbytes == 0 {
            return Ok(0);
        }

        const VREF: f32 = 1.0;
        const ADC_FULL_SCALE: f32 = 524_288.0; // 2^19
        const DEMOD_SCALE: f32 = 2.0 / core::f32::consts::PI;

        let gain_val = match gain {
            BiozGain::Gain1VV => 1.0,
            BiozGain::Gain2VV => 2.0,
            BiozGain::Gain5VV => 5.0,
            BiozGain::Gain10VV => 10.0,
        };

        let mut written = 0;

        for chunk in fifo_buf[..nbytes].chunks_exact(3) {
            let tag = chunk[0] >> 4;

            // On ne garde que BioZ I/Q
            if tag != 0x1 && tag != 0x2 {
                continue;
            }

            // reconstruct 24-bit word
            let raw: u32 = ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32);

            // extract 20-bit signed
            let mut value = ((raw >> 4) & 0x000F_FFFF) as i32;

            // sign extend
            if (value & (1 << 19)) != 0 {
                value |= !0x000F_FFFF;
            }

            // ADC → Impedance
            let z = (value as f32 * VREF) / (ADC_FULL_SCALE * gain_val * DEMOD_SCALE * imag_amp);

            if written < out.len() {
                out[written] = z;
                written += 1;
            }
        }

        Ok(written)
    }

    pub fn read_fifo_impedance_iq(
        &mut self,
        out: &mut [f32],
        imag_current: f32,
    ) -> Result<usize, I::Error> {
        let mut raw = [0u8; 192];
        let nbytes = self.read_fifo_bytes(&mut raw)?;

        if nbytes < 6 {
            return Ok(0);
        }

        let mut written = 0;

        for chunk in raw[..nbytes].chunks_exact(6) {
            let x1 = &chunk[0..3];
            let x2 = &chunk[3..6];

            let a = x1[0] & 0xF0;
            let b = x2[0] & 0xF0;

            // -------------------------------------------------
            // Detect I/Q order exactly like C example
            // -------------------------------------------------

            let (i_bytes, q_bytes) = if a == 0x10 && b == 0x20 {
                (x1, x2)
            } else if a == 0x20 && b == 0x10 {
                (x2, x1)
            } else {
                continue;
            };

            // -------------------------------------------------
            // Rebuild 20-bit values
            // -------------------------------------------------

            let mut adc_i: i32 = ((i_bytes[0] as i32 & 0x0F) << 16)
                | ((i_bytes[1] as i32) << 8)
                | (i_bytes[2] as i32);

            let mut adc_q: i32 = ((q_bytes[0] as i32 & 0x0F) << 16)
                | ((q_bytes[1] as i32) << 8)
                | (q_bytes[2] as i32);

            // -------------------------------------------------
            // 20-bit sign extend (2's complement)
            // -------------------------------------------------

            if (adc_i & (1 << 19)) != 0 {
                adc_i -= 1 << 20;
            }

            if (adc_q & (1 << 19)) != 0 {
                adc_q -= 1 << 20;
            }

            let i_val = adc_i as f32;
            let q_val = adc_q as f32;

            // -------------------------------------------------
            // magnitude = sqrt(I² + Q²)
            // -------------------------------------------------

            let mag = libm::sqrtf(i_val * i_val + q_val * q_val);

            // -------------------------------------------------
            // EXACT SAME FORMULA AS EXAMPLE
            // -------------------------------------------------

            let denom = (524_288.0)      // 2^19
            * 1.0            // gain=1V/V in example
            * (2.0 / 3.14)   // demod scale
            * imag_current;

            let z = mag / denom;

            if written < out.len() {
                out[written] = z;
                written += 1;
            }

            //trace!("I={} Q={} Z={}", i_val, q_val, z);
        }

        Ok(written)
    }

    /*
    --> shutdown
    --> reset (voir procedure!!)
     */
}
