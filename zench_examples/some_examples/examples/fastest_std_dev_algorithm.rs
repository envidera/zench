fn main() {
    // use
    // let _ = std_dev(v, &mean);
}

// ====================================
// fastest version re-exported
pub use std_dev::v3 as std_dev;
// ====================================

#[doc(hidden)]
#[allow(unused)]
mod std_dev {

    #[cfg(test)]
    pub(crate) fn v1(v: &[f64], mean: &f64) -> f64 {
        let mut sum = 0.0;
        for &x in v {
            let diff = x - mean;
            sum += diff * diff;
        }
        (sum / (v.len() as f64)).sqrt()
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
mod test_performance {

    use super::*;
    use zench::bench;
    use zench::bx;
    use zench::dev;
    use zench::issue;

    #[test]
    fn std_dev_performance() {
        let data = dev::mock::generate_data(100_000);
        let mean = dev::algorithm::mean(&data);

        bench! {
            "v1" => bx(std_dev::v1(&data, &mean)),
            "v2 "=> bx(std_dev::v2(&data, &mean)),
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
                issue!("std_dev::v3 is not the fastest algorithm anymore")
            }
        });
    }
}

/*

Report
Filters    Sort Median

Benchmark  [faster] v3
Time       Median: 16.547µs
Stability  Std.Dev: ± 0.042µs | CV: 0.25%
Samples    Count: 30 | Iters/sample: 4,096 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/fastest_std_dev_algorithm.rs:96:9

Benchmark  v1
Time       Median: 65.508µs
Stability  Std.Dev: ± 0.177µs | CV: 0.27%
Samples    Count: 30 | Iters/sample: 1,024 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/fastest_std_dev_algorithm.rs:96:9

Benchmark  v2
Time       Median: 65.597µs
Stability  Std.Dev: ± 0.141µs | CV: 0.22%
Samples    Count: 30 | Iters/sample: 1,024 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/fastest_std_dev_algorithm.rs:96:9


total time: 6.475784562 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-08 19:44:20 UTC

*/
