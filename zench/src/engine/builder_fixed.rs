use std::time::Duration;

use super::interface::BatcherAuto;
use super::interface::SamplerFixed;
use super::EngineFixedSamples;

pub struct BuilderFixed {
    batcher: BatcherAuto,
    sampler: Option<SamplerFixed>,
}

impl EngineFixedSamples {
    pub fn builder() -> BuilderFixed {
        BuilderFixed::builder()
    }
}

impl BuilderFixed {
    pub fn builder() -> Self {
        Self {
            batcher: BatcherAuto::default(),
            sampler: None,
        }
    }

    pub fn build(self) -> EngineFixedSamples {
        EngineFixedSamples {
            batcher: self.batcher,
            sampler: self
                .sampler
                .unwrap_or_default(),
        }
    }

    pub fn samples(mut self, count: usize) -> Self {
        self.sampler = Some(SamplerFixed::default().samples(count));
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

    use std::time::Duration;

    use crate::builder::EngineFixedSamples;
    use crate::Bench;

    #[ignore = "display purpose"]
    #[test]
    fn test_full_fixed() {
        let defaults = EngineFixedSamples::builder().build();
        println!("{:#?}", defaults);
        /*
        Engine {
            batcher: A {
                batch: Batch {
                    min_duration: 50ms,
                    max_capacity: 500000,
                    single_shot_barrier: 500ms,
                },
            },
            sampler: F(
                FixedSamples {
                    samples_count: 100,
                },
            ),
        }
        */

        const CASES: [usize; 5] = [10, 50, 100, 500, 1000];
        for case in CASES {
            let e = EngineFixedSamples::builder()
                .batch_min_duration(Duration::from_millis(51))
                .batch_max_capacity(500_001)
                .batch_single_shot_barrier(Duration::from_millis(501))
                .samples(case)
                .build();

            let mut b = Bench::with_engine(e);

            b.bench(format!("{case}"), || {});

            b.report(|r| {
                r.print();

                let first = r
                    .first()
                    .unwrap();
                assert_eq!(first.samples_count(), case);
                assert!(first.iters_count() > 500_000);
            });
        }
    }
}

/*

─────┬─────────┬───────┬────────────┬──────────┬──────────────
name │ median  │  cv   │  std.dev   │ outliers │ samples/iters
─────┼─────────┼───────┼────────────┼──────────┼──────────────
10   │ 0.215ns │ 0.01% │  ± 0.000ns │   10.00% │  10 / 524,288
─────┴─────────┴───────┴────────────┴──────────┴──────────────
total time: 0.002801945 sec
rust: 1.94.0 (release) | zench: 0.1.4


─────┬─────────┬───────┬────────────┬──────────┬──────────────
name │ median  │  cv   │  std.dev   │ outliers │ samples/iters
─────┼─────────┼───────┼────────────┼──────────┼──────────────
50   │ 0.215ns │ 0.01% │  ± 0.000ns │   14.00% │  50 / 524,288
─────┴─────────┴───────┴────────────┴──────────┴──────────────
total time: 0.01186671 sec
rust: 1.94.0 (release) | zench: 0.1.4


─────┬─────────┬───────┬────────────┬──────────┬──────────────
name │ median  │  cv   │  std.dev   │ outliers │ samples/iters
─────┼─────────┼───────┼────────────┼──────────┼──────────────
100  │ 0.215ns │ 0.01% │  ± 0.000ns │   11.00% │ 100 / 524,288
─────┴─────────┴───────┴────────────┴──────────┴──────────────
total time: 0.023260987 sec
rust: 1.94.0 (release) | zench: 0.1.4


─────┬─────────┬───────┬────────────┬──────────┬──────────────
name │ median  │  cv   │  std.dev   │ outliers │ samples/iters
─────┼─────────┼───────┼────────────┼──────────┼──────────────
500  │ 0.215ns │ 0.01% │  ± 0.000ns │   13.20% │ 500 / 524,288
─────┴─────────┴───────┴────────────┴──────────┴──────────────
total time: 0.114491796 sec
rust: 1.94.0 (release) | zench: 0.1.4


─────┬─────────┬───────┬────────────┬──────────┬────────────────
name │ median  │  cv   │  std.dev   │ outliers │  samples/iters
─────┼─────────┼───────┼────────────┼──────────┼────────────────
1000 │ 0.215ns │ 0.01% │  ± 0.000ns │   11.50% │ 1,000 / 524,288
─────┴─────────┴───────┴────────────┴──────────┴────────────────
total time: 0.227757121 sec
rust: 1.94.0 (release) | zench: 0.1.4

*/
