use super::algorithm;
use super::bx;
use crate::__internal::*;
use std::time::{Duration, Instant};

//const STABILITY_BATCH_SIZE: usize = 100;
const STABILITY_THRESHOLD: f64 = 0.05; // 5% variation

#[derive(Debug, Clone)]
pub struct Warmup {
    /// enable or disable warmup
    ///
    /// default: true
    pub(crate) enabled: bool,

    /// Total warmup time duration in milliseconds
    ///
    /// default: 3_000
    pub(crate) total_duration_in_milliseconds: u64,

    /// Number of stable measurements required to consider warmup complete
    ///
    /// default: 20
    stabilized_trigger_count: u64,

    /// default: 100
    stability_batch_size: usize,
}

impl Default for Warmup {
    fn default() -> Self {
        Self {
            enabled: true,                         // default: true
            total_duration_in_milliseconds: 3_000, // default 3_000
            stabilized_trigger_count: 20,          // default 20
            stability_batch_size: 100,             // default 100
        }
    }
}
impl Warmup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }

    pub fn seconds(value: u64) -> Self {
        Self {
            total_duration_in_milliseconds: value.saturating_mul(1000),
            ..Self::default()
        }
    }

    pub(crate) fn run<F>(&mut self, closure: &mut F)
    where
        F: FnMut(),
    {
        if !self.enabled {
            return;
        }

        fprint!(
            "warming up {time}ms + stabilization",
            time = self.total_duration_in_milliseconds,
        );

        // self.warm_up_self();
        self.warm_up_closure(closure);

        fprintln!("");
    }

    /*
    Warming up in two phases

    - Initial execution + stabilization
    - Detects when the CPU stabilizes the frequency
    - Reduces variability of the first measurements
    */
    pub(crate) fn warm_up_closure<F>(&mut self, closure: &mut F)
    where
        F: FnMut(),
    {
        let half_warmup = Duration::from_millis(self.total_duration_in_milliseconds / 2);

        fprint!(".");
        //Phase 1: Fast initial executions
        self.warm_up_initial_phase(closure, half_warmup);

        fprint!(".");
        // Phase 2: CPU frequency stabilization
        self.warm_up_stabilization_phase(closure, half_warmup);
    }

    //Phase 1: Fast initial executions
    fn warm_up_initial_phase<F, R>(&self, closure: &mut F, duration: Duration)
    where
        F: FnMut() -> R,
    {
        let start = Instant::now();

        while start.elapsed() < duration {
            bx(closure());
        }
    }

    // Phase 2: CPU frequency stabilization
    fn warm_up_stabilization_phase<F, R>(&self, closure: &mut F, max_duration: Duration)
    where
        F: FnMut() -> R,
    {
        let start = Instant::now();
        let mut previous = Duration::ZERO;
        let mut stable_count = 0;

        while stable_count < self.stabilized_trigger_count && start.elapsed() < max_duration {
            let measure_start = Instant::now();
            for _ in 0..self.stability_batch_size {
                bx(closure());
            }
            let current = measure_start.elapsed();

            // Checks if the timing is stabilizing
            if previous > Duration::ZERO {
                let variation =
                    algorithm::pct_variation(current.as_nanos() as f64, previous.as_nanos() as f64);

                if variation < STABILITY_THRESHOLD {
                    stable_count += 1;
                } else {
                    stable_count = 0;
                }
            }
            previous = current;
        }
    }
}
