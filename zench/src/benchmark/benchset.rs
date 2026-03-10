use std::ops::Deref;

use crate::algorithm;
use crate::benchmark::filter::Filter;
use crate::benchmark::filter::Sort;
use crate::benchmark::Benchmark;

#[derive(Debug, Clone, Default)]
pub struct BenchSet {
    items: Vec<Benchmark>,
}

impl BenchSet {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn new_with(items: Vec<Benchmark>) -> Self {
        Self { items }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Benchmark> {
        self.items
            .iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Benchmark> {
        self.items
            .iter_mut()
    }

    pub fn push(&mut self, b: Benchmark) {
        self.items
            .push(b);
    }

    #[rustfmt::skip]
    pub(crate) fn sort_by(&mut self, sort: Sort) {
        match sort {
            Sort::Median => self.items.sort_by(|a, b| compare_f64(a.median, b.median)),
            Sort::MedianReverse => self.items.sort_by(|a, b| compare_f64(b.median, a.median)),
            // ----------------------------------------------------------------
            Sort::Samples => self.items.sort_by_key(|a| a.samples_count),
            Sort::SamplesReverse => self.items.sort_by_key(|a| std::cmp::Reverse(a.samples_count)),
            // ----------------------------------------------------------------
            Sort::Outliers => self.items.sort_by_key(|a| a.outliers_count),
            Sort::OutliersReverse => self.items.sort_by_key(|a| std::cmp::Reverse(a.outliers_count)),
            // ----------------------------------------------------------------
            Sort::StdDev => self.items.sort_by(|a, b| compare_f64(a.std_dev, b.std_dev)),
            Sort::StdDevReverse => self.items.sort_by(|a, b| compare_f64(b.std_dev, a.std_dev)),
        };
    }

    pub(crate) fn filter(self, filter: Filter, sort: Sort) -> (Self, Self) {
        if self.is_empty() {
            return (Self::new(), Self::new());
        }

        let (kept, rest) = match filter {
            Filter::N(value) => self.filter_n(value),
            Filter::Proximity(pct) => {
                //-
                match sort {
                    Sort::Median | Sort::MedianReverse => self.filter_time_similarity(pct),
                    _ => self.filter_percentage(pct, sort),
                }
            }
            Filter::Pct(pct) => self.filter_percentage(pct, sort),
        };

        (kept, rest)
    }

    pub(crate) fn filter_n(mut self, value: usize) -> (Self, Self) {
        let rest = self
            .items
            .split_off(value);
        (self, Self::new_with(rest))
    }

    fn filter_time_similarity(self, pct: f64) -> (Self, Self) {
        let user_pct = pct / 100.0;

        let base_median = self.items[0].median;
        let base_std_dev = self.items[0].std_dev;

        let (keep, rest) = self
            .items
            .into_iter()
            .partition(|bench| {
                let distance = algorithm::log2_distance(base_median, bench.median);

                let cv1 = base_std_dev / base_median;
                let cv2 = bench.std_dev / bench.median;

                let combined_noise = (cv1.powi(2) + cv2.powi(2)).sqrt();
                let noise_pct = combined_noise * 2.0;

                let effective_pct = user_pct.max(noise_pct);
                let log_limit = (1.0 + effective_pct).log2();

                distance <= log_limit
            });
        (Self::new_with(keep), Self::new_with(rest))
    }

    fn filter_percentage(mut self, pct: f64, sort: Sort) -> (Self, Self) {
        if self.is_empty() {
            return (Self::new(), Self::new());
        }

        let pct_val = pct / 100.0;

        let base_median = self.items[0].median;
        let base_std_dev = self.items[0].std_dev;
        let base_samples = self.items[0].samples_count as f64;
        let base_outliers = self.items[0].outliers_count as f64;

        let (keep, rest): (Vec<Benchmark>, Vec<Benchmark>) = self
            .items
            .drain(..)
            .partition(|bench| match sort {
                Sort::Median => bench.median <= base_median * (1.0 + pct_val),
                Sort::MedianReverse => bench.median >= base_median * (1.0 - pct_val),
                // ----------------------------------------------------------------
                Sort::StdDev => bench.std_dev <= base_std_dev * (1.0 + pct_val),
                Sort::StdDevReverse => bench.std_dev >= base_std_dev * (1.0 - pct_val),
                // ----------------------------------------------------------------
                Sort::Samples => bench.samples_count as f64 <= base_samples * (1.0 + pct_val),
                Sort::SamplesReverse => {
                    bench.samples_count as f64 >= base_samples * (1.0 - pct_val)
                }
                // ----------------------------------------------------------------
                Sort::Outliers => bench.outliers_count as f64 <= base_outliers * (1.0 + pct_val),
                Sort::OutliersReverse => {
                    bench.outliers_count as f64 >= base_outliers * (1.0 - pct_val)
                }
            });

        (Self::new_with(keep), Self::new_with(rest))
    }
}

fn compare_f64(a: f64, b: f64) -> std::cmp::Ordering {
    a.partial_cmp(&b)
        .unwrap_or_else(|| {
            if a.is_nan() && b.is_nan() {
                std::cmp::Ordering::Equal
            } else if a.is_nan() {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        })
}

impl Deref for BenchSet {
    type Target = [Benchmark];
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

// Iteration by Reference (&Benches) -> for `for b in &benches` loops
impl<'a> IntoIterator for &'a BenchSet {
    type Item = &'a Benchmark;
    type IntoIter = std::slice::Iter<'a, Benchmark>;

    fn into_iter(self) -> Self::IntoIter {
        self.items
            .iter()
    }
}

// Mutable Reference Iteration (&mut Benches) -> for `for b in &mut benches`
impl<'a> IntoIterator for &'a mut BenchSet {
    type Item = &'a mut Benchmark;
    type IntoIter = std::slice::IterMut<'a, Benchmark>;

    fn into_iter(self) -> Self::IntoIter {
        self.items
            .iter_mut()
    }
}

// ================================================================
// Unit test
// ================================================================

#[cfg(test)]
mod tests {

    // ----------------------------------------------------------------
    // test_sort

    use crate::benchmark::filter::Sort;
    use crate::benchmark::BenchSet;
    use crate::benchmark::Benchmark;

    #[test]
    fn test_sort_median() {
        const UNSORTED_VALUES: &[f64] = &[1.0, 10.0, 5.0, 2.0];
        const WANT_SORT: &[f64] = &[1.0, 2.0, 5.0, 10.0];

        let vec_benchmark = UNSORTED_VALUES
            .iter()
            .map(|&value| Benchmark {
                median: value,
                ..Benchmark::default()
            })
            .collect();

        let mut b = BenchSet::new_with(vec_benchmark);
        b.sort_by(Sort::Median);

        let have: Vec<f64> = b
            .iter()
            .map(|f| f.median())
            .collect();

        assert_eq!(have, WANT_SORT);
    }

    #[test]
    fn test_sort_median_reverse() {
        const UNSORTED_VALUES: &[f64] = &[1.0, 10.0, 5.0, 2.0];
        const WANT_SORT: &[f64] = &[10.0, 5.0, 2.0, 1.0];

        let vec_benchmark = UNSORTED_VALUES
            .iter()
            .map(|&value| Benchmark {
                median: value,
                ..Benchmark::default()
            })
            .collect();

        let mut b = BenchSet::new_with(vec_benchmark);
        b.sort_by(Sort::MedianReverse);

        let have: Vec<f64> = b
            .iter()
            .map(|f| f.median())
            .collect();

        assert_eq!(have, WANT_SORT);
    }

    #[test]
    fn test_sort_samples() {
        const UNSORTED_VALUES: &[usize] = &[10, 100, 50, 20];
        const WANT_SORT: &[usize] = &[10, 20, 50, 100];

        let vec_benchmark = UNSORTED_VALUES
            .iter()
            .map(|&value| Benchmark {
                samples_count: value,
                ..Benchmark::default()
            })
            .collect();

        let mut b = BenchSet::new_with(vec_benchmark);
        b.sort_by(Sort::Samples);

        let have: Vec<usize> = b
            .iter()
            .map(|f| f.samples_count())
            .collect();

        assert_eq!(have, WANT_SORT);
    }

    #[test]
    fn test_sort_samples_reverse() {
        const UNSORTED_VALUES: &[usize] = &[10, 100, 50, 20];
        const WANT_SORT: &[usize] = &[100, 50, 20, 10];

        let vec_benchmark = UNSORTED_VALUES
            .iter()
            .map(|&value| Benchmark {
                samples_count: value,
                ..Benchmark::default()
            })
            .collect();

        let mut b = BenchSet::new_with(vec_benchmark);
        b.sort_by(Sort::SamplesReverse);

        let have: Vec<usize> = b
            .iter()
            .map(|f| f.samples_count())
            .collect();

        assert_eq!(have, WANT_SORT);
    }

    // TODO: continue the test...
}

//     #[test]
//     fn test_sort_samples_outlier() {
//         const UNSORTED_VALUES: &[usize] = &[10, 100, 50, 20];
//         const WANT_SORT: &[usize] = &[10, 20, 50, 100];

//         let mut benches: Vec<Benchmark> = Vec::new();
//         {
//             let mut b = Benchmark::default();
//             for value in UNSORTED_VALUES {
//                 b.outliers_count = *value;
//                 benches.push(b.clone());
//             }
//         }

//         Sort::Outliers.apply(&mut benches);
//         let have: Vec<usize> = benches
//             .iter()
//             .map(|f| f.outliers_count())
//             .collect();

//         assert_eq!(have, WANT_SORT);
//     }

//     #[test]
//     fn test_sort_samples_outlier_reverse() {
//         const UNSORTED_VALUES: &[usize] = &[10, 100, 50, 20];
//         const WANT_SORT: &[usize] = &[100, 50, 20, 10];

//         let mut benches: Vec<Benchmark> = Vec::new();
//         {
//             let mut b = Benchmark::default();
//             for value in UNSORTED_VALUES {
//                 b.outliers_count = *value;
//                 benches.push(b.clone());
//             }
//         }

//         Sort::OutliersReverse.apply(&mut benches);
//         let have: Vec<usize> = benches
//             .iter()
//             .map(|f| f.outliers_count())
//             .collect();

//         assert_eq!(have, WANT_SORT);
//     }

//     #[test]
//     fn test_sort_std_dev() {
//         const UNSORTED_VALUES: &[f64] = &[1.0, 10.0, 5.0, 2.0];
//         const WANT_SORT: &[f64] = &[1.0, 2.0, 5.0, 10.0];

//         let mut benches: Vec<Benchmark> = Vec::new();
//         {
//             let mut b = Benchmark::default();
//             for value in UNSORTED_VALUES {
//                 b.std_dev = *value;
//                 benches.push(b.clone());
//             }
//         }

//         Sort::StdDev.apply(&mut benches);
//         let have: Vec<f64> = benches
//             .iter()
//             .map(|f| f.std_dev())
//             .collect();

//         assert_eq!(have, WANT_SORT);
//     }

//     #[test]
//     fn test_sort_std_dev_reverse() {
//         const UNSORTED_VALUES: &[f64] = &[1.0, 10.0, 5.0, 2.0];
//         const WANT_SORT: &[f64] = &[10.0, 5.0, 2.0, 1.0];

//         let mut benches: Vec<Benchmark> = Vec::new();
//         {
//             let mut b = Benchmark::default();
//             for value in UNSORTED_VALUES {
//                 b.std_dev = *value;
//                 benches.push(b.clone());
//             }
//         }

//         Sort::StdDevReverse.apply(&mut benches);
//         let have: Vec<f64> = benches
//             .iter()
//             .map(|f| f.std_dev())
//             .collect();

//         assert_eq!(have, WANT_SORT);
//     }
// }

// mod test_filter {

//     use crate::benchmark::Benchmark;
//     use crate::report::filter::Filter;
//     use crate::report::filter::Sort;

//     struct Test {
//         sorted_vec: &'static [f64],
//         split_at: f64,
//         want_1: &'static [f64],
//         want_2: &'static [f64],
//     }

//     // FilterN does not need a test case for each Sort enum,
//     // because it only splits an already sorted vec;
//     // Just one case is enough
//     #[test]
//     fn test_filter_n() {
//         let t = Test {
//             sorted_vec: &[1.0, 2.0, 5.0, 10.0, 20.0],
//             split_at: 2.0,
//             want_1: &[1.0, 2.0],
//             want_2: &[5.0, 10.0, 20.0],
//         };

//         let mut benches: Vec<Benchmark> = Vec::new();
//         {
//             let mut b = Benchmark::default();
//             for value in t.sorted_vec {
//                 b.median = *value;
//                 benches.push(b.clone());
//             }
//         }

//         let (keep, rest) = Filter::N(t.split_at as usize).apply(benches, Sort::Median);

//         let have1: Vec<f64> = keep
//             .iter()
//             .map(|f| f.median())
//             .collect();

//         let have2: Vec<f64> = rest
//             .iter()
//             .map(|f| f.median())
//             .collect();

//         assert_eq!(have1, t.want_1);
//         assert_eq!(have2, t.want_2);
//     }

//     #[test]
//     fn test_filter_pct() {
//         const CASES: &[Test] = &[
//             // 50%
//             Test {
//                 sorted_vec: &[1.0, 1.5, 2.0],
//                 split_at: 50.0,
//                 want_1: &[1.0, 1.5],
//                 want_2: &[2.0],
//             },
//             // limit
//             Test {
//                 sorted_vec: &[100.0, 110.0, 110.1],
//                 split_at: 10.0,
//                 want_1: &[100.0, 110.0],
//                 want_2: &[110.1],
//             },
//             // 0%
//             Test {
//                 sorted_vec: &[1.0, 1.1, 1.2],
//                 split_at: 0.0,
//                 want_1: &[1.0],
//                 want_2: &[1.1, 1.2],
//             },
//             // big numbers
//             Test {
//                 sorted_vec: &[1000.0, 1005.0, 1015.0],
//                 split_at: 1.0,
//                 want_1: &[1000.0, 1005.0],
//                 want_2: &[1015.0],
//             },
//             // ----------------------------------
//             // the same used in log cases
//             // 5% (Very strict):
//             Test {
//                 sorted_vec: &[100.0, 96.0, 95.0],
//                 split_at: 5.0,
//                 want_1: &[100.0, 96.0, 95.0],
//                 want_2: &[],
//             },
//             // 10% (to filtering OS noise)
//             Test {
//                 sorted_vec: &[100.0, 91.0, 90.0],
//                 split_at: 10.0,
//                 want_1: &[100.0, 91.0, 90.0],
//                 want_2: &[],
//             },
//             // 15% (More permissive for unstable environments.)
//             Test {
//                 sorted_vec: &[100.0, 87.0, 86.0],
//                 split_at: 15.0,
//                 want_1: &[100.0, 87.0, 86.0],
//                 want_2: &[],
//             },
//         ];

//         for test in CASES {
//             let mut benches: Vec<Benchmark> = Vec::new();
//             {
//                 let mut b = Benchmark::default();
//                 for value in test.sorted_vec {
//                     b.median = *value;
//                     benches.push(b.clone());
//                 }
//             }

//             let (keep, rest) = Filter::Pct(test.split_at).apply(benches, Sort::Median);

//             let have1: Vec<f64> = keep
//                 .iter()
//                 .map(|f| f.median())
//                 .collect();

//             let have2: Vec<f64> = rest
//                 .iter()
//                 .map(|f| f.median())
//                 .collect();

//             assert_eq!(have1, test.want_1);
//             assert_eq!(have2, test.want_2);
//         }
//     }
// }
//}
