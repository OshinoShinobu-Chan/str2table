//! # Tablecell
//! Include a struct ```Tablecell```. It attach some addition to the
//! ```Tablecellcore```, for example color.
use crate::setting::OutputColor;
use crate::tablecellcore::Tablecellcore;

#[derive(Clone)]
pub struct Tablecell {
    pub core: Tablecellcore,
    pub color: OutputColor,
}

/// # TableCell
/// A TableCell is a cell in a table, it has a core value and a color
impl Tablecell {
    /// Create a new Tablecell with a string value, automatically decide its type
    /// and set color to default (black)
    pub fn auto_from(value: String) -> Self {
        Tablecell {
            core: Tablecellcore::auto_from(&value),
            color: OutputColor::default(),
        }
    }
    ///
    pub fn from_type(value: String, force_type: crate::setting::ForceType) -> Self {
        Tablecell {
            core: match force_type {
                crate::setting::ForceType::S => Tablecellcore::force_as_string(&value),
                crate::setting::ForceType::I => Tablecellcore::force_as_int(&value).unwrap(),
                crate::setting::ForceType::F => Tablecellcore::force_as_float(&value).unwrap(),
            },

            color: OutputColor::default(),
        }
    }

    /// Create a new Tablecell with a string value, force it to be string and set color to default (black)
    pub fn force_as_string(value: String) -> Self {
        Tablecell {
            core: Tablecellcore::force_as_string(&value),
            color: OutputColor::default(),
        }
    }

    /// Force to convert a string to a cell of integer
    /// use ```auto_from``` if failed
    pub fn force_as_int(value: String) -> Self {
        if let Ok(cell) = Tablecellcore::force_as_int(&value) {
            Tablecell {
                core: cell,
                color: OutputColor::default(),
            }
        } else {
            Tablecell::auto_from(value)
        }
    }

    /// Force to convert a string to a cell of float
    /// use ```auto_from``` if failed
    pub fn force_as_float(value: String) -> Self {
        if let Ok(cell) = Tablecellcore::force_as_float(&value) {
            Tablecell {
                core: cell,
                color: OutputColor::default(),
            }
        } else {
            Tablecell::auto_from(value)
        }
    }

    /// Set the color of the cell
    pub fn set_color(&mut self, color: OutputColor) {
        self.color = color;
    }

    /// Get the length without counting the escape code for color
    pub fn len(&self) -> usize {
        self.core.to_string().chars().count()
    }
}

/* --------------------------------- Display -------------------------------- */
impl std::fmt::Display for Tablecell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            OutputColor::Black => write!(f, "{}", self.core),
            OutputColor::Red => write!(f, "\x1b[31m{}\x1b[0m", self.core),
            OutputColor::Green => write!(f, "\x1b[32m{}\x1b[0m", self.core),
            OutputColor::Yellow => write!(f, "\x1b[33m{}\x1b[0m", self.core),
            OutputColor::Blue => write!(f, "\x1b[34m{}\x1b[0m", self.core),
            OutputColor::White => write!(f, "\x1b[37m{}\x1b[0m", self.core),
            OutputColor::Grey => write!(f, "\x1b[90m{}\x1b[0m", self.core),
        }
    }
}

impl std::fmt::Debug for Tablecell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}<{}>", self.core, self.color.to_string())
    }
}
