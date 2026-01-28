use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// -------------------------------------------------------------------------
/// Datasheet constraints (MAX30009)
/// -------------------------------------------------------------------------

const PLL_CLK_MIN: u32 = 14_000_000;
const PLL_CLK_MAX: u32 = 28_000_000;

const BIOZ_ADC_CLK_MIN: u32 = 16_000;
const BIOZ_ADC_CLK_MAX: u32 = 36_375;

const BIOZ_SYNTH_CLK_MIN: u32 = 4_096;
const BIOZ_SYNTH_CLK_MAX: u32 = 28_000_000;

/// -------------------------------------------------------------------------
/// Hardware-supported discrete options
/// -------------------------------------------------------------------------

const NDIVS: &[(u32, &str)] = &[(512, "Div512"), (1024, "Div1024")];

const KDIVS: &[(u32, &str)] = &[
    (1, "Div1"),
    (2, "Div2"),
    (4, "Div4"),
    (8, "Div8"),
    (16, "Div16"),
    (32, "Div32"),
    (64, "Div64"),
    (128, "Div128"),
    (256, "Div256"),
    (512, "Div512"),
    (1024, "Div1024"),
    (2048, "Div2048"),
    (4096, "Div4096"),
    (8192, "Div8192"),
];

const ADC_OSRS: &[u16] = &[128, 256, 512, 1024];
const DAC_OSRS: &[u16] = &[32, 64, 128, 256];

/// -------------------------------------------------------------------------
/// Policy used to choose a canonical solution per frequency
/// -------------------------------------------------------------------------
///
/// This mirrors the *runtime* Policy enum, but is used at build-time
/// to generate different lookup tables.
///
#[derive(Copy, Clone)]
enum Policy {
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

/// -------------------------------------------------------------------------
/// Internal build-time representation
/// -------------------------------------------------------------------------

#[derive(Clone)]
struct Entry {
    f_bioz: u32,
    mdiv: u16,
    ndiv: &'static str,
    kdiv: &'static str,
    adc_osr: u16,
    dac_osr: u16,
}

/// -------------------------------------------------------------------------
/// Policy scoring
/// -------------------------------------------------------------------------
///
/// Higher score = better candidate
///
fn score(e: &Entry, policy: Policy) -> i64 {
    match policy {
        Policy::MinLatency => {
            // Prefer:
            // - small ADC_OSR
            // - low MDIV (lower PLL cost)
            -(e.adc_osr as i64) * 10 - (e.mdiv as i64)
        }

        Policy::MaxSNR => {
            // Prefer:
            // - large ADC_OSR
            // - higher integration
            (e.adc_osr as i64) * 10
        }
    }
}

/// -------------------------------------------------------------------------
/// Core generator (policy-aware)
/// -------------------------------------------------------------------------

fn generate(ref_clk: u32, policy: Policy) -> Vec<Entry> {
    let mut all = Vec::new();

    for mdiv in 0u16..=1022 {
        let pll_clk = ref_clk * (mdiv as u32 + 1);
        if !(PLL_CLK_MIN..=PLL_CLK_MAX).contains(&pll_clk) {
            continue;
        }

        for &(ndiv, ndiv_name) in NDIVS {
            let adc_clk = pll_clk / ndiv;
            if !(BIOZ_ADC_CLK_MIN..=BIOZ_ADC_CLK_MAX).contains(&adc_clk) {
                continue;
            }

            for &(kdiv, kdiv_name) in KDIVS {
                let synth_clk = pll_clk / kdiv;
                if !(BIOZ_SYNTH_CLK_MIN..=BIOZ_SYNTH_CLK_MAX).contains(&synth_clk) {
                    continue;
                }

                for &adc_osr in ADC_OSRS {
                    let sr = adc_clk / adc_osr as u32;
                    if sr == 0 {
                        continue;
                    }

                    for &dac_osr in DAC_OSRS {
                        let f_bioz = synth_clk / dac_osr as u32;

                        // C_BIOZ constraint
                        let num = f_bioz * 2;
                        if num % sr != 0 {
                            continue;
                        }

                        let c_x2 = num / sr;
                        if c_x2 != 1 && c_x2 % 2 != 0 {
                            continue;
                        }

                        all.push(Entry {
                            f_bioz,
                            mdiv,
                            ndiv: ndiv_name,
                            kdiv: kdiv_name,
                            adc_osr,
                            dac_osr,
                        });
                    }
                }
            }
        }
    }

    // ---------------------------------------------------------------------
    // Canonicalization per frequency (policy-based)
    // ---------------------------------------------------------------------

    use std::collections::BTreeMap;
    let mut best: BTreeMap<u32, Entry> = BTreeMap::new();

    for e in all {
        best.entry(e.f_bioz)
            .and_modify(|cur| {
                if score(&e, policy) > score(cur, policy) {
                    *cur = e.clone();
                }
            })
            .or_insert(e);
    }

    best.into_values().collect()
}

/// -------------------------------------------------------------------------
/// Rust code emission
/// -------------------------------------------------------------------------

fn write_table(w: &mut dyn Write, name: &str, entries: &[Entry]) -> std::io::Result<()> {
    writeln!(w, "pub static {}: &[PllBioZ] = &[", name)?;
    for e in entries {
        writeln!(
            w,
            "    PllBioZ {{ f_bioz: {}, mdiv: {}, ndiv: NDiv::{}, kdiv: KDiv::{}, adc_osr: {}, dac_osr: {} }},",
            e.f_bioz, e.mdiv, e.ndiv, e.kdiv, e.adc_osr, e.dac_osr
        )?;
    }
    writeln!(w, "];\n")?;
    Ok(())
}

/// -------------------------------------------------------------------------
/// build.rs entry point
/// -------------------------------------------------------------------------

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("bioz_tables.rs");

    let file = File::create(&dest).unwrap();
    let mut w = BufWriter::new(file);

    writeln!(w, "// AUTO-GENERATED — DO NOT EDIT").unwrap();
    writeln!(w, "// Generated by build.rs").unwrap();
    writeln!(w, "// Policies: MinLatency, MaxSNR\n").unwrap();

    let configs = [(32_768, "32K768"), (32_000, "32K")];

    for &(ref_clk, tag) in &configs {
        let min_lat = generate(ref_clk, Policy::MinLatency);
        let max_snr = generate(ref_clk, Policy::MaxSNR);

        write_table(&mut w, &format!("BIOZ_TABLE_{}_MIN_LATENCY", tag), &min_lat).unwrap();
        write_table(&mut w, &format!("BIOZ_TABLE_{}_MAX_SNR", tag), &max_snr).unwrap();
    }

    println!("cargo:rerun-if-changed=build.rs");
}
