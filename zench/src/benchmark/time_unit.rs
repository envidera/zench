#[derive(PartialEq, Debug)]
pub(crate) enum TimeUnit {
    Nanoseconds,  // ns 1.0
    Microseconds, // µs 1_000
    Milliseconds, // ms 1_000_000
}

impl TimeUnit {
    pub fn from_nanos(nanos: f64) -> Self {
        if nanos < 1_000.0 {
            TimeUnit::Nanoseconds
        } else if nanos < 1_000_000.0 {
            TimeUnit::Microseconds
        } else {
            TimeUnit::Milliseconds
        }
    }

    pub fn convert(&self, nanos: f64) -> f64 {
        nanos / self.divisor()
    }

    pub fn divisor(&self) -> f64 {
        match self {
            TimeUnit::Nanoseconds => 1.0,
            TimeUnit::Microseconds => 1_000.0,
            TimeUnit::Milliseconds => 1_000_000.0,
        }
    }

    pub fn suffix(&self) -> &'static str {
        match self {
            TimeUnit::Nanoseconds => "ns",
            TimeUnit::Microseconds => "µs",
            TimeUnit::Milliseconds => "ms",
        }
    }
}

// ================================================================
// Unit test
// ================================================================

#[cfg(test)]
mod tests {
    use super::TimeUnit;

    #[test]
    fn test_from_nanos() {
        assert_eq!(TimeUnit::from_nanos(0.0), TimeUnit::Nanoseconds);
        assert_eq!(TimeUnit::from_nanos(999.9), TimeUnit::Nanoseconds);
        assert_eq!(TimeUnit::from_nanos(1_000.0), TimeUnit::Microseconds);
        assert_eq!(TimeUnit::from_nanos(999_999.0), TimeUnit::Microseconds);
        assert_eq!(TimeUnit::from_nanos(1_000_000.0), TimeUnit::Milliseconds);
        assert_eq!(TimeUnit::from_nanos(5_000_000.0), TimeUnit::Milliseconds);
    }

    #[test]
    fn test_divisor() {
        assert_eq!(TimeUnit::Nanoseconds.divisor(), 1.0);
        assert_eq!(TimeUnit::Microseconds.divisor(), 1_000.0);
        assert_eq!(TimeUnit::Milliseconds.divisor(), 1_000_000.0);
    }

    #[test]
    fn test_suffix() {
        assert_eq!(TimeUnit::Nanoseconds.suffix(), "ns");
        assert_eq!(TimeUnit::Microseconds.suffix(), "µs");
        assert_eq!(TimeUnit::Milliseconds.suffix(), "ms");
    }

    #[test]
    fn test_convert() {
        let nanos = 500.0;
        let micros = 2_500.0;
        let millis = 5_000_000.0;

        assert_eq!(TimeUnit::Nanoseconds.convert(nanos), 500.0);
        assert_eq!(TimeUnit::Microseconds.convert(micros), 2.5);
        assert_eq!(TimeUnit::Milliseconds.convert(millis), 5.0);
    }
}
