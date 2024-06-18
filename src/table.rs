//! # Table
//! Include a vector of tablelines, representing a table.
use crate::export::Export;
use crate::setting::OutputColor;
use crate::tablecell::Tablecell;
use crate::tableline::Tableline;

pub struct Table(Vec<Tableline>);

impl Table {
    pub fn new() -> Table {
        Table(Vec::new())
    }

    pub fn from_vec(lines: Vec<Tableline>) -> Table {
        Table(lines)
    }

    /// Parse a string to a table, assuming the string has '\n' as line seperator
    pub fn from_string(s: String, seperation: &str, end_line: &str) -> Table {
        let mut s = s;
        if !end_line.contains("\n") {
            // remove '\n' from input
            s = s.replace("\n", "");
        }
        let mut lines: Vec<Tableline> = s
            .split(end_line)
            .map(|line| Tableline::from_string(line.to_string(), seperation))
            .collect();
        lines.retain(|line| line.len() > 0);
        Table(lines)
    }

    /// Parse a string to a table, force the cell as string, assuming the string has '\n' as line seperator
    pub fn from_string_force(s: String, seperation: char, end_line: &str) -> Table {
        let mut s = s;
        if !end_line.contains("\n") {
            // remove '\n' from input
            s = s.replace("\n", "");
        }
        let lines: Vec<Tableline> = s
            .split(end_line)
            .map(|line| Tableline::from_string_force(line.to_string(), seperation))
            .collect();
        Table(lines)
    }

    /// Push one line to the end of table
    pub fn push_line(&mut self, line: Tableline) {
        self.0.push(line);
    }

    /// Pop one line from the end of table
    pub fn pop_line(&mut self) -> Option<Tableline> {
        self.0.pop()
    }

    /// Insert a line at the index, return Err if the index is out of range
    pub fn insert_line(&mut self, index: usize, line: Tableline) -> Result<(), String> {
        if index > self.0.len() {
            return Err("Index out of range".to_string());
        }
        self.0.insert(index, line);
        Ok(())
    }

    /// Remove a line at the index, return Err if the index is out of range
    pub fn remove_line(&mut self, index: usize) -> Result<Tableline, String> {
        if index >= self.0.len() {
            return Err("Index out of range".to_string());
        }
        Ok(self.0.remove(index))
    }

    /// Push multiple lines to the end of table
    pub fn push_lines(&mut self, lines: Vec<Tableline>) {
        self.0.extend(lines);
    }

    /// Pop multiple lines from the end of table
    pub fn pop_lines(&mut self, n: usize) -> Vec<Tableline> {
        let n = n.min(self.0.len());
        let mut lines = Vec::with_capacity(n);
        for _ in 0..n {
            lines.push(self.0.pop().unwrap());
        }
        lines
    }

    /// Remove range of lines from the table, return Err if the range is out of range
    pub fn remove_lines(&mut self, start: usize, end: usize) -> Result<(), String> {
        if start >= self.0.len() || end >= self.0.len() {
            return Err("Index out of range".to_string());
        }
        self.0.drain(start..=end);
        Ok(())
    }

    /// Insert multiple lines at the index, return Err if the index is out of range
    pub fn insert_lines(&mut self, index: usize, lines: Vec<Tableline>) -> Result<(), String> {
        if index > self.0.len() {
            return Err("Index out of range".to_string());
        }
        self.0.splice(index..index, lines);
        Ok(())
    }

    /// Get the number of lines of the table
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Get the line at the index
    pub fn get_line(&self, index: usize) -> Option<&Tableline> {
        self.0.get(index)
    }

    /// Get the mutable line at the index
    pub fn get_line_mut(&mut self, index: usize) -> Option<&mut Tableline> {
        self.0.get_mut(index)
    }

    /// Get the lines of the table at the index range
    pub fn get_lines(&self, start: usize, end: usize) -> Option<&[Tableline]> {
        if start >= self.0.len() || end >= self.0.len() {
            return None;
        }
        Some(&self.0[start..=end])
    }

    /// Get the mutable lines of the table at the index range
    pub fn get_lines_mut(&mut self, start: usize, end: usize) -> Option<&mut [Tableline]> {
        if start >= self.0.len() || end >= self.0.len() {
            return None;
        }
        Some(&mut self.0[start..=end])
    }

    /// Get the cell at the index
    pub fn get_cell(&self, (row, col): (usize, usize)) -> Option<&Tablecell> {
        self.0.get(row).and_then(|line| line.get_cell(col))
    }

    /// Get subtable from the table
    pub fn get_subtable(
        self,
        (start_row, start_col): (usize, usize),
        (end_row, end_col): (usize, usize),
    ) -> Option<Table> {
        if start_row >= self.0.len() || end_row >= self.0.len() || end_row < start_row {
            return None;
        }
        let mut subtable = Table::new();
        for i in start_row..=end_row {
            if start_col >= self.0[i].len() || end_col >= self.0[i].len() || end_col < start_col {
                subtable.push_line(Tableline::new());
                continue;
            }
            let line = self.0[i].get_cells(start_col, end_col).unwrap();
            subtable.push_line(Tableline::from_vec(line.to_vec()));
        }
        Some(subtable)
    }

    /// Get the length of longest row of the table
    pub fn get_longest_row(&self) -> usize {
        self.0.iter().map(|line| line.len()).max().unwrap_or(0)
    }

    /// Set the color of a line
    pub fn set_color_line(&mut self, index: usize, color: OutputColor) {
        if index >= self.0.len() {
            return;
        }
        for i in 0..self.0[index].len() {
            self.0[index].get_cell_mut(i).unwrap().set_color(color);
        }
    }

