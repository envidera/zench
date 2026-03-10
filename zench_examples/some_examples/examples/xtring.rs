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
    fn test_performance() {
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

Report     micro:3
Filters    Sort Median

Benchmark  Xtring with_capacity
Time       Median: 18.607ns
Stability  Std.Dev: ± 0.049ns | CV: 0.27%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 1.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  Xtring
Time       Median: 23.616ns
Stability  Std.Dev: ± 4.648ns | CV: 18.32%
Samples    Count: 151 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  Xtring.print()
Time       Median: 51.841ns
Stability  Std.Dev: ± 0.220ns | CV: 0.42%
Samples    Count: 74 | Iters/sample: 524,288 | Outliers: 5.41%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  String
Time       Median: 63.640ns
Stability  Std.Dev: ± 0.153ns | CV: 0.24%
Samples    Count: 60 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  String with_capacity
Time       Median: 68.892ns
Stability  Std.Dev: ± 0.211ns | CV: 0.31%
Samples    Count: 56 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

total time: 9.380176014 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-02-23 16:10:07 UTC

bench .....

Report     small:10
Filters    Sort Median

Benchmark  Xtring with_capacity
Time       Median: 39.639ns
Stability  Std.Dev: ± 0.221ns | CV: 0.56%
Samples    Count: 96 | Iters/sample: 524,288 | Outliers: 5.21%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  Xtring
Time       Median: 104.003ns
Stability  Std.Dev: ± 0.484ns | CV: 0.46%
Samples    Count: 37 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  String with_capacity
Time       Median: 136.071ns
Stability  Std.Dev: ± 0.549ns | CV: 0.40%
Samples    Count: 28 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  String
Time       Median: 143.458ns
Stability  Std.Dev: ± 0.361ns | CV: 0.25%
Samples    Count: 27 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  Xtring.print()
Time       Median: 158.278ns
Stability  Std.Dev: ± 0.484ns | CV: 0.31%
Samples    Count: 25 | Iters/sample: 524,288 | Outliers: 4.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

total time: 10.751016105 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-02-23 16:10:17 UTC

bench .....

Report     small_1:25
Filters    Sort Median

Benchmark  Xtring with_capacity
Time       Median: 86.595ns
Stability  Std.Dev: ± 0.132ns | CV: 0.15%
Samples    Count: 45 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  Xtring
Time       Median: 204.139ns
Stability  Std.Dev: ± 0.930ns | CV: 0.46%
Samples    Count: 38 | Iters/sample: 262,144 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  String
Time       Median: 220.421ns
Stability  Std.Dev: ± 1.257ns | CV: 0.57%
Samples    Count: 35 | Iters/sample: 262,144 | Outliers: 5.71%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  String with_capacity
Time       Median: 233.639ns
Stability  Std.Dev: ± 1.492ns | CV: 0.64%
Samples    Count: 33 | Iters/sample: 262,144 | Outliers: 6.06%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  Xtring.print()
Time       Median: 246.847ns
Stability  Std.Dev: ± 1.235ns | CV: 0.50%
Samples    Count: 31 | Iters/sample: 262,144 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

total time: 10.711796721 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-02-23 16:10:28 UTC

bench .....

Report     small_2:50
Filters    Sort Median

Benchmark  Xtring with_capacity
Time       Median: 187.067ns
Stability  Std.Dev: ± 0.379ns | CV: 0.20%
Samples    Count: 21 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  Xtring
Time       Median: 269.807ns
Stability  Std.Dev: ± 4.568ns | CV: 1.68%
Samples    Count: 29 | Iters/sample: 262,144 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  String with_capacity
Time       Median: 276.450ns
Stability  Std.Dev: ± 0.805ns | CV: 0.29%
Samples    Count: 28 | Iters/sample: 262,144 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  Xtring.print()
Time       Median: 327.012ns
Stability  Std.Dev: ± 1.059ns | CV: 0.32%
Samples    Count: 24 | Iters/sample: 262,144 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  String
Time       Median: 383.181ns
Stability  Std.Dev: ± 0.814ns | CV: 0.21%
Samples    Count: 20 | Iters/sample: 262,144 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

total time: 11.062021367 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-02-23 16:10:39 UTC

bench .....

Report     small_3:100
Filters    Sort Median

Benchmark  Xtring with_capacity
Time       Median: 354.724ns
Stability  Std.Dev: ± 0.861ns | CV: 0.24%
Samples    Count: 22 | Iters/sample: 262,144 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  String with_capacity
Time       Median: 415.852ns
Stability  Std.Dev: ± 0.490ns | CV: 0.12%
Samples    Count: 37 | Iters/sample: 131,072 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  String
Time       Median: 460.421ns
Stability  Std.Dev: ± 0.541ns | CV: 0.12%
Samples    Count: 34 | Iters/sample: 131,072 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  Xtring
Time       Median: 470.405ns
Stability  Std.Dev: ± 1.778ns | CV: 0.38%
Samples    Count: 33 | Iters/sample: 131,072 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  Xtring.print()
Time       Median: 497.693ns
Stability  Std.Dev: ± 1.549ns | CV: 0.31%
Samples    Count: 31 | Iters/sample: 131,072 | Outliers: 22.58%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

total time: 10.878953697 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-02-23 16:10:50 UTC

bench ...test tests::test_performance has been running for over 60 seconds
..

Report     mid:1000
Filters    Sort Median

Benchmark  String
Time       Median: 2.788µs
Stability  Std.Dev: ± 0.005µs | CV: 0.19%
Samples    Count: 22 | Iters/sample: 32,768 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  String with_capacity
Time       Median: 2.833µs
Stability  Std.Dev: ± 0.001µs | CV: 0.04%
Samples    Count: 22 | Iters/sample: 32,768 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  Xtring with_capacity
Time       Median: 3.497µs
Stability  Std.Dev: ± 0.025µs | CV: 0.70%
Samples    Count: 36 | Iters/sample: 16,384 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  Xtring
Time       Median: 3.830µs
Stability  Std.Dev: ± 0.003µs | CV: 0.07%
Samples    Count: 32 | Iters/sample: 16,384 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  Xtring.print()
Time       Median: 3.868µs
Stability  Std.Dev: ± 0.003µs | CV: 0.07%
Samples    Count: 32 | Iters/sample: 16,384 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

total time: 10.900060875 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-02-23 16:11:01 UTC

bench .....

Report     mid_2:10000
Filters    Sort Median

Benchmark  String with_capacity
Time       Median: 26.712µs
Stability  Std.Dev: ± 0.168µs | CV: 0.63%
Samples    Count: 37 | Iters/sample: 2,048 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  String
Time       Median: 27.092µs
Stability  Std.Dev: ± 0.105µs | CV: 0.39%
Samples    Count: 37 | Iters/sample: 2,048 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  Xtring with_capacity
Time       Median: 35.813µs
Stability  Std.Dev: ± 0.113µs | CV: 0.32%
Samples    Count: 28 | Iters/sample: 2,048 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  Xtring
Time       Median: 36.486µs
Stability  Std.Dev: ± 0.158µs | CV: 0.43%
Samples    Count: 27 | Iters/sample: 2,048 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  Xtring.print()
Time       Median: 36.787µs
Stability  Std.Dev: ± 0.132µs | CV: 0.36%
Samples    Count: 27 | Iters/sample: 2,048 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

total time: 10.868992033 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-02-23 16:11:12 UTC

bench .....

Report     large:100000
Filters    Sort Median

Benchmark  String with_capacity
Time       Median: 275.897µs
Stability  Std.Dev: ± 7.450µs | CV: 2.68%
Samples    Count: 29 | Iters/sample: 256 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  String
Time       Median: 287.592µs
Stability  Std.Dev: ± 7.294µs | CV: 2.54%
Samples    Count: 28 | Iters/sample: 256 | Outliers: 7.14%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  Xtring
Time       Median: 382.759µs
Stability  Std.Dev: ± 7.250µs | CV: 1.90%
Samples    Count: 21 | Iters/sample: 256 | Outliers: 19.05%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  Xtring with_capacity
Time       Median: 383.686µs
Stability  Std.Dev: ± 3.843µs | CV: 1.00%
Samples    Count: 21 | Iters/sample: 256 | Outliers: 4.76%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  Xtring.print()
Time       Median: 386.273µs
Stability  Std.Dev: ± 2.885µs | CV: 0.75%
Samples    Count: 21 | Iters/sample: 256 | Outliers: 9.52%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

total time: 11.274150903 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-02-23 16:11:23 UTC

bench .....

Report     large_2:1000000
Filters    Sort Median

Benchmark  Xtring with_capacity
Time       Median: 5.102ms
Stability  Std.Dev: ± 0.073ms | CV: 1.44%
Samples    Count: 25 | Iters/sample: 16 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  Xtring.print()
Time       Median: 11.770ms
Stability  Std.Dev: ± 0.384ms | CV: 3.23%
Samples    Count: 42 | Iters/sample: 4 | Outliers: 11.90%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  Xtring
Time       Median: 12.793ms
Stability  Std.Dev: ± 0.656ms | CV: 5.18%
Samples    Count: 20 | Iters/sample: 8 | Outliers: 10.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  String with_capacity
Time       Median: 15.168ms
Stability  Std.Dev: ± 0.162ms | CV: 1.06%
Samples    Count: 33 | Iters/sample: 4 | Outliers: 3.03%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

Benchmark  String
Time       Median: 15.882ms
Stability  Std.Dev: ± 0.464ms | CV: 2.91%
Samples    Count: 32 | Iters/sample: 4 | Outliers: 0.00%
Location   zench_examples/some_examples/examples/xtring.rs:90:13

total time: 10.89079471 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-02-23 16:11:34 UTC

*/
