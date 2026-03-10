use std::fmt;

pub(crate) enum Filter {
    N(usize),
    Proximity(f64),
    Pct(f64),
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Sort {
    Median,
    MedianReverse,
    Samples,
    SamplesReverse,
    Outliers,
    OutliersReverse,
    StdDev,
    StdDevReverse,
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Median => "Sort Median",
            Self::MedianReverse => "Sort MedianReverse",
            Self::Samples => "Sort Samples",
            Self::SamplesReverse => "Sort SamplesReverse",
            Self::Outliers => "Sort Outliers",
            Self::OutliersReverse => "Sort OutliersReverse",
            Self::StdDev => "Sort Std.Dev",
            Self::StdDevReverse => "Sort Std.DevReverse",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::N(value) => format!("Filter N({value})"),
            Self::Proximity(pct) => format!("Filter Proximity({pct}%)"),
            Self::Pct(pct) => format!("Filter Percentage({pct}%)"),
        };
        write!(f, "{}", s)
    }
}
