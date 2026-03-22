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

Benchmarking Slow 20: Collecting 100 samples in estimated 5.0111 s (419k iterationSlow 20
time:   [11.943 µs 11.946 µs 11.950 µs]
                        change: [−0.1944% −0.1019% −0.0140%] (p = 0.03 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe

Benchmarking Fast 20: Collecting 100 samples in estimated 5.0000 s (2.2B iterationFast 20
time:   [2.2658 ns 2.2702 ns 2.2745 ns]
                        change: [−0.3820% −0.1063% +0.1801%] (p = 0.49 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  2 (2.00%) high mild

Benchmarking Slow 21: Collecting 100 samples in estimated 5.0762 s (263k iterationSlow 21
time:   [19.324 µs 19.327 µs 19.330 µs]
                        change: [−0.3285% −0.1877% −0.0746%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

Benchmarking Fast 21: Collecting 100 samples in estimated 5.0000 s (2.0B iterationFast 21
time:   [2.5445 ns 2.5480 ns 2.5514 ns]
                        change: [−0.1075% +0.1555% +0.4123%] (p = 0.27 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe

*/
