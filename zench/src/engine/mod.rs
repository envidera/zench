mod batch;
mod black_box;
mod builder_auto;
mod builder_fixed;
mod builder_full_fixed;
mod interface;
mod measure;

mod impl_batcher_auto_fixed_match;
mod impl_batcher_auto_loop_increment;
mod impl_batcher_fixed;
mod impl_sampler_auto_by_stability;
mod impl_sampler_fixed;

// ---------------------------------
pub(crate) use interface::IBatcher;
pub(crate) use interface::IEngine;

pub use black_box::bx;

pub use interface::EngineAuto;
pub use interface::EngineFixedSamples;
pub use interface::EngineFullFixed;
