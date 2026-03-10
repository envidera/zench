use zench::bench;
use zench::bx;

fn main() {}

#[ignore = "in development"]
#[test]
fn string_concatenation_performance() {
    let texts = gen_texts();

    for txt in &texts {
        let [size, a, b, c, d, e] = txt;

        bench!(
            format!("{size} - collect_mapped__from_array") => {
                cases::collect_mapped__from_array(bx(a),bx(b),bx(c),bx(d),bx(e));
            },
            format!("{size} - collect_copied__from_array") => {
                cases::collect_copied__from_array(bx(a),bx(b),bx(c),bx(d),bx(e));
            },
            format!("{size} - collect_cloned__from_array") => {
                cases::collect_cloned__from_array(bx(a),bx(b),bx(c),bx(d),bx(e));
            },
            //------------------------------------------------
            format!("{size} - collect_mapped__from_slice") => {
                cases::collect_mapped__from_slice(bx(a),bx(b),bx(c),bx(d),bx(e));
            },
            format!("{size} - collect_copied__from_slice") => {
                cases::collect_copied__from_slice(bx(a),bx(b),bx(c),bx(d),bx(e));
            },
            format!("{size} - collect_cloned__from_slice") => {
                cases::collect_cloned__from_slice(bx(a),bx(b),bx(c),bx(d),bx(e));
            },
            //------------------------------------------------
            format!("{size} - collect_mapped__from_vec") => {
                cases::collect_mapped__from_vec(bx(a),bx(b),bx(c),bx(d),bx(e));
            },
            format!("{size} - collect_copied__from_vec") => {
                cases::collect_copied__from_vec(bx(a),bx(b),bx(c),bx(d),bx(e));
            },
            format!("{size} - collect_cloned__from_vec") => {
                cases::collect_cloned__from_vec(bx(a),bx(b),bx(c),bx(d),bx(e));
            },
            //------------------------------------------------
            format!("{size} - extend_mapped__with_capacity_plus_plus") => {
                cases::extend_mapped__with_capacity_plus_plus(bx(a),bx(b),bx(c),bx(d),bx(e));
            },
            format!("{size} - extend_mapped__with_capacity_sum") => {
                cases::extend_mapped__with_capacity_sum(bx(a),bx(b),bx(c),bx(d),bx(e));
            },
            format!("{size} - extend_copied__with_capacity_plus_plus") => {
                cases::extend_copied__with_capacity_plus_plus(bx(a),bx(b),bx(c),bx(d), bx(e));
            },
            format!("{size} - extend_copied__with_capacity_sum") => {
                cases::extend_copied__with_capacity_sum(bx(a),bx(b),bx(c),bx(d),bx(e));
            },
            format!("{size} - extend_cloned__with_capacity_plus_plus") => {
                cases::extend_cloned__with_capacity_plus_plus(bx(a),bx(b),bx(c),bx(d),bx(e));
            },
            format!("{size} - extend_cloned__with_capacity_sum") => {
                cases::extend_cloned__with_capacity_sum(bx(a),bx(b),bx(c),bx(d),bx(e));
            },
            //------------------------------------------------
            format!("{size} - format") => {
                cases::format(bx(a),bx(b),bx(c),bx(d),bx(e));
            },
            //------------------------------------------------
            format!("{size} - push_str") => {
                cases::push_str(bx(a),bx(b),bx(c),bx(d),bx(e));
            },
            format!("{size} - push_str__with_capacity_plus_plus") => {
                cases::push_str__with_capacity_plus_plus(bx(a),bx(b),bx(c),bx(d),bx(e));
            },
            format!("{size} - push_str__with_capacity_sum") => {
                cases::push_str__with_capacity_sum(bx(a),bx(b),bx(c),bx(d),bx(e));
            },
            //------------------------------------------------
            /* format!("{size} - slice_concat") => {
                cases::slice_concat(bx(a),bx(b),bx(c), bx(d), bx(e));
            },
            format!("{size} - slice_join") => {
                cases::slice_join(bx(a),bx(b),bx(c), bx(d), bx(e));
            },
            format!("{size} - array_concat") => {
                cases::array_concat(bx(a),bx(b),bx(c), bx(d), bx(e));
            },
            format!("{size} - array_join") => {
                cases::array_join(bx(a),bx(b),bx(c), bx(d), bx(e));
            },
            format!("{size} - vec_concat") => {
                cases::vec_concat(bx(a),bx(b),bx(c), bx(d), bx(e));
            },
            format!("{size} - vec_join") => {
                cases::vec_join(bx(a),bx(b),bx(c), bx(d), bx(e));
            },
            //------------------------------------------------
            format!("{size} - from_string_all_plus_plus") => {
                cases::from_string_all_plus_plus(bx(a),bx(b),bx(c), bx(d), bx(e));
            },
            format!("{size} - to_string_plus_plus") => {
                cases::to_string_plus_plus(bx(a),bx(b),bx(c), bx(d), bx(e));
            },
            format!("{size} - to_owned_plus_plus") => {
                cases::to_owned_plus_plus(bx(a),bx(b),bx(c), bx(d), bx(e));
            },
            //------------------------------------------------
            format!("{size} - write") => {
                cases::write(bx(a),bx(b),bx(c), bx(d), bx(e));
            },
            format!("{size} - write_with_capacity") => {
                cases::write_with_capacity(bx(a),bx(b),bx(c), bx(d), bx(e));
            }, */
        )
        .report(|r| {
            r.title(size)
                .sort_by_median()
                .print();
        });
    }
}

