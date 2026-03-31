use super::Report;
use std::fmt;
use std::fmt::Write;

#[cfg(not(feature = "display_vertical"))]
use crate::report::table::TableBuilder;

impl Report {
    #[cfg(not(feature = "display_vertical"))]
    fn write_tabled<W: Write>(&self, w: &mut W) -> fmt::Result {
        // ----------------------------------------------------------------
        // Header

        let mut line: Vec<&str> = Vec::new();

        if !self
            .title
            .is_empty()
        {
            line.push(&self.title);
        }

        if let Some(v) = &self.filter_log {
            line.extend(
                v.iter()
                    .map(|s| s.as_str()),
            );
        }

        if line.is_empty() {
            writeln!(w)?;
        } else {
            write!(w, "\n{}\n", line.join(" > "))?;
        }

        // ----------------------------------------------------------------
        // Benches
        let mut table = TableBuilder::new();
        table
            .column_left("name")
            .column_right("median")
            .column_right("cv")
            .column_right("std.dev")
            .column_right("outliers")
            .column_right("samples/iters");

        for b in &self.benchset {
            let sampler_iters = format!("{} / {}", b.samples_count_fmt(), b.iters_count_fmt());
            table.row(vec![
                b.name(),
                &b.median_fmt(),
                &b.cv_pct_fmt(),
                &b.std_dev_fmt(),
                &b.outliers_pct_fmt(),
                &sampler_iters,
            ]);
        }

        let final_table = table.build();
        write!(w, "{final_table}")?;
        Ok(())
    }

    #[cfg(feature = "display_vertical")]
    fn write_vertical<W: Write>(&self, w: &mut W) -> fmt::Result {
        // ----------------------------------------------------------------
        // Header
        let filters = self
            .filter_log
            .as_ref()
            .map(|v| format!("\nFilters   {}", v.join(" > ")))
            .unwrap_or_default();

        let final_report = format!("\nReport    {}{}", self.title, filters);

        writeln!(w, "{}", final_report)?;
        writeln!(w)?;

        // ----------------------------------------------------------------
        // Benches
        for b in self
            .benchset
            .iter()
        {
            writeln!(
                w,
                "Benchmark  {name}\n\
                Time       Median: {median}\n\
                Stability  Std.Dev: {std_dev} | CV: {cv_pct}\n\
                Samples    Count: {samples} | Iters/sample: {iter_per_sample} | Outliers: {outliers}\n\
                Location   {location}\n",
                name = b.name(),
                median = b.median_fmt(),
                std_dev = b.std_dev_fmt(),
                cv_pct = b.cv_pct_fmt(),
                samples = b.samples_count_fmt(),
                iter_per_sample = b.iters_count_fmt(),
                outliers = b.outliers_pct_fmt(),
                location = b.location,
            )?;
        }
        Ok(())
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ----------------------------------------------------------------
        #[cfg(not(feature = "display_vertical"))]
        self.write_tabled(f)?;
        // ----------------------------------------------------------------
        #[cfg(feature = "display_vertical")]
        self.write_vertical(f)?;
        // ----------------------------------------------------------------
        Ok(())
    }
}

// ================================================================
// Unit display test
// ================================================================
#[cfg(test)]
mod test_display {

    use crate::bench;

    #[test]
    fn bench_display() {
        bench!(
            "version_orig" => {},
            "v2" => {},
            "v3_inline" => {},
            "sleep" => {},

        );

        bench!(
            "ver" => {},
            "v2" => {},
            "v3_" => {},
            "sleep" => {},
        )
        .report(|r| {
            r.title("Title")
                .print();
        });

        bench!(
            "ver" => {},
            "v2" => {},
            "v3_" => {},
            "sleep" => {},
        )
        .report(|r| {
            r.title("Compact mode")
                .sort_by_median_reverse()
                .sort_by_median()
                .print();
        });
    }
}

/* default tabled

bench zench/src/report/display.rs:131:9::version_orig
bench zench/src/report/display.rs:131:9::v2
bench zench/src/report/display.rs:131:9::v3_inline
bench zench/src/report/display.rs:131:9::sleep

─────────────┬─────────┬───────┬────────────┬──────────┬──────────────
    name     │ median  │  cv   │  std.dev   │ outliers │ samples/iters
─────────────┼─────────┼───────┼────────────┼──────────┼──────────────
version_orig │ 0.215ns │ 0.01% │  ± 0.000ns │   13.00% │ 100 / 524,288
v2           │ 0.215ns │ 0.01% │  ± 0.000ns │   13.00% │ 100 / 524,288
v3_inline    │ 0.215ns │ 0.01% │  ± 0.000ns │   14.00% │ 100 / 524,288
sleep        │ 0.215ns │ 0.01% │  ± 0.000ns │   13.00% │ 100 / 524,288
─────────────┴─────────┴───────┴────────────┴──────────┴──────────────
total time: 0.161099611 sec
rust: 1.94.0 (release) | zench: 0.1.4


bench zench/src/report/display.rs:139:9::ver
bench zench/src/report/display.rs:139:9::v2
bench zench/src/report/display.rs:139:9::v3_
bench zench/src/report/display.rs:139:9::sleep

Title
──────┬─────────┬───────┬────────────┬──────────┬──────────────
name  │ median  │  cv   │  std.dev   │ outliers │ samples/iters
──────┼─────────┼───────┼────────────┼──────────┼──────────────
ver   │ 0.215ns │ 0.01% │  ± 0.000ns │   14.00% │ 100 / 524,288
v2    │ 0.215ns │ 0.01% │  ± 0.000ns │   12.00% │ 100 / 524,288
v3_   │ 0.215ns │ 0.01% │  ± 0.000ns │   15.09% │ 285 / 524,288
sleep │ 0.215ns │ 0.01% │  ± 0.000ns │   13.00% │ 100 / 524,288
──────┴─────────┴───────┴────────────┴──────────┴──────────────
total time: 0.07926654 sec
rust: 1.94.0 (release) | zench: 0.1.4


bench zench/src/report/display.rs:150:9::ver
bench zench/src/report/display.rs:150:9::v2
bench zench/src/report/display.rs:150:9::v3_
bench zench/src/report/display.rs:150:9::sleep

Compact mode > Sort MedianReverse > Sort Median
──────┬─────────┬───────┬────────────┬──────────┬──────────────
name  │ median  │  cv   │  std.dev   │ outliers │ samples/iters
──────┼─────────┼───────┼────────────┼──────────┼──────────────
v2    │ 0.215ns │ 0.01% │  ± 0.000ns │   12.00% │ 100 / 524,288
ver   │ 0.215ns │ 0.01% │  ± 0.000ns │   11.00% │ 100 / 524,288
v3_   │ 0.215ns │ 0.01% │  ± 0.000ns │   13.00% │ 100 / 524,288
sleep │ 0.215ns │ 0.01% │  ± 0.000ns │   13.00% │ 100 / 524,288
──────┴─────────┴───────┴────────────┴──────────┴──────────────
total time: 0.05813167 sec

*/

