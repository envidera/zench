fn main() {}

// ================================================================
// Xtring, a simple experiment in a faster String implementation
//
// Current status:
// It's definitely simple. Faster is still loading...
// [X] Simple  [ ] Faster
// ================================================================

#[allow(unused)]
pub(crate) mod laboratory {

    use std::io;
    use std::io::Write;

    pub(crate) struct Xtring<'a> {
        data: Vec<&'a [u8]>,
    }

    impl<'a> Xtring<'a> {
        pub fn new() -> Self {
            Self { data: Vec::new() }
        }

        pub fn push(&mut self, segment: &'a str) {
            self.data
                .push(segment.as_bytes());
        }

        pub fn with_capacity(capacity: usize) -> Self {
            Self {
                data: Vec::with_capacity(capacity),
            }
        }

        pub fn print(&self) {
            let mut writer = std::io::BufWriter::new(io::sink());
            self.write_to(&mut writer)
                .unwrap();
        }

        pub fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
            for part in &self.data {
                writer.write_all(part)?;
            }
            Ok(())
        }
    }
}

// ================================================================
// Unit test
// ================================================================
#[cfg(test)]
mod tests {
    use super::laboratory::Xtring;
    use std::io;
    use std::io::Write;
    use zench::bench;
    use zench::bx;

    #[test]
    fn print_to_null() {
        let mut x = Xtring::new();
        x.push("segment");
        x.print();
    }

    #[test]
    fn bench_xtring() {
        let mut writer = std::io::BufWriter::new(std::io::sink());

        #[rustfmt::skip]
        const CASES: &[(&str, usize)] = &[
            ("micro", 3),
            ("small", 10),
            ("small_1", 25),
            ("small_2", 50),
            ("small_3",100),
            ("mid", 1_000),
            ("mid_2", 10_000),
            ("large", 100_000),
            ("large_2", 1_000_000),
        ];

        for (name, iter) in CASES {
            let mut pool = Vec::new();
            for i in 0..*iter {
                pool.push(format!("log_event_{}: status_ok", i));
            }

            bench!(
                "String" =>{
                     let mut s = String::new();
                     for p in &pool {
                         s.push_str(p);
                     }

                     writer.write_all(s.as_bytes()).unwrap();
                     writer.flush().unwrap();
                },

                "String with_capacity" =>{
                     let mut s = String::with_capacity(pool.capacity());
                     for p in &pool {
                         s.push_str(p);
                     }
                     writer.write_all(s.as_bytes()).unwrap();
                     writer.flush().unwrap();
                },
                // ----------------------------------------------------------------

                "Xtring" =>{
                     let mut x = Xtring::new();
                     for p in &pool {
                         x.push(p);
                     }
                     x.write_to(&mut writer).unwrap();
                     writer.flush().unwrap();
                },

                "Xtring.print()" =>{
                     let mut x = Xtring::new();
                     for p in &pool {
                         x.push(p);
                     }
                     x.print();
                },

                "Xtring with_capacity" =>{
                     let mut x = Xtring::with_capacity(pool.capacity());
                     for p in &pool {
                         x.push(p);
                     }
                     x.write_to(&mut writer).unwrap();
                     writer.flush().unwrap();
                },
            )
            .report(|r| {
                r.title(format!("{name}:{iter}"))
                    .sort_by_median()
                    .print();
            });
        }
    }
}

