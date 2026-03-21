use super::super::location;
use crate::benchmark::BenchSet;
use crate::benchmark::Filter;
use crate::benchmark::Sort;
use crate::utc::Utc;
use std::fmt::Display;
use std::time::Instant;

//use super::header::Header;
use crate::benchmark::Benchmark;

#[derive(Debug, Clone, Default)]
pub struct Report {
    pub(crate) title: String,
    pub(crate) sorted_by: Option<Sort>,
    pub(crate) filter_log: Option<Vec<String>>,
    // ----------------------------------------------------------------
    pub(crate) benchset: BenchSet,
    pub(crate) benchset_rest: BenchSet, // used to saves filtered Benchmarks, the rest
    pub(crate) initial_time: Option<Instant>,
}

impl Report {
    //! # 📦 Core API
    //!
    //! Primary methods to inspect the report..

    pub fn new() -> Self {
        Self {
            benchset: BenchSet::new(),
            benchset_rest: BenchSet::new(),
            initial_time: None,
            title: String::new(),
            sorted_by: None,
            filter_log: None,
        }
    }

    pub fn title(&mut self, value: impl Display) -> &mut Self {
        self.title = value.to_string();
        self
    }

    pub fn sorted_by(&mut self) -> Option<Sort> {
        self.sorted_by
    }

    pub fn sort_by_median(&mut self) -> &mut Report {
        self.apply_sort(Sort::Median);
        self
    }

    pub fn sort_by_median_reverse(&mut self) -> &mut Report {
        self.apply_sort(Sort::MedianReverse);
        self
    }

    pub fn sort_by_samples(&mut self) -> &mut Report {
        self.apply_sort(Sort::Samples);
        self
    }

    pub fn sort_by_samples_reverse(&mut self) -> &mut Report {
        self.apply_sort(Sort::SamplesReverse);
        self
    }

    pub fn sort_by_outliers(&mut self) -> &mut Report {
        self.apply_sort(Sort::Outliers);
        self
    }

    pub fn sort_by_outliers_reverse(&mut self) -> &mut Report {
        self.apply_sort(Sort::OutliersReverse);
        self
    }

    pub fn sort_by_std_dev(&mut self) -> &mut Report {
        self.apply_sort(Sort::StdDev);
        self
    }

    pub fn sort_by_std_dev_reverse(&mut self) -> &mut Report {
        self.apply_sort(Sort::StdDevReverse);
        self
    }

    #[track_caller]
    pub fn filter_n(&mut self, value: usize) -> &mut Report {
        self.apply_filter(Filter::N(value), location!());
        self
    }

    #[track_caller]
    pub fn filter_proximity_pct(&mut self, value: f64) -> &mut Report {
        self.apply_filter(Filter::Proximity(value), location!());
        self
    }

    #[track_caller]
    pub fn filter_pct(&mut self, value: f64) -> &mut Report {
        self.apply_filter(Filter::Pct(value), location!());
        self
    }

    // TODO: think about

    // pub fn average_median_pct(&self) -> f64 {
    //     let medians: Vec<f64> = self
    //         .benchset
    //         .iter()
    //         .map(|b| b.median())
    //         .collect();

    //     let mean = algorithm::mean(&medians);
    //     let std_dev = algorithm::std_dev(&medians, &mean);
    //     algorithm::cv_pct(std_dev, mean)
    // }

    // pub fn cv_pct(&self) -> f64 {
    //     // TODO: connect cv_pct to sort_order_filter, similar to filter_pct
    //     let data: Vec<f64> = self
    //         .benchset
    //         .iter()
    //         .map(|b| b.median())
    //         .collect();

    //     let mean = algorithm::mean(&data);
    //     let std_dev = algorithm::std_dev(&data, &mean);

    //     algorithm::cv_pct(&std_dev, &mean)
    // }

    /// Splits the current report state and returns a tuple containing the current and the remaining reports.
    pub fn split(&mut self) -> (Report, Report) {
        (
            // keep
            Report {
                benchset: std::mem::take(&mut self.benchset),
                filter_log: self
                    .filter_log
                    .take(),

                sorted_by: self.sorted_by,

                ..Default::default()
            },
            //rest
            Report {
                benchset: std::mem::take(&mut self.benchset_rest),
                sorted_by: self.sorted_by,
                ..Default::default()
            },
        )
    }

    pub fn print(&mut self) -> &mut Self {
        if !self
            .benchset
            .is_empty()
        {
            println!("{}", self);
            //self.render_with(DefaultRender);
        }
        self
    }

