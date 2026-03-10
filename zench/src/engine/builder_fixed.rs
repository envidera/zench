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

Report

Benchmark  10
Time       Median: 0.230ns
Stability  Std.Dev: ± 0.030ns | CV: 12.51%
Samples    Count: 10 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench/src/engine/builder_fixed.rs:61:15



Report

Benchmark  10
Time       Median: 0.230ns
Stability  Std.Dev: ± 0.030ns | CV: 12.51%
Samples    Count: 10 | Iters/sample: 524,288 | Outliers: 0.00%
Location   zench/src/engine/builder_fixed.rs:61:15


total time: 0.003118824 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-07 00:38:53 UTC

---------------------------------

bench zench/src/engine/builder_fixed.rs:61:15::50

Report

Benchmark  50
Time       Median: 0.216ns
Stability  Std.Dev: ± 0.025ns | CV: 10.70%
Samples    Count: 50 | Iters/sample: 524,288 | Outliers: 4.00%
Location   zench/src/engine/builder_fixed.rs:61:15



Report

Benchmark  50
Time       Median: 0.216ns
Stability  Std.Dev: ± 0.025ns | CV: 10.70%
Samples    Count: 50 | Iters/sample: 524,288 | Outliers: 4.00%
Location   zench/src/engine/builder_fixed.rs:61:15


total time: 0.012761632 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-07 00:38:53 UTC

---------------------------------

bench zench/src/engine/builder_fixed.rs:61:15::100

Report

Benchmark  100
Time       Median: 0.215ns
Stability  Std.Dev: ± 0.000ns | CV: 0.17%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 21.00%
Location   zench/src/engine/builder_fixed.rs:61:15



Report

Benchmark  100
Time       Median: 0.215ns
Stability  Std.Dev: ± 0.000ns | CV: 0.17%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 21.00%
Location   zench/src/engine/builder_fixed.rs:61:15


total time: 0.023478104 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-07 00:38:53 UTC

---------------------------------

bench zench/src/engine/builder_fixed.rs:61:15::500

Report

Benchmark  500
Time       Median: 0.215ns
Stability  Std.Dev: ± 0.000ns | CV: 0.07%
Samples    Count: 500 | Iters/sample: 524,288 | Outliers: 17.40%
Location   zench/src/engine/builder_fixed.rs:61:15



Report

Benchmark  500
Time       Median: 0.215ns
Stability  Std.Dev: ± 0.000ns | CV: 0.07%
Samples    Count: 500 | Iters/sample: 524,288 | Outliers: 17.40%
Location   zench/src/engine/builder_fixed.rs:61:15


total time: 0.115554131 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-07 00:38:53 UTC

---------------------------------

bench zench/src/engine/builder_fixed.rs:61:15::1000

Report

Benchmark  1000
Time       Median: 0.215ns
Stability  Std.Dev: ± 0.000ns | CV: 0.05%
Samples    Count: 1,000 | Iters/sample: 524,288 | Outliers: 16.60%
Location   zench/src/engine/builder_fixed.rs:61:15



Report

Benchmark  1000
Time       Median: 0.215ns
Stability  Std.Dev: ± 0.000ns | CV: 0.05%
Samples    Count: 1,000 | Iters/sample: 524,288 | Outliers: 16.60%
Location   zench/src/engine/builder_fixed.rs:61:15


total time: 0.229771044 sec
rust: 1.93.1 | profile release
zench: 0.1.0
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-07 00:38:53 UTC

*/
