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
            "fib 20" => fibonacci(bx(20)),
            "fib 30" => fibonacci(bx(30)),
        );
    }
}

/*

Report

Benchmark  fib 10
Time       Mean: 114.179ns
Stability  Std.Dev: ± 0.686ns | CV: 0.60%
Samples    Count: 34 | Iters/sample: 524,288 | Outliers: 8.82%
Location   zench/examples/ex_00b.rs:23:9

Benchmark  fib 20
Time       Mean: 14.298µs
Stability  Std.Dev: ± 0.013µs | CV: 0.09%
Samples    Count: 35 | Iters/sample: 4,096 | Outliers: 0.00%
Location   zench/examples/ex_00b.rs:23:9

Benchmark  fib 30
Time       Mean: 1.759ms
Stability  Std.Dev: ± 0.001ms | CV: 0.07%
Samples    Count: 36 | Iters/sample: 32 | Outliers: 0.00%
Location   zench/examples/ex_00b.rs:23:9

total time: 6.584406126 sec
rust: 1.93.0 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12)
2026-02-19 19:15:10 UTC

*/
