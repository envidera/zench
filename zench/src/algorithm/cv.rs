// coefficient of variation
pub fn cv(std_dev: f64, mean: f64) -> f64 {
    if mean == 0.0 {
        0.0
    } else {
        std_dev / mean
    }
}

pub fn cv_pct(std_dev: f64, mean: f64) -> f64 {
    self::cv(std_dev, mean) * 100.0
}

// ================================================================
// Unit test
// ================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cv_basic() {
        let mean = 50.0;
        let std_dev = 5.0;

        let result = cv(std_dev, mean);
        assert_eq!(result, 0.1); // 5 / 50 = 0.1
    }

    #[test]
    fn test_cv_zero_mean() {
        let mean = 0.0;
        let std_dev = 5.0;

        let result = cv(std_dev, mean);
        assert_eq!(result, 0.0); // mean = 0 return 0%
    }

    #[test]
    fn test_cv_pct_basic() {
        let mean = 50.0;
        let std_dev = 5.0;

        let result = cv_pct(std_dev, mean);
        assert_eq!(result, 10.0); // 0.1 * 100 = 10%
    }

    #[test]
    fn test_cv_pct_zero_mean() {
        let mean = 0.0;
        let std_dev = 5.0;

        let result = cv_pct(std_dev, mean);
        assert_eq!(result, 0.0); // mean = 0 return 0%
    }
}
