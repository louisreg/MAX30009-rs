# max30009-rs

Rust `no_std` driver for the Maxim **MAX30009** bioimpedance AFE.

The project is organized by register domain and stays close to the datasheet:

- typed register encoders/decoders,
- sparse read-modify-write updates,
- high-level runtime helpers on `Max30009`.

Datasheet used for this repository: [`max30009.pdf`](./max30009.pdf).

## Warning

This driver is still **lightly tested** and should be considered **work in progress**.
Use with caution for production or safety-critical usage until validation coverage is expanded.

## Features

- `no_std` friendly (`embedded-hal` v1 trait boundaries)
- abstract bus interface via `RegisterInterface`
- typed configs for:
  - PLL (`pll`)
  - BioZ path (`bioz`)
  - FIFO (`fifo`)
  - Interrupts (`interrupts`)
  - Leads (`leads`)
  - System (`system`)
  - Status (`status`)
- high-level sequencing helpers in `device::Max30009`:
  - PLL lock flow
  - shutdown enter/exit flow
  - soft-reset flow
  - BioZ startup/shutdown/standby
  - FIFO control bits (`MARK`, `FLUSH`, `STAT_CLR`)

## Crate Layout

- `src/device.rs`: runtime driver helpers and register I/O glue.
- `src/register_interface.rs`: hardware abstraction trait.
- `src/register_map.rs`: register addresses.
- `src/*/registers.rs`: typed raw register bitfields.
- `src/*/config.rs`: high-level config payloads (apply/readback/update).

## Quick Start

```rust
use max30009::{Max30009, PllFrequency, BiozConfig1};

fn configure<I: max30009::register_interface::RegisterInterface>(
    dev: &mut Max30009<I>,
) -> Result<(), I::Error> {
    // Enable PLL bit (without changing other PLL_CONFIG1 fields)
    dev.set_pll_enabled(true)?;

    // Example sparse update
    dev.update(PllFrequency {
        enable: Some(true),
        ..Default::default()
    })?;

    // Enable BioZ BG + I + Q
    dev.update(BiozConfig1 {
        bg_en: Some(true),
        i_en: Some(true),
        q_en: Some(true),
        ..Default::default()
    })?;

    Ok(())
}
```

## Sequence Helpers (Datasheet Oriented)

The following methods are available on `Max30009`:

### PLL

- `set_pll_enabled(enabled)`
- `wait_for_pll_lock(delay, poll_interval_ms, max_attempts)`
- `enable_pll_and_wait_lock(delay, poll_interval_ms, max_attempts)`
- `disable_pll_sequence()`
- `is_freq_locked()`
- `is_pll_locked()`
- `sync()` (`TIMING_SYS_RESET = 1`)

### Shutdown

- `enter_shutdown_sequence()`
  - disable BioZ
  - disable PLL
  - set `SHDN = 1`
- `exit_shutdown_sequence()`
  - set `SHDN = 0`
  - enable PLL

### Soft Reset

- `soft_reset_sequence(delay)`
  - `BIOZ_BG_EN = 1`
  - `SHDN = 0`
  - `REF_CLK_SEL = 0`
  - `PLL_EN = 0`
  - wait 1 ms
  - `RESET = 1`
  - `PLL_EN = 1`

### BioZ

- `bioz_startup(delay, enable_i, enable_q)`
  - `BIOZ_BG_EN = 1`
  - wait 200 ms
  - enable I/Q as requested
- `bioz_shutdown()` (`BIOZ_BG_EN`, `BIOZ_I_EN`, `BIOZ_Q_EN` -> `0`)
- `bioz_standby(keep_rx_active)`
  - `BIOZ_DRV_MODE = Standby`
  - `BIOZ_STBYON = keep_rx_active`

### FIFO

- `set_fifo_mark()` (`FIFO_MARK = 1`)
- `flush_fifo()` (`FLUSH_FIFO = 1`)
- `clear_fifo_status()` (`FIFO_STAT_CLR = 1`)

## Register Interface

Implement this trait for your SPI/I2C transport:

```rust
pub trait RegisterInterface {
    type Error;
    fn read_reg(&mut self, addr: u8) -> Result<u8, Self::Error>;
    fn read_reg_burst(&mut self, addr: u8, buf: &mut [u8]) -> Result<(), Self::Error>;
    fn write_reg(&mut self, addr: u8, value: u8) -> Result<(), Self::Error>;
}
```

## Build and Test

This repo currently sets an embedded default target in `.cargo/config.toml`.

- library check (embedded target): `cargo check`
- host-side tests (if needed): `cargo test --target x86_64-apple-darwin`

Adjust host target triple to your machine if you are not on macOS.

## TODO

- Improve and extend test coverage (unit, integration, hardware-in-the-loop).
- Add robust interrupt management flows (configuration, clear/ack strategy, runtime handling).
- Provide ready-to-use SPI/I2C implementations of `RegisterInterface`.
- Add a higher-level API layer for common end-to-end use cases and sequencing.
