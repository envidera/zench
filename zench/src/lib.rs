#![allow(clippy::let_and_return)]
#![doc = include_str!(concat!("../", std::env!("CARGO_PKG_README")))]
//!
//! # Item Details
//! ## Ready-to-use
//!
//! - [`Bench`] or [`bench!`] - for benchmarking
//! - [`bx`] - prevents compiler optimizations
//! - [`issue`] - report unexpected behavior with warn or panic state
//!
//! ## Customization
//!
//! - [`builder`] - configure benchmarks programmatically
//! - [`Report`] - customize report output
//!
//!
//!
mod algorithm;
mod bench;
mod bench_drop;
mod benchmark;
mod engine;
mod global;
mod macro_force_print;
mod macro_location;
mod macro_zench;
mod mock;
mod report;
mod utc;
mod warmup;

// ----------------------------------------------------------------
// internal

#[doc(hidden)]
pub mod __internal {
    pub use crate::fprint;
    pub use crate::fprintln;
    pub use crate::global::Command;
    //pub use crate::global::Ignore;
    pub use crate::bench::Bench;
    pub use crate::report::color;
}

// ----------------------------------------------------------------
// dev

#[doc(hidden)]
pub mod dev {

    pub mod benchmark {
        pub use crate::benchmark::Sort;
    }

    pub mod fibonacci {
        pub use crate::mock::fibonacci::fast;
        pub use crate::mock::fibonacci::slow;
    }

    pub mod mock {
        pub use crate::mock::fibonacci;
        pub use crate::mock::generate_data;
        pub use crate::mock::simulate_cpu_work;
    }

    pub mod algorithm {
        pub use crate::algorithm::cv;
        pub use crate::algorithm::cv_pct;
        pub use crate::algorithm::log2_distance;
        pub use crate::algorithm::log_distance;
        pub use crate::algorithm::mad;
        pub use crate::algorithm::mean;
        pub use crate::algorithm::median;
        pub use crate::algorithm::pct_variation;
        pub use crate::algorithm::std_dev;
    }
}

// ----------------------------------------------------------------
// user

pub use crate::engine::bx;

pub mod builder {
    pub use crate::bench::Bench;
    pub use crate::engine::EngineAuto;
    pub use crate::engine::EngineFixedSamples;
    pub use crate::engine::EngineFullFixed;
    pub use crate::report::Report;
    pub use crate::warmup::Warmup;
}
