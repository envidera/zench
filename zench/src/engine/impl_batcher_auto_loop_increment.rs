use super::batch::Batch;
use super::interface::IBatcher;
use super::measure;

// ----------------------------------------------------------------
// BUG: Measurement Stability & Code Layout Effects
// ----------------------------------------------------------------
// - The problem:
// Micro benchmark results are sensitive to the binary layout of the compiled code.
// Seemingly trivial changes, renaming a benchmark, adding a string literal, or
// modifying unrelated code, can shift function positions in memory, affecting
// CPU instruction cache alignment and branch predictor behavior. This may cause
// the same function to measure differently across compilations.
// ----------------------------------------------------------------
// - Observed in test code at:
//  zench/src/algorithm/median.rs test_performance
//
//   bench! {
//     "[faster] v1" => bx(median_case::v1(&mut data1)),
//     "v2" => bx(median_case::v2(&mut data3)),
//   }
// - When:
// Changes in zench/src/report/display.rs altered the benchmark
// measurements, even though that code runs after the benchmark.
// ----------------------------------------------------------------
// - What zench does to try to mitigate this:
// The measurement engine is aligned to 64-byte boundaries (`#[repr(align(64))]`),
// matching the cache line size of most modern CPUs (x86, ARM). This reduces
// sensitivity to surrounding code layout, though it does not eliminate the effect
// entirely.
// ----------------------------------------------------------------
// RESULT:
// `#[repr(align(64))]` above remains under observation to see if
// the problem has decreased.
// ----------------------------------------------------------------
// Observations:
// 1 - add some little overhead in some situations,
//   see zench_examples/criterion_to_zench/benches/fibonacci_zench.rs
//
// Without #[repr(align(64))]
// ────────┬───────────┬───────┬─────────────┬──────────┬────────────────
//  name   │  median   │  cv   │   std.dev   │ outliers │  samples/iters
// ────────┼───────────┼───────┼─────────────┼──────────┼────────────────
// Slow 20 │  13.364µs │ 0.14% │   ± 0.018µs │    0.00% │     10 / 16,384
// Fast 20 │   2.212ns │ 3.38% │   ± 0.076ns │    0.10% │ 1,000 / 524,288
// Slow 21 │  21.622µs │ 0.02% │   ± 0.004µs │    0.00% │      12 / 8,192
// Fast 21 │   2.585ns │ 3.05% │   ± 0.077ns │    0.10% │ 1,000 / 524,288
// ────────┴───────────┴───────┴─────────────┴──────────┴────────────────
//
// With #[repr(align(64))]
// ────────┬───────────┬───────┬─────────────┬──────────┬────────────────
//  name   │  median   │  cv   │   std.dev   │ outliers │  samples/iters
// ────────┼───────────┼───────┼─────────────┼──────────┼────────────────
// Slow 20 │  14.326µs │ 0.54% │   ± 0.078µs │   22.22% │      9 / 16,384
// Fast 20 │   2.813ns │ 0.44% │   ± 0.012ns │    2.00% │   100 / 524,288
// Slow 21 │  23.639µs │ 1.22% │   ± 0.288µs │    0.00% │      11 / 8,192
// Fast 21 │   3.244ns │ 0.34% │   ± 0.011ns │    7.40% │ 1,000 / 524,288
// ────────┴───────────┴───────┴─────────────┴──────────┴────────────────
// Action: comment #[repr(align(64))] for now

//#[repr(align(64))] //<-- IN OBSERVATION
#[derive(Debug, Clone, Default)]
pub struct A {
    pub(crate) batch: Batch,
}

impl IBatcher for A {
    fn estimate_batch_size<F, R>(&self, closure: &mut F) -> usize
    where
        F: FnMut() -> R,
    {
        let single_duration = measure::batch_duration(closure, 1);
        let hit_single_shot_barrier = single_duration
            >= self
                .batch
                .single_shot_barrier;

        if hit_single_shot_barrier {
            return 1;
        }

        //-------------------

        let mut batch = 2;

        loop {
            let duration = measure::batch_duration(closure, batch);

            // condition ---------------------
            let enough_duration = duration
                >= self
                    .batch
                    .min_duration;

            // condition ---------------------
            let hit_max_capacity = batch
                >= self
                    .batch
                    .max_capacity;

            // -------------------------------
            // BREAK
            if enough_duration || hit_max_capacity {
                return batch;
            }

            // -------------------------------
            // Increment
            // exponential growth
            //if batch < 100_000 {
            batch = batch.saturating_mul(2);
            //} else {
            //batch = batch.saturating_add(100_000);
            //}
        }
    }
}
