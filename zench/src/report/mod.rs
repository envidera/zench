//mod benchmark;

mod colors;
mod display;
mod report;
#[cfg(not(feature = "display_vertical"))]
mod table;

pub use report::Report;

pub mod color {
    pub use super::colors::*;
}
