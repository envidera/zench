// Use log2 to filter benchmark distances
// zench/src/report/filter.rs Filter::FilterPct

pub fn log_distance(a: f64, b: f64) -> f64 {
    if a <= 0.0 || b <= 0.0 {
        return f64::INFINITY;
    }
    (b / a)
        .ln()
        .abs() // log natural (base e)
}

pub fn log2_distance(a: f64, b: f64) -> f64 {
    if a <= 0.0 || b <= 0.0 {
        return f64::INFINITY;
    }
    (b / a)
        .log2()
        .abs() // log base 2
}

// ================================================================
// Unit test
// ================================================================
#[cfg(test)]
mod tests {

    use super::*;

    // epsilon
    const EPS: f64 = 1e-10;

    #[test]
    fn equal_values_have_zero_distance() {
        assert!((log_distance(10.0, 10.0)).abs() < EPS);
        assert!((log2_distance(10.0, 10.0)).abs() < EPS);
    }

    #[test]
    fn symmetric_distance() {
        let a = 10.0;
        let b = 15.0;

        let d1 = log_distance(a, b);
        let d2 = log_distance(b, a);

        let d3 = log2_distance(a, b);
        let d4 = log2_distance(b, a);

        assert!((d1 - d2).abs() < EPS);
        assert!((d3 - d4).abs() < EPS);
    }

    #[test]
    fn doubling_behavior() {
        // ln
        assert!((log_distance(1.0, 2.0) - std::f64::consts::LN_2).abs() < EPS);
        assert!((log_distance(1.0, 4.0) - (2.0 * std::f64::consts::LN_2)).abs() < EPS);

        // log2
        assert!((log2_distance(1.0, 2.0) - 1.0).abs() < EPS);
        assert!((log2_distance(1.0, 4.0) - 2.0).abs() < EPS);
        assert!((log2_distance(1.0, 8.0) - 3.0).abs() < EPS);
    }

    #[test]
    fn five_percent_distance() {
        let a = 350.0;
        let b = 350.0 * 1.05;

        // ln(1.05)
        let expected_ln = (1.05f64).ln();
        let expected_log2 = (1.05f64).log2();

        assert!((log_distance(a, b) - expected_ln).abs() < EPS);
        assert!((log2_distance(a, b) - expected_log2).abs() < EPS);
    }
}
