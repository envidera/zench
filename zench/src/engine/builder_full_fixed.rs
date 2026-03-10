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
    use crate::Bench;

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

Report

Benchmark  10
Time       Median: 3.000ns
Stability  Std.Dev: ± 0.499ns | CV: 19.11%
Samples    Count: 10 | Iters/sample: 10 | Outliers: 0.00%
Location   zench/src/engine/builder_full_fixed.rs:83:15


total time: 0.000039243 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-08 18:29:26 UTC

---------------------------------

bench zench/src/engine/builder_full_fixed.rs:83:15::50

Report

Benchmark  50
Time       Median: 0.800ns
Stability  Std.Dev: ± 0.088ns | CV: 11.70%
Samples    Count: 50 | Iters/sample: 50 | Outliers: 0.00%
Location   zench/src/engine/builder_full_fixed.rs:83:15


total time: 0.000040426 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-08 18:29:26 UTC

---------------------------------

bench zench/src/engine/builder_full_fixed.rs:83:15::100

Report

Benchmark  100
Time       Median: 0.500ns
Stability  Std.Dev: ± 0.047ns | CV: 10.16%
Samples    Count: 100 | Iters/sample: 100 | Outliers: 0.00%
Location   zench/src/engine/builder_full_fixed.rs:83:15


total time: 0.000045605 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-08 18:29:26 UTC

---------------------------------

bench zench/src/engine/builder_full_fixed.rs:83:15::500

Report

Benchmark  500
Time       Median: 0.340ns
Stability  Std.Dev: ± 0.010ns | CV: 2.89%
Samples    Count: 500 | Iters/sample: 500 | Outliers: 0.00%
Location   zench/src/engine/builder_full_fixed.rs:83:15


total time: 0.000259805 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-08 18:29:26 UTC

---------------------------------

bench zench/src/engine/builder_full_fixed.rs:83:15::1000

Report

Benchmark  1000
Time       Median: 0.241ns
Stability  Std.Dev: ± 0.029ns | CV: 11.02%
Samples    Count: 1,000 | Iters/sample: 1,000 | Outliers: 0.00%
Location   zench/src/engine/builder_full_fixed.rs:83:15


total time: 0.000690739 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-08 18:29:26 UTC

*/
