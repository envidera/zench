fn main() {}

// ================================================================
// Unit test
// ================================================================

#[cfg(test)]
mod bench_tests {

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
        );
    }
}

/*

───────┬───────────┬───────┬────────────┬──────────┬──────────────
 name  │  median   │  cv   │  std.dev   │ outliers │ samples/iters
───────┼───────────┼───────┼────────────┼──────────┼──────────────
fib 10 │ 106.416ns │ 0.22% │  ± 0.233ns │    2.78% │  36 / 524,288
───────┴───────────┴───────┴────────────┴──────────┴──────────────
total time: 2.239029501 sec
rust: 1.94.1 (release) | zench: 0.2.x

*/
