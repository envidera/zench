// ZENCH=warn cargo test --release --package readme_examples --example ex_04 bench_fib

fn main() {}

// ================================================================
// Unit test
// ================================================================

#[cfg(test)]
mod tests {

    use zench::bench;
    use zench::bx;

    // the function to be benchmarked
    fn fibonacci(n: u64) -> u64 {
        match n {
            0 => 1,
            1 => 1,
            n => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    #[test]
    fn bench_fib() {
        bench!(
            "fib 10" => fibonacci(bx(10))
            // bx() is a thin wrapper around std::hint::black_box.
            // You can use black_box directly if you prefer.
        );
    }
}

/*

Report

Benchmark  fib 10
Time       Median: 106.353ns
Stability  Std.Dev: ± 0.500ns | CV: 0.47%
Samples    Count: 36 | Iters/sample: 524,288 | Outliers: 5.56%
Location   zench_examples/readme_examples/examples/ex_00.rs:26:9


total time: 2.245204719 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-08 20:17:48 UTC

*/
