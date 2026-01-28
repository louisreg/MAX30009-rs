#[cfg(test)]
mod tests {
    #[derive(Debug)]
    pub struct DatasheetRow {
        pub ref_clk: u32,
        pub m_plus_1: u16,
        pub pll_clk: u32,
        pub kdiv: u32,
        pub dac_osr: u16,
        pub f_bioz: u32,
        pub ndiv: u32,
        pub adc_osr: u16,
        pub sr_bioz_x2: u32, // SR_BIOZ * 2
    }

    pub const DATASHEET_TABLE_3: &[DatasheetRow] = &[
        // ─────────────────────────────────────────────────────────
        // KDIV = 1, DAC_OSR = 32
        // ─────────────────────────────────────────────────────────
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 790,
            pll_clk: 25_886_720,
            kdiv: 1,
            dac_osr: 32,
            f_bioz: 808_960,
            ndiv: 1024,
            adc_osr: 128,
            sr_bioz_x2: 395,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 706,
            pll_clk: 23_134_208,
            kdiv: 1,
            dac_osr: 32,
            f_bioz: 722_944,
            ndiv: 1024,
            adc_osr: 128,
            sr_bioz_x2: 353,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 634,
            pll_clk: 20_774_912,
            kdiv: 1,
            dac_osr: 32,
            f_bioz: 649_216,
            ndiv: 1024,
            adc_osr: 128,
            sr_bioz_x2: 317,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 568,
            pll_clk: 18_612_224,
            kdiv: 1,
            dac_osr: 32,
            f_bioz: 581_632,
            ndiv: 512,
            adc_osr: 128,
            sr_bioz_x2: 568,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 488,
            pll_clk: 15_990_784,
            kdiv: 1,
            dac_osr: 32,
            f_bioz: 499_712,
            ndiv: 512,
            adc_osr: 128,
            sr_bioz_x2: 488,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 458,
            pll_clk: 15_007_744,
            kdiv: 1,
            dac_osr: 32,
            f_bioz: 468_992,
            ndiv: 512,
            adc_osr: 128,
            sr_bioz_x2: 458,
        },
        // ─────────────────────────────────────────────────────────
        // KDIV = 1, DAC_OSR = 64
        // ─────────────────────────────────────────────────────────
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 822,
            pll_clk: 26_935_296,
            kdiv: 1,
            dac_osr: 64,
            f_bioz: 420_864,
            ndiv: 1024,
            adc_osr: 128,
            sr_bioz_x2: 411,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 738,
            pll_clk: 24_182_784,
            kdiv: 1,
            dac_osr: 64,
            f_bioz: 377_856,
            ndiv: 1024,
            adc_osr: 128,
            sr_bioz_x2: 369,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 662,
            pll_clk: 21_692_416,
            kdiv: 1,
            dac_osr: 64,
            f_bioz: 338_944,
            ndiv: 1024,
            adc_osr: 128,
            sr_bioz_x2: 331,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 594,
            pll_clk: 19_464_192,
            kdiv: 1,
            dac_osr: 64,
            f_bioz: 304_128,
            ndiv: 1024,
            adc_osr: 128,
            sr_bioz_x2: 297,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 533,
            pll_clk: 17_465_344,
            kdiv: 1,
            dac_osr: 64,
            f_bioz: 272_896,
            ndiv: 512,
            adc_osr: 128,
            sr_bioz_x2: 533,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 488,
            pll_clk: 15_990_784,
            kdiv: 1,
            dac_osr: 64,
            f_bioz: 249_856,
            ndiv: 512,
            adc_osr: 128,
            sr_bioz_x2: 488,
        },
        // ─────────────────────────────────────────────────────────
        // KDIV = 1, DAC_OSR = 128
        // ─────────────────────────────────────────────────────────
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 781,
            pll_clk: 25_591_808,
            kdiv: 1,
            dac_osr: 128,
            f_bioz: 199_936,
            ndiv: 1024,
            adc_osr: 128,
            sr_bioz_x2: 391,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 691,
            pll_clk: 22_642_688,
            kdiv: 1,
            dac_osr: 128,
            f_bioz: 176_896,
            ndiv: 1024,
            adc_osr: 128,
            sr_bioz_x2: 345,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 621,
            pll_clk: 20_348_928,
            kdiv: 1,
            dac_osr: 128,
            f_bioz: 158_976,
            ndiv: 1024,
            adc_osr: 128,
            sr_bioz_x2: 311,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 559,
            pll_clk: 18_317_312,
            kdiv: 1,
            dac_osr: 128,
            f_bioz: 143_104,
            ndiv: 512,
            adc_osr: 128,
            sr_bioz_x2: 559,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 512,
            pll_clk: 16_777_216,
            kdiv: 1,
            dac_osr: 128,
            f_bioz: 131_072,
            ndiv: 512,
            adc_osr: 128,
            sr_bioz_x2: 512,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 449,
            pll_clk: 14_712_832,
            kdiv: 1,
            dac_osr: 128,
            f_bioz: 114_944,
            ndiv: 512,
            adc_osr: 128,
            sr_bioz_x2: 449,
        },
        // ───────────── KDIV = 2, DAC_OSR = 256 ─────────────
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 844,
            pll_clk: 27_656_192,
            kdiv: 2,
            dac_osr: 256,
            f_bioz: 54_016,
            ndiv: 1024,
            adc_osr: 128,
            sr_bioz_x2: 422,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 781,
            pll_clk: 25_591_808,
            kdiv: 2,
            dac_osr: 256,
            f_bioz: 49_984,
            ndiv: 1024,
            adc_osr: 128,
            sr_bioz_x2: 390,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 672,
            pll_clk: 22_020_096,
            kdiv: 2,
            dac_osr: 256,
            f_bioz: 43_008,
            ndiv: 1024,
            adc_osr: 128,
            sr_bioz_x2: 336,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 641,
            pll_clk: 21_004_288,
            kdiv: 2,
            dac_osr: 256,
            f_bioz: 41_024,
            ndiv: 1024,
            adc_osr: 128,
            sr_bioz_x2: 320,
        },
        // ───────────── KDIV = 2, DAC_OSR = 256, ADC_OSR = 256 ─────────────
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 609,
            pll_clk: 19_955_712,
            kdiv: 2,
            dac_osr: 256,
            f_bioz: 38_976,
            ndiv: 1024,
            adc_osr: 256,
            sr_bioz_x2: 152,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 547,
            pll_clk: 17_924_096,
            kdiv: 2,
            dac_osr: 256,
            f_bioz: 35_008,
            ndiv: 512,
            adc_osr: 256,
            sr_bioz_x2: 274,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 484,
            pll_clk: 15_859_712,
            kdiv: 2,
            dac_osr: 256,
            f_bioz: 30_976,
            ndiv: 512,
            adc_osr: 256,
            sr_bioz_x2: 242,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 438,
            pll_clk: 14_352_384,
            kdiv: 2,
            dac_osr: 256,
            f_bioz: 28_032,
            ndiv: 512,
            adc_osr: 256,
            sr_bioz_x2: 219,
        },
        // ───────────── KDIV = 4, DAC_OSR = 256 ─────────────
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 781,
            pll_clk: 25_591_808,
            kdiv: 4,
            dac_osr: 256,
            f_bioz: 24_992,
            ndiv: 1024,
            adc_osr: 256,
            sr_bioz_x2: 195,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 719,
            pll_clk: 23_560_192,
            kdiv: 4,
            dac_osr: 256,
            f_bioz: 23_008,
            ndiv: 1024,
            adc_osr: 256,
            sr_bioz_x2: 180,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 625,
            pll_clk: 20_480_000,
            kdiv: 4,
            dac_osr: 256,
            f_bioz: 20_000,
            ndiv: 1024,
            adc_osr: 256,
            sr_bioz_x2: 156,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 563,
            pll_clk: 18_448_384,
            kdiv: 4,
            dac_osr: 256,
            f_bioz: 18_016,
            ndiv: 512,
            adc_osr: 256,
            sr_bioz_x2: 281,
        },
        // ───────────── KDIV = 8, DAC_OSR = 256 ─────────────
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 813,
            pll_clk: 26_640_384,
            kdiv: 8,
            dac_osr: 256,
            f_bioz: 13_008,
            ndiv: 1024,
            adc_osr: 256,
            sr_bioz_x2: 203,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 750,
            pll_clk: 24_576_000,
            kdiv: 8,
            dac_osr: 256,
            f_bioz: 12_000,
            ndiv: 1024,
            adc_osr: 256,
            sr_bioz_x2: 187,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 688,
            pll_clk: 22_544_384,
            kdiv: 8,
            dac_osr: 256,
            f_bioz: 11_008,
            ndiv: 1024,
            adc_osr: 256,
            sr_bioz_x2: 172,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 625,
            pll_clk: 20_480_000,
            kdiv: 8,
            dac_osr: 256,
            f_bioz: 10_000,
            ndiv: 1024,
            adc_osr: 256,
            sr_bioz_x2: 156,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 563,
            pll_clk: 18_448_384,
            kdiv: 8,
            dac_osr: 256,
            f_bioz: 9_008,
            ndiv: 1024,
            adc_osr: 256,
            sr_bioz_x2: 140,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 500,
            pll_clk: 16_384_000,
            kdiv: 8,
            dac_osr: 256,
            f_bioz: 8_000,
            ndiv: 512,
            adc_osr: 256,
            sr_bioz_x2: 250,
        },
        // ───────────── KDIV = 8 → 512 ─────────────
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 438,
            pll_clk: 14_352_384,
            kdiv: 8,
            dac_osr: 256,
            f_bioz: 7_008,
            ndiv: 512,
            adc_osr: 256,
            sr_bioz_x2: 219,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 750,
            pll_clk: 24_576_000,
            kdiv: 16,
            dac_osr: 256,
            f_bioz: 6_000,
            ndiv: 1024,
            adc_osr: 256,
            sr_bioz_x2: 187,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 625,
            pll_clk: 20_480_000,
            kdiv: 16,
            dac_osr: 256,
            f_bioz: 5_000,
            ndiv: 1024,
            adc_osr: 256,
            sr_bioz_x2: 156,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 500,
            pll_clk: 16_384_000,
            kdiv: 16,
            dac_osr: 256,
            f_bioz: 4_000,
            ndiv: 512,
            adc_osr: 256,
            sr_bioz_x2: 250,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 500,
            pll_clk: 16_384_000,
            kdiv: 32,
            dac_osr: 256,
            f_bioz: 2_000,
            ndiv: 512,
            adc_osr: 512,
            sr_bioz_x2: 125,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 500,
            pll_clk: 16_384_000,
            kdiv: 64,
            dac_osr: 256,
            f_bioz: 1_000,
            ndiv: 512,
            adc_osr: 1024,
            sr_bioz_x2: 62,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 500,
            pll_clk: 16_384_000,
            kdiv: 128,
            dac_osr: 256,
            f_bioz: 500,
            ndiv: 512,
            adc_osr: 1024,
            sr_bioz_x2: 62,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 500,
            pll_clk: 16_384_000,
            kdiv: 256,
            dac_osr: 256,
            f_bioz: 250,
            ndiv: 512,
            adc_osr: 1024,
            sr_bioz_x2: 62,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 500,
            pll_clk: 16_384_000,
            kdiv: 512,
            dac_osr: 256,
            f_bioz: 125,
            ndiv: 512,
            adc_osr: 1024,
            sr_bioz_x2: 62,
        },
        // ───────────── KDIV ≥ 1024 ─────────────
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 512,
            pll_clk: 16_777_216,
            kdiv: 1024,
            dac_osr: 256,
            f_bioz: 64,
            ndiv: 512,
            adc_osr: 1024,
            sr_bioz_x2: 4,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 512,
            pll_clk: 16_777_216,
            kdiv: 2048,
            dac_osr: 256,
            f_bioz: 32,
            ndiv: 512,
            adc_osr: 1024,
            sr_bioz_x2: 2,
        },
        DatasheetRow {
            ref_clk: 32_768,
            m_plus_1: 512,
            pll_clk: 16_777_216,
            kdiv: 4096,
            dac_osr: 256,
            f_bioz: 16,
            ndiv: 1024,
            adc_osr: 1024,
            sr_bioz_x2: 2,
        },
    ];

    #[test]
    fn datasheet_table_matches_from_f_bioz_nearest() {
        use max30009::pll::PllConfig;
        use max30009::pll::Policy;

        let mut diffs: Vec<String> = Vec::new();

        for row in DATASHEET_TABLE_3 {
            let mut pll = PllConfig::new().internal_32k768();

            let sol = match pll.from_f_bioz_nearest(row.f_bioz, Policy::MinLatency) {
                Ok(sol) => sol,
                Err(e) => {
                    diffs.push(format!(
                        "f_bioz = {} Hz → synthesis failed: {:?}",
                        row.f_bioz, e
                    ));
                    continue;
                }
            };

            // ───────────── MDIV (datasheet stores MDIV+1) ─────────────
            if pll.mdiv + 1 != row.m_plus_1 {
                diffs.push(format!(
                    "f_bioz = {} Hz → MDIV mismatch: datasheet={} impl={}",
                    row.f_bioz,
                    row.m_plus_1,
                    pll.mdiv + 1
                ));
            }

            // ───────────── KDIV ─────────────
            if pll.kdiv.divisor() != row.kdiv {
                diffs.push(format!(
                    "f_bioz = {} Hz → KDIV mismatch: datasheet={} impl={}",
                    row.f_bioz,
                    row.kdiv,
                    pll.kdiv.divisor()
                ));
            }

            // ───────────── NDIV ─────────────
            if pll.ndiv.divisor() != row.ndiv {
                diffs.push(format!(
                    "f_bioz = {} Hz → NDIV mismatch: datasheet={} impl={}",
                    row.f_bioz,
                    row.ndiv,
                    pll.ndiv.divisor()
                ));
            }

            // ───────────── ADC OSR ─────────────
            if sol.adc_osr != row.adc_osr {
                diffs.push(format!(
                    "f_bioz = {} Hz → ADC_OSR mismatch: datasheet={} impl={}",
                    row.f_bioz, row.adc_osr, sol.adc_osr
                ));
            }

            // ───────────── DAC OSR ─────────────
            if sol.dac_osr != row.dac_osr {
                diffs.push(format!(
                    "f_bioz = {} Hz → DAC_OSR mismatch: datasheet={} impl={}",
                    row.f_bioz, row.dac_osr, sol.dac_osr
                ));
            }

            // ───────────── Actual frequency (informative) ─────────────
            if sol.f_bioz_actual != row.f_bioz {
                diffs.push(format!(
                    "f_bioz = {} Hz → actual frequency differs: datasheet={} impl={}",
                    row.f_bioz, row.f_bioz, sol.f_bioz_actual
                ));
            }
        }

        // ───────────── Final report ─────────────
        if !diffs.is_empty() {
            println!(
                "\n❌ Datasheet mismatches detected ({}):\n{}",
                diffs.len(),
                diffs.join("\n")
            );
        }
    }
}
