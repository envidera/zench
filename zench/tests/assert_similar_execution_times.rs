/*
Bug:
what > Different execution times depending on:
 - return vs discard
 - macro vs procedural
 - dynamic input vs constant input (compiler optimizations)

why >
- The macro and the procedural macro were handling the return value differently.
- Compiler optimizations affect constant inputs

Solution >
- Fix macro and bench, do not handle the return > preventing user to return values to bench
- Dev should always use bx in returned values.


There was a measurable difference in execution time between:

- Macro

bench(""=> )       // returning value to macro
bench(""=> {       // block returning value
})

bench(""=>{ ;})     // discarding value
bench(""=> {       // block discarding value
;
})

- Procedural

bench(""|| ;)   // discarding value
bench(""|| {    // block discarding value
;
})

*/

// use std::thread::sleep;
// use std::time::Duration;

use zench::dev::algorithm;
use zench::issue;
use zench::Report;

const DATA_SIZE: usize = 100;

fn has_similar_times(r: &mut Report) -> bool {
    let original_len = r.len();
    let filtered_len = r
        .sort_by_median()
        .filter_proximity_pct(10.0)
        .len();

    original_len == filtered_len
}
// // a hypothetical function to check potential code elimination
// fn have_code_elimination(r: &mut Report) -> bool {
//     let mut base_discarded_time: f64 = 0.0;

//     let wait = || sleep(Duration::from_millis(1));

//     bench!(
//         "" =>{},
//         "" => wait(),
//         "" =>{},
//         "" => wait(),
//         "" =>{},
//         "" => wait(),
//         "" =>{},
//         "" => wait(),
//         "" =>{},
//     )
//     .report(|r| {
//         r.sort_by_median();

//         //println!(">>>{r}<<<");

//         base_discarded_time = r
//             .first()
//             .unwrap()
//             .median()
//             * 2.5;
//         // *2.5 because sometimes Median: 0.215ns and Median: 0.430ns
//     });

//     let ok = r
//         .iter()
//         .any(|f| f.median() < base_discarded_time);

//     ok
// }

fn std_dev(data: &[f64], mean: &f64) -> f64 {
    let mut sum = 0.0;
    for &x in data {
        let diff = x - mean;
        sum += diff * diff;
    }
    (sum / (data.len() as f64)).sqrt()
}

#[cfg(test)]
mod tests {

    use super::*;
    use zench::bench;
    use zench::bx;
    use zench::dev::mock;
    use zench::Bench;

    // ----------------------------------------------------------------
    // With dynamically generated input data, the compiler sometimes does not
    // optimize the function, even without bx
    // - However, this approach is not reliable.
    // - Always use bx
    // ----------------------------------------------------------------
    //with_dynamic_data

    // ----------------------------------------------------------------
    // ANTI PATTERN
    // ----------------------------------------------------------------
    #[test]
    fn test_macro_with_dynamic_data_1() {
        bench! {
            "returning 1" => mock::generate_data(DATA_SIZE),
            "returning 2" => {
                mock::generate_data(DATA_SIZE)
            },
            "discarding 1" => { mock::generate_data(DATA_SIZE); },
            "discarding 2" => {
                mock::generate_data(DATA_SIZE);
            },

        }
        .report(|r| {
            r.print();

            let ok = has_similar_times(r);
            if !ok {
                issue!();
            }

            // bench_assert!(
            //     have_code_elimination(r),
            //     "The benchmark has function discarded by the compiler"
            // );
        });
    }
    /*

    - Test: passed
    - Benchmark: passed

    Report

    Benchmark  returning 1
    Time       Median: 220.181ns
    Stability  Std.Dev: ± 0.451ns | CV: 0.20%
    Samples    Count: 35 | Iters/sample: 262,144 | Outliers: 0.00%
    Location   zench/tests/assert_proximity_diff_times.rs:100:13

    Benchmark  returning 2
    Time       Median: 220.049ns
    Stability  Std.Dev: ± 0.543ns | CV: 0.25%
    Samples    Count: 35 | Iters/sample: 262,144 | Outliers: 0.00%
    Location   zench/tests/assert_proximity_diff_times.rs:100:13

    Benchmark  discarding 1
    Time       Median: 219.905ns
    Stability  Std.Dev: ± 0.497ns | CV: 0.23%
    Samples    Count: 35 | Iters/sample: 262,144 | Outliers: 0.00%
    Location   zench/tests/assert_proximity_diff_times.rs:100:13

    Benchmark  discarding 2
    Time       Median: 219.980ns
    Stability  Std.Dev: ± 0.377ns | CV: 0.17%
    Samples    Count: 35 | Iters/sample: 262,144 | Outliers: 0.00%
    Location   zench/tests/assert_proximity_diff_times.rs:100:13


    total time: 8.549431341 sec
    rust: 1.93.1 | profile release
    zench: 0.1.0
    system: linux x86_64
    cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
    2026-03-03 21:46:36 UTC
    */

