fn main() {}
// ================================================================
// Unit test
// ================================================================

/*
In this example, we will compare three implementations of the same algorithm for squaring
the elements of a Vec, determine the fastest version, and `issue!` if the known fastest
version is no longer the fastest.

*/

#[cfg(test)]
mod tests {

    // three implementations of the same algorithm

    pub fn square_loop(data: &[u64]) -> Vec<u64> {
        let mut out = Vec::with_capacity(data.len());
        for &v in data {
            out.push(v * v);
        }
        out
    }

    pub fn square_iterator(data: &[u64]) -> Vec<u64> {
        data.iter()
            .map(|&v| v * v)
            .collect()
    }

    pub fn square_fold(data: &[u64]) -> Vec<u64> {
        data.iter()
            .fold(Vec::with_capacity(data.len()), |mut acc, &v| {
                acc.push(v * v);
                acc
            })
    }

    #[test]
    fn bench_fastest_version() {
        use zench::bench;
        use zench::bx;

        // Use the `issue!` macro.
        // Emits a diagnostic message that either warns or panics,
        // depending on the `ZENCH` environment variable.
        use zench::issue;

        let data: Vec<u64> = (0..100_000).collect();

        bench!(
            "loop" => bx(square_loop(bx(&data))),
            "iterator" => bx(square_iterator(bx(&data))),
            "fold" => bx(square_fold(bx(&data))),
        )
        .report(|r| {
            // For this benchmark, we consider performance roughly equal
            // when the time difference between implementations is within 10%.
            // Benchmarks within this range are grouped as `faster_group`,
            // and the remaining ones as `slower_group`.
            let (mut faster_group, mut slower_group) = r
                .sort_by_median()
                .filter_proximity_pct(10.0)
                // Split the current filtered state from the remaining
                // benchmarks
                .split();

            // We expect only one benchmark in the fastest group;
            // issue if more are present
            if faster_group.len() > 1 {
                issue!("some implementations changed performance");
            }

            // We expect the benchmark named "iterator" to be the fastest;
            // issue if it is not
            if !faster_group
                .first()
                .unwrap()
                .name()
                .contains("iterator")
            {
                issue!("the iterator is no longer the fastest");
            }

            faster_group
                .title("Faster group")
                .print();

            slower_group
                .title("Slower group")
                .print();
        });
    }
}

/*

Report     Faster group
Filters    Sort Median > Filter Proximity(10%)

Benchmark  iterator
Time       Median: 19.140µs
Stability  Std.Dev: ± 0.936µs | CV: 4.94%
Samples    Count: 7 | Iters/sample: 16,384 | Outliers: 0.00%
Location   zench_examples/readme_examples/examples/ex_04.rs:48:9



Report     Slower group

Benchmark  loop
Time       Median: 55.502µs
Stability  Std.Dev: ± 0.198µs | CV: 0.36%
Samples    Count: 9 | Iters/sample: 4,096 | Outliers: 0.00%
Location   zench_examples/readme_examples/examples/ex_04.rs:48:9

Benchmark  fold
Time       Median: 114.093µs
Stability  Std.Dev: ± 0.230µs | CV: 0.20%
Samples    Count: 9 | Iters/sample: 2,048 | Outliers: 0.00%
Location   zench_examples/readme_examples/examples/ex_04.rs:48:9


total time: 7.926189331 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-09 01:25:21 UTC

*/

/*
bench::issue::warn some implementations have changed performance
--> zench_examples/readme_examples/examples/ex_04.rs:66:17

bench::issue::warn the iterator is no longer the fastest
--> zench_examples/readme_examples/examples/ex_04.rs:77:17

*/

/*
bench::issue::panic some implementations have changed performance
--> zench_examples/readme_examples/examples/ex_04.rs:66:17

bench::issue::panic the iterator is no longer the fastest
--> zench_examples/readme_examples/examples/ex_04.rs:77:17

*/
