use super::batch::Batch;
use super::interface::IBatcher;
use super::measure;

#[derive(Debug, Clone, Default)]
pub struct A {
    pub(crate) batch: Batch,
}

impl IBatcher for A {
    fn estimate_batch_size<F, R>(&self, closure: &mut F) -> usize
    where
        F: FnMut() -> R,
    {
        let single_duration = measure::batch_duration(closure, 1);
        let hit_single_shot_barrier = single_duration
            >= self
                .batch
                .single_shot_barrier;

        if hit_single_shot_barrier {
            return 1;
        }

        //-------------------

        let mut batch = 2;

        loop {
            let duration = measure::batch_duration(closure, batch);

            // condition ---------------------
            let enough_duration = duration
                >= self
                    .batch
                    .min_duration;

            // condition ---------------------
            let hit_max_capacity = batch
                >= self
                    .batch
                    .max_capacity;

            // -------------------------------
            // BREAK
            if enough_duration || hit_max_capacity {
                return batch;
            }

            // -------------------------------
            // Increment
            // exponential growth
            //if batch < 100_000 {
            batch = batch.saturating_mul(2);
            //} else {
            //batch = batch.saturating_add(100_000);
            //}
        }
    }
}
