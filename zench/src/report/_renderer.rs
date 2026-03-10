use super::Report;
use crate::report::color;
use std::{
    fs::File,
    io::{BufWriter, Write},
};

// pub struct RenderWith<'a, R> {
//     pub(crate) report: &'a Report,
//     pub(crate) renderer: R,
// }

// impl<'a, R: Formatter> RenderWith<'a, R> {
//     pub fn to_stdout(self) {
//         let stdout = std::io::stdout().lock();
//         let mut writer = BufWriter::new(stdout);

//         if let Err(e) = self
//             .renderer
//             .format(self.report, &mut writer)
//         {
//             eprintln!("Zench Error: {}", e);
//         }

//         let _ = writer.flush();
//     }
// }

// impl<'a, R: Formatter> RenderWith<'a, R> {
//     pub fn to_file(self, path: &str) {
//         match File::create(path) {
//             Ok(file) => {
//                 let mut writer = BufWriter::new(file);
//                 if let Err(e) = self
//                     .renderer
//                     .format(self.report, &mut writer)
//                 {
//                     eprintln!("Zench Error: {}", e);
//                 }
//                 let _ = writer.flush();
//             }
//             Err(e) => {
//                 eprintln!("Zench Error: Could not create file: {}", e);
//             }
//         }
//     }
// }

pub trait Renderer {
    fn write<W>(&self, report: &Report, writer: &mut W) -> std::io::Result<()>
    where
        W: Write;
}

pub(crate) struct DefaultRender;

impl Renderer for DefaultRender {
    fn write<W>(&self, r: &Report, w: &mut W) -> std::io::Result<()>
    where
        W: Write,
    {
        // ----------------------------------------------------------------
        // Header
        let title = &r.title;
        let modifiers = match &r.modifiers {
            Some(v) => &format!("\nFilters    {}", v.join(" > ")),
            None => "",
        };

        write!(
            w,
            r#"
Report     {title}{modifiers}
"#
        )?;

        // ----------------------------------------------------------------
        // benches

        for b in r
            .benchset
            .iter()
        {
            write!(
                w,
                r#"
Benchmark  {name}
Time       Median: {median}
Stability  Std.Dev: {std_dev} | CV: {cv_pct}
Samples    Count: {samples} | Iters/sample: {iter_per_sample} | Outliers: {outliers}
Location   {location}
"#,
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
        // ----------------------------------------------------------------

        Ok(())
    }
}
