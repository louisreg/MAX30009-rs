# MAX30009-rs

Rust driver for the **MAX30009 Analog Front-End**, with a strong focus on
**correctness**, **datasheet fidelity**, and **zero-runtime-cost abstractions**.

This crate is designed for embedded / no_std environments and follows
the MAX30009 datasheet *exactly*.

---

## PLL Configuration

This driver implements the **Timing Subsystem / PLL** of the MAX30009
exactly as specified in the datasheet.

It provides:
- a safe, fluent Rust API
- strict datasheet-compliant sequencing
- frequency validation
- optional PLL lock handling
- support for internal and external reference clocks
- BioZ-driven PLL synthesis (compile-time optimized)

---

### Example: Basic PLL Configuration

```rust
use max30009::pll::PllConfig;
use max30009::registers::pll::{KDiv, NDiv};

let pll = PllConfig::new()
    .internal_32k768()     // Internal 32.768 kHz reference clock
    .mdiv(439)             // PLL ≈ 14.4 MHz
    .ndiv(NDiv::Div512)    // BioZ ADC clock divider
    .kdiv(KDiv::Div64)     // DDS synthesis divider
    .lock_window_2clk()   // More tolerant lock detection
    .enable();             // Enable PLL

pll.apply(&mut device)?;
```

---

## BioZ-driven PLL synthesis (recommended)

Instead of manually selecting PLL dividers, the driver can **derive a valid PLL
configuration directly from a target BioZ stimulus frequency**.

This is the **recommended** way to configure the timing subsystem.

```rust
use max30009::pll::{PllConfig, Policy};

let mut pll = PllConfig::new()
    .internal_32k768()
    .lock_window_2clk()
    .enable();

let bioz = pll.from_f_bioz_nearest(
    50_000,                // Target BioZ stimulus frequency (Hz)
    Policy::MaxSNR,        // Optimization policy
)?;

// Configure BioZ using returned OSRs
bioz_cfg
    .adc_osr(bioz.adc_osr)
    .dac_osr(bioz.dac_osr);

pll.apply(&mut device)?;
```

---

### What does `from_f_bioz_nearest()` do?

- Uses **precomputed compile-time tables**
- Finds the **nearest achievable** BioZ frequency
- Selects a **datasheet-valid** PLL configuration
- Updates:
  - `MDIV`
  - `NDIV`
  - `KDIV`
- Returns:
  - actual achievable `f_bioz`
  - `BIOZ_ADC_OSR`
  - `BIOZ_DAC_OSR`
- **Does not touch hardware**

Runtime complexity: **O(log N)**  
Runtime allocations: **none**

---

## Synthesis policies

The MAX30009 allows multiple valid configurations for the same BioZ frequency.
This driver exposes **explicit policies** to choose between them.

### `Policy::MinLatency`

Optimized for:
- lowest ADC oversampling
- minimal group delay
- behavior closest to the datasheet examples

Recommended when:
- tight timing constraints
- real-time feedback
- minimal filtering latency

---

### `Policy::MaxSNR` (default)

Optimized for:
- higher ADC oversampling
- longer integration
- improved noise performance

Recommended when:
- BioZ precision matters

---

## How is this implemented?

At **build time**, the driver:

1. Enumerates **all valid hardware combinations**:
   - MDIV, NDIV, KDIV
   - BIOZ_ADC_OSR, BIOZ_DAC_OSR
2. Applies **all datasheet constraints**
3. Enforces the BioZ integration rule:
   ```
   C_BIOZ = F_BIOZ / SR_BIOZ ∈ { 0.5, 1, 2, ... }
   ```
4. Builds **canonical lookup tables**:
   - one table per reference clock
   - one table per policy
5. Embeds the tables as `static` data

At runtime, only a **binary search** is performed.

---

## What happens under the hood (PLL apply)

When `apply()` is called, the driver follows the exact sequence
recommended in the MAX30009 datasheet.

### 1. Frequency validation

The PLL output frequency is computed as:

```
PLL_CLK = REF_CLK × (MDIV + 1)
```

