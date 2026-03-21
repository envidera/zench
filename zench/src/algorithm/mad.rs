// MAD (Median Absolute Deviation)
pub use mad_case::v3c as mad;

#[allow(unused)]
mod mad_case {

    use crate::algorithm::median;

    pub fn v1a(data: &mut [f64]) -> f64 {
        let median = median::median_slower(data);

        let mut deviations: Vec<f64> = data
            .iter()
            .map(|&x| (x - median).abs())
            .collect();

        let mad = self::median::median_slower(&mut deviations);

        mad
    }

    pub fn v1b(data: &mut [f64]) -> f64 {
        let median = median::median(data);

        let mut deviations: Vec<f64> = data
            .iter()
            .map(|&x| (x - median).abs())
            .collect();

        let mad = median::median(&mut deviations);

        mad
    }

    pub fn v1c(data: &mut [f64]) -> f64 {
        let median = median::median(data);

        let mut deviations: Vec<f64> = data
            .iter()
            .map(|&x| (x - median).abs())
            .collect();

        let mad = median::median_slower(&mut deviations);

        mad
    }

    pub fn v1d(data: &mut [f64]) -> f64 {
        let median = median::median_slower(data);

        let mut deviations: Vec<f64> = data
            .iter()
            .map(|&x| (x - median).abs())
            .collect();

        let mad = median::median(&mut deviations);

        mad
    }

    // ----------------------------------------------------------------
    pub(crate) fn v2a(data: &mut [f64]) -> f64 {
        let median = median::median_slower(data);

        for x in data.iter_mut() {
            *x = (*x - median).abs();
        }

        median::median_slower(data)
    }

    pub(crate) fn v2b(data: &mut [f64]) -> f64 {
        let median = median::median(data);

        for x in data.iter_mut() {
            *x = (*x - median).abs();
        }

        median::median(data)
    }

    pub(crate) fn v2c(data: &mut [f64]) -> f64 {
        let median = median::median(data);

        for x in data.iter_mut() {
            *x = (*x - median).abs();
        }

        median::median_slower(data)
    }

    pub(crate) fn v2d(data: &mut [f64]) -> f64 {
        let median = median::median_slower(data);

        for x in data.iter_mut() {
            *x = (*x - median).abs();
        }

        median::median(data)
    }

    // ----------------------------------------------------------------

    pub(crate) fn v3a(data: &mut [f64]) -> f64 {
        let median = median::median_slower(data);

        let mut chunks = data.chunks_exact_mut(4);

        for chunk in &mut chunks {
            chunk[0] = (chunk[0] - median).abs();
            chunk[1] = (chunk[1] - median).abs();
            chunk[2] = (chunk[2] - median).abs();
            chunk[3] = (chunk[3] - median).abs();
        }

        for x in chunks.into_remainder() {
            *x = (*x - median).abs();
        }

        median::median_slower(data)
    }

    pub(crate) fn v3b(data: &mut [f64]) -> f64 {
        let median = self::median::median(data);

        let mut chunks = data.chunks_exact_mut(4);

        for chunk in &mut chunks {
            chunk[0] = (chunk[0] - median).abs();
            chunk[1] = (chunk[1] - median).abs();
            chunk[2] = (chunk[2] - median).abs();
            chunk[3] = (chunk[3] - median).abs();
        }

        for x in chunks.into_remainder() {
            *x = (*x - median).abs();
        }

        median::median(data)
    }

    pub fn v3c(data: &mut [f64]) -> f64 {
        let median = self::median::median(data);

        let mut chunks = data.chunks_exact_mut(4);

        for chunk in &mut chunks {
            chunk[0] = (chunk[0] - median).abs();
            chunk[1] = (chunk[1] - median).abs();
            chunk[2] = (chunk[2] - median).abs();
            chunk[3] = (chunk[3] - median).abs();
        }

        for x in chunks.into_remainder() {
            *x = (*x - median).abs();
        }

        median::median_slower(data)
    }

    pub(crate) fn v3d(data: &mut [f64]) -> f64 {
        let median = self::median::median_slower(data);

        let mut chunks = data.chunks_exact_mut(4);

        for chunk in &mut chunks {
            chunk[0] = (chunk[0] - median).abs();
            chunk[1] = (chunk[1] - median).abs();
            chunk[2] = (chunk[2] - median).abs();
            chunk[3] = (chunk[3] - median).abs();
        }

        for x in chunks.into_remainder() {
            *x = (*x - median).abs();
        }

        median::median(data)
    }

    // ----------------------------------------------------------------

    pub(crate) fn v_in_place(data: &mut [f64]) -> f64 {
        let n = data.len();
        let mid = n / 2;

        let median = {
            let (_, med, _) = data.select_nth_unstable_by(mid, |a, b| {
                a.partial_cmp(b)
                    .unwrap()
            });
            *med
        };

        // Transforming in-place data into absolute deviations
        for x in data.iter_mut() {
            *x = (*x - median).abs();
        }

        let mad = {
            let (_, med_dev, _) = data.select_nth_unstable_by(mid, |a, b| {
                a.partial_cmp(b)
                    .unwrap()
            });
            *med_dev
        };

        // If the size is even, calculate the average using the largest element from the lower half.
        if n.is_multiple_of(2) {
            let max_lower = data[..mid]
                .iter()
                .copied()
                .max_by(|a, b| {
                    a.partial_cmp(b)
                        .unwrap()
                })
                .unwrap();
            (mad + max_lower) / 2.0
        } else {
            mad
        }
    }
}

