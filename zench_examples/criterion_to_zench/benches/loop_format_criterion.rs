// How to run:
// cargo bench --package criterion_to_zench --bench loop_format_criterion

use criterion::{criterion_group, criterion_main, Criterion};

fn bench_format(c: &mut Criterion) {
    let argument = std::hint::black_box("world");

    for i in 0..5 {
        let name = format!("format_hello_world_{i}");
        c.bench_function(&name, |b| {
            b.iter(|| {
                let result = format!("Hello {}", argument);
                std::hint::black_box(result);
            });
        });
    }
}

criterion_group!(benches, bench_format);
criterion_main!(benches);

/* RESULT

                     |       Zench         |     Criterion
---------------------|---------------------|--------|----------------
format_hello_world_0 | 20.154ns | 20.138ns | 20.822 | 20.687 ns
format_hello_world_1 | 20.145ns | 22.725ns | 20.209 | 20.231 ns
format_hello_world_2 | 22.735ns | 20.167ns | 20.779 | 23.498 ns
format_hello_world_3 | 20.153ns | 22.742ns | 20.920 | 22.711 ns
format_hello_world_4 | 22.724ns | 20.148ns | 21.496 | 20.414 ns



Benchmarking format_hello_world_0: Collecting 100 samples in est format_hello_world_0
time:   [20.628 ns 20.822 ns 21.074 ns]

Benchmarking format_hello_world_1: Collecting 100 samples in est format_hello_world_1
time:   [20.182 ns 20.209 ns 20.246 ns]
Found 25 outliers among 100 measurements (25.00%)
  25 (25.00%) high mild

Benchmarking format_hello_world_2: Collecting 100 samples in est format_hello_world_2
time:   [20.702 ns 20.779 ns 20.883 ns]

Benchmarking format_hello_world_3: Collecting 100 samples in est format_hello_world_3
time:   [20.774 ns 20.920 ns 21.112 ns]

Benchmarking format_hello_world_4: Collecting 100 samples in est format_hello_world_4
time:   [21.236 ns 21.496 ns 21.818 ns]

*/

/*

Benchmarking format_hello_world_0: Collecting 100 samples in estima format_hello_world_0
time:   [20.598 ns 20.687 ns 20.784 ns]
                        change: [+2.0479% +3.5144% +5.2334%] (p = 0.00 < 0.05)
                        Performance has regressed.

Benchmarking format_hello_world_1: Collecting 100 samples in estima format_hello_world_1
time:   [20.199 ns 20.231 ns 20.273 ns]
                        change: [−0.8417% +0.6691% +2.1414%] (p = 0.39 > 0.05)
                        No change in performance detected.

Benchmarking format_hello_world_2: Collecting 100 samples in estima format_hello_world_2
time:   [23.472 ns 23.498 ns 23.528 ns]
                        change: [+11.398% +12.602% +13.756%] (p = 0.00 < 0.05)
                        Performance has regressed.

Benchmarking format_hello_world_3: Collecting 100 samples in estima format_hello_world_3
time:   [22.213 ns 22.711 ns 23.229 ns]
                        change: [+8.2913% +9.7265% +11.071%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 12 outliers among 100 measurements (12.00%)
  11 (11.00%) low severe
  1 (1.00%) low mild

Benchmarking format_hello_world_4: Collecting 100 samples in estima format_hello_world_4
time:   [20.310 ns 20.414 ns 20.552 ns]
                        change: [+2.6311% +4.4554% +6.2907%] (p = 0.00 < 0.05)
                        Performance has regressed.

*/
