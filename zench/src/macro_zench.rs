#[macro_export]
macro_rules! bench {
    // ----------------------------------------------------------------
    // Arm for blocks { ... }
    ( $( $name:expr => { $($block:tt)* } ),+ $(,)? ) => {{
        let mut b = $crate::Bench::new();
        $(
            b.bench($name, || { $($block)* });
        )+
        b
    }};

    // ----------------------------------------------------------------
    // Arm for simple expressions
    ( $( $name:expr => $expr:expr ),+ $(,)? ) => {{
        let mut b = $crate::Bench::new();
        $(
              b.bench($name, || { $expr; });
        )+
        b
    }};
}

#[macro_export]
macro_rules! issue {
    // ----------------------------------------------------------------
    // empty issue
    () => {
           $crate::issue!("triggered");
    };

    // ----------------------------------------------------------------
    // formatted issue
    ($($arg:tt)*) => {{
        let mode = $crate::__internal::Command::from_env();

        let prefix_color = match mode {
            $crate::__internal::Command::Warn => {
                $crate::__internal::color::yellow_bold("bench::issue::warn")
            }
            $crate::__internal::Command::Panic => {
                $crate::__internal::color::red_bold("bench::issue::panic")
            }
            _ => String::new(),
        };

        let formatted = format!($($arg)*);
        let msg = format!("{} {}\n\
            --> {}\n", prefix_color, formatted, $crate::location!());

        // ----------------------------------------------------------------

        match mode {
            $crate::__internal::Command::Panic => panic!("{msg}"),
            $crate::__internal::Command::Warn => $crate::__internal::fprintln!("{msg}"),
            _ => {}
        }
    }};
}

// ================================================================
// Unit test
// ================================================================
#[cfg(test)]
mod tests {

    #[ignore = "issue test"]
    #[test]
    fn test_() {
        issue!();
    }

    #[ignore = "issue test"]
    #[test]
    fn test_2() {
        issue!("use the number {}", 100);
    }
}

// #[macro_export]
// macro_rules! bench_assert {

//     ($cond:expr $(,)?) => {{
//         if !$cond {
//             let mode = $crate::__internal::Command::from_env();

//             let prefix_color = match mode {
//                 $crate::__internal::Command::Warn => {
//                     $crate::__internal::color::yellow_bold("bench::assert::warn")
//                 }
//                 $crate::__internal::Command::Panic => {
//                     $crate::__internal::color::red_bold("bench::assert::panic")
//                 }
//                 _ => String::new(),
//             };

//             //let formatted = format!("assertion failed: {}", stringify!($cond));
//             //let msg = format!("{} {} > {}\n", prefix_color, formatted, $crate::location!());
//             let msg = format!(
//                 "{prefix_color} assertion failed: {}\n\
//                 {location}\n\
//                 \n",
//                 stringify!($cond),
//                 location = $crate::location!(),
//             );

//             match mode {
//                 $crate::__internal::Command::Panic => panic!("{msg}"),
//                 $crate::__internal::Command::Warn => $crate::__internal::fprint!("{msg}"),
//                 _ => {}
//             }
//         }
//     }};

//     ($cond:expr, $($arg:tt)+) => {{
//         if !$cond {
//             let mode = $crate::__internal::Command::from_env();

//             let prefix_color = match mode {
//                 $crate::__internal::Command::Warn => {
//                     $crate::__internal::color::yellow_bold("bench::assert::warn")
//                 }
//                 $crate::__internal::Command::Panic => {
//                     $crate::__internal::color::red_bold("bench::assert::panic")
//                 }
//                 _ => String::new(),
//             };

//             let formatted = format!($($arg)+);
//             //let msg = format!("{} {} > {}\n", prefix_color, formatted, $crate::location!());
//             let msg = format!(
//                 "{prefix_color} assertion failed: {}\n\
//                 {location}\n\
//                 > {formatted}\n\
//                 \n",
//                 stringify!($cond),
//                 location = $crate::location!(),
//             );