// ================================================================
// Unit test
// ================================================================
#[cfg(test)]
mod test_performance {

    use super::mad_case;
    use crate::bench;
    use crate::bx;
    use crate::issue;
    use crate::mock;

    //TODO: test bench with dirent data len

    #[test]
    fn mad_performance() {
        let mut data1 = mock::generate_data(100_000);
        let mut data2 = data1.clone();
        let mut data3 = data1.clone();
        let mut data4 = data1.clone();
        let mut data5 = data1.clone();
        let mut data6 = data1.clone();
        let mut data7 = data1.clone();
        let mut data8 = data1.clone();
        let mut data9 = data1.clone();
        let mut data10 = data1.clone();
        let mut data11 = data1.clone();
        let mut data12 = data1.clone();
        let mut data13 = data1.clone();

        bench!(
            "v1a" => bx(mad_case::v1a(&mut data1)),
            "v1a" => bx(mad_case::v1a(&mut data1)),
            "v1b" => bx(mad_case::v1b(&mut data2)),
            "v1c" => bx(mad_case::v1c(&mut data3)),
            "v1d" => bx(mad_case::v1d(&mut data4)),

            "v2a" => bx(mad_case::v2a(&mut data5)),
            "v2b" => bx(mad_case::v2b(&mut data6)),
            "[faster] v2c" => bx(mad_case::v2c(&mut data7)),
            "v2d" => bx(mad_case::v2d(&mut data8)),

            "v3a" => bx(mad_case::v3a(&mut data9)),
            "v3b" => bx(mad_case::v3b(&mut data10)),
            "[faster] v3c" => bx(mad_case::v3c(&mut data11)),
            "v3d" => bx(mad_case::v3d(&mut data12)),

            "mad_in_place" => bx(mad_case::v_in_place(&mut data13)),
        )
        .report(|r| {
            r.sort_by_median();

            let (mut fast_group, mut slow_group) = r
                .filter_proximity_pct(5.0) //10%
                .split();

            fast_group
                .title("fast group")
                .print();

            let (mut slow1, mut slow2) = slow_group
                .filter_proximity_pct(10.0)
                .split();

            slow1
                .title("slow 1")
                .print();

            slow2
                .title("slow 2")
                .print();

            let ok = fast_group
                .iter()
                .all(|f| {
                    f.name()
                        .contains("[faster]")
                });

            if !ok {
                issue!();
            }
        });
    }
}

/*

fast group > Sort Median > Filter Proximity(5%)
─────────────┬────────────┬───────┬─────────────┬──────────┬──────────────
    name     │   median   │  cv   │   std.dev   │ outliers │ samples/iters
─────────────┼────────────┼───────┼─────────────┼──────────┼──────────────
[faster] v3c │  145.208µs │ 0.21% │   ± 0.308µs │    0.00% │     7 / 2,048
[faster] v2c │  146.017µs │ 0.10% │   ± 0.142µs │    0.00% │     7 / 2,048
─────────────┴────────────┴───────┴─────────────┴──────────┴──────────────

slow 1 > Filter Proximity(10%)
─────────────┬────────────┬───────┬─────────────┬──────────┬──────────────
    name     │   median   │  cv   │   std.dev   │ outliers │ samples/iters
─────────────┼────────────┼───────┼─────────────┼──────────┼──────────────
mad_in_place │  211.112µs │ 0.33% │   ± 0.697µs │    0.00% │    10 / 1,024
v2a          │  212.548µs │ 0.26% │   ± 0.546µs │    0.00% │    10 / 1,024
v3a          │  212.956µs │ 0.21% │   ± 0.448µs │    0.00% │    10 / 1,024
─────────────┴────────────┴───────┴─────────────┴──────────┴──────────────

slow 2
─────┬────────────┬───────┬──────────────┬──────────┬──────────────
name │   median   │  cv   │   std.dev    │ outliers │ samples/iters
─────┼────────────┼───────┼──────────────┼──────────┼──────────────
v1c  │  249.046µs │ 7.72% │   ± 19.059µs │    0.00% │     8 / 1,024
v1a  │  306.535µs │ 1.26% │    ± 3.862µs │    0.00% │      13 / 512
v1a  │  313.268µs │ 0.53% │    ± 1.662µs │   15.38% │      13 / 512
v1b  │  320.626µs │ 0.52% │    ± 1.663µs │    0.00% │      13 / 512
v3b  │  374.843µs │ 0.10% │    ± 0.392µs │    0.00% │      11 / 512
v2b  │  375.033µs │ 0.14% │    ± 0.522µs │    0.00% │      11 / 512
v2d  │  442.399µs │ 0.16% │    ± 0.688µs │    0.00% │       9 / 512
v3d  │  442.557µs │ 0.27% │    ± 1.203µs │    0.00% │       9 / 512
v1d  │    2.375ms │ 0.23% │    ± 0.006ms │    0.00% │       14 / 64
─────┴────────────┴───────┴──────────────┴──────────┴──────────────
total time: 36.516340552 sec
rust: 1.94.0 (release) | zench: 0.1.4

*/