    pub fn env_rust_profile(&self) -> &'static str {
        env!("BUILD_PROFILE")
    }

    pub fn env_rust_version(&self) -> &'static str {
        env!("RUSTC_VERSION")
    }

    pub fn env_current_date_time(&self) -> String {
        Utc::now().to_string()
    }

    pub fn env_zench_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    pub fn env_zench_total_time_execution(&self) -> Option<f64> {
        self.initial_time
            .map(|t| {
                t.elapsed()
                    .as_secs_f64()
            })
        //.unwrap_or("unknown".to_string())
    }

    pub fn env_sys_os(&self) -> &'static str {
        env!("SYS_OS")
    }

    pub fn env_sys_arch(&self) -> &'static str {
        env!("SYS_ARCH")
    }

    pub fn env_sys_cpu_threads(&self) -> &'static str {
        env!("SYS_CPU_THREADS")
    }

    pub fn env_sys_cpu_model(&self) -> &'static str {
        env!("SYS_CPU_MODEL")
    }

    /* pub fn signature(&self) -> u64 {
        let word = format!(
            "{}{}{}",
            self.env_rust_profile(),
            self.env_rust_version(),
            self.env_zench_version()
        );

        fnv1a(&word)
    } */

    // fn render_with<R: Renderer>(&mut self, renderer: R) -> &mut Self {
    //     let stdout = std::io::stdout().lock();
    //     let mut writer = std::io::BufWriter::new(stdout);

    //     if let Err(e) = renderer.write(self, &mut writer) {
    //         eprintln!("Zench Error: Failed to write report: {}", e);
    //     }

    //     self
    // }

    // pub fn render_with<R: Renderer>(&self, renderer: R) {
    //     let stdout = std::io::stdout().lock();
    //     let mut writer = std::io::BufWriter::new(stdout);

    //     if let Err(e) = renderer.render(self, &mut writer) {
    //         eprintln!("Zench Error: Failed to write report: {}", e);
    //     }

    //     let _ = writer.flush();
    // }

    /// Returns all benchmarks inside the current report
    pub fn benchset(&self) -> BenchSet {
        self.benchset
            .clone()
    }

    pub fn first(&self) -> Option<&Benchmark> {
        self.benchset
            .first()
    }

    /// Returns the number of benchmarks contained in the report.
    pub fn len(&self) -> usize {
        self.benchset
            .len()
    }

    /// Returns `true` if the report contains no benchmarks.
    pub fn is_empty(&self) -> bool {
        self.benchset
            .is_empty()
    }

    /// Returns an iterator over the benchmarks in the report.
    ///
    /// This enables direct iteration with `for` loops and `iter()`
    pub fn iter(&self) -> std::slice::Iter<'_, Benchmark> {
        self.benchset
            .iter()
    }

    // ----------------------------------------------

    fn push_modifier(&mut self, value: String) {
        self.filter_log
            .get_or_insert_with(Vec::new)
            .push(value);
    }

    fn apply_filter(&mut self, filter: Filter, location: String) {
        let sort = self
            .sorted_by
            .unwrap_or_else(|| {
                panic!("Zench - filters must be sorted before being used. \n{location}\n")
            });

        self.push_modifier(filter.to_string());

        let benchset = std::mem::replace(&mut self.benchset, BenchSet::new());
        let (keep, rest) = benchset.filter(filter, sort);

        self.benchset = keep;
        self.benchset_rest = rest;
    }

    fn apply_sort(&mut self, sort: Sort) {
        self.benchset
            .sort_by(sort);

        self.push_modifier(sort.to_string());

        self.sorted_by = Some(sort);
    }

    // ----------------------------------------------

    pub(crate) fn push(&mut self, b: Benchmark) {
        self.benchset
            .push(b);
    }
}

impl<'a> IntoIterator for &'a Report {
    type Item = &'a Benchmark;
    type IntoIter = std::slice::Iter<'a, Benchmark>;

    fn into_iter(self) -> Self::IntoIter {
        self.benchset
            .iter()
    }
}

// ================================================================
// Unit test
// ================================================================
#[cfg(test)]
mod tests {

    use crate::benchmark::Sort;
    use crate::bx;

    #[test]
    fn test_split() {
        let mut b = Bench::new();

        for _ in 1..=10 {
            b.bench("empty bench", || {
                bx(());
            });
        }

        b.report(|r| {
            let (mut r1, mut r2) = r
                .sort_by_median()
                .filter_n(2)
                .split();

            // --------------------------------
            // assert Report len()
            assert_eq!(r1.len(), 2);
            assert_eq!(r2.len(), 8);

            // --------------------------------
            // assert Report title

            r1.title("fastest");
            r2.title("rest");

            assert_eq!(r1.title, "fastest");
            assert_eq!(r2.title, "rest");

            // --------------------------------
            // assert Report Filter caption

            assert_eq!(
                r1.filter_log
                    .as_ref()
                    .unwrap()
                    .join(" > "),
                "Sort Median > Filter N(2)"
            );

            assert!(r2
                .filter_log
                .is_none());

            // --------------------------------
            // assert sorted_by

            match r1.sorted_by() {
                Some(s) => assert_eq!(s, Sort::Median),
                None => panic!("r1 expected Sort::Median"),
            }

            match r2.sorted_by() {
                Some(s) => assert_eq!(s, Sort::Median),
                None => panic!("r2 expected Sort::Median"),
            }
        });
    }
}
