use crate::bench::Bench;
use crate::global::Ignore;

impl<E> Drop for Bench<E> {
    fn drop(&mut self) {
        if Ignore::should_ignore() {
            return;
        }

        let report = &self.report;

        if !report.is_empty() {
            println!("{}", self.report);
        }

        // ================================================================
        #[cfg(not(feature = "display_vertical"))]
        {
            println!(
                "total time: {elapsed} sec",
                elapsed = report
                    .env_zench_total_time_execution()
                    .map(|t| t.to_string())
                    .unwrap_or("unknown".to_string())
            );

            println!(
                "rust: {} ({}) | zench: {}",
                report.env_rust_version(),
                report.env_rust_profile(),
                report.env_zench_version(),
            );

            println!("\n---------------------------------\n");
        }

        // ================================================================
        #[cfg(feature = "display_vertical")]
        {
            println!(
                "total time: {elapsed} sec",
                elapsed = report
                    .env_zench_total_time_execution()
                    .map(|t| t.to_string())
                    .unwrap_or("unknown".to_string())
            );

            println!(
                "rust: {} | profile {} ",
                report.env_rust_version(),
                report.env_rust_profile(),
            );

            println!("zench: {}", report.env_zench_version());
            println!("system: {} {}", report.env_sys_os(), report.env_sys_arch());

            println!(
                "cpu: {} (x{} threads)",
                report.env_sys_cpu_model(),
                report.env_sys_cpu_threads(),
            );
            println!("{}", report.env_current_date_time());
            println!("\n---------------------------------\n");
        }
    }
}
