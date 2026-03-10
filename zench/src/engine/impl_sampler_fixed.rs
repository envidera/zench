use super::interface::ISampler;
use super::measure;

// ----------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct F {
    samples_count: usize,
}

impl Default for F {
    fn default() -> Self {
        Self { samples_count: 100 }
    }
}

impl F {
    pub fn samples(mut self, count: usize) -> Self {
        self.samples_count = count;
        self
    }
}

impl ISampler for F {
    fn collect<F>(&self, closure: &mut F, batch_size: usize) -> (Vec<f64>, usize)
    where
        F: FnMut(),
    {
        let mut data = Vec::with_capacity(self.samples_count);

        for _ in 0..self.samples_count {
            let duration = measure::batch_duration(closure, batch_size);
            let single_duration = duration.as_nanos() as f64 / batch_size as f64;
            data.push(single_duration);
        }

        (data, batch_size)
    }
}
