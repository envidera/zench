// How to run:
// cargo bench --package criterion_to_zench --bench fibonacci_zench

use zench::bench;
use zench::bx;
use zench::dev::fibonacci;

fn main() {
    let mut b = bench!();

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

────────┬───────────┬───────┬─────────────┬──────────┬────────────────
 name   │  median   │  cv   │   std.dev   │ outliers │  samples/iters
────────┼───────────┼───────┼─────────────┼──────────┼────────────────
Slow 20 │  13.386µs │ 0.12% │   ± 0.016µs │    0.00% │     10 / 16,384
Fast 20 │   2.206ns │ 3.49% │   ± 0.078ns │    0.20% │ 1,000 / 524,288
Slow 21 │  21.654µs │ 0.20% │   ± 0.044µs │    0.00% │      12 / 8,192
Fast 21 │   2.547ns │ 2.73% │   ± 0.069ns │    0.00% │   100 / 524,288
────────┴───────────┴───────┴─────────────┴──────────┴────────────────
total time: 6.4460668850000005 sec
rust: 1.94.0 (release) | zench: 0.1.4
*/