//             match mode {
//                 $crate::__internal::Command::Panic => panic!("{msg}"),
//                 $crate::__internal::Command::Warn => $crate::__internal::fprint!("{msg}"),
//                 _ => {}
//             }
//         }
//     }};
// }

// #[macro_export]
// macro_rules! bench_assert_eq {

//     ($left:expr, $right:expr $(,)?) => {{
//         match (&$left, &$right) {
//             (left_val, right_val) => {
//                 if !(*left_val == *right_val) {

//                     let mode = $crate::__internal::Command::from_env();

//                     let prefix_color = match mode {
//                         $crate::__internal::Command::Warn => {
//                             $crate::__internal::color::yellow_bold("bench::assert_eq::warn")
//                         }
//                         $crate::__internal::Command::Panic => {
//                             $crate::__internal::color::red_bold("bench::assert_eq::panic")
//                         }
//                         _ => String::new(),
//                     };

//                     let msg = format!(
//                         "{prefix_color} assertion failed: (left == right)\n\
//                         {location}\n\
//                         \n \
//                          left: {:?}\n\
//                         right: {:?}\n\
//                         \n",
//                         left_val,
//                         right_val,
//                         location = $crate::location!(),
//                     );

//                     match mode {
//                         $crate::__internal::Command::Panic => panic!("{msg}"),
//                         $crate::__internal::Command::Warn => $crate::__internal::fprint!("{msg}"),
//                         _ => {}
//                     }
//                 }
//             }
//         }
//     }};

//     ($left:expr, $right:expr, $($arg:tt)+) => {{
//         match (&$left, &$right) {
//             (left_val, right_val) => {
//                 if !(*left_val == *right_val) {

//                     let mode = $crate::__internal::Command::from_env();

//                     let prefix_color = match mode {
//                         $crate::__internal::Command::Warn => {
//                             $crate::__internal::color::yellow_bold("bench::assert_eq::warn")
//                         }
//                         $crate::__internal::Command::Panic => {
//                             $crate::__internal::color::red_bold("bench::assert_eq::panic")
//                         }
//                         _ => String::new(),
//                     };

//                     let formatted = format!($($arg)+);

//                     let msg = format!(
//                         "{prefix_color} assertion failed: (left == right)\n\
//                         {location}\n\
//                         > {formatted}\n\
//                         \n \
//                          left: {:?}\n\
//                         right: {:?}\n\
//                         \n",
//                         left_val,
//                         right_val,
//                         location = $crate::location!(),
//                     );

//                     match mode {
//                         $crate::__internal::Command::Panic => panic!("{msg}"),
//                         $crate::__internal::Command::Warn => $crate::__internal::fprint!("{msg}"),
//                         _ => {}
//                     }
//                 }
//             }
//         }
//     }};
// }

// ================================================================
// Unit test
// ================================================================
// #[cfg(test)]
// mod tests {

//     // ----------------------------------------------------------------
//     //PASS

//     #[test]
//     fn test_assert_pass() {
//         bench_assert!(true);
//         bench_assert!(true, "msg {}", 1);
//     }

//     #[test]
//     fn test_assert_eq_pass() {
//         bench_assert_eq!(1, 1);
//         bench_assert_eq!(true, true);
//         bench_assert_eq!("p1", "p1");
//         bench_assert_eq!(true, true, "msg {}", 1);
//     }

//     // ----------------------------------------------------------------
//     //FAIL

//     #[ignore = "should panic"]
//     #[test]
//     fn test_assert_fail() {
//         bench_assert!(false);
//         bench_assert!(false, "number {} is not the correct", 1);
//     }

//     #[ignore = "should panic"]
//     #[test]
//     fn test_assert_eq_false() {
//         bench_assert_eq!(1, 2);
//         bench_assert_eq!(1, 2, "use the number {}", 1);
//     }
// }
