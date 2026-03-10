// How to run:
// cargo bench --package criterion_to_zench --bench loop_format_zench

use zench::bx;
use zench::Bench;

fn main() {
    let mut b = Bench::new();
    let argument = bx("world");

    for i in 0..5 {
        let name = format!("format_hello_world_{i}");
        b.bench(name, || {
            let result = format!("Hello {}", argument);
            bx(result);
        });
    }
}

/* RESULT

                     |       Zench         |     Criterion
---------------------|---------------------|--------|----------------
format_hello_world_0 | 20.154ns | 20.138ns | 20.822 | 20.687 ns
format_hello_world_1 | 20.145ns | 22.725ns | 20.209 | 20.231 ns
format_hello_world_2 | 22.735ns | 20.167ns | 20.779 | 23.498 ns
format_hello_world_3 | 20.153ns | 22.742ns | 20.920 | 22.711 ns
format_hello_world_4 | 22.724ns | 20.148ns | 21.496 | 20.414 ns

Report

Benchmark  format_hello_world_0
Time       Median: 20.154ns
Stability  Std.Dev: ± 0.054ns | CV: 0.27%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 8.00%
Location   zench_examples/criterion_to_zench/benches/loop_format_zench.rs:13:11

Benchmark  format_hello_world_1
Time       Median: 20.145ns
Stability  Std.Dev: ± 0.055ns | CV: 0.28%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 5.00%
Location   zench_examples/criterion_to_zench/benches/loop_format_zench.rs:13:11

Benchmark  format_hello_world_2
Time       Median: 22.735ns
Stability  Std.Dev: ± 0.040ns | CV: 0.17%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench_examples/criterion_to_zench/benches/loop_format_zench.rs:13:11

Benchmark  format_hello_world_3
Time       Median: 20.153ns
Stability  Std.Dev: ± 1.083ns | CV: 5.19%
Samples    Count: 183 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench_examples/criterion_to_zench/benches/loop_format_zench.rs:13:11

Benchmark  format_hello_world_4
Time       Median: 22.724ns
Stability  Std.Dev: ± 0.040ns | CV: 0.17%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 2.00%
Location   zench_examples/criterion_to_zench/benches/loop_format_zench.rs:13:11


total time: 6.640625814 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-08 19:19:43 UTC

*/

/*

Report

Benchmark  format_hello_world_0
Time       Median: 20.138ns
Stability  Std.Dev: ± 0.045ns | CV: 0.22%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 16.00%
Location   zench_examples/criterion_to_zench/benches/loop_format_zench.rs:13:11

Benchmark  format_hello_world_1
Time       Median: 22.725ns
Stability  Std.Dev: ± 0.047ns | CV: 0.21%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 1.00%
Location   zench_examples/criterion_to_zench/benches/loop_format_zench.rs:13:11

Benchmark  format_hello_world_2
Time       Median: 20.167ns
Stability  Std.Dev: ± 0.398ns | CV: 1.95%
Samples    Count: 184 | Iters/sample: 524,288 | Outliers: 14.67%
Location   zench_examples/criterion_to_zench/benches/loop_format_zench.rs:13:11

Benchmark  format_hello_world_3
Time       Median: 22.742ns
Stability  Std.Dev: ± 0.052ns | CV: 0.23%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 1.00%
Location   zench_examples/criterion_to_zench/benches/loop_format_zench.rs:13:11

Benchmark  format_hello_world_4
Time       Median: 20.148ns
Stability  Std.Dev: ± 0.302ns | CV: 1.48%
Samples    Count: 185 | Iters/sample: 524,288 | Outliers: 14.59%
Location   zench_examples/criterion_to_zench/benches/loop_format_zench.rs:13:11


total time: 7.697392582 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-08 19:25:38 UTC

*/
