// How to run:
// cargo bench --package criterion_to_zench --bench loop_format_zench

use zench::bench;
use zench::bx;

fn main() {
    let mut b = bench!();
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

─────────────────────┬──────────┬───────┬────────────┬──────────┬──────────────
        name         │  median  │  cv   │  std.dev   │ outliers │ samples/iters
─────────────────────┼──────────┼───────┼────────────┼──────────┼──────────────
format_hello_world_0 │ 19.940ns │ 0.80% │  ± 0.159ns │    4.00% │ 100 / 524,288
format_hello_world_1 │ 19.944ns │ 0.91% │  ± 0.182ns │    5.00% │ 100 / 524,288
format_hello_world_2 │ 19.735ns │ 0.66% │  ± 0.131ns │    4.00% │ 100 / 524,288
format_hello_world_3 │ 19.698ns │ 0.21% │  ± 0.041ns │    3.00% │ 100 / 524,288
format_hello_world_4 │ 19.699ns │ 0.28% │  ± 0.056ns │   13.02% │ 192 / 524,288
─────────────────────┴──────────┴───────┴────────────┴──────────┴──────────────
total time: 6.341432619 sec
rust: 1.94.0 (release) | zench: 0.1.4

─────────────────────┬──────────┬───────┬────────────┬──────────┬──────────────
        name         │  median  │  cv   │  std.dev   │ outliers │ samples/iters
─────────────────────┼──────────┼───────┼────────────┼──────────┼──────────────
format_hello_world_0 │ 19.926ns │ 0.50% │  ± 0.100ns │   17.00% │ 100 / 524,288
format_hello_world_1 │ 19.914ns │ 0.28% │  ± 0.057ns │   12.17% │ 189 / 524,288
format_hello_world_2 │ 22.733ns │ 0.30% │  ± 0.069ns │   17.00% │ 100 / 524,288
format_hello_world_3 │ 22.732ns │ 0.21% │  ± 0.047ns │    2.00% │ 100 / 524,288
format_hello_world_4 │ 22.731ns │ 0.12% │  ± 0.027ns │    1.00% │ 100 / 524,288
─────────────────────┴──────────┴───────┴────────────┴──────────┴──────────────
total time: 6.866795759 sec
rust: 1.94.0 (release) | zench: 0.1.4

*/
