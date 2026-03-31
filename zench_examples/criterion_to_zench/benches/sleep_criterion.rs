// How to run:
// cargo bench --package criterion_to_zench --bench sleep_criterion

use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;

fn bench_format(c: &mut Criterion) {
    c.bench_function("time 50 millis", |b| {
        b.iter(|| std::thread::sleep(Duration::from_millis(50)));
    });
}

criterion_group!(benches, bench_format);
criterion_main!(benches);

/* RESULT

Benchmarking time 50 millis: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s.
You may wish to increase target time to 5.0s, or reduce sample count to 90.
Benchmarking time 50 millis: Collecting 100 samples in estimated 5.time 50 millis
time:   [50.232 ms 50.263 ms 50.293 ms]

*/
