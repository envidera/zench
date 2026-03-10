fn main() {}

// ================================================================
// Unit test
// ================================================================

#[cfg(test)]
mod tests {

    use zench::bench;
    use zench::bx;

    fn fibonacci(n: u64) -> u64 {
        match n {
            0 => 1,
            1 => 1,
            n => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    #[test]
    fn bench_fibs() {
        bench!(
            "fib 10" => fibonacci(bx(10)),
            "fib 5"  => fibonacci(bx(5)),
            "fib 12" => fibonacci(bx(12)),
            "fib 8"  => fibonacci(bx(8)),
        )
        .report(|r| {
            r.title("Top 2") // Define a title
                .sort_by_median() // Sort by fastest first
                .filter_n(2) // Top 2 benchmarks
                .print();
        });
    }
}

/*

Report     Top 2
Filters    Sort Mean > Filter N(2)

Benchmark  fib 5
Time       Mean: 9.300ns
Stability  Std.Dev: ± 0.014ns | CV: 0.15%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 2.00%
Location   zench/examples/ex_03.rs:23:9

Benchmark  fib 8
Time       Mean: 40.650ns
Stability  Std.Dev: ± 0.245ns | CV: 0.60%
Samples    Count: 94 | Iters/sample: 524,288 | Outliers: 8.51%
Location   zench/examples/ex_03.rs:23:9

total time: 6.985312904 sec
rust: 1.93.0 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12)
2026-02-19 19:38:26 UTC

*/
