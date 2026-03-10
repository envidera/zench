/*
IDEA:
Problem > Median benchmark time changes depending on hardware, and Rust version,
so it cannot be used as a fixed reference for correctness across machines.

Hypothetical idea > Implement a hardware-independent normalization unit for benchmarks.

- Create a stable `bench_unit` (pure ALU + black_box).
- Measure its median and define it as 1 ZU (Zench Unit).
- Normalize all benchmark results by dividing by this unit.
- Compare results using normalized units instead of raw time.
- Add tolerance-based assertions (e.g. ±10% ZU).
- Ensure stability across Rust versions and CPUs.

hypothetical example

"[5.4 ZU] format!" => {
    bx(format!("Hello {}", bx("world")));
},
...
1 - extract zu [5.4]
2 - compare with actual zu
...
r.match_zu(5%) == true ...

*/

// ================================================================
// Unit test
// ================================================================

#[cfg(test)]
mod tests {

    use zench::bench;
    use zench::bx;

    #[test]
    #[ignore = "display purpose"]
    fn format_macro_performance() {
        bench!(

            "format!" => {
                bx(format!("Hello {}", bx("world")));
            },

            // ----------------------------------------------------------------

            "format! optimization 1" => {
                bx(format!("Hello {}", "world"));
            },

            // "format! optimization 2" => {
            //     format!("Hello {}", "world");
            // },

            // ----------------------------------------------------------------
            // Baseline overhead of the benchmark loop (≈ 1 CPU cycle)
            "empty" => {},

            // ----------------------------------------------------------------
            // 'bx' and 'std::hint::black_box' have the same execution time,
            "bx" => std::hint::black_box(bx("")),
            "black_box" =>  std::hint::black_box(std::hint::black_box("")),
            // ----------------------------------------------------------------
            "bx x2" => {
                std::hint::black_box(bx(""));
                std::hint::black_box(bx(""));
            },
            "black_box x2" => {
                std::hint::black_box(std::hint::black_box(""));
                std::hint::black_box(std::hint::black_box(""));
            },
            "bx x3" => {
                std::hint::black_box(bx(""));
                std::hint::black_box(bx(""));
                std::hint::black_box(bx(""));

            },
            "black_box x3" => {
                std::hint::black_box(std::hint::black_box(""));
                std::hint::black_box(std::hint::black_box(""));
                std::hint::black_box(std::hint::black_box(""));
            },
            "bx x4" => {
                std::hint::black_box(bx(""));
                std::hint::black_box(bx(""));
                std::hint::black_box(bx(""));
                std::hint::black_box(bx(""));
            },

            "black_box x4" => {
                std::hint::black_box(std::hint::black_box(""));
                std::hint::black_box(std::hint::black_box(""));
                std::hint::black_box(std::hint::black_box(""));
                std::hint::black_box(std::hint::black_box(""));
            },


        );
    }
}

/*

Report

Benchmark  format!
Time       Median: 21.267ns
Stability  Std.Dev: ± 0.052ns | CV: 0.25%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 2.00%
Location   zench/tests/display_self.rs:40:9

Benchmark  format! optimization 1
Time       Median: 6.059ns
Stability  Std.Dev: ± 0.025ns | CV: 0.42%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 5.00%
Location   zench/tests/display_self.rs:40:9

Benchmark  format! optimization 2
Time       Median: 1.083ns
Stability  Std.Dev: ± 0.005ns | CV: 0.47%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 4.00%
Location   zench/tests/display_self.rs:40:9

Benchmark  empty
Time       Median: 0.215ns
Stability  Std.Dev: ± 0.000ns | CV: 0.01%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 12.00%
Location   zench/tests/display_self.rs:40:9

Benchmark  bx
Time       Median: 4.542ns
Stability  Std.Dev: ± 0.009ns | CV: 0.20%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 1.00%
Location   zench/tests/display_self.rs:40:9

Benchmark  black_box
Time       Median: 4.540ns
Stability  Std.Dev: ± 0.008ns | CV: 0.18%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 2.00%
Location   zench/tests/display_self.rs:40:9

Benchmark  bx x2
Time       Median: 9.623ns
Stability  Std.Dev: ± 0.010ns | CV: 0.10%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench/tests/display_self.rs:40:9

Benchmark  black_box x2
Time       Median: 9.623ns
Stability  Std.Dev: ± 0.018ns | CV: 0.19%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench/tests/display_self.rs:40:9

Benchmark  bx x3
Time       Median: 13.619ns
Stability  Std.Dev: ± 0.018ns | CV: 0.13%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 3.00%
Location   zench/tests/display_self.rs:40:9

Benchmark  black_box x3
Time       Median: 13.621ns
Stability  Std.Dev: ± 0.021ns | CV: 0.16%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 1.00%
Location   zench/tests/display_self.rs:40:9

Benchmark  bx x4
Time       Median: 18.160ns
Stability  Std.Dev: ± 0.037ns | CV: 0.21%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 1.00%
Location   zench/tests/display_self.rs:40:9

Benchmark  black_box x4
Time       Median: 18.159ns
Stability  Std.Dev: ± 0.032ns | CV: 0.17%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 1.00%
Location   zench/tests/display_self.rs:40:9


total time: 6.56764738 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-06 21:44:19 UTC

*/
