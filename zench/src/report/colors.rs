use std::{fmt::Display, io::IsTerminal};

fn apply_style(v: impl Display, ansi_code: &str) -> String {
    if std::io::stderr().is_terminal() {
        format!("\x1b[{}m{}\x1b[0m", ansi_code, v)
    } else {
        v.to_string()
    }
}

pub fn red(v: impl Display) -> String {
    apply_style(v, "91")
}

#[allow(unused)]
pub fn red_bold(v: impl Display) -> String {
    apply_style(v, "1;91")
}

#[allow(unused)]
pub fn yellow(v: impl Display) -> String {
    apply_style(v, "33")
}

#[allow(unused)]
pub fn yellow_bold(v: impl Display) -> String {
    apply_style(v, "1;33")
}

#[allow(unused)]
pub fn bold(v: impl Display) -> String {
    apply_style(v, "1")
}
