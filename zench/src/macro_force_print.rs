#[doc(hidden)]
#[macro_export]
macro_rules! fprintln {
    ($($arg:tt)*) => {{
        use std::io::Write;
        let mut stdout = std::io::stdout().lock();
        writeln!(stdout, $($arg)*).unwrap();
        stdout.flush().unwrap();
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! fprint {
    ($($arg:tt)*) => {{
        use std::io::Write;
        let mut stdout = std::io::stdout().lock();
        write!(stdout, $($arg)*).unwrap();
        stdout.flush().unwrap();
    }};
}
