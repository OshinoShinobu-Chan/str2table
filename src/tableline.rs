//! # Tableline
//! Include a vector of tablecells, representing a line of a table.
use crate::setting::Args;
use crate::tablecell::Tablecell;
#[derive(Clone)]
pub struct Tableline(Vec<Tablecell>);

impl Tableline {
    pub fn new() -> Tableline {
        Tableline(Vec::new())
    }

    pub fn from_vec(cells: Vec<Tablecell>) -> Tableline {
        Tableline(cells)
    }

    /// Parse a string a tableline, use the settings from `args`
    // pub fn from_string_arg(s: String, args: Args) -> Tableline {
    //     // TODO
    // }

    /// Parse a string to a tableline, assuming the string has no '\n' or '\r' in it
    ///
    /// The seperation char is used to seperate the cells
    ///
    /// ignore the blank at start, end and around the seperation char
    ///
    /// empty cells will also be ignored
    pub fn from_string(s: String, seperation: &str) -> Tableline {
        let s = s.as_str().trim();
        let cells: Vec<Tablecell> = s
            .split(seperation)
            .map(|cell| cell.trim())
            .filter(|cell| !cell.is_empty())
            .map(|cell| Tablecell::auto_from(cell.to_string()))
            .collect();
        Tableline(cells)
    }

    /// Parse a string to a tableline linke ```from_string()``` but force the cell as string
    pub fn from_string_force(s: String, seperation: &str) -> Tableline {
        let s = s.as_str().trim();
        let cells: Vec<Tablecell> = s
            .split(seperation)
            .map(|cell| cell.trim())
            .filter(|cell| !cell.is_empty())
            .map(|cell| Tablecell::force_as_string(cell.to_string()))
            .collect();
        Tableline(cells)
    }

