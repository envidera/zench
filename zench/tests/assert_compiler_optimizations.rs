/*
Compiler optimizations

Different execution times depending on:
 - dynamic input VS constant input

why:
- Compiler optimizations affect constant inputs

Solution:
- Dev should always use bx in input and output values.
*/

use zench::dev::algorithm;
use zench::issue;

#[cfg(test)]
mod tests {

    use super::*;
    use zench::bench;
    use zench::bx;
    use zench::dev::mock;

    // ----------------------------------------------------------------
    // DYNAMIC - ONLY DYNAMIC INPUT
    // ----------------------------------------------------------------
    // Te compiler sometimes does not
    // optimize the function with dynamically generated input,
    // even without bx
    //
    // - The benchmark results are correct.
    // - If in doubt, always use bx
    // ----------------------------------------------------------------
    /*

    - Test: passed (all similar times)
    - Benchmark: passed (all correct)

    Sort Median
    ─────┬───────────┬───────┬────────────┬──────────┬──────────────
    name │  median   │  cv   │  std.dev   │ outliers │ samples/iters
    ─────┼───────────┼───────┼────────────┼──────────┼──────────────
    OK   │ 219.789ns │ 0.06% │  ± 0.127ns │    0.00% │  18 / 524,288
    OK   │ 219.845ns │ 0.17% │  ± 0.376ns │    0.00% │  18 / 524,288
    OK   │ 219.950ns │ 0.08% │  ± 0.169ns │    0.00% │  18 / 524,288
    ─────┴───────────┴───────┴────────────┴──────────┴──────────────
    */

    #[test]
    fn bench_dynamic_input() {
        bench! {
            "OK" => mock::generate_data(100),
            "OK" => {
                mock::generate_data(100)
            },
            "OK" => {
                mock::generate_data(100);
            },
        }
        .report(|r| {
            r.sort_by_median()
                .print();

            if r.len()
                > r.filter_proximity_pct(10.0)
                    .len()
            {
                issue!()
            }
        });
    }

    // ----------------------------------------------------------------
    // MIXED - CONSTANT + DYNAMIC INPUT
    // ----------------------------------------------------------------
    // When constant input is combined with dynamically generated data,
    // the compiler may or may not aggressively optimize the function.
    //
    // - This may or may not invalidate benchmark results.
    // - If in doubt, always use bx
    // ----------------------------------------------------------------
    /*

    - Test: passed (ok, invalid, ok)
    - Benchmark: Failed (1 invalid)

    Sort Median
    ──────┬──────────┬───────┬────────────┬──────────┬──────────────
    name  │  median  │  cv   │  std.dev   │ outliers │ samples/iters
    ──────┼──────────┼───────┼────────────┼──────────┼──────────────
    WRONG │  0.215ns │ 0.01% │  ± 0.000ns │   13.04% │ 184 / 524,288
    OK    │ 55.127ns │ 0.18% │  ± 0.097ns │    0.00% │  70 / 524,288
    OK    │ 55.481ns │ 0.36% │  ± 0.199ns │    1.45% │  69 / 524,288
    ──────┴──────────┴───────┴────────────┴──────────┴──────────────
    */

    fn std_dev(data: &[f64], mean: &f64) -> f64 {
        let mut sum = 0.0;
        for &x in data {
            let diff = x - mean;
            sum += diff * diff;
        }
        (sum / (data.len() as f64)).sqrt()
    }

    #[test]
    fn bench_mixed_input() {
        let data = &mock::generate_data(100);
        let mean = &algorithm::mean(data);
        bench! {
            "OK" => std_dev(data, mean),
            "WRONG" => {
                std_dev(data, mean);
            },
            "OK" => {
                bx(std_dev(data, mean));
            },
        }
        .report(|r| {
            r.sort_by_median()
                .print();

            if r.filter_proximity_pct(10.0)
                .len()
                != 1
            {
                issue!()
            }
        });
    }

    // ----------------------------------------------------------------
    // CONSTANT - CONSTANT INPUT
    // ----------------------------------------------------------------
    // With constant input data, the compiler aggressively optimize the
    // function.
    // - This invalidate benchmark results.
    // - Always use bx
    // ----------------------------------------------------------------
    /*

    - Test: passed (invalid, invalid, invalid, ok, ok, ok)
    - Benchmark: Failed (3 invalid)

    Sort Median
    ──────┬─────────┬───────┬────────────┬──────────┬──────────────
    name  │ median  │  cv   │  std.dev   │ outliers │ samples/iters
    ──────┼─────────┼───────┼────────────┼──────────┼──────────────
    WRONG │ 0.215ns │ 0.01% │  ± 0.000ns │   11.00% │ 100 / 524,288
    WRONG │ 0.215ns │ 0.01% │  ± 0.000ns │   12.00% │ 100 / 524,288
    WRONG │ 0.215ns │ 0.01% │  ± 0.000ns │   14.00% │ 100 / 524,288
    OK    │ 0.948ns │ 0.46% │  ± 0.004ns │    2.00% │ 100 / 524,288
    OK    │ 0.948ns │ 0.45% │  ± 0.004ns │    1.00% │ 100 / 524,288
    OK    │ 0.949ns │ 0.41% │  ± 0.004ns │    3.00% │ 100 / 524,288
    ──────┴─────────┴───────┴────────────┴──────────┴──────────────
    */

    pub fn sum_loop(n: u64) -> u64 {
        let mut total = 0;
        for i in 0..n {
            total += i;
        }
        total
    }

    #[test]
    fn bench_constant_input() {
        bench! {
            // --------------------------
            "WRONG" => sum_loop(10),
            "WRONG" => {
                sum_loop(10);
            },
            "WRONG" => {
                bx(sum_loop(10));
            },
            // --------------------------
            "OK" => sum_loop(bx(10)),
            "OK" => {
                bx(sum_loop(bx(10)));
            },
            "OK" => {
                bx(sum_loop(bx(10)));
            },
            // --------------------------
        }
        .report(|r| {
            r.sort_by_median()
                .print();

            if r.filter_proximity_pct(10.0)
                .len()
                != 3
            {
                issue!()
            }
        });
    }
}
