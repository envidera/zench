pub fn mean(v: &[f64]) -> f64 {
    v.iter()
        .sum::<f64>()
        / v.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(self::mean(&data), 3.0);
    }
}
