const KEY: &str = "ZENCH";
const WARN_VALUE: &str = "warn";
const PANIC_VALUE: &str = "panic";
const MSG_EXPECTED_VALUES: &str = "(expected warn or panic)";

fn in_debug_mode() -> bool {
    cfg!(debug_assertions)
}

// ================================================================

#[doc(hidden)]
pub use ignore::Ignore;

mod ignore {
    use super::*;
    use crate::report::color;
    use std::collections::HashSet;
    use std::sync::Mutex;
    use std::sync::OnceLock;

    pub struct Ignore {}

    static IGNORED: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();
    impl Ignore {
        // ----------------------------------------------------------------
        //  Is release?
        // ├─ NO  > ignore
        // └─ YES >
        //       ZENCH = warn or panic?
        //           ├─ YES > RUN
        //           └─ NO  > ignore
        // ----------------------------------------------------------------
        //             cargo test --release     no command + release = ignore / quiet
        //             cargo test               no command + debug   = ignore / quiet
        //
        // ZENCH=      cargo test               no command + release = ignore / print reason (not release)
        // ZENCH=      cargo test --release     no command + release = ignore / print reason (command empty)
        //
        // ZENCH=asd   cargo test               no command + release = ignore / print reason (not release)
        // ZENCH=asd   cargo test --release     no command + release = ignore / print reason (unknown command)
        //
        // ZENCH=warn  cargo test                     warn + debug   = ignore / print reason (not release)
        // ZENCH=warn  cargo test --release           warn + release = RUN
        // ZENCH=panic cargo test --release           warn + release = RUN
        // ----------------------------------------------------------------
        pub(crate) fn should_ignore() -> bool {
            let cmd = Command::from_env();

            in_debug_mode()
                || matches!(cmd, Command::NotSet)
                || matches!(cmd, Command::Empty)
                || matches!(cmd, Command::Unknown(_))
        }

        #[rustfmt::skip]
        pub(crate) fn reason() -> Option<String> {
            if in_debug_mode() {
                return Some(color::yellow("ignored, debug profile (use --release)"));
            }

            match Command::from_env() {
                Command::Unknown(v) => Some(color::yellow(format!(
                    "ignored, invalid {KEY} value: '{v}' {MSG_EXPECTED_VALUES}"
                ))),

                Command::Empty => Some(color::yellow(format!(
                    "ignored, {KEY} is set but empty {MSG_EXPECTED_VALUES}"
                ))),

                _ => None,
            }
        }

        fn ignored_set() -> &'static Mutex<HashSet<String>> {
            IGNORED.get_or_init(|| Mutex::new(HashSet::new()))
        }

        pub fn already_seen(location: &str) -> bool {
            let set = Self::ignored_set()
                .lock()
                .unwrap();
            set.contains(location)
        }

        pub fn mark_seen(location: String) {
            let mut set = Self::ignored_set()
                .lock()
                .unwrap();
            set.insert(location);
        }
    }
}

// ================================================================

#[doc(hidden)]
pub use command::Command;

mod command {

    use super::*;
    use std::sync::OnceLock;

    static COMMAND: OnceLock<Command> = OnceLock::new();

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub enum Command {
        #[default]
        NotSet, //ZENCH not defined in environment
        Empty,           //ZENCH=
        Unknown(String), //ZENCH=asd
        Warn,            //ZENCH=warn
        Panic,           //ZENCH=panic
    }

    impl Command {
        pub fn from_env() -> &'static Command {
            COMMAND.get_or_init(Self::from_env_once)
        }

        pub(crate) fn not_set() -> bool {
            matches!(Self::from_env(), Command::NotSet)
        }

        fn from_env_once() -> Self {
            match std::env::var(KEY).as_deref() {
                Ok(v) if v.eq_ignore_ascii_case(WARN_VALUE) => Self::Warn,
                Ok(v) if v.eq_ignore_ascii_case(PANIC_VALUE) => Self::Panic,
                Ok("") => Self::Empty,
                Ok(v) => Self::Unknown(v.to_string()),
                Err(_) => Self::NotSet,
            }
        }
    }
}
