// Measuring sleep across a wide range of durations

use std::time::Duration;
use zench::Bench;

struct Test {
    name: &'static str,
    dur: Duration,
}

pub(crate) const CASES: &[Test] = &[
    Test {
        name: "1 nano (ns)",
        dur: Duration::from_nanos(1),
    },
    Test {
        name: "10 nanos (ns)",
        dur: Duration::from_nanos(10),
    },
    Test {
        name: "50 nanos (ns)",
        dur: Duration::from_nanos(50),
    },
    Test {
        name: "100 nanos (ns)",
        dur: Duration::from_nanos(100),
    },
    Test {
        name: "500 nanos (ns)",
        dur: Duration::from_nanos(500),
    },
    // -----------------------------------
    Test {
        name: "1 micro (µs)",
        dur: Duration::from_micros(1),
    },
    // -----------------------------------
    // IMPORTANT
    // std::thread::sleep takes approximately 50(µs) micros to start
    // real time ≈ fixed cost (~50 µs) + requested time
    // Below 1 micro the std::thread::sleep time dominates the result
    // -----------------------------------
    Test {
        name: "10 micro (µs)",
        dur: Duration::from_micros(10),
    },
    Test {
        name: "50 micro (µs)",
        dur: Duration::from_micros(50),
    },
    Test {
        name: "100 micro (µs)",
        dur: Duration::from_micros(100),
    },
    Test {
        name: "500 micro (µs)",
        dur: Duration::from_micros(500),
    },
    // -----------------------------------
    Test {
        name: "1 millisecond (ms)",
        dur: Duration::from_millis(1),
    },
    Test {
        name: "10 millis (ms)",
        dur: Duration::from_millis(10),
    },
    Test {
        name: "50 millis (ms)",
        dur: Duration::from_millis(50),
    },
    Test {
        name: "100 millis (ms)",
        dur: Duration::from_millis(100),
    },
    Test {
        name: "500 millis (ms)",
        dur: Duration::from_millis(500),
    },
    // -----------------------------------
    Test {
        name: "1 second (s)",
        dur: Duration::from_secs(1),
    },
    Test {
        name: "5 second (s)",
        dur: Duration::from_secs(5),
    },
    Test {
        name: "10 second (s)",
        dur: Duration::from_secs(10),
    },
];

#[ignore = "display purpose"]
#[test]
fn timing_range() {
    let mut b = bench!();

    for t in CASES {
        b.bench(t.name, || {
            std::thread::sleep(t.dur);
        });
    }
    b.report(|r| {
        r.print();
    });
}