    // ----------------------------------------------------------------
    // USE INSTEAD
    // ----------------------------------------------------------------
    #[test]
    fn test_macro_with_dynamic_data_fix() {
        bench! {
            "returning 1" => bx(mock::generate_data(DATA_SIZE)),
            "returning 2" => {
                bx(mock::generate_data(DATA_SIZE));
            },
            "discarding 1" => { bx(mock::generate_data(DATA_SIZE)); },
            "discarding 2" => {
                bx(mock::generate_data(DATA_SIZE));
            },

        }
        .report(|r| {
            r.print();
            let ok = has_similar_times(r);
            if !ok {
                issue!();
            }
            // bench_assert!(
            //     have_code_elimination(r),
            //     "The benchmark has function discarded by the compiler"
            // );
        });
    }

    /*

    - Test: passed
    - Benchmark: passed

    Report

    Benchmark  returning 1
    Time       Median: 219.793ns
    Stability  Std.Dev: ± 0.385ns | CV: 0.17%
    Samples    Count: 35 | Iters/sample: 262,144 | Outliers: 0.00%
    Location   zench/tests/assert_proximity_diff_times.rs:160:13

    Benchmark  returning 2
    Time       Median: 219.689ns
    Stability  Std.Dev: ± 0.388ns | CV: 0.18%
    Samples    Count: 35 | Iters/sample: 262,144 | Outliers: 0.00%
    Location   zench/tests/assert_proximity_diff_times.rs:160:13

    Benchmark  discarding 1
    Time       Median: 219.711ns
    Stability  Std.Dev: ± 0.494ns | CV: 0.22%
    Samples    Count: 35 | Iters/sample: 262,144 | Outliers: 0.00%
    Location   zench/tests/assert_proximity_diff_times.rs:160:13

    Benchmark  discarding 2
    Time       Median: 219.575ns
    Stability  Std.Dev: ± 0.615ns | CV: 0.28%
    Samples    Count: 35 | Iters/sample: 262,144 | Outliers: 0.00%
    Location   zench/tests/assert_proximity_diff_times.rs:160:13


    total time: 8.64608223 sec
    rust: 1.93.1 | profile release
    zench: 0.1.0
    system: linux x86_64
    cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
    2026-03-03 21:48:04 UTC

    */

    // ----------------------------------------------------------------
    // ANTI PATTERN
    // ----------------------------------------------------------------
    #[test]
    fn test_procedural_with_dynamic_data_1() {
        let mut b = Bench::new();
        // procedural always discard output
        b.bench("discarding 1", || {
            mock::generate_data(DATA_SIZE);
        });

        b.bench("discarding 2", || {
            mock::generate_data(DATA_SIZE);
        });

        b.report(|r| {
            r.print();
            let ok = has_similar_times(r);
            if !ok {
                issue!();
            }
            // bench_assert!(
            //     have_code_elimination(r),
            //     "The benchmark has function discarded by the compiler"
            // );
        });
    }
    /*

    - Test: passed
    - Benchmark: passed

    OK, all tests passed correctly

    Report

    Benchmark  discarding 1
    Time       Median: 219.998ns
    Stability  Std.Dev: ± 0.264ns | CV: 0.12%
    Samples    Count: 35 | Iters/sample: 262,144 | Outliers: 0.00%
    Location   zench/tests/assert_proximity_diff_times.rs:225:19

    Benchmark  discarding 2
    Time       Median: 219.901ns
    Stability  Std.Dev: ± 0.383ns | CV: 0.17%
    Samples    Count: 35 | Iters/sample: 262,144 | Outliers: 0.00%
    Location   zench/tests/assert_proximity_diff_times.rs:226:19


    total time: 4.383184504 sec
    rust: 1.93.1 | profile release
    zench: 0.1.0
    system: linux x86_64
    cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
    2026-03-03 21:48:49 UTC

    */

    // ----------------------------------------------------------------
    // USE INSTEAD
    // ----------------------------------------------------------------
    #[test]
    fn test_procedural_with_dynamic_data_fix() {
        let mut b = Bench::new();
        // procedural always discard output
        b.bench("discarding 1", || {
            bx(mock::generate_data(DATA_SIZE));
        });

        b.bench("discarding 2", || {
            bx(mock::generate_data(DATA_SIZE));
        });

        b.report(|r| {
            r.print();
            let ok = has_similar_times(r);
            if !ok {
                issue!();
            }
            // bench_assert!(
            //     have_code_elimination(r),
            //     "The benchmark has function discarded by the compiler"
            // );
        });
    }

