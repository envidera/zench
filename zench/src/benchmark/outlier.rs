use crate::algorithm;
use crate::benchmark::samples::Samples;

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub(crate) enum Outlier {
    Iqr,
    #[default]
    Iqrz,
    ModifiedZScore,
}

// docs/decisions/outlier-algorithm-iqrz.md
pub(crate) fn iqrz(samples: &mut Samples) {
    // Creates an ordered copy just for calculating quartiles
    let mut data = samples.data();
    data.sort_by(|a, b| {
        a.partial_cmp(b)
            .unwrap()
    });

    let q1 = iqr_percent(&data, 0.25);
    let q3 = iqr_percent(&data, 0.75);

    // ----------------------------------------------------------------
    // iqrz adjustment
    // ----------------------------------------------------------------
    const IQRZ_PERCENTAGE_THRESHOLD: f64 = 0.01; // 1%
    let min_iqr_floor: f64 = IQRZ_PERCENTAGE_THRESHOLD * q3; // 1% of q3
    let iqrz = (q3 - q1).max(min_iqr_floor);
    // ----------------------------------------------------------------
    // end iqrz adjustment
    // ----------------------------------------------------------------

    let lower_bound = q1 - 1.5 * iqrz;
    let upper_bound = q3 + 1.5 * iqrz;

    for sample in samples.iter_mut() {
        if !sample.outlier && sample.value < lower_bound || sample.value > upper_bound {
            sample.outlier()
        }
    }
}

// IQR (Interquartile Range) with linear interpolation for quartis and
// Tukey's Fences to filter outliers
// https://en.wikipedia.org/wiki/Interquartile_range
pub(crate) fn iqr(samples: &mut Samples) {
    // Creates an ordered copy just for calculating quartiles
    let mut data = samples.data();
    data.sort_by(|a, b| {
        a.partial_cmp(b)
            .unwrap()
    });

    let q1 = iqr_percent(&data, 0.25);
    let q3 = iqr_percent(&data, 0.75);

    // iqr traditional -----------------------------
    let iqr = q3 - q1;
    // end iqr traditional -------------------------

    let lower_bound = q1 - 1.5 * iqr;
    let upper_bound = q3 + 1.5 * iqr;

    for sample in samples.iter_mut() {
        if !sample.outlier && (sample.value < lower_bound || sample.value > upper_bound) {
            sample.outlier()
        }
    }
}

// Linear interpolation for quartis
fn iqr_percent(data: &[f64], percent: f64) -> f64 {
    let len = data.len() as f64;
    let index = percent * (len - 1.0);
    let lower = index.floor() as usize;
    let upper = index.ceil() as usize;

    if lower == upper {
        data[lower]
    } else {
        let weight = index - lower as f64;
        data[lower] * (1.0 - weight) + data[upper] * weight
    }
}

///////////////////////////////////////////////////////////////////////
// https://docs.oracle.com/en/cloud/saas/freeform/ffuuu/insights_metrics_MODIFIED_Z_SCORE.html
/// Modified Z-Score using median and MAD (Median Absolute Deviation)
/// More robust than mean/standard deviation for extreme outliers
pub(crate) fn modified_z_score(samples: &mut Samples) {
    //Typical threshold: 3.5 (equivalent to sigma_clip with ~3-sigma but more robust)
    const THRESHOLD: f64 = 3.5;

    if samples.len() < 3 {
        return;
    }

    let mut data = samples.data();

    let median = algorithm::median(&mut data);
    let mad = algorithm::mad(&mut data);

    // If MAD is zero, all values are equal (no outliers)
    if mad == 0.0 {
        return;
    }

    // Apply Modified Z-Score
    // Formula: |0.6745 * (x - median) / MAD| <= threshold
    for sample in samples.iter_mut() {
        if !sample.outlier {
            let modified_z = 0.6745 * (sample.value - median).abs() / mad;
            if modified_z > THRESHOLD {
                sample.outlier = true
            }
        }
    }
}

// ================================================================
// Unit test
// ================================================================

#[cfg(test)]
mod tests {
    use super::Samples;
    use super::*;

    struct Test {
        name: &'static str,
        data: &'static [f64],
        want_iqrz: &'static [f64],
        want_iqr: &'static [f64],
        want_mod_z: &'static [f64],
    }

