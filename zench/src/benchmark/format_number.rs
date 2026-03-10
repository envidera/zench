#[derive(Clone, Copy)]
pub(crate) enum FormatNumber {
    Comma, // 1,000,000
    Rust,  // 1_000_000
    Dot,   // 1.000.000
    Space, // 1 000 000
}

#[allow(unused)]
impl FormatNumber {
    pub(crate) fn coma(n: usize) -> String {
        format_number(n, Self::Comma)
    }

    pub(crate) fn rust(n: usize) -> String {
        format_number(n, Self::Rust)
    }

    pub(crate) fn space(n: usize) -> String {
        format_number(n, Self::Space)
    }

    pub(crate) fn dot(n: usize) -> String {
        format_number(n, Self::Dot)
    }
}

pub(crate) use format_number_case::v1 as format_number;

#[allow(unused)]
mod format_number_case {

    use super::*;

    pub(crate) fn v1(n: usize, format: FormatNumber) -> String {
        let separator = match format {
            FormatNumber::Comma => ',',
            FormatNumber::Rust => '_',
            FormatNumber::Space => ' ',
            FormatNumber::Dot => '.',
        };

        let s = n.to_string();
        let len = s.len();
        let mut result = String::with_capacity(len + len / 3);

        for (i, c) in s
            .chars()
            .enumerate()
        {
            if i != 0 && (len - i).is_multiple_of(3) {
                result.push(separator);
            }
            result.push(c);
        }

        result
    }
}

// ================================================================
// Unit test
// ================================================================

#[allow(unused)]
const CASES: &[(usize, FormatNumber, &str)] = &[
    (999, FormatNumber::Comma, "999"),
    (999, FormatNumber::Rust, "999"),
    (999, FormatNumber::Space, "999"),
    (999, FormatNumber::Dot, "999"),
    // ----------------------------------------------------------------
    (1_000, FormatNumber::Comma, "1,000"),
    (1_000, FormatNumber::Rust, "1_000"),
    (1_000, FormatNumber::Space, "1 000"),
    (1_000, FormatNumber::Dot, "1.000"),
    // ----------------------------------------------------------------
    (10_000, FormatNumber::Comma, "10,000"),
    (10_000, FormatNumber::Rust, "10_000"),
    (10_000, FormatNumber::Space, "10 000"),
    (10_000, FormatNumber::Dot, "10.000"),
    // ----------------------------------------------------------------
    (1_000_000, FormatNumber::Comma, "1,000,000"),
    (1_000_000, FormatNumber::Rust, "1_000_000"),
    (1_000_000, FormatNumber::Space, "1 000 000"),
    (1_000_000, FormatNumber::Dot, "1.000.000"),
];

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cases() {
        #[rustfmt::skip]

        #[rustfmt::skip]
        let versions = [
            format_number_case::v1,
        ];

        for v in versions {
            for (n, fmt, expected) in CASES {
                assert_eq!(v(*n, *fmt), **expected);
            }
        }
    }
}

#[cfg(test)]
mod test_performance {

    use super::*;
    use crate::bx;
    use crate::Bench;

    #[ignore = "display purpose"]
    #[test]
    fn test_2() {
        let mut b = Bench::new();

        for (n, fmt, _) in CASES {
            b.bench(format!("v1 {n}"), || {
                bx(format_number_case::v1(*n, *fmt));
            });
        }
    }
}

/*

Report

Benchmark  v1 999
Time       Median: 23.594ns
Stability  Std.Dev: ± 0.077ns | CV: 0.33%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench/src/benchmark/format_number.rs:120:15

Benchmark  v1 999
Time       Median: 23.783ns
Stability  Std.Dev: ± 0.029ns | CV: 0.12%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench/src/benchmark/format_number.rs:120:15

Benchmark  v1 999
Time       Median: 23.781ns
Stability  Std.Dev: ± 0.036ns | CV: 0.15%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench/src/benchmark/format_number.rs:120:15

Benchmark  v1 999
Time       Median: 23.781ns
Stability  Std.Dev: ± 0.018ns | CV: 0.08%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 1.00%
Location   zench/src/benchmark/format_number.rs:120:15

Benchmark  v1 1000
Time       Median: 25.177ns
Stability  Std.Dev: ± 0.055ns | CV: 0.22%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 1.00%
Location   zench/src/benchmark/format_number.rs:120:15

Benchmark  v1 1000
Time       Median: 25.053ns
Stability  Std.Dev: ± 0.113ns | CV: 0.45%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 1.00%
Location   zench/src/benchmark/format_number.rs:120:15

Benchmark  v1 1000
Time       Median: 25.006ns
Stability  Std.Dev: ± 0.113ns | CV: 0.45%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 2.00%
Location   zench/src/benchmark/format_number.rs:120:15

Benchmark  v1 1000
Time       Median: 24.996ns
Stability  Std.Dev: ± 0.071ns | CV: 0.28%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 2.00%
Location   zench/src/benchmark/format_number.rs:120:15

Benchmark  v1 10000
Time       Median: 26.192ns
Stability  Std.Dev: ± 0.123ns | CV: 0.47%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 1.00%
Location   zench/src/benchmark/format_number.rs:120:15

Benchmark  v1 10000
Time       Median: 26.213ns
Stability  Std.Dev: ± 0.069ns | CV: 0.26%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 1.00%
Location   zench/src/benchmark/format_number.rs:120:15

Benchmark  v1 10000
Time       Median: 26.229ns
Stability  Std.Dev: ± 0.049ns | CV: 0.19%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 2.00%
Location   zench/src/benchmark/format_number.rs:120:15

Benchmark  v1 10000
Time       Median: 26.217ns
Stability  Std.Dev: ± 0.052ns | CV: 0.20%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 2.00%
Location   zench/src/benchmark/format_number.rs:120:15

Benchmark  v1 1000000
Time       Median: 29.660ns
Stability  Std.Dev: ± 0.141ns | CV: 0.47%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench/src/benchmark/format_number.rs:120:15

Benchmark  v1 1000000
Time       Median: 29.664ns
Stability  Std.Dev: ± 0.123ns | CV: 0.41%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 1.00%
Location   zench/src/benchmark/format_number.rs:120:15

Benchmark  v1 1000000
Time       Median: 29.652ns
Stability  Std.Dev: ± 0.110ns | CV: 0.37%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench/src/benchmark/format_number.rs:120:15

Benchmark  v1 1000000
Time       Median: 29.657ns
Stability  Std.Dev: ± 0.131ns | CV: 0.44%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench/src/benchmark/format_number.rs:120:15


total time: 22.523009033 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-03 12:20:47 UTC

*/
