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
            let (mut faster_group, mut slower_group) = r
                .sort_by_median() // Sort benchmarks by median time
                .filter_n(2) // Keep the first two results
                .split();

            faster_group
                .title("Top 2") // Define a group title
                .print(); // Print the results

            slower_group
                .title("Rest")
                .print();
        });
    }
}

/*

Top 2 > Sort Median > Filter N(2)
──────┬──────────┬───────┬────────────┬──────────┬──────────────
name  │  median  │  cv   │  std.dev   │ outliers │ samples/iters
──────┼──────────┼───────┼────────────┼──────────┼──────────────
fib 5 │  9.299ns │ 0.58% │  ± 0.054ns │    4.00% │ 100 / 524,288
fib 8 │ 40.298ns │ 0.39% │  ± 0.157ns │    4.21% │  95 / 524,288
──────┴──────────┴───────┴────────────┴──────────┴──────────────

Rest
───────┬───────────┬───────┬────────────┬──────────┬──────────────
 name  │  median   │  cv   │  std.dev   │ outliers │ samples/iters
───────┼───────────┼───────┼────────────┼──────────┼──────────────
fib 10 │ 106.412ns │ 0.39% │  ± 0.416ns │   13.89% │  36 / 524,288
fib 12 │ 279.791ns │ 0.66% │  ± 1.852ns │    0.00% │  14 / 524,288
───────┴───────────┴───────┴────────────┴──────────┴──────────────
total time: 7.075344703 sec
rust: 1.94.1 (release) | zench: 0.1.4

*/