    #[rustfmt::skip]
    const TEST_CASES: &[Test] = &[

        // Classic obvious outlier: all methods should remove (100.0)
        Test{
            name: "simple data",
            data:       &[1.0, 2.0, 3.0, 4.0, 5.0, /*obvious outlier*/ 100.0],
            want_iqrz:  &[1.0, 2.0, 3.0, 4.0, 5.0],
            want_iqr:   &[1.0, 2.0, 3.0, 4.0, 5.0],
            want_mod_z: &[1.0, 2.0, 3.0, 4.0, 5.0],
        },

        // Stable cluster > no method should flag outliers
        Test{
            name: "stable cluster",
            data:       &[6.000, 6.001, 6.002, 6.000, 6.003, 6.001, 6.002, 6.000, 6.004, 6.001],
            want_iqrz:  &[6.000, 6.001, 6.002, 6.000, 6.003, 6.001, 6.002, 6.000, 6.004, 6.001],
            want_iqr:   &[6.000, 6.001, 6.002, 6.000, 6.003, 6.001, 6.002, 6.000, 6.004, 6.001],
            want_mod_z: &[6.000, 6.001, 6.002, 6.000, 6.003, 6.001, 6.002, 6.000, 6.004, 6.001],
        },

        // Stable cluster + small perturbation (6.010) > IQR/Modified Z-Score may remove it, IQRZ keeps it
        Test{
            name: "stable data plus one small perturbation",
            data:       &[6.000, 6.001, 6.002, 6.000, 6.003, 6.001, 6.002, 6.000, 6.004, 6.001, /*small perturbation*/ 6.010],
            want_iqrz:  &[6.000, 6.001, 6.002, 6.000, 6.003, 6.001, 6.002, 6.000, 6.004, 6.001, /*keeps it*/ 6.010],
            want_iqr:   &[6.000, 6.001, 6.002, 6.000, 6.003, 6.001, 6.002, 6.000, 6.004, 6.001, /*removes it*/],
            want_mod_z: &[6.000, 6.001, 6.002, 6.000, 6.003, 6.001, 6.002, 6.000, 6.004, 6.001, /*removes it*/],
        },

        // Tight cluster + some small deviations  > IQR/Modified Z-Score may remove it, IQRZ keeps it
        Test{
            name: "stable data plus some small perturbations",
            data:       &[6.000, 6.001, 6.000, 6.001, 6.002, 6.000, 6.002, 6.000, 6.003, 6.001, 6.002, 6.000, 6.004, 6.001, /*some small deviations*/ 6.009, 6.010, 6.011],
            want_iqrz:  &[6.000, 6.001, 6.000, 6.001, 6.002, 6.000, 6.002, 6.000, 6.003, 6.001, 6.002, 6.000, 6.004, 6.001, /*keeps it*/ 6.009, 6.010, 6.011],
            want_iqr:   &[6.000, 6.001, 6.000, 6.001, 6.002, 6.000, 6.002, 6.000, 6.003, 6.001, 6.002, 6.000, 6.004, 6.001, /*removes it*/],
            want_mod_z: &[6.000, 6.001, 6.000, 6.001, 6.002, 6.000, 6.002, 6.000, 6.003, 6.001, 6.002, 6.000, 6.004, 6.001, /*removes it*/],
        },
        // Tight cluster + real anomaly > all methods should remove
        Test{
            name: "stable data plus one big perturbation",
            data:       &[6.000, 6.001, 6.002, 6.000, 6.003, 6.001, 6.002, 6.000, 6.004, 6.001, /*one real anomaly*/ 6.200],
            want_iqrz:  &[6.000, 6.001, 6.002, 6.000, 6.003, 6.001, 6.002, 6.000, 6.004, 6.001, /*removes it*/],
            want_iqr:   &[6.000, 6.001, 6.002, 6.000, 6.003, 6.001, 6.002, 6.000, 6.004, 6.001, /*removes it*/],
            want_mod_z: &[6.000, 6.001, 6.002, 6.000, 6.003, 6.001, 6.002, 6.000, 6.004, 6.001, /*removes it*/],
        },
        // Tight cluster + some variable perturbations >
        // 6,200 / 6,300 > grey area
        // 6,400 > strong candidate
        // 8,000 > definitely an anomaly
        //
        // Result
        // Modified Z was more aggressive
        // IQR/IQRZ were more pragmatic
        Test{
            name: "stable data plus some variable perturbations",
            data:       &[6.000, 6.001, 6.002, 6.000, 6.003, 6.001, 6.002, 6.000, 6.004, 6.001, /*grey area*/ 6.200, 6.300, /*strong candidate*/ 6.400, /*anomaly*/ 8.000],
            want_iqrz:  &[6.000, 6.001, 6.002, 6.000, 6.003, 6.001, 6.002, 6.000, 6.004, 6.001, /*grey area*/ 6.200, 6.300],
            want_iqr:   &[6.000, 6.001, 6.002, 6.000, 6.003, 6.001, 6.002, 6.000, 6.004, 6.001, /*grey area*/ 6.200, 6.300],
            want_mod_z: &[6.000, 6.001, 6.002, 6.000, 6.003, 6.001, 6.002, 6.000, 6.004, 6.001, /*removes it*/],
        },

    ];

