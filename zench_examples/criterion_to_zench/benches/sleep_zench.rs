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

───────────────┬──────────┬───────┬────────────┬──────────┬──────────────
     name      │  median  │  cv   │  std.dev   │ outliers │ samples/iters
───────────────┼──────────┼───────┼────────────┼──────────┼──────────────
time 50 millis │ 50.244ms │ 0.23% │  ± 0.116ms │    0.00% │        10 / 4
───────────────┴──────────┴───────┴────────────┴──────────┴──────────────
total time: 2.471444519 sec
rust: 1.94.0 (release) | zench: 0.1.4

*/
