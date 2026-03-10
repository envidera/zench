use super::super::algorithm;
use super::interface::ISampler;
use super::measure;
use std::time::Duration;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct A {
    // The minimum number of samples that must always be collected
    pub(crate) min_samples_count: usize,

    // The maximum number of samples allowed, primarily to avoid excessive iterations
    pub(crate) max_samples_count: usize,

    pub(crate) timeout_in_seconds: Duration,

    // Maximum acceptable variance (CV coefficient of variation = stddev /mean)
    pub(crate) stability_threshold: f64,
}

impl Default for A {
    fn default() -> Self {
        Self {
            min_samples_count: 100,
            max_samples_count: 1000,
            timeout_in_seconds: Duration::from_secs(2),
            stability_threshold: 0.03, //3%
        }
    }
}

impl ISampler for A {
    fn collect<F>(&self, closure: &mut F, batch_size: usize) -> (Vec<f64>, usize)
    where
        F: FnMut(),
    {
        let mut data = Vec::with_capacity(self.max_samples_count);

        let start_time = Instant::now();

        loop {
            let duration = measure::batch_duration(closure, batch_size);
            let single_duration = duration.as_nanos() as f64 / batch_size as f64;

            data.push(single_duration);

            let count = data.len();

            // -------------------------------
            // CONDITION AUTO

            // condition
            let enough_data = count >= self.min_samples_count;

            // condition
            let is_stable = {
                if enough_data {
                    let mean = algorithm::mean(&data);
                    let std_dev = algorithm::std_dev(&data, &mean);
                    let cv = algorithm::cv(std_dev, mean);

                    cv < self.stability_threshold
                } else {
                    false
                }
            };

            // condition
            let hit_max_data_capacity = count >= self.max_samples_count;

            // condition
            let timeout = start_time.elapsed() >= self.timeout_in_seconds;

            // -------------------------------

            if enough_data && is_stable {
                break (data, batch_size);
            }

            if hit_max_data_capacity || timeout {
                break (data, batch_size);
            }
        }
    }
}
