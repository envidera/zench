# zench

[![GitHub](https://img.shields.io/badge/github-envidera%2Fzench-grey.svg?style=flat&logo=github)](https://github.com/envidera/zench)
[![CI](https://github.com/envidera/zench/actions/workflows/ci.yml/badge.svg)](https://github.com/envidera/zench/actions)
[![Documentation](https://docs.rs/zench/badge.svg)](https://docs.rs/zench)
[![Latest version](https://img.shields.io/crates/v/zench.svg)](https://crates.io/crates/zench)
[![Minimum Rust Version](https://img.shields.io/badge/rust-1.87.0%2B-blue.svg?maxAge=3600)](https://www.rust-lang.org/)

Zench is a lightweight benchmarking library for Rust, built for seamless workflow integration, speed, and productivity. Run benchmarks anywhere in your codebase and integrate performance checks directly into your cargo test pipeline.

## Features
- **Benchmark everywhere** - in `src/`, `tests/`, `examples/`, `benches/`, including private functions
- **Data manipulation** - filter, inspect, and act on benchmark results in code
- **Performance Assertions** - warn or panic when performance expectations are not met
- **Cargo-native** - runs with `cargo test` and `cargo bench`
- **Zero dependencies** - pure Rust standard library
- **Stable Rust** -  no nightly required

## Install

```bash
cargo add zench
```
or

```toml
[dependencies]
zench = "0.2.1"
```

## Example


```rust,ignore
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
```

Run the benchmark test

```bash
cargo test bench --release
```

You'll get a detailed report directly in your terminal:

```txt
───────┬───────────┬───────┬────────────┬──────────┬──────────────
 name  │  median   │  cv   │  std.dev   │ outliers │ samples/iters
───────┼───────────┼───────┼────────────┼──────────┼──────────────
fib 10 │ 106.416ns │ 0.22% │  ± 0.233ns │    2.78% │  36 / 524,288
───────┴───────────┴───────┴────────────┴──────────┴──────────────
total time: 2.239029501 sec
rust: 1.94.1 (release) | zench: 0.2.x
```

You can also test many cases at once

```rust,ignore
#[test]
fn bench_fibs() {
    bench!(
        "fib 10" => fibonacci(bx(10)),
        "fib 20" => fibonacci(bx(20)),
        "fib 30" => fibonacci(bx(30)),
    );
}

// Alternative: using a loop

#[test]
fn bench_fibs() {
    let mut b = bench!();

    for case in [10, 20, 30] {
        b.bench(case, || {
            fibonacci(bx(case));
        });
    }
}
```
Result

```txt
───────┬───────────┬───────┬─────────────┬──────────┬──────────────
 name  │  median   │  cv   │   std.dev   │ outliers │ samples/iters
───────┼───────────┼───────┼─────────────┼──────────┼──────────────
fib 10 │ 106.512ns │ 0.39% │   ± 0.415ns │    0.00% │  36 / 524,288
fib 20 │  13.398µs │ 0.35% │   ± 0.047µs │    0.00% │   10 / 16,384
fib 30 │   1.651ms │ 0.41% │   ± 0.007ms │    0.00% │      10 / 128
───────┴───────────┴───────┴─────────────┴──────────┴──────────────
total time: 7.41873511 sec
rust: 1.94.1 (release) | zench: 0.2.x
```


## Data Manipulation

Zench provides access to all benchmark metrics through the report API, allowing you to manipulate them in code.

1. `bench!` measures and prints the default report with benchmark data
2. `Report` (optional) lets you manipulate benchmark data and report output


![img](https://raw.githubusercontent.com/envidera/zench/main/design/export/report_diagram.min.png)

### 1. Report Filtering

Focus on the data that matters by sorting or limiting results.

This example sorts benchmark results, keeps only the two fastest entries, and adds a title to identify the group.

```rust,ignore
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
            .filter_n(2)      // Keep the first two results
            .split();         // split at current state

        faster_group
            .title("Top 2")   // Define a group title
            .print();         // Print the results

        slower_group
            .title("Rest")    // Define a group title
            .print();         // Print the results
    });
}
```

The output will look like this:

```txt
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
rust: 1.94.1 (release) | zench: 0.2.x
```


### 2 - Report issues! (Warn or Panic)

`issue!` is a Zench macro that emits a `warn` diagnostic message by default. It is used as a replacement for `print!`, `debug!`, or `panic!` because its behavior can change depending on configuration.

`issue!` can be configured to panic if the `ZENCH` environment variable is set to `panic`:


```txt
ZENCH=panic
```

Currently, Zench focuses on relative comparisons and regression detection within the same run.

In many cases, you already know the expected baseline or acceptable range for a function, and you can assert that directly in the benchmark.

For example, if a function normally takes around 1 ms, you can simply
fail the test if it exceeds 15% regression.

```rust,ignore
#[test]
fn bench_simple_regression_example() {
    bench!(
        "my func" => sleep(Duration::from_millis(1)),
    )
    .report(|r| {
        r.print();

        // Expected baseline time
        let baseline = Duration::from_millis(1).as_nanos() as f64;
        let tolerance = 0.15; // 15%

        // get the first benchmark time (median)
        let median = r
            .first()
            .unwrap()
            .median();

        let upper = baseline * (1.0 + tolerance);
        let lower = baseline * (1.0 - tolerance);

        if median > upper {
            issue!("relative regression (>15%)");
        }

        if median < lower {
            issue!("performance improvement (>15%)");
        }

        // Note: Ensure the system is in a stable state
        // during benchmarking, as background activity
        // can influence the results.

        // Note: Fixed baseline values may vary across
        // different hardware. Adjust the baseline
        // accordingly for your system.
    });
}
```

### 3. More examples

See [zench_examples/](https://github.com/envidera/zench/blob/main/zench_examples) for a variety of examples.


## Running benchmarks 

Zench integrates with `cargo test` and `cargo bench`

### From the terminal

With cargo test

```bash
# Run only tests with names starting with "bench_"
cargo test bench --release

# or
cargo test bench --release -- --no-capture
```

With cargo bench


```bash
cargo bench

# Also add the traditional benchmark configuration to Cargo.toml
# [[bench]]
# name = ""
# harness = false
```

### From the editor

Run benchmarks directly from your editor by clicking `▶ Run Test`. See the [pre-configured setups](https://github.com/envidera/zench/blob/main/docs/configure-editors.md).




## Zench Goals
| Principle | Description |
| --- | --- | 
| **Primary Goal** | Developer workflow integration
| **Philosophy**	| **Pragmatic** - Fail-fast, detecting performance changes the moment they are introduced.
| **Feedback**	|  **Actionable** - turn metrics into immediate decisions (Pass / Warn / Fail).|


## Zench Limitations

- **Function naming:** It is recommended that Benchmark test functions start with `bench_` to enable filtering via Cargo. Zench follows Cargo's test filtering conventions to distinguish between regular tests and benchmark tests.

```rust
// cargo test bench --release
#[test]
fn bench_fib() {
    // benchmark code
}
```


- **Requires release profile** -  To ensure accurate results. Debug profile includes overhead and lack the optimizations necessary for realistic performance measurements.


- **Compiler Optimization** (Dead Code Elimination) - To prevent the compiler from optimizing away your benchmark code, you must wrap inputs and outputs with `bx()`. This is a thin wrapper around std::hint::black_box, which you can also use directly. 


- **Platform Support** – Currently developed and tested primarily on Linux (Fedora). Feedback from Windows and macOS users is highly appreciated.


- **Shared environments** – Benchmarking on heavily loaded systems, shared CI runners, or virtual machines may produce unstable or "noisy" measurements.

- **Not a profiler** – Zench measures execution time and stability; it does not provide CPU flame graphs or memory allocation analysis.

> Zench is the "alarm" that tells you performance changed. A profiler is the diagnostic tool that helps you find the cause.

- **Documentation** - Still in progress as the API stabilizes.

## Project Status

Zench is in early development. APIs and behavior may change between releases while the project stabilizes.


## Requirements

- Rust stable 1.87.0 or newer ([MSRV](https://github.com/foresterre/cargo-msrv))

## License

Dual-licensed under [MIT](https://opensource.org/licenses/MIT) and [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0)

Copyright (c) 2026-present [Envidera](https://github.com/envidera)