fn text(size: usize) -> String {
    (0..size)
        .map(|i| (b'a' + ((i * 7) % 26) as u8) as char)
        .collect()
}

fn gen_texts() -> Vec<[String; 6]> {
    let texts: Vec<[String; 6]> = vec![
        [
            "Small".to_string(),
            text(1),
            text(1),
            text(1),
            text(1),
            text(1),
        ],
        [
            "Mid".to_string(),
            text(100),
            text(100),
            text(100),
            text(100),
            text(100),
        ],
        [
            "Large".to_string(),
            text(500),
            text(500),
            text(500),
            text(500),
            text(500),
        ],
        [
            "Extra".to_string(),
            text(15_000),
            text(15_000),
            text(15_000),
            text(15_000),
            text(15_000),
        ],
        [
            "Huge".to_string(),
            text(50_000),
            text(50_000),
            text(50_000),
            text(50_000),
            text(50_000),
        ],
    ];

    texts
}

#[allow(unused)]
mod cases {
    use zench::bx;

    #[allow(warnings)]
    pub(crate) fn collect_mapped__from_array(a: &str, b: &str, c: &str, d: &str, e: &str) {
        let list = [a, b, c, d, e];
        let s: String = list
            .iter()
            .map(|x| *x) // <-- mapped
            .collect();
        bx(s);
    }

    #[allow(warnings)]
    pub(crate) fn collect_copied__from_array(a: &str, b: &str, c: &str, d: &str, e: &str) {
        let list = [a, b, c, d, e];
        let s: String = list
            .iter()
            .copied()
            .collect();
        bx(s);
    }

    #[allow(warnings)]
    pub(crate) fn collect_cloned__from_array(a: &str, b: &str, c: &str, d: &str, e: &str) {
        let list = [a, b, c, d, e];
        let s: String = list
            .iter()
            .cloned() // <-- cloned
            .collect();
        bx(s);
    }

    //------------------------------------------------------------------

    #[allow(warnings)]
    pub(crate) fn collect_mapped__from_slice(a: &str, b: &str, c: &str, d: &str, e: &str) {
        let list = &[a, b, c, d, e];
        let s: String = list
            .iter()
            .map(|x| *x)
            .collect();
        bx(s);
    }

