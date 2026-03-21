use fmt::Result;
use std::fmt::Write;
use std::fmt::{self, Display};

// ----------------------------------------------------------------
// ascii table for the table
// https://www.ascii-code.com/CP437
// ----------------------------------------------------------------

const T_DIV: char = '│';
const T_LINE: &str = "─";
const T_LINE_DIV_DOWN: char = '┬';
const T_LINE_DIV_UP_DOWN: char = '┼';
const T_LINE_DIV_UP: char = '┴';

#[derive(Debug, Default)]
pub struct Table<'a> {
    columns: Vec<Column<'a>>,
    rows: Vec<Vec<String>>,
    auto_width: bool,
}

#[derive(Debug, Clone)]
pub struct Column<'a> {
    pub name: &'a str,
    pub width: usize,
    pub alignment: Alignment,
    pub padding: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Alignment {
    #[default]
    Left,
    Center,
    Right,
}

impl<'a> Column<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            width: name.len(),
            name,
            alignment: Alignment::Left,
            padding: 0,
        }
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn with_padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }

    fn total_width(&self) -> usize {
        self.width + self.padding
    }

    fn format_cell(&self, content: &str) -> String {
        let truncated = if content.len() > self.width {
            &content[..self
                .width
                .saturating_sub(3)]
        } else {
            content
        };

        match self.alignment {
            Alignment::Left => format!("{:<width$}", truncated, width = self.width),
            Alignment::Center => format!("{:^width$}", truncated, width = self.width),
            Alignment::Right => format!("{:>width$}", truncated, width = self.width),
        }
    }
}

impl<'a> Table<'a> {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            auto_width: true,
        }
    }

    /// Add a column
    pub fn add_column(&mut self, column: Column<'a>) -> &mut Self {
        self.columns
            .push(column);
        self
    }

    /// Add multiple columns
    pub fn add_columns(&mut self, columns: Vec<Column<'a>>) -> &mut Self {
        self.columns
            .extend(columns);
        self
    }

    /// Defines whether to calculate width automatically.
    pub fn auto_width(mut self, auto: bool) -> Self {
        self.auto_width = auto;
        self
    }

    /// Add one line
    pub fn add_row(&mut self, row: Vec<String>) -> &mut Self {
        self.rows
            .push(row);
        self
    }

    /// Calculates maximum widths based on the data.
    pub fn calculate_widths(&mut self) {
        if !self.auto_width {
            return;
        }

        for (i, col) in self
            .columns
            .iter_mut()
            .enumerate()
        {
            let max_data_len = self
                .rows
                .iter()
                .filter_map(|row| row.get(i))
                .map(|cell| cell.len())
                .max()
                .unwrap_or(0);

            col.width = col
                .width
                .max(max_data_len);
        }
    }

    pub fn write_live_div<W: Write>(&self, writer: &mut W, separator: char) -> Result {
        for (i, col) in self
            .columns
            .iter()
            .enumerate()
        {
            if i > 0 {
                write!(writer, "{T_LINE}{separator}{T_LINE}")?;
            }

            write!(writer, "{}", T_LINE.repeat(col.total_width()))?;
            //write!(writer, "{:─<width$}", "", width = col.total_width())?;
        }

        Ok(())
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result {
        // Calculates widths if necessary
        let mut table = self.clone();
        if self.auto_width {
            table.calculate_widths();
        }

        // ----------------------------------------------------------------
        // Write the first dividing line.
        self.write_live_div(writer, T_LINE_DIV_DOWN)?;
        writeln!(writer)?;

        // ----------------------------------------------------------------
        // Write the header
        for (i, col) in table
            .columns
            .iter()
            .enumerate()
        {
            if i > 0 {
                write!(writer, " {T_DIV} ")?;
            }
            write!(writer, "{:^width$}", col.name, width = col.total_width())?;
        }
        writeln!(writer)?;

        // ----------------------------------------------------------------
        // Write the second dividing line.
        self.write_live_div(writer, T_LINE_DIV_UP_DOWN)?;
        writeln!(writer)?;
        // ----------------------------------------------------------------
        // Write data table
        for row in &table.rows {
            for (i, (col, cell)) in table
                .columns
                .iter()
                .zip(row.iter())
                .enumerate()
            {
                if i > 0 {
                    write!(writer, " {T_DIV} ")?;
                }
                let formatted = col.format_cell(cell);
                write!(writer, "{:width$}", formatted, width = col.total_width())?;
            }
            writeln!(writer)?;
        }

        // ----------------------------------------------------------------
        // Write the last dividing line.
        self.write_live_div(writer, T_LINE_DIV_UP)?;
        // ----------------------------------------------------------------

        Ok(())
    }
}

