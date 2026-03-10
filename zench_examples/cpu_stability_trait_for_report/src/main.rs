mod extend;

fn main() {}
/*
================================================================
 STABILITY TEST ASSERTION EXPERIMENT
 ! Do not use in production, this is an experiment
================================================================
- Stable system   -> the test should pass
- Unstable system -> the test should fail

How to test:

Run stress-ng in one terminal.Try different combinations according
to the number of your CPU threads.

examples:

    stress-ng --cpu 6 --cpu-load 100 -t 3600
    stress-ng --cpu 11 --cpu-load 100 -t 3600

    stress-ng --cpu 12 --cpu-load 10 -t 3600
    stress-ng --cpu 12 --cpu-load 50 -t 3600

Then run the benchmark in other terminal:

    ZENCH=warn cargo test --release --package cpu_stability_trait_for_report -- --no-capture
*/
// ================================================================

#[cfg(test)]
mod tests {

    use super::extend::IStability;
    use zench::dev::mock;
    use zench::Bench;

    #[test]
    fn test_system_stability() {
        let mut b = Bench::new();

        let mut list: Vec<String> = vec![String::from("")];

        for _ in 1..=10 {
            b.bench("cpu_work", || {
                mock::simulate_cpu_work();
            });

            b.report(|r| {
                let stability = r.system_stability();
                let stb = format!("Stability: {} - {}", stability.value(), stability);
                list.push(stb);
            });
        }

        list.sort();
        list.iter()
            .for_each(|f| println!("{f}"));

        println!();
    }
}

