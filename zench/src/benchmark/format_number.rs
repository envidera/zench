#[derive(Clone, Copy)]
pub(crate) enum FormatNumber {
    Comma, // 1,000,000
    Rust,  // 1_000_000
    Dot,   // 1.000.000
    Space, // 1 000 000
}

#[allow(unused)]
impl FormatNumber {
    pub(crate) fn coma(n: usize) -> String {
        format_number(n, Self::Comma)
    }

    pub(crate) fn rust(n: usize) -> String {
        format_number(n, Self::Rust)
    }

    pub(crate) fn space(n: usize) -> String {
        format_number(n, Self::Space)
    }

    pub(crate) fn dot(n: usize) -> String {
        format_number(n, Self::Dot)
    }
}

pub(crate) use format_number_case::v1 as format_number;

#[allow(unused)]
mod format_number_case {

    use super::*;

    pub(crate) fn v1(n: usize, format: FormatNumber) -> String {
        let separator = match format {
            FormatNumber::Comma => ',',
            FormatNumber::Rust => '_',
            FormatNumber::Space => ' ',
            FormatNumber::Dot => '.',
        };

        let s = n.to_string();
        let len = s.len();
        let mut result = String::with_capacity(len + len / 3);

        for (i, c) in s
            .chars()
            .enumerate()
        {
            if i != 0 && (len - i).is_multiple_of(3) {
                result.push(separator);
            }
            result.push(c);
        }

        result
    }
}

// ================================================================
// Unit test
// ================================================================

#[allow(unused)]
const CASES: &[(usize, FormatNumber, &str)] = &[
    (999, FormatNumber::Comma, "999"),
    (999, FormatNumber::Rust, "999"),
    (999, FormatNumber::Space, "999"),
    (999, FormatNumber::Dot, "999"),
    // ----------------------------------------------------------------
    (1_000, FormatNumber::Comma, "1,000"),
    (1_000, FormatNumber::Rust, "1_000"),
    (1_000, FormatNumber::Space, "1 000"),
    (1_000, FormatNumber::Dot, "1.000"),
    // ----------------------------------------------------------------
    (10_000, FormatNumber::Comma, "10,000"),
    (10_000, FormatNumber::Rust, "10_000"),
    (10_000, FormatNumber::Space, "10 000"),
    (10_000, FormatNumber::Dot, "10.000"),
    // ----------------------------------------------------------------
    (1_000_000, FormatNumber::Comma, "1,000,000"),
    (1_000_000, FormatNumber::Rust, "1_000_000"),
    (1_000_000, FormatNumber::Space, "1 000 000"),
    (1_000_000, FormatNumber::Dot, "1.000.000"),
];

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cases() {
        #[rustfmt::skip]

        #[rustfmt::skip]
        let versions = [
            format_number_case::v1,
        ];

        for v in versions {
            for (n, fmt, expected) in CASES {
                assert_eq!(v(*n, *fmt), **expected);
            }
        }
    }
}

#[cfg(test)]
mod test_performance {

    use super::*;
    use crate::bx;

    #[ignore = "display purpose"]
    #[test]
    fn test_2() {
        let mut b = Bench::new();

        for (n, fmt, _) in CASES {
            b.bench(format!("v1 {n}"), || {
                bx(format_number_case::v1(*n, *fmt));
            });
        }
    }
}

/*

───────────┬──────────┬───────┬────────────┬──────────┬──────────────
   name    │  median  │  cv   │  std.dev   │ outliers │ samples/iters
───────────┼──────────┼───────┼────────────┼──────────┼──────────────
v1 999     │ 23.826ns │ 1.40% │  ± 0.335ns │    0.00% │ 100 / 524,288
v1 999     │ 23.932ns │ 0.85% │  ± 0.202ns │    0.00% │ 100 / 524,288
v1 999     │ 23.822ns │ 0.61% │  ± 0.145ns │    3.00% │ 100 / 524,288
v1 999     │ 23.796ns │ 0.58% │  ± 0.137ns │    3.00% │ 100 / 524,288
v1 1000    │ 25.187ns │ 0.22% │  ± 0.056ns │    1.00% │ 100 / 524,288
v1 1000    │ 25.198ns │ 0.28% │  ± 0.071ns │    3.00% │ 100 / 524,288
v1 1000    │ 25.184ns │ 0.25% │  ± 0.063ns │    2.00% │ 100 / 524,288
v1 1000    │ 25.270ns │ 0.41% │  ± 0.103ns │    3.00% │ 100 / 524,288
v1 10000   │ 26.304ns │ 1.07% │  ± 0.282ns │    5.00% │ 100 / 524,288
v1 10000   │ 26.206ns │ 0.27% │  ± 0.070ns │    2.00% │ 100 / 524,288
v1 10000   │ 28.067ns │ 0.35% │  ± 0.097ns │    3.00% │ 100 / 524,288
v1 10000   │ 26.221ns │ 0.31% │  ± 0.081ns │    1.00% │ 100 / 524,288
v1 1000000 │ 29.623ns │ 0.40% │  ± 0.120ns │    0.00% │ 100 / 524,288
v1 1000000 │ 29.608ns │ 0.42% │  ± 0.124ns │    0.00% │ 100 / 524,288
v1 1000000 │ 29.709ns │ 0.41% │  ± 0.122ns │    1.00% │ 100 / 524,288
v1 1000000 │ 29.755ns │ 0.37% │  ± 0.109ns │    0.00% │ 100 / 524,288
───────────┴──────────┴───────┴────────────┴──────────┴──────────────
total time: 22.693040312 sec
rust: 1.94.0 (release) | zench: 0.1.4

*/