    /// convert a tableline to string, with | as seperation and align to given width, in displau mode
    pub fn to_string_display(&self, widths: &Vec<usize>) -> Result<String, &'static str> {
        if self.0.len() == 0 {
            return Err("Empty line");
        }
        let mut s = String::new();
        s.push_str("\x1b[90m|\x1b[0m ");
        for (i, cell) in self.0.iter().enumerate() {
            if widths[i] < cell.len() {
                return Err("Width too small");
            }
            s.push_str(format!("{}", cell).as_str());
            s.push_str(" ".repeat(widths[i] - cell.len()).as_str());
            s.push_str(" \x1b[90m|\x1b[0m ");
        }
        for i in self.0.len()..widths.len() {
            s.push_str(" ".repeat(widths[i]).as_str());
            s.push_str(" \x1b[90m|\x1b[0m ");
        }
        Ok(s)
    }

    /// convert a tableline to string, with | as seperation and align to given width, in debug mode
    pub fn to_string_debug(&self, widths: &Vec<usize>) -> Result<String, &'static str> {
        let mut s = String::new();
        s.push_str("\x1b[90m|\x1b[0m ");
        for (i, cell) in self.0.iter().enumerate() {
            if widths[i] < format!("{:?}", cell).len() {
                return Err("Width too small");
            }
            s.push_str(format!("{:?}", cell).as_str());
            s.push_str(" ".repeat(widths[i] - format!("{:?}", cell).len()).as_str());
            s.push_str(" \x1b[90m|\x1b[0m ");
        }
        for i in self.0.len()..widths.len() {
            s.push_str(" ".repeat(widths[i]).as_str());
            s.push_str(" \x1b[90m|\x1b[0m ");
        }
        Ok(s)
    }

    /// convert a tableline to string with given seperation char
    pub fn to_string_format(&self, seperation: char) -> String {
        let mut s = String::new();
        for cell in self.0.iter() {
            s.push_str(cell.to_string().as_str());
            s.push(seperation);
            s.push(' ');
        }
        s
    }

    /// convert a tableline to string without color info
    pub fn to_string_raw(&self, seperation: char) -> String {
        let mut s = String::new();
        for cell in self.0.iter() {
            s.push_str(cell.core.to_string().as_str());
            s.push(seperation);
            s.push(' ');
        }
        s
    }

    /// Push one cell to the end of line
    pub fn push_cell(&mut self, cell: Tablecell) {
        self.0.push(cell);
    }

    /// Pop one cell from the end of line
    pub fn pop_cell(&mut self) -> Option<Tablecell> {
        self.0.pop()
    }

    /// Insert a cell at the index, return Err if the index is out of range
    pub fn insert_cell(&mut self, index: usize, cell: Tablecell) -> Result<(), String> {
        if index > self.0.len() {
            return Err("Index out of range".to_string());
        }
        self.0.insert(index, cell);
        Ok(())
    }

    /// Remove a cell at the index, return Err if the index is out of range
    pub fn remove_cell(&mut self, index: usize) -> Result<Tablecell, String> {
        if index >= self.0.len() {
            return Err("Index out of range".to_string());
        }
        Ok(self.0.remove(index))
    }

    /// Push multiple cells to the end of line
    pub fn push_cells(&mut self, cells: Vec<Tablecell>) {
        self.0.extend(cells);
    }

    /// Remove range of cells from the line, return Err if the range is out of range
    pub fn remove_cells(&mut self, start: usize, end: usize) -> Result<(), String> {
        if start >= self.0.len() || end >= self.0.len() {
            return Err("Index out of range".to_string());
        }
        self.0.drain(start..=end);
        Ok(())
    }

    /// Insert multiple cells at the index, return Err if the index is out of range
    pub fn insert_cells(&mut self, index: usize, cells: Vec<Tablecell>) -> Result<(), String> {
        if index > self.0.len() {
            return Err("Index out of range".to_string());
        }
        self.0.splice(index..index, cells);
        Ok(())
    }

    /// Get the length of the line
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Get the cell at the index
    pub fn get_cell(&self, index: usize) -> Option<&Tablecell> {
        self.0.get(index)
    }

    /// Get the mutable cell at the index
    pub fn get_cell_mut(&mut self, index: usize) -> Option<&mut Tablecell> {
        self.0.get_mut(index)
    }

    /// Get the cells of the line at the index range
    pub fn get_cells(&self, start: usize, end: usize) -> Option<&[Tablecell]> {
        if start >= self.0.len() || end >= self.0.len() {
            return None;
        }
        Some(&self.0[start..=end])
    }

    /// Get the mutable cells of the line at the index range
    pub fn get_cells_mut(&mut self, start: usize, end: usize) -> Option<&mut [Tablecell]> {
        if start >= self.0.len() || end >= self.0.len() {
            return None;
        }
        Some(&mut self.0[start..=end])
    }
}

/* --------------------------------- Display -------------------------------- */

impl std::fmt::Display for Tableline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.is_empty() {
            return write!(f, "");
        }
        let mut s = String::new();
        s.push_str("| ");
        for cell in self.0.iter() {
            s.push_str(&cell.to_string());
            s.push_str(" | ");
        }
        write!(f, "{}", s)
    }
}

impl std::fmt::Debug for Tableline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str("| ");
        for cell in self.0.iter() {
            s.push_str(format!("{:?}", cell).as_str());
            s.push_str(" | ");
        }
        write!(f, "{}", s)
    }
}

/* ---------------------------------- test ---------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string_simple() {
        let s = "  a  |  123.456 |  100  ".to_string();
        let line = Tableline::from_string(s, "|");
        let output = format!("{:?}", line);
        assert_eq!(
            output,
            "| a<str><Black> | 123.456<float><Black> | 100<int><Black> | "
        );
        let s = "  a  |  123.456 |  100  |   |".to_string();
        let line = Tableline::from_string(s, "|");
        let output = format!("{:?}", line);
        assert_eq!(
            output,
            "| a<str><Black> | 123.456<float><Black> | 100<int><Black> | "
        );
    }
}