/*

stress-ng --cpu 12 --cpu-load 10 -t 3600
    Stability: 0.9969644317831193 - ✔ Extremely stable
    Stability: 1.0110391488406856 - ✔ Very stable
    Stability: 1.0289091862515498 - ✔ Very stable
    Stability: 1.037634368182392 - ✔ Very stable
    Stability: 1.119829475311255 - ✔ Very stable
    Stability: 1.1425922647149613 - ✔ Very stable
    Stability: 1.1731892225658065 - ✔ Very stable
    Stability: 1.2410228551512037 - ✔ Very stable
    Stability: 1.2907719411923404 - ✔ Very stable
    Stability: 1.3933635620309919 - ✔ Very stable

stress-ng --cpu 12 --cpu-load 20 -t 3600
    Stability: 1.2047414399319012 - ✔ Very stable
    Stability: 1.2150701080619908 - ✔ Very stable
    Stability: 1.3031615862850146 - ✔ Very stable
    Stability: 1.409940267320102 - ✔ Very stable
    Stability: 1.4748814246493296 - ✔ Very stable
    Stability: 1.47697604416553 - ✔ Very stable
    Stability: 1.5075715315261857 - ✔ Very stable
    Stability: 1.6192627379238815 - ✔ Very stable
    Stability: 1.6575999570888535 - ✔ Very stable
    Stability: 1.9268027614815133 - ✔ Very stable

stress-ng --cpu 12 --cpu-load 25 -t 3600
    Stability: 1.1230637673231412 - ✔ Very stable
    Stability: 1.2768668510445673 - ✔ Very stable
    Stability: 1.3333372815402207 - ✔ Very stable
    Stability: 1.3842128048465738 - ✔ Very stable
    Stability: 1.4779240695277762 - ✔ Very stable
    Stability: 1.5196790323971114 - ✔ Very stable
    Stability: 1.5631900088566626 - ✔ Very stable
    Stability: 1.5680425880098305 - ✔ Very stable
    Stability: 1.5689937240125673 - ✔ Very stable
    Stability: 1.577324442505276 - ✔ Very stable

stress-ng --cpu 12 --cpu-load 30 -t 3600
    Stability: 1.389642119744919 - ✔ Very stable
    Stability: 1.8235188375084457 - ✔ Very stable
    Stability: 2.009415125203529 - ✔ Very stable
    Stability: 2.1535949462882398 - ✔ Very stable
    Stability: 2.158287434407311 - ✔ Very stable
    Stability: 2.6485989388199997 - ✔ Very stable
    Stability: 2.6530915877371273 - ✔ Very stable
    Stability: 2.8230209520720697 - ✔ Very stable
    Stability: 2.887029154528728 - ✔ Very stable
    Stability: 3.1135806639411596 - ! Moderately stable

stress-ng --cpu 12 --cpu-load 40 -t 3600
    Stability: 2.122394738189294 - ✔ Very stable
    Stability: 2.1449107562512415 - ✔ Very stable
    Stability: 2.3860250970656818 - ✔ Very stable
    Stability: 21.80389243883343 - ✖ Highly unstable
    Stability: 23.20373225214535 - ✖ Highly unstable
    Stability: 23.754919959254785 - ✖ Highly unstable
    Stability: 3.4308823829254105 - ! Moderately stable
    Stability: 3.545000670579026 - ! Moderately stable
    Stability: 3.824885620495337 - ! Moderately stable
    Stability: 3.852718952326993 - ! Moderately stable

stress-ng --cpu 12 --cpu-load 50 -t 3600
    Stability: 2.3160831606656878 - ✔ Very stable
    Stability: 22.844984488317255 - ✖ Highly unstable
    Stability: 25.22824993022068 - ✖ Highly unstable
    Stability: 3.4685972385933272 - ! Moderately stable
    Stability: 3.7256019874284445 - ! Moderately stable
    Stability: 3.7586351821294715 - ! Moderately stable
    Stability: 3.8008263596171057 - ! Moderately stable
    Stability: 4.196692775965577 - ! Moderately stable
    Stability: 6.051988073501475 - ✖ Unstable
    Stability: 8.70327333719518 - ✖ Unstable

stress-ng --cpu 12 --cpu-load 60 -t 3600
    Stability: 25.006640599274554 - ✖ Highly unstable
    Stability: 25.067521563740918 - ✖ Highly unstable
    Stability: 25.240772449480787 - ✖ Highly unstable
    Stability: 25.331526623249285 - ✖ Highly unstable
    Stability: 25.345006136017396 - ✖ Highly unstable
    Stability: 25.401121790749077 - ✖ Highly unstable
    Stability: 25.451037170630915 - ✖ Highly unstable
    Stability: 25.49963835610972 - ✖ Highly unstable
    Stability: 25.846389887765326 - ✖ Highly unstable
    Stability: 25.906436938164006 - ✖ Highly unstable

stress-ng --cpu 12 --cpu-load 70 -t 3600
    Stability: 21.02850441354486 - ✖ Highly unstable
    Stability: 21.836670721429762 - ✖ Highly unstable
    Stability: 22.369317780052505 - ✖ Highly unstable
    Stability: 22.45157066913283 - ✖ Highly unstable
    Stability: 22.80961768705255 - ✖ Highly unstable
    Stability: 22.936567896653408 - ✖ Highly unstable
    Stability: 23.22930210831605 - ✖ Highly unstable
    Stability: 23.278558694974894 - ✖ Highly unstable
    Stability: 23.530468478138648 - ✖ Highly unstable
    Stability: 23.699997017820706 - ✖ Highly unstable

stress-ng --cpu 12 --cpu-load 80 -t 3600
    Stability: 1.7575102894044379 - ✔ Very stable
    Stability: 1.876338735113294 - ✔ Very stable
    Stability: 1.894246346885754 - ✔ Very stable
    Stability: 1.9923428569405026 - ✔ Very stable
    Stability: 19.063516245230115 - ✖ Highly unstable
    Stability: 2.059363579475345 - ✔ Very stable
    Stability: 5.593974497038095 - ✖ Unstable
    Stability: 5.948079926105923 - ✖ Unstable
    Stability: 9.010723786824514 - ✖ Unstable
    Stability: 9.13775161611791 - ✖ Unstable

// ----------------------------------------------------------------

stress-ng --cpu 6 --cpu-load 100 -t 3600
    Stability: 1.424348823406901 - ✔ Very stable
    Stability: 1.4546326197817763 - ✔ Very stable
    Stability: 1.4576913365956812 - ✔ Very stable
    Stability: 1.5054353255203525 - ✔ Very stable
    Stability: 1.50837103357257 - ✔ Very stable
    Stability: 1.510383234479218 - ✔ Very stable
    Stability: 1.5228803764814987 - ✔ Very stable
    Stability: 1.5304963011462949 - ✔ Very stable
    Stability: 1.5638973954021522 - ✔ Very stable
    Stability: 1.639899858435792 - ✔ Very stable

*/
