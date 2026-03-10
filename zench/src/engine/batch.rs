use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Batch {
    // The minimum total measurement time, in microseconds.
    pub(crate) min_duration: Duration,

    // The maximum iters size
    pub(crate) max_capacity: usize,

    pub(crate) single_shot_barrier: Duration,
}

impl Default for Batch {
    fn default() -> Self {
        Self {
            min_duration: BatchStability::Stable.duration(),
            max_capacity: 500_000,
            single_shot_barrier: Duration::from_millis(500),
        }
    }
}

// impl Batch {
//     pub fn min_duration(&self) -> Duration {
//         self.min_duration
//     }

//     pub fn max_capacity(&self) -> usize {
//         self.max_capacity
//     }

//     pub fn single_shot_barrier_duration(&self) -> Duration {
//         self.single_shot_barrier
//     }
// }

#[allow(unused)]
pub enum BatchStability {
    Fast,     // 1ms
    Balanced, // 10ms
    Stable,   // 50ms
}

impl BatchStability {
    pub fn duration(&self) -> Duration {
        match self {
            Self::Fast => Duration::from_millis(1),
            Self::Balanced => Duration::from_millis(10),
            Self::Stable => Duration::from_millis(150),
        }
    }
}