impl<'a> fmt::Display for Table<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write(f)?;
        Ok(())
    }
}

impl<'a> Clone for Table<'a> {
    fn clone(&self) -> Self {
        Self {
            columns: self
                .columns
                .clone(),
            rows: self
                .rows
                .clone(),
            auto_width: self.auto_width,
        }
    }
}

/// Builder pattern para facilitar criação
pub struct TableBuilder<'a> {
    table: Table<'a>,
}

impl<'a> TableBuilder<'a> {
    pub fn new() -> Self {
        Self {
            table: Table::new(),
        }
    }

    pub fn column(&mut self, name: &'a str) -> &mut Self {
        self.table
            .add_column(Column::new(name));
        self
    }

    pub fn column_with_width(&mut self, name: &'a str, width: usize) -> &mut Self {
        self.table
            .add_column(Column::new(name).with_width(width));
        self
    }

    pub fn column_left(&mut self, name: &'a str) -> &mut Self {
        self.table
            .add_column(Column::new(name).with_alignment(Alignment::Left));
        self
    }

    pub fn column_right(&mut self, name: &'a str) -> &mut Self {
        self.table
            .add_column(Column::new(name).with_alignment(Alignment::Right));
        self
    }

    pub fn column_center(&mut self, name: &'a str) -> &mut Self {
        self.table
            .add_column(Column::new(name).with_alignment(Alignment::Center));
        self
    }

    pub fn row(&mut self, row: Vec<impl Display>) -> &mut Self {
        let row_str = row
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        self.table
            .add_row(row_str);
        self
    }

    pub fn build(mut self) -> Table<'a> {
        self.table
            .calculate_widths();
        self.table
    }
}

// ================================================================
// Unit test
// ================================================================
const WANT: &str = r#"
─────────────────┬───────────┬─────────┬───────────┬──────────┬──────────────
      name       │  median   │   cv    │  std.dev  │ outliers │ samples/iters
─────────────────┼───────────┼─────────┼───────────┼──────────┼──────────────
version_orig     │   0.215ns │  00.01% │     0.0ns │   13.00% │ 100 / 524,288
version_opt_plus │ 0.18984ns │  00.01% │ 000.000ns │    5.00% │  1000 / 1,288
v1               │   0.189ns │ 000.01% │   0.000ns │  500.00% │     100 / 524
─────────────────┴───────────┴─────────┴───────────┴──────────┴──────────────
"#;
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_() {
        let mut table = TableBuilder::new();
        table
            .column_left("name")
            .column_right("median")
            .column_right("cv")
            .column_right("std.dev")
            .column_right("outliers")
            .column_right("samples/iters")
            .row(vec![
                "version_orig",
                "0.215ns",
                "00.01%",
                "0.0ns",
                "13.00%",
                "100 / 524,288",
            ])
            .row(vec![
                "version_opt_plus",
                "0.18984ns",
                "00.01%",
                "000.000ns",
                "5.00%",
                "1000 / 1,288",
            ])
            .row(vec![
                "v1",
                "0.189ns",
                "000.01%",
                "0.000ns",
                "500.00%",
                "100 / 524",
            ]);

        let t = table.build();
        println!("{t}");
        let have = format!("\n{t}\n");
        assert_eq!(have, WANT);
    }
}