    #[allow(warnings)]
    pub(crate) fn collect_copied__from_slice(a: &str, b: &str, c: &str, d: &str, e: &str) {
        let list = &[a, b, c, d, e];
        let s: String = list
            .iter()
            .copied()
            .collect();
        bx(s);
    }

    #[allow(warnings)]
    pub(crate) fn collect_cloned__from_slice(a: &str, b: &str, c: &str, d: &str, e: &str) {
        let list = &[a, b, c, d, e];
        let s: String = list
            .iter()
            .cloned()
            .collect();
        bx(s);
    }

    //------------------------------------------------------------------

    #[allow(warnings)]
    pub(crate) fn collect_mapped__from_vec(a: &str, b: &str, c: &str, d: &str, e: &str) {
        let list = vec![a, b, c, d, e];
        let s: String = list
            .iter()
            .map(|x| *x)
            .collect();
        bx(s);
    }

    #[allow(warnings)]
    pub(crate) fn collect_copied__from_vec(a: &str, b: &str, c: &str, d: &str, e: &str) {
        let list = vec![a, b, c, d, e];
        let s: String = list
            .iter()
            .copied()
            .collect();
        bx(s);
    }

    #[allow(warnings)]
    pub(crate) fn collect_cloned__from_vec(a: &str, b: &str, c: &str, d: &str, e: &str) {
        let list = vec![a, b, c, d, e];
        let s: String = list
            .iter()
            .cloned()
            .collect();
        bx(s);
    }

    //------------------------------------------------------------------

    #[allow(warnings)]
    pub(crate) fn extend_mapped__with_capacity_plus_plus(
        a: &str,
        b: &str,
        c: &str,
        d: &str,
        e: &str,
    ) {
        let mut s = String::with_capacity(a.len() + b.len() + c.len() + d.len() + e.len());
        s.extend(
            [a, b, c, d, e]
                .iter()
                .map(|x| *x),
        );
        bx(s);
    }

    #[allow(warnings)]
    pub(crate) fn extend_mapped__with_capacity_sum(a: &str, b: &str, c: &str, d: &str, e: &str) {
        let parts = [a, b, c, d, e];

        let mut s = String::with_capacity(
            parts
                .iter()
                .map(|p| p.len())
                .sum(),
        );
        s.extend(
            parts
                .iter()
                .map(|x| *x),
        );

        bx(s);
    }

    #[allow(warnings)]
    pub(crate) fn extend_copied__with_capacity_plus_plus(
        a: &str,
        b: &str,
        c: &str,
        d: &str,
        e: &str,
    ) {
        let mut s = String::with_capacity(a.len() + b.len() + c.len() + d.len() + e.len());
        s.extend(
            [a, b, c, d, e]
                .iter()
                .copied(),
        );
        bx(s);
    }

    #[allow(warnings)]
    pub(crate) fn extend_copied__with_capacity_sum(a: &str, b: &str, c: &str, d: &str, e: &str) {
        let parts = [a, b, c, d, e];

        let mut s = String::with_capacity(
            parts
                .iter()
                .map(|p| p.len())
                .sum(),
        );

        s.extend(
            parts
                .iter()
                .copied(),
        );

        bx(s);
    }

    #[allow(warnings)]
    pub(crate) fn extend_cloned__with_capacity_plus_plus(
        a: &str,
        b: &str,
        c: &str,
        d: &str,
        e: &str,
    ) {
        let mut s = String::with_capacity(a.len() + b.len() + c.len() + d.len() + e.len());
        s.extend(
            [a, b, c, d, e]
                .iter()
                .cloned(),
        );
        bx(s);
    }

    #[allow(warnings)]
    pub(crate) fn extend_cloned__with_capacity_sum(a: &str, b: &str, c: &str, d: &str, e: &str) {
        let parts = [a, b, c, d, e];

        let mut s = String::with_capacity(
            parts
                .iter()
                .map(|p| p.len())
                .sum(),
        );

        s.extend(
            [a, b, c, d, e]
                .iter()
                .cloned(),
        );
        bx(s);
    }

