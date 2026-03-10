// Run
// ZENCH=1 cargo test --release -p zench --test modes -- --no-capture

// use zench::bench;

// // TODO: move this functions to mock?
// fn remove_zench_env() -> Option<String> {
//     let old = std::env::var("ZENCH").ok();
//     unsafe { std::env::remove_var("ZENCH") };
//     old
// }

// fn restore_zench_env(old: Option<String>) {
//     if let Some(v) = old {
//         unsafe { std::env::set_var("ZENCH", v) };
//     }
// }

// fn set_zench_env(value: &str) {
//     unsafe { std::env::set_var("ZENCH", value) };
// }

// cargo test --release -p zench --test modes
// #[test]
// fn test_does_not_run() {
//     //let old = remove_zench_env();
//     bench!("empty" => {panic!()});
//     //restore_zench_env(old);
// }

// #[test]
// fn test_must_run() {
//     let old = remove_zench_env();

//     set_zench_env("");

//     bench!("empty" => {});

//     restore_zench_env(old);
// }
