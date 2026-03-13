# Performance testing with programmable results

Zench is a lightweight Rust benchmark library that works anywhere in your codebase: `src/`, `tests/`, `examples/`, or `benches/`. Run performance tests with `cargo test`, then filter, assert, and react to benchmark results directly in code.


[![GitHub](https://img.shields.io/badge/github-envidera%2Fzench-grey.svg?style=flat&logo=github)](https://github.com/envidera/zench)
[![CI](https://github.com/envidera/zench/actions/workflows/ci.yml/badge.svg)](https://github.com/envidera/zench/actions)
[![Documentation](https://docs.rs/zench/badge.svg)](https://docs.rs/zench)
[![Latest version](https://img.shields.io/crates/v/zench.svg)](https://crates.io/crates/zench)
[![Minimum Rust Version](https://img.shields.io/badge/rust-1.87.0%2B-blue.svg?maxAge=3600)](https://www.rust-lang.org/)
![Rust Stable](https://img.shields.io/badge/rust-stable-blue.svg)

## Features
- **Benchmark everywhere** - in `src/`, `tests/`, `examples/`, `benches/`
- **Benchmark private functions** - directly inside unit tests
- **Cargo-native** - runs with `cargo test` and `cargo bench`.
- **Auto-scaling measurement** - benchmark from nanoseconds, to several seconds
- **Configurable** - fine-tune to your project's specific needs.
- **Programmable results** - filter, inspect, and react to benchmark data in code
- **Performance Assertions** - warn or fail when performance expectations are not met
- **Zero dependencies** - pure Rust standard library.
- **Stable Rust** -  no nightly required.  


## Install

```bash
cargo add zench --dev
```
or

```toml
[dev-dependencies]
zench = "0.1.1"
```

## Example

**As easy as:**


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
        // bx() is a thin wrapper around std::hint::black_box.
        // You can use black_box directly if you prefer.
    );
}
```

Run the benchmark test

```bash
ZENCH=warn cargo test --release -- --no-capture
```

You'll get a detailed report directly in your terminal:

```txt
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
```

[source](zench_examples/readme_examples/examples/ex_00.rs)

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
```
[source](zench_examples/readme_examples/examples/ex_00b.rs)


## Advanced Usage

Zench scales with your needs, offering full control over the benchmarking engine and reporting.

See examples below.

### 1. Manual Control (No Macro)

If you prefer not to use macros, use the Bench type:

```rust,ignore
use zench::Bench;
use zench::bx;

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
```
[source](zench_examples/readme_examples/examples/ex_01.rs)

### 2. Custom Engine Builder

Fine-tune how samples are collected, Zench provides three engines for this purpose:

- `EngineAuto` - Fully automatic, stability-based (measures until results are stable enough)
- `EngineFixedSamples` - Uses a fixed sample count
- `EngineFullFixed` - Uses fixed values for sample count and Iters/sample values

In this example, we will use `EngineFixedSamples` to define a fixed number of samples.

```rust,ignore
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
```

Partial output

```txt
Benchmark  fib 10
Time       Median: 113.847ns
Stability  Std.Dev: ± 0.454ns | CV: 0.40%
Samples    Count: 100 <<--- here

Benchmark  fib 20
Time       Median: 14.302µs
Stability  Std.Dev: ± 0.037µs | CV: 0.26%
Samples    Count: 100 <<--- here
```

[source](zench_examples/readme_examples/examples/ex_02.rs)

### 3. Report Filtering

Fine-tune how benchmark results are processed. Focus on the data that matters by sorting or limiting results.

In this example we will:

- Define a report title - used to identify the benchmark group.
- Apply two filters to refine the results:
  - 1) Sort results by execution time (fastest to slowest)
  - 2) Keep only the first two benchmarks

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
        r.title("Top 2") // Define a title
            .sort_by_median() // Sort by fastest first
            .filter_n(2) // Top 2 benchmarks
            .print();
    });
}
```
[source](zench_examples/readme_examples/examples/ex_03.rs)

The output will look like this:

```txt
Report     Top 2                     <<--- our title
Filters    Sort Median > Filter N(2) <<--- applied filters

                                     <<--- our two top benchs
                                           ordered by time
Benchmark  fib 5
Time       Median: 9.294ns
Stability  Std.Dev: ± 0.019ns | CV: 0.21%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 2.00%
Location   zench_examples/readme_examples/examples/ex_03.rs:23:9

Benchmark  fib 8
Time       Median: 40.243ns
Stability  Std.Dev: ± 0.110ns | CV: 0.27%
Samples    Count: 95 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench_examples/readme_examples/examples/ex_03.rs:23:9


total time: 7.124152612 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-10 10:45:35 UTC
```


### 4. Programmable Reports

Fine-tune how benchmarks can trigger custom logic, and warn or fail tests when performance expectations are not met.

With full access to benchmark data, you can:

- Read metrics
- Apply custom logic
- Decide whether a test should fail or pass
- Generate custom output
- Integrate with CI pipelines
- Trigger automated actions


In this example, we will:
- Compare three implementations of the same algorithm for squaring the elements of a Vec
- Determine the fastest version
- `issue!` if the known fastest version is no longer the fastest.

**issue!**

Is a Zench macro that emits a diagnostic message that either warns or panics, depending on the `ZENCH` environment variable.



```rust,ignore
// three implementations of the same algorithm

pub fn square_loop(data: &[u64]) -> Vec<u64> {
    let mut out = Vec::with_capacity(data.len());
    for &v in data {
        out.push(v * v);
    }
    out
}

pub fn square_iterator(data: &[u64]) -> Vec<u64> {
    data.iter()
        .map(|&v| v * v)
        .collect()
}

pub fn square_fold(data: &[u64]) -> Vec<u64> {
    data.iter()
        .fold(Vec::with_capacity(data.len()), |mut acc, &v| {
            acc.push(v * v);
            acc
        })
}

#[test]
fn bench_fastest_version() {
    use zench::bench;
    use zench::bx;
    
    // Use the `issue!` macro.
    use zench::issue;

    let data: Vec<u64> = (0..100_000).collect();

    bench!(
        "loop" => bx(square_loop(bx(&data))),
        "iterator" => bx(square_iterator(bx(&data))),
        "fold" => bx(square_fold(bx(&data))),
    )
    .report(|r| {
        // For this benchmark, we consider performance roughly equal
        // when the time difference between implementations is within 10%.
        // Benchmarks within this range are grouped as `faster_group`,
        // and the remaining ones as `slower_group`.
        let (mut faster_group, mut slower_group) = r
            .sort_by_median()
            .filter_proximity_pct(10.0)
            // Split the current filtered state from the remaining 
            // benchmarks
            .split();

        // We expect only one benchmark in the fastest group; 
        // issue if more are present
        if faster_group.len() > 1 {
            issue!("some implementations changed performance");
        }

        // We expect the benchmark named "iterator" to be the fastest; 
        // issue if it is not
        if !faster_group
            .first()
            .unwrap()
            .name()
            .contains("iterator")
        {
            issue!("the iterator is no longer the fastest");
        }

        faster_group
            .title("Faster group")
            .print();

        slower_group
            .title("Slower group")
            .print();
    });
}
```
Run the benchmark test

```bash
ZENCH=warn cargo test --release -- --no-capture
```

You'll get a detailed report

```txt
Report     Faster group
Filters    Sort Median > Filter Proximity(10%)

Benchmark  iterator
Time       Median: 19.140µs
Stability  Std.Dev: ± 0.936µs | CV: 4.94%
Samples    Count: 7 | Iters/sample: 16,384 | Outliers: 0.00%
Location   zench_examples/readme_examples/examples/ex_04.rs:48:9



Report     Slower group

Benchmark  loop
Time       Median: 55.502µs
Stability  Std.Dev: ± 0.198µs | CV: 0.36%
Samples    Count: 9 | Iters/sample: 4,096 | Outliers: 0.00%
Location   zench_examples/readme_examples/examples/ex_04.rs:48:9

Benchmark  fold
Time       Median: 114.093µs
Stability  Std.Dev: ± 0.230µs | CV: 0.20%
Samples    Count: 9 | Iters/sample: 2,048 | Outliers: 0.00%
Location   zench_examples/readme_examples/examples/ex_04.rs:48:9


total time: 7.926189331 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-09 01:25:21 UTC
```

**Warn and Panic**

If you run the benchmark test with `warn`, and an issue! is triggered, the output will display warnings like this:

```txt
ZENCH=warn cargo test --release

bench::issue::warn some implementations have changed performance
--> zench_examples/readme_examples/examples/ex_04.rs:66:17

bench::issue::warn the iterator is no longer the fastest
--> zench_examples/readme_examples/examples/ex_04.rs:77:17
```

If you run the benchmark tests with `panic`, and an issue! is triggered, the output will display the messages and panic, like this:

```txt
ZENCH=panic cargo test --release

bench::issue::panic some implementations have changed performance
--> zench_examples/readme_examples/examples/ex_04.rs:66:17

bench::issue::panic the iterator is no longer the fastest
--> zench_examples/readme_examples/examples/ex_04.rs:77:17
``` 

[source](zench_examples/readme_examples/examples/ex_04.rs)


### 5. More examples

See [zench_examples/](zench_examples) for a variety of examples.



## Running benchmarks

Zench integrates with `cargo test` and `cargo bench`, but ignores all benchmarks by default.
Benchmarks run only when the following conditions are met:

- The code is executed under the `release` profile
- The `ZENCH` environment variable is set

### In `src/`, `tests/`, `examples/` with `#[test]`

```bash
# Runs Standard Tests, including Zench Tests
ZENCH=warn cargo test --release
ZENCH=panic cargo test --release

# Runs Standard Tests, Zench Tests are ignored
cargo test 
cargo test --release

# No additional setup is required
```

### In `/benches`

```bash
# Runs Standard Benchmarks, including Zench Benchmarks
ZENCH=warn cargo bench 
ZENCH=panic cargo bench 

# Runs Standard Benchmarks, Zench Benchmarks are ignored
cargo bench 

# Need to be configured in Cargo.toml
# [[bench]]
# name = ""
# harness = false
```

> Note: `cargo bench` runs in release mode by default.

> Note: Use `-- --no-capture` to see the benchmark report results.
>
> ZENCH=warn cargo test --release -- --no-capture

### Run in editor

Run benchmarks directly from your editor by clicking `▶ Run Test`. See the [pre-configured setups](docs/configure-editors.md).


### Run in CI
With a custom .github/workflows action
>> todo!


## Zench Goals
| Principle | Description |
| --- | --- | 
| **Primary Goal** | Developer workflow integration
| **Philosophy**	| **Pragmatic** - Fail-fast, detecting performance changes the moment they are introduced.
| **Feedback**	|  **Actionable** - turn metrics into immediate decisions (Pass / Warn / Fail).|


## Zench Limitations

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
