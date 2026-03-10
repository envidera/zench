use super::color;
use super::Report;
use std::fmt;

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // ----------------------------------------------------------------
        // Header

        #[rustfmt::skip]
        writeln!(f, "\n\
            Report     {}", self.title)?;

        if let Some(v) = &self.filter_log {
            writeln!(f, "Filters    {}", v.join(" > "))?;
        };

        writeln!(f)?;

        // ----------------------------------------------------------------
        // Benches

        for b in self
            .benchset
            .iter()
        {
            writeln!(
                f,
                "Benchmark  {name}\n\
                Time       Median: {median}\n\
                Stability  Std.Dev: {std_dev} | CV: {cv_pct}\n\
                Samples    Count: {samples} | Iters/sample: {iter_per_sample} | Outliers: {outliers}\n\
                Location   {location}\n",
                name = b.name,
                median = color::bold(b.median_fmt()),
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
