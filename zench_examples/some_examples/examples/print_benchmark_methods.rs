fn main() {}

// ================================================================
// Unit test
// ================================================================
#[cfg(test)]
mod tests {

    use std::fmt::Display;
    use zench::bench;

    #[test]
    fn test_print() {
        bench!("print example" => {}).report(|r| {
            r.benchset()
                .iter()
                .for_each(|f| {
                    println!();
                    println!();
                    println!("name: {}", f.name());
                    println!("cv: {}", f.cv());
                    println!("cv_pct: {}", f.cv_pct());
                    println!("cv_pct_fmt: {}", f.cv_pct_fmt());
                    println!("iters_count: {}", f.iters_count());
                    println!("iters_count_fmt: {}", f.iters_count_fmt());
                    println!("median: {}", f.median());
                    println!("median_fmt: {}", f.median_fmt());
                    println!("outliers_count: {}", f.outliers_count());
                    println!("outliers_pct: {}", f.outliers_pct());
                    println!("outliers_pct_fmt: {}", f.outliers_pct_fmt());
                    println!("samples_count: {}", f.samples_count());
                    println!("samples_count_fmt: {}", f.samples_count_fmt());
                    println!("std_dev: {}", f.std_dev());
                    println!("std_dev_fmt: {}", f.std_dev_fmt());
                });
            r.print();
        });
    }

    #[test]
    fn test_print_aligned() {
        fn print_aligned(label: &str, value: impl Display) {
            println!("{:.<20} {}", label, value);
        }

        bench!("print example" => {}).report(|r| {
            #[rustfmt::skip]
            r.benchset()
                .iter()
                .for_each(|f| {
                    println!();
                    println!();
                    print_aligned("name", f.name());
                    print_aligned("cv", f.cv());
                    print_aligned("cv_pct", f.cv_pct());
                    print_aligned("cv_pct_fmt", f.cv_pct_fmt());
                    print_aligned("iters_count", f.iters_count());
                    print_aligned("iters_count_fmt", f.iters_count_fmt());
                    print_aligned("median", f.median());
                    print_aligned("median_fmt", f.median_fmt());
                    print_aligned("outliers_count", f.outliers_count());
                    print_aligned("outliers_pct", f.outliers_pct());
                    print_aligned("outliers_pct_fmt", f.outliers_pct_fmt());
                    print_aligned("samples_count", f.samples_count());
                    print_aligned("samples_count_fmt", f.samples_count_fmt());
                    print_aligned("std_dev", f.std_dev());
                    print_aligned("std_dev_fmt", f.std_dev_fmt());


                });
            r.print();
        });
    }
}

/*

name: print example
cv: 0.0013717243526667137
cv_pct: 0.13717243526667136
cv_pct_fmt: 0.14%
iters_count: 524288
iters_count_fmt: 524,288
median: 0.21549415588378906
median_fmt: 0.215ns
outliers_count: 13
outliers_pct: 13
outliers_pct_fmt: 13.00%
samples_count: 100
samples_count_fmt: 100
std_dev: 0.0002956443226135059
std_dev_fmt: ± 0.000ns


Report

Benchmark  print example
Time       Median: 0.215ns
Stability  Std.Dev: ± 0.000ns | CV: 0.14%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 13.00%
Location   zench_examples/some_examples/examples/print_benchmark_methods.rs:15:9

total time: 0.127041562 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-02-27 12:33:52 UTC

test tests::test_print ... ok
.

name................ print example
cv.................. 0.00006422248167537466
cv_pct.............. 0.006422248167537466
cv_pct_fmt.......... 0.01%
iters_count......... 524288
iters_count_fmt..... 524,288
median.............. 0.21549415588378906
median_fmt.......... 0.215ns
outliers_count...... 171
outliers_pct........ 17.1
outliers_pct_fmt.... 17.10%
samples_count....... 1000
samples_count_fmt... 1,000
std_dev............. 0.00001383953889067181
std_dev_fmt......... ± 0.000ns


Report

Benchmark  print example
Time       Median: 0.215ns
Stability  Std.Dev: ± 0.000ns | CV: 0.01%
Samples    Count: 1,000 | Iters/sample: 524,288 | Outliers: 17.10%
Location   zench_examples/some_examples/examples/print_benchmark_methods.rs:47:9

*/