    /*

    - Test: passed
    - Benchmark: passed

    Report

    Benchmark  discarding 1
    Time       Median: 219.993ns
    Stability  Std.Dev: ± 0.247ns | CV: 0.11%
    Samples    Count: 35 | Iters/sample: 262,144 | Outliers: 0.00%
    Location   zench/tests/assert_proximity_diff_times.rs:271:15

    Benchmark  discarding 2
    Time       Median: 219.864ns
    Stability  Std.Dev: ± 0.317ns | CV: 0.14%
    Samples    Count: 35 | Iters/sample: 262,144 | Outliers: 0.00%
    Location   zench/tests/assert_proximity_diff_times.rs:275:15


    total time: 4.38276942 sec
    rust: 1.93.1 | profile release
    zench: 0.1.0
    system: linux x86_64
    cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
    2026-03-03 22:50:57 UTC

    */

    // ----------------------------------------------------------------
    // With constant input data, the compiler can aggressively optimize the function.
    // - This can invalidate benchmark results.
    // - Always use bx
    // ----------------------------------------------------------------
    // with_constant_data

    // ----------------------------------------------------------------
    // ANTI PATTERN
    // ----------------------------------------------------------------
    #[test]
    fn test_macro_with_constant_algorithm_data_1() {
        let data = mock::generate_data(DATA_SIZE);
        let mean = algorithm::mean(&data);
        bench! {
            "returning 1" => std_dev(&data, &mean),
            "returning 2" => {
                std_dev(&data, &mean)
            },
            "discarding 1" => { std_dev(&data, &mean);},
            "discarding 2" => {
               std_dev(&data, &mean);
            },

        }
        .report(|r| {
            r.print();
            let ok = has_similar_times(r);
            if !ok {
                issue!();
            }
            // bench_assert!(
            //     !have_code_elimination(r), //<--
            //     // NOT to prevent panic, this example
            //     // discard all benchmark
            //     "The benchmark has function discarded by the compiler"
            // );
        });
    }

    /*

    - Test: passed with #[should_panic]
    - Benchmark: Failed (all discarded)

    Report

    Benchmark  returning 1
    Time       Median: 0.215ns
    Stability  Std.Dev: ± 0.000ns | CV: 0.08%
    Samples    Count: 1,000 | Iters/sample: 524,288 | Outliers: 22.50%
    Location   zench/tests/assert_proximity_diff_times.rs:277:13

    Benchmark  returning 2
    Time       Median: 0.215ns
    Stability  Std.Dev: ± 0.000ns | CV: 0.01%
    Samples    Count: 127 | Iters/sample: 524,288 | Outliers: 24.41%
    Location   zench/tests/assert_proximity_diff_times.rs:277:13

    Benchmark  discarding 1
    Time       Median: 0.215ns
    Stability  Std.Dev: ± 0.000ns | CV: 0.01%
    Samples    Count: 160 | Iters/sample: 524,288 | Outliers: 23.12%
    Location   zench/tests/assert_proximity_diff_times.rs:277:13

    Benchmark  discarding 2
    Time       Median: 0.215ns
    Stability  Std.Dev: ± 0.000ns | CV: 0.01%
    Samples    Count: 106 | Iters/sample: 524,288 | Outliers: 17.92%
    Location   zench/tests/assert_proximity_diff_times.rs:277:13


    total time: 0.277573927 sec
    rust: 1.93.1 | profile release
    zench: 0.1.0
    system: linux x86_64
    cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
    2026-03-03 22:31:12 UTC

    */

    // ----------------------------------------------------------------
    // USE INSTEAD
    // ----------------------------------------------------------------
    #[test]
    fn test_macro_with_constant_algorithm_data_fix() {
        let data = mock::generate_data(DATA_SIZE);
        let mean = algorithm::mean(&data);
        bench! {
            "returning 1" => bx(std_dev(&data, &mean)),
            "returning 2" => {
                bx(std_dev(&data, &mean));
            },
            "discarding 1" => { bx(std_dev(&data, &mean));},
            "discarding 2" => {
                bx(std_dev(&data, &mean));
            },

        }
        .report(|r| {
            r.print();
            let ok = has_similar_times(r);
            if !ok {
                issue!();
            }
            // bench_assert!(
            //     have_code_elimination(r),
            //     "The benchmark has function discarded by the compiler"
            // );
        });
    }

