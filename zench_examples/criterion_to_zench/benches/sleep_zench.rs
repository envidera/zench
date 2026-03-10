// How to run:
// cargo bench --package criterion_to_zench --bench sleep_zench

use std::time::Duration;
use zench::bench;

fn main() {
    bench!(
        "time 50 millis" => {
            std::thread::sleep(Duration::from_millis(50));
        }
    );
}

/* RESULT
               | Zench    | Criterion
---------------------------------------
time 50 millis | 50.226ms | 50.263 ms

Report

Benchmark  time 50 millis
Time       Median: 50.226ms
Stability  Std.Dev: ± 0.117ms | CV: 0.23%
Samples    Count: 20 | Iters/sample: 2 | Outliers: 0.00%
Location   zench_examples/criterion_to_zench/benches/sleep_zench.rs:8:5


total time: 2.277408834 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-08 19:35:37 UTC

*/
