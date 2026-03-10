//use super::batcher_core::Batch;
use super::interface::IBatcher;
use super::measure;

use std::time::Duration;

#[allow(unused)]
#[derive(Debug, Clone, Default)]
pub struct A;

impl IBatcher for A {
    fn estimate_batch_size<F, R>(&self, closure: &mut F) -> usize
    where
        F: FnMut() -> R,
    {
        let single_duration = measure::batch_duration(closure, 1);

        // TODO: analyze this times
        // It can be compared to the automatic version
        match single_duration {
            d if d < Duration::from_nanos(500) => 100_000, // batch ~50ms
            d if d < Duration::from_micros(1) => 50_000,   // batch ~50ms
            d if d < Duration::from_micros(5) => 10_000,   // batch ~50ms
            d if d < Duration::from_micros(10) => 5_000,   // batch ~50ms
            d if d < Duration::from_micros(50) => 1_000,   // batch ~50ms
            d if d < Duration::from_micros(100) => 500,    // batch ~50ms
            d if d < Duration::from_micros(500) => 100,    // batch ~50ms
            d if d < Duration::from_millis(1) => 50,       // batch ~50ms
            d if d < Duration::from_millis(5) => 10,       // batch ~50ms
            d if d < Duration::from_millis(10) => 5,       // batch ~50ms
            d if d < Duration::from_millis(100) => 1,      // batch ~50-100ms
            _ => 1,
        }
    }
}
