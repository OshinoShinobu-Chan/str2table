#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// A enum to specify the line or column, used in force parce and output color
pub enum LineColumn {
    Line,
    Column,
}