The driver verifies:

```
14 MHz ≤ PLL_CLK ≤ 28 MHz
```

If the frequency is out of range, the configuration is rejected
**before any register is written**.

---

### 2. Register programming (strict order)

```
PLL_CONFIG_1 (0x17)  → MDIV[9:8], NDIV, KDIV, PLL_EN
PLL_CONFIG_2 (0x18)  → MDIV[7:0]
PLL_CONFIG_3 (0x19)  → PLL lock detection window
PLL_CONFIG_4 (0x1A)  → Reference clock selection
```

---

### 3. PLL settling delay

```
~6 ms
```

Allows bandgap + PLL to stabilize.

---

### 4. PLL lock wait (optional)

If `PLL_EN = 1`, the driver waits for the PLL to lock.

---

## PLL timing overview

```
        REF_CLK (32.0 / 32.768 kHz)
                 │
                 ▼
        ┌───────────────────┐
        │       PLL         │
        │  MDIV multiplier  │
        │  (× (MDIV + 1))   │
        └───────────────────┘
                 │
           PLL_CLK (14–28 MHz)
                 │
        ┌────────┴─────────┐
        │                  │
        ▼                  ▼
   BioZ ADC CLK        DDS DAC CLK
   (PLL_CLK / NDIV)   (PLL_CLK / KDIV)
```

---

## External reference clock

```rust
.external_clock(ClockFreqSel::Khz32768)
```

⚠️ The selected frequency **must match** the clock on FCLK.

---

## PLL Synchronization

The MAX30009 provides a hardware mechanism to synchronize the **PLL timing subsystems**
of multiple AFEs so that all devices produce **time-aligned samples**.

The driver exposes this via the `PllSync` helper.

### TRIG-based synchronization

Controller:

```rust
let sync = PllSync::trig_controller();
sync.synchronize(&mut device)?;
```

Target:

```rust
let sync = PllSync::trig_target();
sync.synchronize(&mut device)?;
```

---

### Broadcast synchronization (I2C / SPI)

```rust
let sync = PllSync::broadcast();
sync.synchronize(&mut device)?;
```

---

## Important notes

- `PLL_EN` must be set before enabling BioZ
- BioZ must be **disabled** during `TIMING_SYS_RESET`
- All synchronized devices must:
  - share the same reference clock
  - have PLL enabled and locked
- Synchronization does **not** configure the PLL itself

---

## Datasheet reference

MAX30009 Datasheet  
Section: **Timing Subsystem / PLL**




TODO:
    PLL : powerup-down sequence pll: 
Sequence of Operation When PLL is UsedWhen enabling or disabling PLL, the proper sequence of operations must be followed. This section describes therecommended sequence of operations for various scenarios when PLL is used.Enabling and Disabling the PLLThe following sequence is recommended when enabling and disabling the PLL.● Disable BioZ, if enabled.● Enable PLL by setting PLL_EN to 1.● Wait for PLL to lock using either the FREQ_LOCK[3](0x02) or PHASE_LOCK[2](0x02) status bits.● Enable BioZ I and Q, as needed.● Disable BioZ when data collection is done.● Disable PLL by setting PLL_EN to 0.Entering and Exiting ShutdownThe following sequence is recommended when putting the device into a shutdown state and to exit it.● Disable BioZ, if enabled.● Disable PLL by setting PLL_EN to 0, if enabled.● Set SHDN to 1, to enter the shutdown mode.● ...● Set SHDN to 0 to enter the normal mode.● Enable PLL by setting PLL_EN to 1.● Enable BioZ I and Q as needed.● ...Soft-Reset SequenceThe following sequence is required when resetting the device using the RESET bit. Failure to follow this sequence mayresult in registers becoming unresponsive until a power-on reset is performed.● Set BIOZ_BG_EN = 1.● Set SHDN = 0.● Set REF_CLK_SEL = 0.● Set PLL_EN = 0.● Wait for 1ms.● Set RESET = 1 to reset all registers.● Enable PLL by setting PLL_EN to 1.● ...

--> TimingSubsystem