/*

Report

Benchmark  1 nano (ns)
Time       Median: 52.748µs
Stability  Std.Dev: ± 0.202µs | CV: 0.38%
Samples    Count: 37 | Iters/sample: 1,024 | Outliers: 8.11%
Location   zench/tests/display_timing_sleep_range.rs:100:11

Benchmark  10 nanos (ns)
Time       Median: 52.821µs
Stability  Std.Dev: ± 0.219µs | CV: 0.41%
Samples    Count: 37 | Iters/sample: 1,024 | Outliers: 2.70%
Location   zench/tests/display_timing_sleep_range.rs:100:11

Benchmark  50 nanos (ns)
Time       Median: 52.740µs
Stability  Std.Dev: ± 0.162µs | CV: 0.31%
Samples    Count: 37 | Iters/sample: 1,024 | Outliers: 2.70%
Location   zench/tests/display_timing_sleep_range.rs:100:11

Benchmark  100 nanos (ns)
Time       Median: 52.791µs
Stability  Std.Dev: ± 0.152µs | CV: 0.29%
Samples    Count: 37 | Iters/sample: 1,024 | Outliers: 0.00%
Location   zench/tests/display_timing_sleep_range.rs:100:11

Benchmark  500 nanos (ns)
Time       Median: 54.644µs
Stability  Std.Dev: ± 0.409µs | CV: 0.75%
Samples    Count: 36 | Iters/sample: 1,024 | Outliers: 0.00%
Location   zench/tests/display_timing_sleep_range.rs:100:11

Benchmark  1 micro (µs)
Time       Median: 54.961µs
Stability  Std.Dev: ± 0.356µs | CV: 0.65%
Samples    Count: 36 | Iters/sample: 1,024 | Outliers: 2.78%
Location   zench/tests/display_timing_sleep_range.rs:100:11

Benchmark  10 micro (µs)
Time       Median: 62.636µs
Stability  Std.Dev: ± 0.163µs | CV: 0.26%
Samples    Count: 32 | Iters/sample: 1,024 | Outliers: 0.00%
Location   zench/tests/display_timing_sleep_range.rs:100:11

Benchmark  50 micro (µs)
Time       Median: 102.689µs
Stability  Std.Dev: ± 0.690µs | CV: 0.67%
Samples    Count: 39 | Iters/sample: 512 | Outliers: 0.00%
Location   zench/tests/display_timing_sleep_range.rs:100:11

Benchmark  100 micro (µs)
Time       Median: 153.857µs
Stability  Std.Dev: ± 0.216µs | CV: 0.14%
Samples    Count: 26 | Iters/sample: 512 | Outliers: 0.00%
Location   zench/tests/display_timing_sleep_range.rs:100:11

Benchmark  500 micro (µs)
Time       Median: 555.268µs
Stability  Std.Dev: ± 0.226µs | CV: 0.04%
Samples    Count: 29 | Iters/sample: 128 | Outliers: 0.00%
Location   zench/tests/display_timing_sleep_range.rs:100:11

Benchmark  1 millisecond (ms)
Time       Median: 1.056ms
Stability  Std.Dev: ± 0.000ms | CV: 0.04%
Samples    Count: 30 | Iters/sample: 64 | Outliers: 0.00%
Location   zench/tests/display_timing_sleep_range.rs:100:11

Benchmark  10 millis (ms)
Time       Median: 10.182ms
Stability  Std.Dev: ± 0.048ms | CV: 0.47%
Samples    Count: 25 | Iters/sample: 8 | Outliers: 0.00%
Location   zench/tests/display_timing_sleep_range.rs:100:11

Benchmark  50 millis (ms)
Time       Median: 50.202ms
Stability  Std.Dev: ± 0.120ms | CV: 0.24%
Samples    Count: 20 | Iters/sample: 2 | Outliers: 0.00%
Location   zench/tests/display_timing_sleep_range.rs:100:11

Benchmark  100 millis (ms)
Time       Median: 100.206ms
Stability  Std.Dev: ± 0.094ms | CV: 0.09%
Samples    Count: 10 | Iters/sample: 2 | Outliers: 0.00%
Location   zench/tests/display_timing_sleep_range.rs:100:11

Benchmark  500 millis (ms)
Time       Median: 500.336ms
Stability  Std.Dev: ± 0.117ms | CV: 0.02%
Samples    Count: 4 | Iters/sample: 1 | Outliers: 0.00%
Location   zench/tests/display_timing_sleep_range.rs:100:11

Benchmark  1 second (s)
Time       Median: 1000.217ms
Stability  Std.Dev: ± 0.140ms | CV: 0.01%
Samples    Count: 2 | Iters/sample: 1 | Outliers: 0.00%
Location   zench/tests/display_timing_sleep_range.rs:100:11

Benchmark  5 second (s)
Time       Median: 5000.083ms
Stability  Std.Dev: ± 0.000ms | CV: 0.00%
Samples    Count: 1 | Iters/sample: 1 | Outliers: 0.00%
Location   zench/tests/display_timing_sleep_range.rs:100:11

Benchmark  10 second (s)
Time       Median: 10000.082ms
Stability  Std.Dev: ± 0.000ms | CV: 0.00%
Samples    Count: 1 | Iters/sample: 1 | Outliers: 0.00%
Location   zench/tests/display_timing_sleep_range.rs:100:11


total time: 65.873559033 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-02 20:22:44 UTC

*/
