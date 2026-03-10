//mod benchmark;

mod colors;
mod display;
mod report;

pub use report::Report;

pub mod color {
    pub use super::colors::*;
}
