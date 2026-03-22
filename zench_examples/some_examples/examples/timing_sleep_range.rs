fn main() {}

// ================================================================
// Unit performance test
// ================================================================
#[cfg(test)]
mod test_performance {
    use std::time::Duration;
    use zench::bench;

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

    // Measuring sleep across a wide range of durations
    #[test]
    fn bench_timing_range() {
        let mut b = bench!();

        for t in CASES {
            b.bench(t.name, || {
                std::thread::sleep(t.dur);
            });
        }
        b.report(|r| {
            let (mut same, mut rest) = r
                .filter_pct(10.0)
                .split();

            same.title("initialization dominates the time")
                .print();

            rest.print();
        });
    }
}

/*
std::thread::sleep takes approximately 50(µs) micros to start
real time ≈ fixed cost (~50 µs) + requested time
Below 1 micro the std::thread::sleep time dominates the result


initialization dominates the time > Filter Percentage(10%)
───────────────┬───────────┬───────┬─────────────┬──────────┬──────────────
     name      │  median   │  cv   │   std.dev   │ outliers │ samples/iters
───────────────┼───────────┼───────┼─────────────┼──────────┼──────────────
1 nano (ns)    │  52.703µs │ 0.07% │   ± 0.037µs │    0.00% │    10 / 4,096
10 nanos (ns)  │  52.685µs │ 0.07% │   ± 0.039µs │    0.00% │    10 / 4,096
50 nanos (ns)  │  52.694µs │ 0.14% │   ± 0.072µs │    0.00% │    10 / 4,096
100 nanos (ns) │  52.701µs │ 0.11% │   ± 0.058µs │    0.00% │    10 / 4,096
500 nanos (ns) │  53.398µs │ 0.39% │   ± 0.206µs │   10.00% │    10 / 4,096
1 micro (µs)   │  55.255µs │ 0.43% │   ± 0.235µs │    0.00% │     9 / 4,096
───────────────┴───────────┴───────┴─────────────┴──────────┴──────────────

───────────────────┬─────────────┬───────┬─────────────┬──────────┬──────────────
       name        │   median    │  cv   │   std.dev   │ outliers │ samples/iters
───────────────────┼─────────────┼───────┼─────────────┼──────────┼──────────────
10 micro (µs)      │    62.541µs │ 0.30% │   ± 0.186µs │    0.00% │     8 / 4,096
50 micro (µs)      │   101.100µs │ 0.42% │   ± 0.425µs │    0.00% │    10 / 2,048
100 micro (µs)     │   153.925µs │ 0.10% │   ± 0.150µs │    0.00% │    13 / 1,024
500 micro (µs)     │   554.647µs │ 0.02% │   ± 0.115µs │    0.00% │       8 / 512
1 millisecond (ms) │     1.055ms │ 0.03% │   ± 0.000ms │   25.00% │       8 / 256
10 millis (ms)     │    10.275ms │ 0.68% │   ± 0.069ms │    0.00% │       13 / 16
50 millis (ms)     │    50.306ms │ 0.19% │   ± 0.097ms │    0.00% │        10 / 4
100 millis (ms)    │   100.308ms │ 0.14% │   ± 0.144ms │    0.00% │        10 / 2
500 millis (ms)    │   500.083ms │ 0.02% │   ± 0.123ms │    0.00% │         4 / 1
1 second (s)       │  1000.367ms │ 0.00% │   ± 0.001ms │    0.00% │         2 / 1
5 second (s)       │  5000.371ms │ 0.00% │   ± 0.000ms │    0.00% │         1 / 1
10 second (s)      │ 10000.328ms │ 0.00% │   ± 0.000ms │    0.00% │         1 / 1
───────────────────┴─────────────┴───────┴─────────────┴──────────┴──────────────
total time: 71.198411219 sec
rust: 1.94.0 (release) | zench: 0.1.4

*/
