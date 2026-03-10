fn main() {}

// ================================================================
// Unit test
// ================================================================
/*

Currently, Zench focuses on relative comparisons and regression
detection within the same run.

In many cases, you already know the expected baseline or acceptable
range for a function, and you can assert that directly in the benchmark.

For example, if a function normally takes around 1 ms, you can simply
fail the test if it exceeds 15% regression:

*/

#[cfg(test)]
mod tests {

    use std::{thread::sleep, time::Duration};
    use zench::bench;
    use zench::issue;

    #[test]
    fn simple_regression_example() {
        bench!(
                "my func" =>{
                    sleep(Duration::from_millis(1));
                },

        )
        .report(|r| {
            r.print();

            // Expected baseline time (from Duration::from_millis(1))
            let baseline = 1_000_000.0;
            let tolerance = 0.15; // 15%

            let median = r
                .first()
                .unwrap()
                .median();

            let upper = baseline * (1.0 + tolerance);
            let lower = baseline * (1.0 - tolerance);

            if median > upper {
                issue!("relative regression (>15%)");
            }

            if median < lower {
                issue!("performance improvement (>15%)");
            }

            // Note: Ensure the system is in a stable state
            // during benchmarking, as background activity
            // can influence the results.

            // Note: Fixed baseline values may vary across
            // different hardware. Adjust the baseline
            // accordingly for your system.
        });
    }
}