/*
micro:3 > Sort Median
─────────────────────┬──────────┬───────┬────────────┬──────────┬──────────────
        name         │  median  │  cv   │  std.dev   │ outliers │ samples/iters
─────────────────────┼──────────┼───────┼────────────┼──────────┼──────────────
Xtring with_capacity │ 17.973ns │ 0.42% │  ± 0.076ns │    7.00% │ 100 / 524,288
Xtring               │ 24.482ns │ 0.47% │  ± 0.116ns │    1.00% │ 100 / 524,288
Xtring.print()       │ 51.252ns │ 0.65% │  ± 0.335ns │    0.00% │  75 / 524,288
String               │ 56.872ns │ 0.36% │  ± 0.204ns │    5.97% │  67 / 524,288
String with_capacity │ 71.398ns │ 0.27% │  ± 0.196ns │    1.85% │  54 / 524,288
─────────────────────┴──────────┴───────┴────────────┴──────────┴──────────────

small:10 > Sort Median
─────────────────────┬───────────┬───────┬────────────┬──────────┬──────────────
        name         │  median   │  cv   │  std.dev   │ outliers │ samples/iters
─────────────────────┼───────────┼───────┼────────────┼──────────┼──────────────
Xtring with_capacity │  38.849ns │ 0.34% │  ± 0.131ns │    2.02% │  99 / 524,288
Xtring               │ 124.943ns │ 0.24% │  ± 0.304ns │    0.00% │  31 / 524,288
String               │ 135.669ns │ 0.26% │  ± 0.355ns │    0.00% │  29 / 524,288
String with_capacity │ 148.643ns │ 0.36% │  ± 0.540ns │    0.00% │  26 / 524,288
Xtring.print()       │ 159.455ns │ 0.59% │  ± 0.935ns │    0.00% │  24 / 524,288
─────────────────────┴───────────┴───────┴────────────┴──────────┴──────────────

small_1:25 > Sort Median
─────────────────────┬───────────┬───────┬────────────┬──────────┬──────────────
        name         │  median   │  cv   │  std.dev   │ outliers │ samples/iters
─────────────────────┼───────────┼───────┼────────────┼──────────┼──────────────
Xtring with_capacity │  84.745ns │ 0.27% │  ± 0.231ns │    4.44% │  45 / 524,288
String               │ 223.395ns │ 0.08% │  ± 0.176ns │    0.00% │  18 / 524,288
String with_capacity │ 228.086ns │ 0.20% │  ± 0.462ns │    0.00% │  17 / 524,288
Xtring.print()       │ 237.765ns │ 0.60% │  ± 1.419ns │    0.00% │  17 / 524,288
Xtring               │ 244.353ns │ 1.22% │  ± 3.000ns │    0.00% │  16 / 524,288
─────────────────────┴───────────┴───────┴────────────┴──────────┴──────────────

small_2:50 > Sort Median
─────────────────────┬───────────┬───────┬────────────┬──────────┬──────────────
        name         │  median   │  cv   │  std.dev   │ outliers │ samples/iters
─────────────────────┼───────────┼───────┼────────────┼──────────┼──────────────
Xtring with_capacity │ 174.606ns │ 0.26% │  ± 0.456ns │    0.00% │  22 / 524,288
String with_capacity │ 309.387ns │ 0.24% │  ± 0.738ns │    7.69% │  13 / 524,288
String               │ 340.719ns │ 0.65% │  ± 2.203ns │    8.33% │  12 / 524,288
Xtring.print()       │ 374.401ns │ 0.29% │  ± 1.092ns │    0.00% │  11 / 524,288
Xtring               │ 380.420ns │ 0.26% │  ± 1.009ns │    9.09% │  11 / 524,288
─────────────────────┴───────────┴───────┴────────────┴──────────┴──────────────

small_3:100 > Sort Median
─────────────────────┬───────────┬───────┬────────────┬──────────┬──────────────
        name         │  median   │  cv   │  std.dev   │ outliers │ samples/iters
─────────────────────┼───────────┼───────┼────────────┼──────────┼──────────────
Xtring with_capacity │ 365.578ns │ 0.60% │  ± 2.198ns │    9.09% │  11 / 524,288
String with_capacity │ 449.928ns │ 0.10% │  ± 0.465ns │    0.00% │   9 / 524,288
String               │ 465.769ns │ 0.81% │  ± 3.748ns │    0.00% │   9 / 524,288
Xtring.print()       │ 561.551ns │ 0.16% │  ± 0.877ns │    0.00% │   7 / 524,288
Xtring               │ 641.139ns │ 0.52% │  ± 3.340ns │    0.00% │  12 / 262,144
─────────────────────┴───────────┴───────┴────────────┴──────────┴──────────────

mid:1000 > Sort Median
─────────────────────┬──────────┬───────┬─────────────┬──────────┬──────────────
        name         │  median  │  cv   │   std.dev   │ outliers │ samples/iters
─────────────────────┼──────────┼───────┼─────────────┼──────────┼──────────────
String               │  2.629µs │ 0.19% │   ± 0.005µs │    0.00% │   12 / 65,536
String with_capacity │  2.898µs │ 0.15% │   ± 0.004µs │    0.00% │   11 / 65,536
Xtring with_capacity │  3.502µs │ 0.32% │   ± 0.011µs │    0.00% │    9 / 65,536
Xtring.print()       │  3.736µs │ 0.60% │   ± 0.022µs │    0.00% │    9 / 65,536
Xtring               │  4.879µs │ 0.19% │   ± 0.009µs │    0.00% │   13 / 32,768
─────────────────────┴──────────┴───────┴─────────────┴──────────┴──────────────

mid_2:10000 > Sort Median
─────────────────────┬───────────┬───────┬─────────────┬──────────┬──────────────
        name         │  median   │  cv   │   std.dev   │ outliers │ samples/iters
─────────────────────┼───────────┼───────┼─────────────┼──────────┼──────────────
String               │  25.320µs │ 0.51% │   ± 0.128µs │   20.00% │    10 / 8,192
String with_capacity │  28.152µs │ 0.34% │   ± 0.096µs │    0.00% │     9 / 8,192
Xtring.print()       │  34.604µs │ 0.30% │   ± 0.103µs │   12.50% │     8 / 8,192
Xtring with_capacity │  36.914µs │ 1.44% │   ± 0.526µs │    0.00% │    14 / 4,096
Xtring               │  42.466µs │ 0.41% │   ± 0.175µs │    0.00% │    12 / 4,096
─────────────────────┴───────────┴───────┴─────────────┴──────────┴──────────────

large:100000 > Sort Median
─────────────────────┬────────────┬───────┬──────────────┬──────────┬──────────────
        name         │   median   │  cv   │   std.dev    │ outliers │ samples/iters
─────────────────────┼────────────┼───────┼──────────────┼──────────┼──────────────
String               │  270.578µs │ 2.19% │    ± 5.956µs │    0.00% │     8 / 1,024
String with_capacity │  314.230µs │ 4.24% │   ± 12.903µs │    0.00% │     7 / 1,024
Xtring.print()       │  363.982µs │ 0.95% │    ± 3.457µs │    9.09% │      11 / 512
Xtring with_capacity │  400.215µs │ 1.39% │    ± 5.573µs │   10.00% │      10 / 512
Xtring               │  480.315µs │ 1.49% │    ± 7.148µs │   11.11% │       9 / 512
─────────────────────┴────────────┴───────┴──────────────┴──────────┴──────────────

large_2:1000000 > Sort Median
─────────────────────┬──────────┬───────┬────────────┬──────────┬──────────────
        name         │  median  │  cv   │  std.dev   │ outliers │ samples/iters
─────────────────────┼──────────┼───────┼────────────┼──────────┼──────────────
Xtring with_capacity │  5.949ms │ 3.49% │  ± 0.207ms │    0.00% │       11 / 32
Xtring               │ 12.628ms │ 2.96% │  ± 0.371ms │    0.00% │       10 / 16
Xtring.print()       │ 12.663ms │ 1.51% │  ± 0.190ms │    0.00% │       10 / 16
String with_capacity │ 16.653ms │ 0.56% │  ± 0.094ms │   12.50% │        8 / 16
String               │ 17.065ms │ 0.85% │  ± 0.144ms │    0.00% │        8 / 16
─────────────────────┴──────────┴───────┴────────────┴──────────┴──────────────
total time: 12.614359046 sec
rust: 1.94.1 (release) | zench: 0.1.4

*/