    /*

    - Test: passed
    - Benchmark: passed

    Report

    Benchmark  returning 1
    Time       Median: 55.534ns
    Stability  Std.Dev: ± 0.193ns | CV: 0.35%
    Samples    Count: 69 | Iters/sample: 524,288 | Outliers: 1.45%
    Location   zench/tests/assert_proximity_diff_times.rs:346:13

    Benchmark  returning 2
    Time       Median: 55.567ns
    Stability  Std.Dev: ± 0.210ns | CV: 0.38%
    Samples    Count: 69 | Iters/sample: 524,288 | Outliers: 0.00%
    Location   zench/tests/assert_proximity_diff_times.rs:346:13

    Benchmark  discarding 1
    Time       Median: 55.766ns
    Stability  Std.Dev: ± 0.196ns | CV: 0.35%
    Samples    Count: 69 | Iters/sample: 524,288 | Outliers: 0.00%
    Location   zench/tests/assert_proximity_diff_times.rs:346:13

    Benchmark  discarding 2
    Time       Median: 55.415ns
    Stability  Std.Dev: ± 0.194ns | CV: 0.35%
    Samples    Count: 69 | Iters/sample: 524,288 | Outliers: 0.00%
    Location   zench/tests/assert_proximity_diff_times.rs:346:13


    total time: 8.394912539 sec
    rust: 1.93.1 | profile release
    zench: 0.1.0
    system: linux x86_64
    cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
    2026-03-03 22:43:15 UTC

    */

    // ----------------------------------------------------------------
    // ANTI PATTERN
    // ----------------------------------------------------------------
    #[test]
    fn test_procedural_constant_algorithm_data_1() {
        let mut b = Bench::new();
        let data = mock::generate_data(DATA_SIZE);
        let mean = algorithm::mean(&data);

        b.bench("discarding 1", || {
            std_dev(&data, &mean);
        });
        b.bench("discarding 2", || {
            std_dev(&data, &mean);
        });
        b.report(|r| {
            r.print();
            let ok = has_similar_times(r);
            if !ok {
                issue!();
            }
            // bench_assert!(
            //     !have_code_elimination(r), //<--
            //     // NOT to prevent panic, this example
            //     // discard all benchmark
            //     "The benchmark has function discarded by the compiler"
            // );
        });
    }
    /*

    - Test: passed with #[should_panic]
    - Benchmark: Failed (all discarded)

    Report

    Benchmark  discarding 1
    Time       Median: 0.215ns
    Stability  Std.Dev: ± 0.000ns | CV: 0.01%
    Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 12.00%
    Location   zench/tests/assert_proximity_diff_times.rs:413:15

    Benchmark  discarding 2
    Time       Median: 0.215ns
    Stability  Std.Dev: ± 0.000ns | CV: 0.01%
    Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 12.00%
    Location   zench/tests/assert_proximity_diff_times.rs:416:15


    total time: 0.139795051 sec
    rust: 1.93.1 | profile release
    zench: 0.1.0
    system: linux x86_64
    cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
    2026-03-03 22:45:40 UTC

    */

    // ----------------------------------------------------------------
    // USE INSTEAD
    // ----------------------------------------------------------------
    #[test]
    fn test_procedural_constant_algorithm_data_fix() {
        let mut b = Bench::new();

        let data = mock::generate_data(DATA_SIZE);
        let mean = algorithm::mean(&data);

        b.bench("discarding 1", || {
            bx(std_dev(&data, &mean));
        });
        b.bench("discarding 2", || {
            bx(std_dev(&data, &mean));
        });
        b.report(|r| {
            r.print();
            let ok = has_similar_times(r);
            if !ok {
                issue!();
            }
        });
    }

    /*

    - Test: passed
    - Benchmark: passed

    Report

    Benchmark  discarding 1
    Time       Median: 55.009ns
    Stability  Std.Dev: ± 0.337ns | CV: 0.61%
    Samples    Count: 70 | Iters/sample: 524,288 | Outliers: 0.00%
    Location   zench/tests/assert_proximity_diff_times.rs:461:15

    Benchmark  discarding 2
    Time       Median: 55.403ns
    Stability  Std.Dev: ± 0.062ns | CV: 0.11%
    Samples    Count: 69 | Iters/sample: 524,288 | Outliers: 0.00%
    Location   zench/tests/assert_proximity_diff_times.rs:464:15


    total time: 4.259993283 sec
    rust: 1.93.1 | profile release
    zench: 0.1.0
    system: linux x86_64
    cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
    2026-03-03 22:48:16 UTC

    */
}

/*



*/
