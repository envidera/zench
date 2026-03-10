use std::time::Duration;
use std::time::Instant;

pub(crate) fn batch_duration<F, R>(closure: &mut F, size: usize) -> Duration
where
    F: FnMut() -> R,
{
    let start = Instant::now();

    for _ in 0..size {
        // std::hint::black_box(closure()) prevents the compiler from
        // optimizing away the loop or ignoring the closure execution.
        //
        // Zench protects:
        // - The closure loop
        //
        // // User must protect with bx/black_box:
        // - Constant inputs: To prevent pre-calculation.
        // - Outputs: If you have a complex block and want to be 100%
        //   sure a specific part is not ignored or removed.

        // DO NOT REMOVE IT
        std::hint::black_box(closure());
    }
    start.elapsed()
}
