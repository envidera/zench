// https://en.wikipedia.org/wiki/Median

/*
v1 is faster than v2 on the first pass over unsorted data.
However,
v1 becomes slower than v2 when the slice is already sorted or partially ordered.

used both in mad (zench/src/algorithm/mad.rs)
*/
pub use median_case::v1 as median;
pub use median_case::v2 as median_slower;

#[allow(unused)]
mod median_case {

    // ----------------------------------------------------------------
    #[inline(always)]
    pub fn v1(data: &mut [f64]) -> f64 {
        // Sort to calculate median
        data.sort_by(|a, b| {
            a.partial_cmp(b)
                .unwrap()
        });

        let len = data.len();
        if len.is_multiple_of(2) {
            (data[len / 2 - 1] + data[len / 2]) / 2.0
        } else {
            data[len / 2]
        }
    }

    // ----------------------------------------------------------------
    #[inline(always)]
    /// Use median_slower when the slice has already been processed or
    /// partially ordered by median, and you only need to compute the
    /// median again without fully sorting the data. In this scenario,
    /// it can take advantage of the existing data layout and avoid the
    /// cost of another complete sort.
    pub fn v2(data: &mut [f64]) -> f64 {
        let len = data.len();
        let mid = len / 2;

        let median_value = {
            let (_, median, _) = data.select_nth_unstable_by(mid, |a, b| {
                a.partial_cmp(b)
                    .unwrap()
            });
            *median
        };

        if len.is_multiple_of(2) {
            let max_lower = data[..mid]
                .iter()
                .copied()
                .fold(f64::NEG_INFINITY, f64::max);

            (max_lower + median_value) / 2.0
        } else {
            median_value
        }
    }
}

// ================================================================
// Unit test
// ================================================================
#[cfg(test)]
mod test_performance {

    use super::median_case;
    use crate::bench;
    use crate::bx;
    use crate::issue;
    use crate::mock;

    #[test]
    fn test_performance() {
        let mut data1 = mock::generate_data(10_000);
        let mut data2 = data1.clone();

        bench! {
            "[faster] v1" => bx(median_case::v1(&mut data1)),
            "v2" => bx(median_case::v2(&mut data2)),
        }
        .report(|r| {
            r.sort_by_median()
                .print();

            r.filter_proximity_pct(10.0)
                .print();

            let ok = r
                .iter()
                .all(|f| {
                    f.name()
                        .contains("faster")
                });

            if !ok {
                issue!();
            }
        });
    }
}

/*

Report
Filters    Sort Median

Benchmark  [faster] v1
Time       Median: 3.289µs
Stability  Std.Dev: ± 0.005µs | CV: 0.16%
Samples    Count: 38 | Iters/sample: 16,384 | Outliers: 0.00%
Location   zench/src/algorithm/median.rs:82:9

Benchmark  v2
Time       Median: 13.507µs
Stability  Std.Dev: ± 0.025µs | CV: 0.18%
Samples    Count: 37 | Iters/sample: 4,096 | Outliers: 0.00%
Location   zench/src/algorithm/median.rs:82:9



Report
Filters    Sort Median > Filter Proximity(10%)

Benchmark  [faster] v1
Time       Median: 3.289µs
Stability  Std.Dev: ± 0.005µs | CV: 0.16%
Samples    Count: 38 | Iters/sample: 16,384 | Outliers: 0.00%
Location   zench/src/algorithm/median.rs:82:9


total time: unknown sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-07 10:33:20 UTC

*/