    //------------------------------------------------------------------

    #[allow(warnings)]
    pub(crate) fn format(a: &str, b: &str, c: &str, d: &str, e: &str) {
        let s = format!("{}{}{}{}{}", a, b, c, d, e);
        bx(s);
    }
    //------------------------------------------------------------------
    #[allow(warnings)]
    pub(crate) fn push_str(a: &str, b: &str, c: &str, d: &str, e: &str) {
        let mut s = String::new();
        s.push_str(a);
        s.push_str(b);
        s.push_str(c);
        s.push_str(d);
        s.push_str(e);
        bx(s);
    }

    #[allow(warnings)]
    pub(crate) fn push_str__with_capacity_plus_plus(a: &str, b: &str, c: &str, d: &str, e: &str) {
        let mut s = String::with_capacity(a.len() + b.len() + c.len() + d.len() + e.len());
        s.push_str(a);
        s.push_str(b);
        s.push_str(c);
        s.push_str(d);
        s.push_str(e);
        bx(s);
    }

    #[allow(warnings)]
    pub(crate) fn push_str__with_capacity_sum(a: &str, b: &str, c: &str, d: &str, e: &str) {
        let mut s = String::with_capacity(
            [a, b, c, d, e]
                .iter()
                .map(|p| p.len())
                .sum(),
        );
        s.push_str(a);
        s.push_str(b);
        s.push_str(c);
        s.push_str(d);
        s.push_str(e);
        bx(s);
    }

    //------------------------------------------------------------------

    pub(crate) fn slice_concat(a: &str, b: &str, c: &str) {
        let s = &[bx(a), bx(b), bx(c)].concat();
        bx(s);
    }

    pub(crate) fn slice_join(a: &str, b: &str, c: &str) {
        let s = &[bx(a), bx(b), bx(c)].join("");
        bx(s);
    }

    pub(crate) fn array_concat(a: &str, b: &str, c: &str) {
        let s = [bx(a), bx(b), bx(c)].concat();
        bx(s);
    }

    pub(crate) fn array_join(a: &str, b: &str, c: &str) {
        let s = [bx(a), bx(b), bx(c)].join("");
        bx(s);
    }

    #[allow(clippy::useless_vec)]
    pub(crate) fn vec_concat(a: &str, b: &str, c: &str) {
        let s = vec![bx(a), bx(b), bx(c)].concat();
        bx(s);
    }

    #[allow(clippy::useless_vec)]
    pub(crate) fn vec_join(a: &str, b: &str, c: &str) {
        let s = vec![bx(a), bx(b), bx(c)].join("");
        bx(s);
    }

    //------------------------------------------------------------------

    pub(crate) fn from_string_all_plus_plus(a: &str, b: &str, c: &str) {
        let s = bx(String::from(a)) + bx(b) + bx(c);
        bx(s);
    }

    pub(crate) fn to_string_plus_plus(a: &str, b: &str, c: &str) {
        let s = bx(a).to_string() + bx(b) + bx(c);
        bx(s);
    }

    pub(crate) fn to_owned_plus_plus(a: &str, b: &str, c: &str) {
        let s = bx(a).to_owned() + bx(b) + bx(c);
        bx(s);
    }

    //------------------------------------------------------------------
    use std::fmt::Write;
    pub(crate) fn write(a: &str, b: &str, c: &str) {
        let mut s = String::new();
        write!(s, "{}{}{}", bx(a), bx(b), bx(c)).unwrap();
        bx(s);
    }

    pub(crate) fn write_with_capacity(a: &str, b: &str, c: &str) {
        let mut s = String::with_capacity(a.len() + b.len() + c.len());
        write!(s, "{}{}{}", bx(a), bx(b), bx(c)).unwrap();
        bx(s);
    }
    //------------------------------------------------------------------
}
