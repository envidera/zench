// ====================================
// fastest version re-exported
pub use std_dev::v3 as std_dev;
// ====================================

#[doc(hidden)]
#[allow(unused)]
mod std_dev {

    #[cfg(test)]
    pub(crate) fn v1(data: &[f64], mean: &f64) -> f64 {
        let mut sum = 0.0;
        for &x in data {
            let diff = x - mean;
            sum += diff * diff;
        }
        (sum / (data.len() as f64)).sqrt()
    }

    #[cfg(test)]
    pub(crate) fn v2(data: &[f64], mean: &f64) -> f64 {
        // IS_SAMPLE = true   => sample variance
        // IS_SAMPLE = false  => population variance
        const IS_SAMPLE: bool = false;

        let variance = data
            .iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>()
            / if IS_SAMPLE {
                // Use N - 1 for sample standard deviation
                (data.len() - 1) as f64
            } else {
                // Use N for population standard deviation
                data.len() as f64
            };

        variance.sqrt()
    }

    pub fn v3(data: &[f64], mean: &f64) -> f64 {
        let mut sum0 = 0.0;
        let mut sum1 = 0.0;
        let mut sum2 = 0.0;
        let mut sum3 = 0.0;

        let chunks = data.chunks_exact(4);
        let remainder = chunks.remainder();

        for chunk in chunks {
            let d0 = chunk[0] - mean;
            let d1 = chunk[1] - mean;
            let d2 = chunk[2] - mean;
            let d3 = chunk[3] - mean;

            sum0 += d0 * d0;
            sum1 += d1 * d1;
            sum2 += d2 * d2;
            sum3 += d3 * d3;
        }

        let mut sum = sum0 + sum1 + sum2 + sum3;

        for &x in remainder {
            let d = x - mean;
            sum += d * d;
        }

        (sum / data.len() as f64).sqrt()
    }
}

// ================================================================
// Unit test
// ================================================================

#[cfg(test)]
mod tests {
    use super::std_dev;
    use crate::algorithm;
    use crate::mock;

    // ---------------------------------------------
    // Classic known values
    // ---------------------------------------------
    #[test]
    fn known_values_population() {
        let data = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];

        let mean = 5.0;

        // expected std dev  = 2
        let expected = 2.0;

        let r1 = std_dev::v1(&data, &mean);
        let r2 = std_dev::v2(&data, &mean);
        let r3 = std_dev::v3(&data, &mean);

        assert_eq!(r1, expected);
        assert_eq!(r2, expected);
        assert_eq!(r3, expected);
    }

    // ---------------------------------------------
    //  All versions must match
    // ---------------------------------------------

    const EPS: f64 = 1e-12;

    fn approx_eq(a: f64, b: f64) {
        assert!(
            (a - b).abs() < EPS,
            "values not approximately equal: left={} right={}",
            a,
            b
        );
    }

    #[test]
    fn all_versions_match_random_data() {
        let data = mock::generate_data(10_000);
        let mean = algorithm::mean(&data);

        let r1 = std_dev::v1(&data, &mean);
        let r2 = std_dev::v2(&data, &mean);
        let r3 = std_dev::v3(&data, &mean);

        approx_eq(r1, r2);
        approx_eq(r1, r3);
    }

    // ---------------------------------------------
    // All data values are the same (10.0) = std_dev = 0
    // ---------------------------------------------
    #[test]
    fn zero_variance() {
        let data = [10.0; 100]; //array of 100 elements 10.0
        let mean = 10.0;

        let r1 = std_dev::v1(&data, &mean);
        let r2 = std_dev::v2(&data, &mean);
        let r3 = std_dev::v3(&data, &mean);

        assert_eq!(r1, 0.0);
        assert_eq!(r2, 0.0);
        assert_eq!(r3, 0.0);
    }

    // ---------------------------------------------
    // data with only one element
    // ---------------------------------------------
    #[test]
    fn single_value() {
        let data = [42.0];
        let mean = 42.0;

        let r1 = std_dev::v1(&data, &mean);
        let r2 = std_dev::v2(&data, &mean);
        let r3 = std_dev::v3(&data, &mean);

        assert_eq!(r1, 0.0);
        assert_eq!(r2, 0.0);
        assert_eq!(r3, 0.0);
    }

    // ---------------------------------------------
    // Negative numbers
    // ---------------------------------------------
    #[test]
    fn negative_numbers() {
        let data = [-5.0, -1.0, -3.0, -4.0, -2.0];
        let mean = -3.0;

        let r1 = std_dev::v1(&data, &mean);
        let r2 = std_dev::v2(&data, &mean);
        let r3 = std_dev::v3(&data, &mean);

        assert_eq!(r1, r3);
        assert_eq!(r1, r2);
    }
}

#[cfg(test)]
mod test_performance {

    use super::std_dev;
    use crate::algorithm;
    use crate::bench;
    use crate::bx;
    use crate::issue;
    use crate::mock;

    #[test]
    fn std_dev_performance() {
        let data = mock::generate_data(100_000);
        let mean = algorithm::mean(&data);

        bench! {
            "v1" => bx(std_dev::v1(&data, &mean)),
            "v2" => bx(std_dev::v2(&data, &mean)),
            "[faster] v3" => bx(std_dev::v3(&data, &mean)),
        }
        .report(|r| {
            r.sort_by_median()
                .print();

            let ok = r
                .first()
                .unwrap()
                .name()
                .contains("[faster]");

            if !ok {
                issue!();
            }
        });
    }
}

/*

Report
Filters    Sort Median

Benchmark  [faster] v3
Time       Median: 16.430µs
Stability  Std.Dev: ± 0.065µs | CV: 0.39%
Samples    Count: 30 | Iters/sample: 4,096 | Outliers: 0.00%
Location   zench/src/algorithm/std_dev.rs:198:9

Benchmark  v1
Time       Median: 65.464µs
Stability  Std.Dev: ± 0.192µs | CV: 0.29%
Samples    Count: 30 | Iters/sample: 1,024 | Outliers: 0.00%
Location   zench/src/algorithm/std_dev.rs:198:9

Benchmark  v2
Time       Median: 65.548µs
Stability  Std.Dev: ± 0.139µs | CV: 0.21%
Samples    Count: 30 | Iters/sample: 1,024 | Outliers: 0.00%
Location   zench/src/algorithm/std_dev.rs:198:9


total time: 6.458402465 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-07 10:52:05 UTC

*/