    #[test]
    fn test_iqrz() {
        for t in TEST_CASES {
            let mut samples = Samples::from(t.data);
            self::iqrz(&mut samples);
            let have = samples.data_without_outliers();
            assert_eq!(have, t.want_iqrz, "{}", t.name);
        }
    }

    #[test]
    fn test_iqr() {
        for t in TEST_CASES {
            let mut samples = Samples::from(t.data);
            self::iqr(&mut samples);
            let have = samples.data_without_outliers();
            assert_eq!(have, t.want_iqr, "{}", t.name);
        }
    }

    #[test]
    fn test_modified_z_score() {
        for t in TEST_CASES {
            let mut samples = Samples::from(t.data);
            self::modified_z_score(&mut samples);
            let have = samples.data_without_outliers();
            assert_eq!(have, t.want_mod_z, "{}", t.name);
        }
    }
}

#[cfg(test)]
mod test_performance {
    use super::Samples;
    use super::*;
    use crate::bench;
    use crate::mock;

    #[ignore = "display purpose"]
    #[test]
    fn test_algorithms() {
        const SAMPLES_COUNT_CASES: &[usize] = &[5_000, 100_000, 500_000, 1_000_000];

        for sample in SAMPLES_COUNT_CASES {
            let data = mock::generate_data(*sample);

            let mut samples1 = Samples::from(data);
            let mut samples2 = samples1.clone();
            let mut samples3 = samples1.clone();

            bench! {
                "iqrz" => iqrz(&mut samples1),
                "irq" => iqr(&mut samples2),
                "Modified Z Score" => modified_z_score(&mut samples3),
            }
            .report(|r| {
                r.title(format!("Samples:{sample}"));
                r.sort_by_median()
                    .print();

                println!("iqrz outliers:{}", samples1.count_outliers());
                println!("iqr outliers:{}", samples2.count_outliers());
                println!("Modified Z Score outliers:{}", samples3.count_outliers());
                println!()
            });
        }
    }
}

/*
RESULT:

Algorithm           Sensitivity   Detection Logic
-----------------------------------------------------------
IQR,                Balanced,     Positional (Quartiles),
IQR-Z,              Low (Safe),   Quartile + 1% Floor,
Modified Z-Score,   High,         Distance-based (MAD),
*/

/*
Samples:100000 > Sort Median
─────────────────┬─────────┬───────┬────────────┬──────────┬──────────────
      name       │ median  │  cv   │  std.dev   │ outliers │ samples/iters
─────────────────┼─────────┼───────┼────────────┼──────────┼──────────────
iqrz             │ 2.240ms │ 0.30% │  ± 0.007ms │    0.00% │       7 / 128
irq              │ 2.266ms │ 0.24% │  ± 0.005ms │    0.00% │       7 / 128
Modified Z Score │ 2.503ms │ 0.18% │  ± 0.005ms │    0.00% │       13 / 64
─────────────────┴─────────┴───────┴────────────┴──────────┴──────────────
iqrz outliers:199
iqr outliers:999
Modified Z Score outliers:34000

Samples:500000 > Sort Median
─────────────────┬──────────┬───────┬────────────┬──────────┬──────────────
      name       │  median  │  cv   │  std.dev   │ outliers │ samples/iters
─────────────────┼──────────┼───────┼────────────┼──────────┼──────────────
iqrz             │ 12.835ms │ 1.14% │  ± 0.146ms │    0.00% │       10 / 16
irq              │ 12.858ms │ 0.46% │  ± 0.059ms │   20.00% │       10 / 16
Modified Z Score │ 14.145ms │ 0.69% │  ± 0.097ms │    0.00% │        9 / 16
─────────────────┴──────────┴───────┴────────────┴──────────┴──────────────
iqrz outliers:999
iqr outliers:4999
Modified Z Score outliers:170000

Samples:1000000 > Sort Median
─────────────────┬──────────┬───────┬────────────┬──────────┬──────────────
      name       │  median  │  cv   │  std.dev   │ outliers │ samples/iters
─────────────────┼──────────┼───────┼────────────┼──────────┼──────────────
iqrz             │ 27.379ms │ 1.34% │  ± 0.369ms │    0.00% │        10 / 8
irq              │ 27.642ms │ 0.87% │  ± 0.240ms │   10.00% │        10 / 8
Modified Z Score │ 29.886ms │ 1.28% │  ± 0.384ms │    0.00% │         9 / 8
─────────────────┴──────────┴───────┴────────────┴──────────┴──────────────
iqrz outliers:1999
iqr outliers:9999
Modified Z Score outliers:340000

*/
