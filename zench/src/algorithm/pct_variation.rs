// https://en.m.wikipedia.org/wiki/Relative_change

// symmetrical version
pub fn pct_variation(a: f64, b: f64) -> f64 {
    let scale = a
        .abs()
        .max(b.abs());

    if scale == 0.0 {
        return 0.0;
    }

    ((a - b).abs() / scale) * 100.0
}

// ================================================================
// Unit test
// ================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_does_not_matter() {
        let a = 100.0;
        let b = 110.0;

        let ab = pct_variation(a, b);
        let ba = pct_variation(b, a);

        assert_eq!(ab, ba);
    }

    #[test]
    fn equal_values_is_zero() {
        assert_eq!(pct_variation(50.0, 50.0), 0.0);
    }

    #[test]
    fn simple_percentage_difference() {
        let a = 100.0;
        let b = 110.0;

        let variation = pct_variation(a, b);

        assert!((variation - 9.0909).abs() < 0.0001);
    }

    #[test]
    fn works_with_negative_values() {
        let a = -100.0;
        let b = -110.0;

        let ab = pct_variation(a, b);
        let ba = pct_variation(b, a);

        assert_eq!(ab, ba);
    }

    #[test]
    fn zero_vs_non_zero() {
        let a = 0.0;
        let b = 100.0;

        let variation = pct_variation(a, b);

        assert_eq!(variation, 100.0);
    }

    #[test]
    fn both_zero_is_zero() {
        assert_eq!(pct_variation(0.0, 0.0), 0.0);
    }
}
