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
    fn bench_std_dev_performance() {
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

Sort Median
────────────┬───────────┬───────┬─────────────┬──────────┬──────────────
   name     │  median   │  cv   │   std.dev   │ outliers │ samples/iters
────────────┼───────────┼───────┼─────────────┼──────────┼──────────────
[faster] v3 │  16.630µs │ 0.18% │   ± 0.029µs │    0.00% │    8 / 16,384
v2          │  65.682µs │ 0.16% │   ± 0.103µs │    0.00% │     8 / 4,096
v1          │  65.699µs │ 0.12% │   ± 0.078µs │    0.00% │     8 / 4,096
────────────┴───────────┴───────┴─────────────┴──────────┴──────────────
total time: 8.220617702 sec
rust: 1.94.1 (release) | zench: 0.1.4

*/
