fn clone_vec(v: &[u8]) -> Vec<u8> {
    v.to_vec()
}

#[rustfmt::skip]
const CASES: &[i32] = &[
    0,
    100,
    1000,
    10_000,
    100_000,
    500_000,
    1_000_000,
];

// ================================================================
// Unit test
// ================================================================
#[cfg(test)]
mod test_performance {

    use super::*;
    use zench::bench;
    use zench::bx;

    #[test]
    #[ignore = "display purpose"]
    fn benchmark_clone() {
        let mut b = bench!();

        for &size in CASES {
            let original: Vec<u8> = (0..size)
                .map(|i| i as u8)
                .collect();

            b.bench(format!("clone_vec {}", size), || {
                bx(clone_vec(&original));
            });
        }
    }
}

/*

Report

Benchmark  clone_vec 0
Time       Median: 2.814ns
Stability  Std.Dev: ± 0.027ns | CV: 0.96%
Samples    Count: 486 | Iters/sample: 524,288 | Outliers: 6.58%
Location   zench/tests/clone.rs:35:15

Benchmark  clone_vec 100
Time       Median: 8.054ns
Stability  Std.Dev: ± 0.065ns | CV: 0.80%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 2.00%
Location   zench/tests/clone.rs:35:15

Benchmark  clone_vec 1000
Time       Median: 13.132ns
Stability  Std.Dev: ± 0.058ns | CV: 0.44%
Samples    Count: 289 | Iters/sample: 524,288 | Outliers: 4.50%
Location   zench/tests/clone.rs:35:15

Benchmark  clone_vec 10000
Time       Median: 84.876ns
Stability  Std.Dev: ± 0.113ns | CV: 0.13%
Samples    Count: 45 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench/tests/clone.rs:35:15

Benchmark  clone_vec 100000
Time       Median: 1.409µs
Stability  Std.Dev: ± 0.000µs | CV: 0.01%
Samples    Count: 22 | Iters/sample: 65,536 | Outliers: 0.00%
Location   zench/tests/clone.rs:35:15

Benchmark  clone_vec 500000
Time       Median: 8.910µs
Stability  Std.Dev: ± 0.077µs | CV: 0.87%
Samples    Count: 28 | Iters/sample: 8,192 | Outliers: 0.00%
Location   zench/tests/clone.rs:35:15

Benchmark  clone_vec 1000000
Time       Median: 17.748µs
Stability  Std.Dev: ± 0.138µs | CV: 0.77%
Samples    Count: 28 | Iters/sample: 4,096 | Outliers: 3.57%
Location   zench/tests/clone.rs:35:15


total time: 11.978993934 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-02 22:44:30 UTC

*/
