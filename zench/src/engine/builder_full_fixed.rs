use super::interface::BatcherFixed;
use super::interface::SamplerFixed;
use super::EngineFullFixed;

pub struct BuilderFullFixed {
    batcher: Option<BatcherFixed>,
    sampler: Option<SamplerFixed>,
}

impl EngineFullFixed {
    pub fn builder() -> BuilderFullFixed {
        BuilderFullFixed::builder()
    }
}

impl BuilderFullFixed {
    pub fn builder() -> Self {
        Self {
            batcher: None,
            sampler: None,
        }
    }

    pub fn build(self) -> EngineFullFixed {
        EngineFullFixed {
            batcher: self
                .batcher
                .unwrap_or_default(),
            sampler: self
                .sampler
                .unwrap_or_default(),
        }
    }

    pub fn samples(mut self, count: usize) -> Self {
        self.sampler = Some(SamplerFixed::default().samples(count));
        self
    }

    pub fn batch(mut self, size: usize) -> Self {
        self.batcher = Some(BatcherFixed::default().set_batch_size(size));
        self
    }
}

// ================================================================
// Unit test
// ================================================================
#[cfg(test)]
mod tests {

    use crate::builder::EngineFullFixed;

    #[test]
    fn test_full_fixed() {
        let defaults = EngineFullFixed::builder().build();
        println!("{:#?}", defaults);
        /*
        Engine {
            batcher: F(
                FixedBatch {
                    batch_size: 100,
                },
            ),
            sampler: F(
                FixedSamples {
                    samples_count: 100,
                },
            ),
        }
        */

        const CASES: [usize; 5] = [10, 50, 100, 500, 1000];
        for case in CASES {
            let e = EngineFullFixed::builder()
                .batch(case)
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
                assert_eq!(first.iters_count(), case);
            });
        }
    }
}

/*

─────┬─────────┬────────┬────────────┬──────────┬──────────────
name │ median  │   cv   │  std.dev   │ outliers │ samples/iters
─────┼─────────┼────────┼────────────┼──────────┼──────────────
10   │ 3.000ns │ 19.11% │  ± 0.499ns │    0.00% │       10 / 10
─────┴─────────┴────────┴────────────┴──────────┴──────────────
total time: 0.000044233 sec
rust: 1.94.0 (release) | zench: 0.1.4


─────┬─────────┬────────┬────────────┬──────────┬──────────────
name │ median  │   cv   │  std.dev   │ outliers │ samples/iters
─────┼─────────┼────────┼────────────┼──────────┼──────────────
50   │ 0.800ns │ 12.53% │  ± 0.094ns │    0.00% │       50 / 50
─────┴─────────┴────────┴────────────┴──────────┴──────────────
total time: 0.000039524 sec
rust: 1.94.0 (release) | zench: 0.1.4


─────┬─────────┬────────┬────────────┬──────────┬──────────────
name │ median  │   cv   │  std.dev   │ outliers │ samples/iters
─────┼─────────┼────────┼────────────┼──────────┼──────────────
100  │ 0.500ns │ 10.17% │  ± 0.047ns │    0.00% │     100 / 100
─────┴─────────┴────────┴────────────┴──────────┴──────────────
total time: 0.000046698 sec
rust: 1.94.0 (release) | zench: 0.1.4


─────┬─────────┬───────┬────────────┬──────────┬──────────────
name │ median  │  cv   │  std.dev   │ outliers │ samples/iters
─────┼─────────┼───────┼────────────┼──────────┼──────────────
500  │ 0.340ns │ 2.78% │  ± 0.009ns │    0.00% │     500 / 500
─────┴─────────┴───────┴────────────┴──────────┴──────────────
total time: 0.000260807 sec
rust: 1.94.0 (release) | zench: 0.1.4


─────┬─────────┬───────┬────────────┬──────────┬──────────────
name │ median  │  cv   │  std.dev   │ outliers │ samples/iters
─────┼─────────┼───────┼────────────┼──────────┼──────────────
1000 │ 0.301ns │ 0.17% │  ± 0.000ns │   13.50% │ 1,000 / 1,000
─────┴─────────┴───────┴────────────┴──────────┴──────────────
total time: 0.000738159 sec
rust: 1.94.0 (release) | zench: 0.1.4

*/
