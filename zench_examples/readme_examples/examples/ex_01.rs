fn main() {}

// ================================================================
// Unit test
// ================================================================

#[cfg(test)]
mod tests {

    use zench::bx;
    use zench::Bench;

    fn fibonacci(n: u64) -> u64 {
        match n {
            0 => 1,
            1 => 1,
            n => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }

    #[test]
    fn bench_fibs() {
        let mut b = Bench::new();

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
Time       Mean: 114.015ns
Stability  Std.Dev: ± 0.597ns | CV: 0.52%
Samples    Count: 34 | Iters/sample: 524,288 | Outliers: 2.94%
Location   zench/examples/ex_01.rs:25:11

Benchmark  fib 20
Time       Mean: 14.330µs
Stability  Std.Dev: ± 0.048µs | CV: 0.34%
Samples    Count: 35 | Iters/sample: 4,096 | Outliers: 0.00%
Location   zench/examples/ex_01.rs:29:11

total time: 4.439892718 sec
rust: 1.93.0 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12)
2026-02-19 19:17:16 UTC

*/