    /// Set the color of a column
    pub fn set_color_column(&mut self, index: usize, color: OutputColor) {
        for i in 0..self.0.len() {
            if let Some(cell) = self.0[i].get_cell_mut(index) {
                cell.set_color(color);
            }
        }
    }
}

/* --------------------------------- Export --------------------------------- */

impl Export for Table {
    fn to_console(&self) {
        println!("{}", self);
    }

    fn to_txt(&self, file: &str, seperation: char) -> Result<(), String> {
        let mut s = String::new();
        for line in self.0.iter() {
            s.push_str(line.to_string_format(seperation).as_str());
            s.push('\n');
        }
        std::fs::write(file, s).map_err(|err| err.to_string())
    }

    fn to_csv(&self, _file: &str) -> Result<(), String> {
        //TODO
        Ok(())
    }

    fn to_excel(&self, file: &str, sheet: &str) -> Result<(), String> {
        //TODO
        Ok(())
    }
}

/* --------------------------------- Display -------------------------------- */
/// Generate parallel line of a cell with given width, start with +, but not end with +
fn generate_parallel_line(width: usize) -> String {
    let mut parallel_line = String::from("\x1b[90m+");
    parallel_line.push_str("-".repeat(width + 2).as_str());
    parallel_line
}

/// Default display mode is left aligned
impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        // get the longest row first
        let width = self.get_longest_row();

        // get the width of the widest cell in each column in display mode
        let widths: Vec<usize> = (0..width)
            .map(|col| {
                self.0
                    .iter()
                    .map(|line| line.get_cell(col).map(|cell| cell.len()).unwrap_or(0))
                    .max()
                    .unwrap_or(0)
            })
            .collect();

        // draw proper parallel line with widths
        let mut parallel_line = String::from("");
        for width in &widths {
            parallel_line.push_str(generate_parallel_line(*width).as_str());
        }
        parallel_line.push_str("+\x1b[0m\n");
        s.push_str(&parallel_line);

        for line in self.0.iter() {
            s.push_str(&line.to_string_display(&widths).unwrap().as_str());
            s.push_str("\n");
            s.push_str(&parallel_line);
        }
        write!(f, "{}", s)
    }
}

impl std::fmt::Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        // get the longest row first
        let width = self.get_longest_row();

        // get the width of the widest cell in each column in debug mode
        let widths: Vec<usize> = (0..width)
            .map(|col| {
                self.0
                    .iter()
                    .map(|line| {
                        line.get_cell(col)
                            .map(|cell| format!("{:?}", cell).len())
                            .unwrap_or(0)
                    })
                    .max()
                    .unwrap_or(0)
            })
            .collect();

        // draw proper parallel line with widths
        let mut parallel_line = String::from("");
        for width in &widths {
            parallel_line.push_str(generate_parallel_line(*width).as_str());
        }
        parallel_line.push_str("+\x1b[0m\n");
        s.push_str(&parallel_line);

        for line in self.0.iter() {
            s.push_str(&line.to_string_debug(&widths).unwrap().as_str());
            s.push_str("\n");
            s.push_str(&parallel_line);
        }
        write!(f, "{}", s)
    }
}

/* ---------------------------------- tests --------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string_simple() {
        let s = "1,2223,3\n4,5,6\n7,8,9".to_string();
        let table = Table::from_string(s, ",", "\n");
        println!("{:?}", table);
        assert_eq!(table.len(), 3);
        assert_eq!(table.get_line(0).unwrap().len(), 3);
        assert_eq!(table.get_line(1).unwrap().len(), 3);
        assert_eq!(table.get_line(2).unwrap().len(), 3);
        assert_eq!(table.get_cell((0, 0)).unwrap().to_string(), "1");
        assert_eq!(table.get_cell((0, 1)).unwrap().to_string(), "2223");
        assert_eq!(table.get_cell((0, 2)).unwrap().to_string(), "3");
        assert_eq!(table.get_cell((1, 0)).unwrap().to_string(), "4");
        assert_eq!(table.get_cell((1, 1)).unwrap().to_string(), "5");
        assert_eq!(table.get_cell((1, 2)).unwrap().to_string(), "6");
        assert_eq!(table.get_cell((2, 0)).unwrap().to_string(), "7");
        assert_eq!(table.get_cell((2, 1)).unwrap().to_string(), "8");
        assert_eq!(table.get_cell((2, 2)).unwrap().to_string(), "9");
    }

    #[test]
    fn test_to_txt() {
        let s = "1,2223,3\n4,5,6\n7,8,9".to_string();
        let table = Table::from_string(s, ",", "\n");
        table.to_txt("test.txt", ',').unwrap();
        let s = std::fs::read_to_string("test.txt").unwrap();
        let table = Table::from_string(s, ",", "\n");
        println!("{:?}", table);
        assert_eq!(table.len(), 3);
        assert_eq!(table.get_line(0).unwrap().len(), 3);
        assert_eq!(table.get_line(1).unwrap().len(), 3);
        assert_eq!(table.get_line(2).unwrap().len(), 3);
        assert_eq!(table.get_cell((0, 0)).unwrap().to_string(), "1");
        assert_eq!(table.get_cell((0, 1)).unwrap().to_string(), "2223");
        assert_eq!(table.get_cell((0, 2)).unwrap().to_string(), "3");
        assert_eq!(table.get_cell((1, 0)).unwrap().to_string(), "4");
        assert_eq!(table.get_cell((1, 1)).unwrap().to_string(), "5");
        assert_eq!(table.get_cell((1, 2)).unwrap().to_string(), "6");
        assert_eq!(table.get_cell((2, 0)).unwrap().to_string(), "7");
        assert_eq!(table.get_cell((2, 1)).unwrap().to_string(), "8");
        assert_eq!(table.get_cell((2, 2)).unwrap().to_string(), "9");
    }
}
