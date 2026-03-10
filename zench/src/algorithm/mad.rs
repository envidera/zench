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

Report     fast group
Filters    Sort Median > Filter Proximity(5%)

Benchmark  [faster] v3c
Time       Median: 145.099µs
Stability  Std.Dev: ± 0.413µs | CV: 0.28%
Samples    Count: 53 | Iters/sample: 256 | Outliers: 1.89%
Location   zench/src/algorithm/mad.rs:254:9

Benchmark  [faster] v2c
Time       Median: 145.828µs
Stability  Std.Dev: ± 0.792µs | CV: 0.54%
Samples    Count: 53 | Iters/sample: 256 | Outliers: 1.89%
Location   zench/src/algorithm/mad.rs:254:9



Report     slow 1
Filters    Filter Proximity(10%)

Benchmark  mad_in_place
Time       Median: 203.620µs
Stability  Std.Dev: ± 0.531µs | CV: 0.26%
Samples    Count: 39 | Iters/sample: 256 | Outliers: 2.56%
Location   zench/src/algorithm/mad.rs:254:9

Benchmark  v3a
Time       Median: 213.248µs
Stability  Std.Dev: ± 0.488µs | CV: 0.23%
Samples    Count: 37 | Iters/sample: 256 | Outliers: 2.70%
Location   zench/src/algorithm/mad.rs:254:9

Benchmark  v2a
Time       Median: 213.466µs
Stability  Std.Dev: ± 0.249µs | CV: 0.12%
Samples    Count: 37 | Iters/sample: 256 | Outliers: 2.70%
Location   zench/src/algorithm/mad.rs:254:9



Report     slow 2

Benchmark  v1c
Time       Median: 265.389µs
Stability  Std.Dev: ± 10.311µs | CV: 3.93%
Samples    Count: 31 | Iters/sample: 256 | Outliers: 9.68%
Location   zench/src/algorithm/mad.rs:254:9

Benchmark  v1a
Time       Median: 307.342µs
Stability  Std.Dev: ± 3.787µs | CV: 1.23%
Samples    Count: 26 | Iters/sample: 256 | Outliers: 0.00%
Location   zench/src/algorithm/mad.rs:254:9

Benchmark  v1b
Time       Median: 312.190µs
Stability  Std.Dev: ± 1.439µs | CV: 0.46%
Samples    Count: 25 | Iters/sample: 256 | Outliers: 12.00%
Location   zench/src/algorithm/mad.rs:254:9

Benchmark  v1a
Time       Median: 312.891µs
Stability  Std.Dev: ± 1.469µs | CV: 0.47%
Samples    Count: 25 | Iters/sample: 256 | Outliers: 4.00%
Location   zench/src/algorithm/mad.rs:254:9

Benchmark  v2b
Time       Median: 374.180µs
Stability  Std.Dev: ± 0.419µs | CV: 0.11%
Samples    Count: 41 | Iters/sample: 128 | Outliers: 7.32%
Location   zench/src/algorithm/mad.rs:254:9

Benchmark  v3b
Time       Median: 374.352µs
Stability  Std.Dev: ± 0.420µs | CV: 0.11%
Samples    Count: 41 | Iters/sample: 128 | Outliers: 7.32%
Location   zench/src/algorithm/mad.rs:254:9

Benchmark  v3d
Time       Median: 441.600µs
Stability  Std.Dev: ± 0.671µs | CV: 0.15%
Samples    Count: 35 | Iters/sample: 128 | Outliers: 8.57%
Location   zench/src/algorithm/mad.rs:254:9

Benchmark  v2d
Time       Median: 442.411µs
Stability  Std.Dev: ± 0.477µs | CV: 0.11%
Samples    Count: 35 | Iters/sample: 128 | Outliers: 8.57%
Location   zench/src/algorithm/mad.rs:254:9

Benchmark  v1d
Time       Median: 2.351ms
Stability  Std.Dev: ± 0.005ms | CV: 0.20%
Samples    Count: 27 | Iters/sample: 32 | Outliers: 0.00%
Location   zench/src/algorithm/mad.rs:254:9


total time: unknown sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-07 10:30:58 UTC

*/
