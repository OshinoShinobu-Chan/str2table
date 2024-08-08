use std::str::FromStr;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A enum to specify the line or column, used in force parce and output color
pub enum LineColumn {
    Line,
    Column,
}

impl FromStr for LineColumn {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "l" => Ok(LineColumn::Line),
            "c" => Ok(LineColumn::Column),
            "L" => Ok(LineColumn::Line),
            "C" => Ok(LineColumn::Column),
            "Line" => Ok(LineColumn::Line),
            "Column" => Ok(LineColumn::Column),
            _ => Err(()),
        }
    }
}
