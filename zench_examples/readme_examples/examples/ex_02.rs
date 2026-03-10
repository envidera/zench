fn main() {}

// ================================================================
// Unit test
// ================================================================

#[cfg(test)]
mod tests {

    fn fibonacci(n: u64) -> u64 {
        match n {
            0 => 1,
            1 => 1,
            n => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    use zench::builder::EngineFixedSamples;
    use zench::bx;
    use zench::Bench;

    #[test]
    fn bench_fibs() {
        let e = EngineFixedSamples::builder()
            .samples(100) // samples count
            .build();

        let mut b = Bench::with_engine(e);

        b.bench("fib 10", || {
            fibonacci(bx(10));
        });

        b.bench("fib 20", || {
            fibonacci(bx(20));
        });
    }
}

/*

Report

Benchmark  fib 10
Time       Median: 113.847ns
Stability  Std.Dev: ± 0.454ns | CV: 0.40%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 4.00%
Location   zench_examples/readme_examples/examples/ex_02.rs:30:11

Benchmark  fib 20
Time       Median: 14.302µs
Stability  Std.Dev: ± 0.037µs | CV: 0.26%
Samples    Count: 100 | Iters/sample: 4,096 | Outliers: 1.00%
Location   zench_examples/readme_examples/examples/ex_02.rs:34:11


total time: 12.099601063 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-08 21:14:37 UTC

*/
