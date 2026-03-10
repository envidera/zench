// How to run:
// cargo bench --package criterion_to_zench --bench fibonacci_criterion

use criterion::{criterion_group, criterion_main, Criterion};
use zench::dev::fibonacci;

fn bench_fibs(c: &mut Criterion) {
    for i in [20, 21].iter() {
        c.bench_function(&format!("Slow {}", i), |b| {
            b.iter(|| {
                let r = fibonacci::slow(std::hint::black_box(*i));
                std::hint::black_box(r);
            })
        });

        c.bench_function(&format!("Fast {}", i), |b| {
            b.iter(|| {
                let r = fibonacci::fast(std::hint::black_box(*i));
                std::hint::black_box(r);
            })
        });
    }
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);

/* RESULT

Zench   | Criterion
------------------------------------
Slow 20 | 13.382µs  | 13.381 µs
Fast 20 |  2.377ns  |  2.2726 ns
Slow 21 | 21.661µs  | 21.655 µs
Fast 21 |  2.592ns  |  2.5234 ns


Benchmarking Slow 20: Collecting 100 samples in estimated 5.0005 Slow 20
time:   [13.379 µs 13.381 µs 13.383 µs]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe

Benchmarking Fast 20: Collecting 100 samples in estimated 5.0000Fast 20
time:   [2.2554 ns 2.2726 ns 2.2897 ns]

Benchmarking Slow 21: Collecting 100 samples in estimated 5.0371Slow 21
time:   [21.653 µs 21.655 µs 21.657 µs]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

Benchmarking Fast 21: Collecting 100 samples in estimated 5.0000Fast 21
time:   [2.5077 ns 2.5234 ns 2.5383 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

*/
