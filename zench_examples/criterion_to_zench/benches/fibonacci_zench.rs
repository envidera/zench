// How to run:
// cargo bench --package criterion_to_zench --bench fibonacci_zench

use zench::bx;
use zench::dev::fibonacci;
use zench::Bench;

fn main() {
    let mut b = Bench::new();

    for i in [20, 21].iter() {
        b.bench(format!("Slow {}", i), || {
            let r = fibonacci::slow(bx(*i));
            bx(r);
        });

        b.bench(format!("Fast {}", i), || {
            let r = fibonacci::fast(bx(*i));
            bx(r);
        });
    }
}

/* RESULT

Zench   | Criterion
------------------------------------
Slow 20 | 13.382µs  | 13.381 µs
Fast 20 |  2.377ns  |  2.2726 ns
Slow 21 | 21.661µs  | 21.655 µs
Fast 21 |  2.592ns  |  2.5234 ns

Report

Benchmark  Slow 20
Time       Median: 13.382µs
Stability  Std.Dev: ± 0.022µs | CV: 0.16%
Samples    Count: 37 | Iters/sample: 4,096 | Outliers: 0.00%
Location   zench_examples/criterion_to_zench/benches/fibonacci_zench.rs:12:11

Benchmark  Fast 20
Time       Median: 2.377ns
Stability  Std.Dev: ± 0.005ns | CV: 0.22%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench_examples/criterion_to_zench/benches/fibonacci_zench.rs:17:11

Benchmark  Slow 21
Time       Median: 21.661µs
Stability  Std.Dev: ± 0.011µs | CV: 0.05%
Samples    Count: 23 | Iters/sample: 4,096 | Outliers: 0.00%
Location   zench_examples/criterion_to_zench/benches/fibonacci_zench.rs:12:11

Benchmark  Fast 21
Time       Median: 2.592ns
Stability  Std.Dev: ± 0.006ns | CV: 0.22%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench_examples/criterion_to_zench/benches/fibonacci_zench.rs:17:11


total time: 4.637805659 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-08 19:08:15 UTC

*/

/*


*/
