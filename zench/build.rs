use std::env;
use std::process::Command;

include!("src/utc.rs");

fn main() {
    // Get rustc version
    // use
    // let rust_version = env!("RUSTC_VERSION");
    let rustc_version = Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
        .and_then(|s| {
            s.split_whitespace()
                .nth(1)
                .map(|version| version.to_string())
        })
        .unwrap_or_else(|| "unknown".into());

    println!("cargo:rustc-env=RUSTC_VERSION={}", rustc_version);

    //------------------------------------------------------------
    // Get current date and time
    // use
    // let build_date = env!("BUILD_DATE");

    let build_date = Utc::now().to_string();
    println!("cargo:rustc-env=BUILD_DATE={}", build_date);

    //------------------------------------------------------------
    // Get current build profile
    // use
    // let build_profile = env!("BUILD_PROFILE");

    let profile = std::env::var("PROFILE").unwrap_or("unknown".into());
    println!("cargo:rustc-env=BUILD_PROFILE={}", profile);

    //------------------------------------------------------------
    // Get current system
    // use
    // let sys_os = env!("SYS_OS");
    // let sys_arch = env!("SYS_ARCH");
    // let sys_cpu_number = env!("SYS_CPU_PARALLELISM");
    // let sys_cpu_model = env!("SYS_CPU_MODEL");

    println!("cargo:rustc-env=SYS_OS={}", env::consts::OS);
    println!("cargo:rustc-env=SYS_ARCH={}", env::consts::ARCH);

    /* INFO: Not the real parallelism: 'cargo test' spawns subcommands outside my control,
    so I can't isolate it with 'nice' and 'taskset'.
        nice -n -20 taskset -c 2 cargo test --release --example example_name -- --nocapture
    */

    println!(
        "cargo:rustc-env=SYS_CPU_THREADS={}",
        std::thread::available_parallelism()
            .unwrap()
            .get()
    );

    let cpu_model = if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg("grep 'model name' /proc/cpuinfo | head -1 | cut -d: -f2-")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| {
                s.trim()
                    .to_string()
            })
            .unwrap_or("unknown".into())
    } else if cfg!(target_os = "macos") {
        // TODO: test in github action on Macos
        Command::new("sysctl")
            .arg("-n")
            .arg("machdep.cpu.brand_string")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| {
                s.trim()
                    .to_string()
            })
            .unwrap_or("unknown".into())
    } else if cfg!(target_os = "windows") {
        // TODO: test in github action on Windows
        Command::new("wmic")
            .args(["cpu", "get", "name"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| {
                s.lines()
                    .nth(1)
                    .unwrap_or("unknown")
                    .trim()
                    .to_string()
            })
            .unwrap_or("unknown".into())
    } else {
        "unknown".to_string()
    };

    println!("cargo:rustc-env=SYS_CPU_MODEL={}", cpu_model);
}
