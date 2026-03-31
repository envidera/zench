// ================================================================
//  CPU STABILITY TRAIT FOR REPORT
//  ! Do not use in production, this is an experiment
// ================================================================

use std::fmt::Display;
use zench::builder::Report;
use zench::dev::algorithm;

#[allow(unused)]
pub trait IStability {
    fn system_stability(&self) -> Stability;
}

impl IStability for Report {
    fn system_stability(&self) -> Stability {
        let cv_pct = median_cv_pct(self);
        Stability::from(cv_pct)
    }
}

pub enum Stability {
    StableExtreme(f64),
    Stable(f64),
    StableModerate(f64),
    Unstable(f64),
    UnstableExtreme(f64),
}

#[allow(unused)]
impl Stability {
    pub(crate) fn is_unstable(&self) -> bool {
        Self::value(self) >= 5.0
    }

    fn from(value: f64) -> Self {
        match value {
            0.0..=1.0 => Self::StableExtreme(value),
            1.0..=3.0 => Self::Stable(value),
            3.0..=5.0 => Self::StableModerate(value),
            5.0..=10.0 => Self::Unstable(value),
            _ => Self::UnstableExtreme(value),
        }
    }

    pub(crate) fn value(&self) -> f64 {
        match self {
            Self::StableExtreme(v)
            | Self::Stable(v)
            | Self::StableModerate(v)
            | Self::Unstable(v)
            | Self::UnstableExtreme(v) => *v,
        }
    }
}

impl Display for Stability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StableExtreme(_) => write!(f, "✔ Extremely stable"),
            Self::Stable(_) => write!(f, "✔ Very stable"),
            Self::StableModerate(_) => write!(f, "! Moderately stable"),
            Self::Unstable(_) => write!(f, "✖ Unstable"),
            Self::UnstableExtreme(_) => write!(f, "✖ Highly unstable"),
        }
    }
}

#[allow(unused)]
fn median_cv_pct(r: &Report) -> f64 {
    let cvs: Vec<f64> = r
        .iter()
        .map(|b| b.cv())
        .collect();

    algorithm::mean(&cvs) * 100.0
}
