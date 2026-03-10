#[allow(unused)]
use super::impl_batcher_auto_fixed_match;
use super::impl_batcher_auto_loop_increment;
use super::impl_batcher_fixed;
use super::impl_sampler_auto_by_stability;
use super::impl_sampler_fixed;

// config versions
pub(crate) type BatcherAuto = impl_batcher_auto_loop_increment::A;
pub(crate) type BatcherFixed = impl_batcher_fixed::F;
pub(crate) type SamplerAuto = impl_sampler_auto_by_stability::A;
pub(crate) type SamplerFixed = impl_sampler_fixed::F;

pub type EngineAuto = Engine<BatcherAuto, SamplerAuto>;
pub type EngineFixedSamples = Engine<BatcherAuto, SamplerFixed>;
pub type EngineFullFixed = Engine<BatcherFixed, SamplerFixed>;

// ================================================================
// INTERFACE

type Data = Vec<f64>;
type BatchSize = usize;

pub trait IEngine {
    fn collect_data<F>(&self, closure: &mut F) -> (Data, BatchSize)
    where
        F: FnMut();

    //fn name() -> String;
}
/// Trait to estimate batch size (iters/sample).
pub trait IBatcher {
    fn estimate_batch_size<F, R>(&self, closure: &mut F) -> BatchSize
    where
        F: FnMut() -> R;
}

/// Trait to collect raw timing data (`Vec<f64>`) using a batch size.
pub trait ISampler {
    fn collect<F>(&self, closure: &mut F, batch_size: usize) -> (Data, BatchSize)
    where
        F: FnMut();
}

// ----------------------------------------------------------------

/// Trait to coordinate batching and sampling.
#[derive(Clone, Debug)]
pub struct Engine<B, S> {
    pub(crate) batcher: B,
    pub(crate) sampler: S,
}

impl<B, S> Default for Engine<B, S>
where
    B: IBatcher + Default,
    S: ISampler + Default,
{
    fn default() -> Self {
        Self {
            batcher: B::default(),
            sampler: S::default(),
        }
    }
}

impl<B, S> IEngine for Engine<B, S>
where
    B: IBatcher,
    S: ISampler,
{
    /// Estimates batch size and collects raw timing data (`Vec<f64>`)
    fn collect_data<F>(&self, closure: &mut F) -> (Data, BatchSize)
    where
        F: FnMut(),
    {
        let batch_size = self
            .batcher
            .estimate_batch_size(closure);

        self.sampler
            .collect(closure, batch_size)
    }
}