/* Full

bench zench/src/report/display.rs:131:9::version_orig
bench zench/src/report/display.rs:131:9::v2
bench zench/src/report/display.rs:131:9::v3_inline
bench zench/src/report/display.rs:131:9::sleep

Report

Benchmark  version_orig
Time       Median: 0.215ns
Stability  Std.Dev: ± 0.000ns | CV: 0.01%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 13.00%
Location   zench/src/report/display.rs:131:9

Benchmark  v2
Time       Median: 0.215ns
Stability  Std.Dev: ± 0.000ns | CV: 0.06%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 12.00%
Location   zench/src/report/display.rs:131:9

Benchmark  v3_inline
Time       Median: 0.215ns
Stability  Std.Dev: ± 0.000ns | CV: 0.01%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 11.00%
Location   zench/src/report/display.rs:131:9

Benchmark  sleep
Time       Median: 0.215ns
Stability  Std.Dev: ± 0.000ns | CV: 0.01%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 13.00%
Location   zench/src/report/display.rs:131:9


total time: 0.168777833 sec
rust: 1.94.0 | profile release
zench: 0.1.4
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-21 02:05:45 UTC

---------------------------------

bench zench/src/report/display.rs:139:9::ver
bench zench/src/report/display.rs:139:9::v2
bench zench/src/report/display.rs:139:9::v3_
bench zench/src/report/display.rs:139:9::sleep

Report    Title

Benchmark  ver
Time       Median: 0.215ns
Stability  Std.Dev: ± 0.000ns | CV: 0.01%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 11.00%
Location   zench/src/report/display.rs:139:9

Benchmark  v2
Time       Median: 0.215ns
Stability  Std.Dev: ± 0.000ns | CV: 0.04%
Samples    Count: 1,000 | Iters/sample: 524,288 | Outliers: 11.90%
Location   zench/src/report/display.rs:139:9

Benchmark  v3_
Time       Median: 0.215ns
Stability  Std.Dev: ± 0.000ns | CV: 0.01%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 11.00%
Location   zench/src/report/display.rs:139:9

Benchmark  sleep
Time       Median: 0.215ns
Stability  Std.Dev: ± 0.000ns | CV: 0.01%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 12.00%
Location   zench/src/report/display.rs:139:9


total time: 0.160893186 sec
rust: 1.94.0 | profile release
zench: 0.1.4
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-21 02:05:45 UTC

---------------------------------

bench zench/src/report/display.rs:150:9::ver
bench zench/src/report/display.rs:150:9::v2
bench zench/src/report/display.rs:150:9::v3_
bench zench/src/report/display.rs:150:9::sleep

Report    Compact mode
Filters   Sort MedianReverse > Sort Median

Benchmark  ver
Time       Median: 0.215ns
Stability  Std.Dev: ± 0.000ns | CV: 0.01%
Samples    Count: 220 | Iters/sample: 524,288 | Outliers: 12.27%
Location   zench/src/report/display.rs:150:9

Benchmark  v2
Time       Median: 0.215ns
Stability  Std.Dev: ± 0.000ns | CV: 0.05%
Samples    Count: 490 | Iters/sample: 524,288 | Outliers: 12.24%
Location   zench/src/report/display.rs:150:9

Benchmark  v3_
Time       Median: 0.215ns
Stability  Std.Dev: ± 0.000ns | CV: 0.01%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 12.00%
Location   zench/src/report/display.rs:150:9

Benchmark  sleep
Time       Median: 0.215ns
Stability  Std.Dev: ± 0.000ns | CV: 0.01%
Samples    Count: 100 | Iters/sample: 524,288 | Outliers: 12.00%
Location   zench/src/report/display.rs:150:9


total time: 0.121515312 sec
rust: 1.94.0 | profile release
zench: 0.1.4
system: linux x86_64
cpu: AMD Ryzen 5 5600GT with Radeon Graphics (x12 threads)
2026-03-21 02:05:46 UTC

---------------------------------
*/
