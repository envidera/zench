use crate::algorithm;
use crate::benchmark::format_number::FormatNumber;
use crate::benchmark::time_unit::TimeUnit;
use crate::benchmark::Samples;

#[derive(Debug, Clone, PartialEq)]
pub struct Benchmark {
    pub(crate) name: String,
    pub(crate) median: f64,
    pub(crate) std_dev: f64,
    pub(crate) cv: f64,
    pub(crate) cv_pct: f64,
    pub(crate) samples: Samples,
    pub(crate) samples_count: usize,
    pub(crate) batch_size: usize,
    pub(crate) outliers_count: usize,
    pub(crate) outliers_pct: f64, // percent
    pub(crate) location: String,  // the caller file and line
}

impl Default for Benchmark {
    fn default() -> Self {
        Self {
            name: String::new(),
            median: 0.0,
            std_dev: 0.0,
            cv: 0.0,
            cv_pct: 0.0,
            samples: Samples::new(),
            samples_count: 0,
            batch_size: 0,
            outliers_count: 0,
            outliers_pct: 0.0,
            location: String::new(),
        }
    }
}

impl Benchmark {
    pub fn new(name: String, location: String, data: Vec<f64>, iters: usize) -> Self {
        Self {
            name,
            location,
            ..Self::default()
        }
        .compute_stats(&data, iters)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn median(&self) -> f64 {
        self.median
    }

    pub fn std_dev(&self) -> f64 {
        self.std_dev
    }

    pub fn cv(&self) -> f64 {
        self.cv
    }

    pub fn cv_pct(&self) -> f64 {
        self.cv_pct
    }

    pub fn samples_count(&self) -> usize {
        self.samples_count
    }

    pub fn iters_count(&self) -> usize {
        self.batch_size
    }

    pub fn outliers_count(&self) -> usize {
        self.outliers_count
    }

    pub fn outliers_pct(&self) -> f64 {
        self.outliers_pct
    }

    pub fn location(&self) -> &str {
        &self.location
    }

    //-------------------------------------

    /// Compute statistical metrics from the engine.collect_data times.
    pub(crate) fn compute_stats(mut self, data: &[f64], batch_size: usize) -> Self {
        if data.is_empty() {
            return Benchmark::default();
        }

        let samples = Samples::from(data);

        // Do not include outliers to calculate statistics
        let mut data_without_outliers = samples.data_without_outliers();

        let samples_count = samples.len();

        let mean = algorithm::mean(&data_without_outliers);
        let median = algorithm::median(&mut data_without_outliers);
        let std_dev = algorithm::std_dev(&data_without_outliers, &mean);
        let outliers_count = samples.count_outliers();
        let outlier_pct = (outliers_count as f64 / samples_count as f64) * 100.0;
        let cv = algorithm::cv(std_dev, mean);
        let cv_pct = algorithm::cv_pct(std_dev, mean);

        self.samples_count = samples_count;
        self.batch_size = batch_size;
        self.median = median;
        self.std_dev = std_dev;
        self.outliers_count = outliers_count;
        self.samples = samples;
        self.cv = cv;
        self.cv_pct = cv_pct;
        self.outliers_pct = outlier_pct;

        self
    }

    //-------------------------------------

    pub fn median_fmt(&self) -> String {
        let unit = TimeUnit::from_nanos(self.median);
        let value = unit.convert(self.median);
        format!("{:.3}{}", value, unit.suffix())
    }

    pub fn std_dev_fmt(&self) -> String {
        let unit = TimeUnit::from_nanos(self.median);
        let value = unit.convert(self.std_dev);
        format!("± {:.3}{}", value, unit.suffix())
    }

    pub fn cv_pct_fmt(&self) -> String {
        format!("{:.2}%", self.cv_pct)
    }

    pub fn samples_count_fmt(&self) -> String {
        FormatNumber::coma(self.samples_count)
    }

    pub fn iters_count_fmt(&self) -> String {
        FormatNumber::coma(self.batch_size)
    }

    pub fn outliers_pct_fmt(&self) -> String {
        format!("{:.2}%", self.outliers_pct)
    }
}

// ================================================================
// Unit test
// ================================================================
#[cfg(test)]
mod tests {
    use std::f64;

    use super::*;

    #[test]
    fn test_compute_stats_fixed_values() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0];

        let bench = Benchmark::default().compute_stats(&data, 1);

        assert_eq!(bench.samples_count, 5);
        assert_eq!(bench.batch_size, 1);

        // median
        assert!((bench.median - 3.0).abs() < 1e-10);

        // std deviation (population)
        let expected_std = f64::consts::SQRT_2; // 1.41421356237;
        assert!((bench.std_dev - expected_std).abs() < 1e-10);

        // cv
        let expected_cv = expected_std / 3.0;
        assert!((bench.cv - expected_cv).abs() < 1e-10);

        // cv pct
        let expected_cv_pct = expected_cv * 100.0;
        assert!((bench.cv_pct - expected_cv_pct).abs() < 1e-10);

        // outliers
        assert_eq!(bench.outliers_count, 0);
        assert!((bench.outliers_pct - 0.0).abs() < 1e-10);
    }
}
