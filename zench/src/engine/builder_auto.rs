use std::time::Duration;

use super::interface::BatcherAuto;
use super::interface::SamplerAuto;
use super::EngineAuto;

pub struct BuilderAuto {
    batcher: BatcherAuto,
    sampler: SamplerAuto,
}

impl EngineAuto {
    pub fn builder() -> BuilderAuto {
        BuilderAuto::builder()
    }
}

impl BuilderAuto {
    pub fn builder() -> Self {
        Self {
            batcher: BatcherAuto::default(),
            sampler: SamplerAuto::default(),
        }
    }

    pub fn build(self) -> EngineAuto {
        EngineAuto {
            batcher: self.batcher,
            sampler: self.sampler,
        }
    }

    pub fn samples_min_count(mut self, count: usize) -> Self {
        self.sampler
            .min_samples_count = count;

        self
    }

    pub fn samples_max_count(mut self, count: usize) -> Self {
        self.sampler
            .max_samples_count = count;

        self
    }

    pub fn samples_timeout_in_seconds(mut self, secs: usize) -> Self {
        self.sampler
            .timeout_in_seconds = Duration::from_secs(secs as u64);

        self
    }

    pub fn samples_stability_threshold(mut self, value: f64) -> Self {
        self.sampler
            .stability_threshold = value / 100.0;

        self
    }

    pub fn batch_max_capacity(mut self, value: usize) -> Self {
        self.batcher
            .batch
            .max_capacity = value;

        self
    }

    pub fn batch_min_duration(mut self, dur: Duration) -> Self {
        self.batcher
            .batch
            .min_duration = dur;

        self
    }

    pub fn batch_single_shot_barrier(mut self, dur: Duration) -> Self {
        self.batcher
            .batch
            .single_shot_barrier = dur;

        self
    }
}

// ================================================================
// Unit test
// ================================================================
#[cfg(test)]
mod tests {

    use crate::builder::EngineAuto;
    use crate::Bench;
    use std::time::Duration;

    #[ignore = "display purpose"]
    #[test]
    fn test_full_fixed() {
        let defaults = EngineAuto::builder().build();
        println!("{:#?}", defaults);
        /*
        Engine {
            batcher: A(
                LoopIncrement {
                    batch: Batch {
                        min_duration: 50ms,
                        max_capacity: 500000,
                        single_shot_barrier: 500ms,
                    },
                },
            ),
            sampler: A(
                SamplesByStability {
                    min_samples_count: 100,
                    max_samples_count: 1000,
                    timeout_in_seconds: 2s,
                    stability_threshold: 0.03,
                },
            ),
        }
        */

        let e = EngineAuto::builder()
            .batch_min_duration(Duration::from_millis(51))
            .batch_max_capacity(500_001)
            .batch_single_shot_barrier(Duration::from_millis(501))
            .samples_min_count(101)
            .samples_max_count(1_001)
            .samples_timeout_in_seconds(3)
            .samples_stability_threshold(0.031)
            .build();

        let mut b = Bench::with_engine(e);

        b.bench("empty", || {});

        b.report(|r| {
            r.print();

            // let first = r
            //     .first()
            //     .unwrap();
            //assert_eq!(first.samples_count(), case);
            //assert!(first.iters_count() > 500_000);
        });
    }
}

/*

──────┬─────────┬───────┬────────────┬──────────┬────────────────
name  │ median  │  cv   │  std.dev   │ outliers │  samples/iters
──────┼─────────┼───────┼────────────┼──────────┼────────────────
empty │ 0.215ns │ 0.01% │  ± 0.000ns │   11.59% │ 1,001 / 524,288
──────┴─────────┴───────┴────────────┴──────────┴────────────────
total time: 0.229525536 sec
rust: 1.94.0 (release) | zench: 0.1.4

*/
