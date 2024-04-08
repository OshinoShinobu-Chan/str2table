//! # Tablecell
//! Include enum called ```Tablecell``` represents a cell in a table,
//! with some useful methods

/// # Tablecell
/// Store the value within a cell with its type, valid types are listed below
///     - String
///     - i8
///     - i16
///     - i32
///     - i64
///     - i128
///     - u8
///     - u16
///     - u32
///     - u64
///     - u128  
///     - f32
///     - f64

#[derive(Clone)]
pub enum Tablecell {
    String(String),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(f32),
    F64(f64),
}

impl Tablecell {
    /// Create a new Tablecell with a string value, automatically decide its type
    pub fn auto_from(value: String) -> Tablecell {
        if let Ok(v) = value.parse::<u8>() {
            Self::U8(v)
        } else if let Ok(v) = value.parse::<u16>() {
            Self::U16(v)
        } else if let Ok(v) = value.parse::<u32>() {
            Self::U32(v)
        } else if let Ok(v) = value.parse::<u64>() {
            Self::U64(v)
        } else if let Ok(v) = value.parse::<u128>() {
            Self::U128(v)
        } else if let Ok(v) = value.parse::<i8>() {
            Self::I8(v)
        } else if let Ok(v) = value.parse::<i16>() {
            Self::I16(v)
        } else if let Ok(v) = value.parse::<i32>() {
            Self::I32(v)
        } else if let Ok(v) = value.parse::<i64>() {
            Self::I64(v)
        } else if let Ok(v) = value.parse::<i128>() {
            Self::I128(v)
        } else {
            let v_f32 = value.parse::<f32>();
            if v_f32.is_err() {
                // If parse to f32 get error, it must be a string
                return Self::String(value);
            }
            let v_f32 = v_f32.unwrap();
            let v_f64 = value.parse::<f64>().unwrap();
            if v_f32.is_infinite() {
                return Self::F64(v_f64);
            }
            if v_f32.is_nan() {
                return Self::F32(v_f32);
            }
            // println!("{} {}", v_f32.to_string(), value);
            if v_f32.to_string() == v_f64.to_string() && (v_f32 == 0.0 || v_f32 == -0.0) {
                let parts = value
                    .split(|c| c == '.' || c == 'e' || c == 'E')
                    .collect::<Vec<&str>>();
                for part in parts {
                    if let Ok(v) = part.parse::<f64>() {
                        if v != 0.0 {
                            return Self::F64(v_f64);
                        }
                    }
                }
                Self::F32(v_f32)
            } else if v_f32.to_string() == v_f64.to_string() {
                Self::F32(v_f32)
            } else {
                Self::F64(v_f64)
            }
        }
    }
    /// Convert the value to a string
    pub fn to_string(&self) -> String {
        match self {
            Self::String(v) => v.clone(),
            Self::I8(v) => v.to_string(),
            Self::I16(v) => v.to_string(),
            Self::I32(v) => v.to_string(),
            Self::I64(v) => v.to_string(),
            Self::I128(v) => v.to_string(),
            Self::U8(v) => v.to_string(),
            Self::U16(v) => v.to_string(),
            Self::U32(v) => v.to_string(),
            Self::U64(v) => v.to_string(),
            Self::U128(v) => v.to_string(),
            Self::F32(v) => v.to_string(),
            Self::F64(v) => v.to_string(),
        }
    }
    /// Force to convert a string to a cell of i8, return Err if the conversion failed
    pub fn force_as_i8(value: String) -> Result<Tablecell, String> {
        match value.parse::<i8>() {
            Ok(v) => Ok(Self::I8(v)),
            Err(e) => Err(e.to_string()),
        }
    }
    /// Force to convert a string to a cell of i16, return Err if the conversion failed
    pub fn force_as_i16(value: String) -> Result<Tablecell, String> {
        match value.parse::<i16>() {
            Ok(v) => Ok(Self::I16(v)),
            Err(e) => Err(e.to_string()),
        }
    }
    /// Force to convert a string to a cell of i32, return Err if the conversion failed
    pub fn force_as_i32(value: String) -> Result<Tablecell, String> {
        match value.parse::<i32>() {
            Ok(v) => Ok(Self::I32(v)),
            Err(e) => Err(e.to_string()),
        }
    }
    /// Force to convert a string to a cell of i64, return Err if the conversion failed
    pub fn force_as_i64(value: String) -> Result<Tablecell, String> {
        match value.parse::<i64>() {
            Ok(v) => Ok(Self::I64(v)),
            Err(e) => Err(e.to_string()),
        }
    }
    /// Force to convert a string to a cell of i128, return Err if the conversion failed
    pub fn force_as_i128(value: String) -> Result<Tablecell, String> {
        match value.parse::<i128>() {
            Ok(v) => Ok(Self::I128(v)),
            Err(e) => Err(e.to_string()),
        }
    }
    /// Force to convert a string to a cell of u8, return Err if the conversion failed
    pub fn force_as_u8(value: String) -> Result<Tablecell, String> {
        match value.parse::<u8>() {
            Ok(v) => Ok(Self::U8(v)),
            Err(e) => Err(e.to_string()),
        }
    }
    /// Force to convert a string to a cell of u16, return Err if the conversion failed
    pub fn force_as_u16(value: String) -> Result<Tablecell, String> {
        match value.parse::<u16>() {
            Ok(v) => Ok(Self::U16(v)),
            Err(e) => Err(e.to_string()),
        }
    }
    /// Force to convert a string to a cell of u32, return Err if the conversion failed
    pub fn force_as_u32(value: String) -> Result<Tablecell, String> {
        match value.parse::<u32>() {
            Ok(v) => Ok(Self::U32(v)),
            Err(e) => Err(e.to_string()),
        }
    }
    /// Force to convert a string to a cell of u64, return Err if the conversion failed
    pub fn force_as_u64(value: String) -> Result<Tablecell, String> {
        match value.parse::<u64>() {
            Ok(v) => Ok(Self::U64(v)),
            Err(e) => Err(e.to_string()),
        }
    }
    /// Force to convert a string to a cell of u128, return Err if the conversion failed
    pub fn force_as_u128(value: String) -> Result<Tablecell, String> {
        match value.parse::<u128>() {
            Ok(v) => Ok(Self::U128(v)),
            Err(e) => Err(e.to_string()),
        }
    }
    /// Force to convert a string to a cell of f32, return Err if the conversion failed
    pub fn force_as_f32(value: String) -> Result<Tablecell, String> {
        match value.parse::<f32>() {
            Ok(v) => Ok(Self::F32(v)),
            Err(e) => Err(e.to_string()),
        }
    }
    /// Force to convert a string to a cell of f64, return Err if the conversion failed
    pub fn force_as_f64(value: String) -> Result<Tablecell, String> {
        match value.parse::<f64>() {
            Ok(v) => Ok(Self::F64(v)),
            Err(e) => Err(e.to_string()),
        }
    }
    /// Force to convert a string to a cell of string, won't fail
    pub fn force_as_string(value: String) -> Tablecell {
        Self::String(value)
    }
}

/* --------------------------------- Display -------------------------------- */

impl std::fmt::Display for Tablecell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::fmt::Debug for Tablecell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::String(v) => write!(f, "{}<str>", v),
            Self::I8(v) => write!(f, "{}<i8>", v),
            Self::I16(v) => write!(f, "{}<i16>", v),
            Self::I32(v) => write!(f, "{}<i32>", v),
            Self::I64(v) => write!(f, "{}<i64>", v),
            Self::I128(v) => write!(f, "{}<i128>", v),
            Self::U8(v) => write!(f, "{}<u8>", v),
            Self::U16(v) => write!(f, "{}<u16>", v),
            Self::U32(v) => write!(f, "{}<u32>", v),
            Self::U64(v) => write!(f, "{}<u64>", v),
            Self::U128(v) => write!(f, "{}<u128>", v),
            Self::F32(v) => write!(f, "{}<f32>", v),
            Self::F64(v) => write!(f, "{}<f64>", v),
        }
    }
}

/* ---------------------------------- test ---------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_from() {
        let v = Tablecell::auto_from("123".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "123<u8>");
        let v = Tablecell::auto_from("123456".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "123456<u32>");
        let v = Tablecell::auto_from("123456789".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "123456789<u32>");
        let v = Tablecell::auto_from("12345678901".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "12345678901<u64>");
        let v = Tablecell::auto_from("123456789012345678901".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "123456789012345678901<u128>");
        let v = Tablecell::auto_from("-123".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "-123<i8>");
        let v = Tablecell::auto_from("-12345".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "-12345<i16>");
        let v = Tablecell::auto_from("-123456789".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "-123456789<i32>");
        let v = Tablecell::auto_from("-12345678901".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "-12345678901<i64>");
        let v = Tablecell::auto_from("-123456789012345678901".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "-123456789012345678901<i128>");
        let v = Tablecell::auto_from("123.456".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "123.456<f32>");
        let v = Tablecell::auto_from("123.45678901234567890123456789".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "123.45678901234568<f64>");
        let v = Tablecell::auto_from("Hello, world!".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "Hello, world!<string>");
    }

    #[test]
    fn test_auto_from_special() {
        let v = Tablecell::auto_from("inf".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "inf<f64>");
        let v = Tablecell::auto_from("-inf".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "-inf<f64>");
        let v = Tablecell::auto_from("NAN".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "NaN<f32>");
        let v = Tablecell::auto_from("-NaN".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "NaN<f32>");
        let v = Tablecell::auto_from("1e400".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "inf<f64>");
        let v = Tablecell::auto_from("-1e400".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "-inf<f64>");
        let v = Tablecell::auto_from("0.00".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "0<f32>");
        let v = Tablecell::auto_from("1e-400".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "0<f64>");
        let v = Tablecell::auto_from("-1e-400".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "-0<f64>");
        let v = Tablecell::auto_from("0.2e-400".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "0<f64>");
        let v = Tablecell::auto_from("1.00".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "1<f32>");
        let v = Tablecell::auto_from("0.2e-10".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "0.00000000002<f32>");
        let v = Tablecell::auto_from("1.00".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "1<f32>");
        let v = Tablecell::auto_from("10_0".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "10_0<string>");
    }

    #[test]
    fn test_to_string() {
        let v = Tablecell::U8(123);
        assert_eq!(v.to_string(), "123");
        let v = Tablecell::U16(12345);
        assert_eq!(v.to_string(), "12345");
        let v = Tablecell::U32(123456);
        assert_eq!(v.to_string(), "123456");
        let v = Tablecell::U64(123456789);
        assert_eq!(v.to_string(), "123456789");
        let v = Tablecell::U128(12345678901234567890);
        assert_eq!(v.to_string(), "12345678901234567890");
        let v = Tablecell::I8(-123);
        assert_eq!(v.to_string(), "-123");
        let v = Tablecell::I16(-12345);
        assert_eq!(v.to_string(), "-12345");
        let v = Tablecell::I32(-123456789);
        assert_eq!(v.to_string(), "-123456789");
        let v = Tablecell::I64(-12345678901);
        assert_eq!(v.to_string(), "-12345678901");
        let v = Tablecell::I128(-12345678901234567890);
        assert_eq!(v.to_string(), "-12345678901234567890");
        let v = Tablecell::F64(123.456);
        assert_eq!(v.to_string(), "123.456");
        let v = Tablecell::String("Hello, world!".to_string());
        assert_eq!(v.to_string(), "Hello, world!");
    }

    #[test]
    fn test_force_as_i8() {
        let v = Tablecell::force_as_i8("123".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "123<i8>");
        let v = Tablecell::force_as_i8("-123".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "-123<i8>");
        let v = Tablecell::force_as_i8("123.456".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_i8("1234".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_i8("value123".to_string());
        assert!(v.is_err());
    }

    #[test]
    fn test_force_as_i16() {
        let v = Tablecell::force_as_i16("123".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "123<i16>");
        let v = Tablecell::force_as_i16("-123".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "-123<i16>");
        let v = Tablecell::force_as_i16("123.456".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_i16("123456".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_i16("value123".to_string());
        assert!(v.is_err());
    }

    #[test]
    fn test_force_as_i32() {
        let v = Tablecell::force_as_i32("123".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "123<i32>");
        let v = Tablecell::force_as_i32("-123".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "-123<i32>");
        let v = Tablecell::force_as_i32("123.456".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_i32("12345678901".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_i32("value123".to_string());
        assert!(v.is_err());
    }

    #[test]
    fn test_force_as_i64() {
        let v = Tablecell::force_as_i64("123".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "123<i64>");
        let v = Tablecell::force_as_i64("-123".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "-123<i64>");
        let v = Tablecell::force_as_i64("123.456".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_i64("123456789012345678901".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_i64("value123".to_string());
        assert!(v.is_err());
    }

    #[test]
    fn test_force_as_i128() {
        let v = Tablecell::force_as_i128("123".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "123<i128>");
        let v = Tablecell::force_as_i128("-123".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "-123<i128>");
        let v = Tablecell::force_as_i128("123.456".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_i128("1234567890123456789012345678901234567890".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_i128("value123".to_string());
        assert!(v.is_err());
    }

    #[test]
    fn test_force_as_u8() {
        let v = Tablecell::force_as_u8("123".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "123<u8>");
        let v = Tablecell::force_as_u8("123.456".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_u8("1234".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_u8("-123".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_u8("value123".to_string());
        assert!(v.is_err());
    }

    #[test]
    fn test_force_as_u16() {
        let v = Tablecell::force_as_u16("123".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "123<u16>");
        let v = Tablecell::force_as_u16("123.456".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_u16("123456".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_u16("-123".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_u16("value123".to_string());
        assert!(v.is_err());
    }

    #[test]
    fn test_force_as_u32() {
        let v = Tablecell::force_as_u32("123".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "123<u32>");
        let v = Tablecell::force_as_u32("123.456".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_u32("12345678901".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_u32("-123".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_u32("value123".to_string());
        assert!(v.is_err());
    }

    #[test]
    fn test_force_as_u64() {
        let v = Tablecell::force_as_u64("123".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "123<u64>");
        let v = Tablecell::force_as_u64("123.456".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_u64("123456789012345678901".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_u64("-123".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_u64("value123".to_string());
        assert!(v.is_err());
    }

    #[test]
    fn test_force_as_u128() {
        let v = Tablecell::force_as_u128("123".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "123<u128>");
        let v = Tablecell::force_as_u128("123.456".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_u128("1234567890123456789012345678901234567890".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_u128("-123".to_string());
        assert!(v.is_err());
        let v = Tablecell::force_as_u128("value123".to_string());
        assert!(v.is_err());
    }

    #[test]
    fn test_force_as_f32() {
        let v = Tablecell::force_as_f32("123".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "123<f32>");
        let v = Tablecell::force_as_f32("123.456".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "123.456<f32>");
        let v = Tablecell::force_as_f32("-123".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "-123<f32>");
        let v = Tablecell::force_as_f32("inf".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "inf<f32>");
        let v = Tablecell::force_as_f32("NAN".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "NaN<f32>");
        let v = Tablecell::force_as_f32("-inf".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "-inf<f32>");
        let v = Tablecell::force_as_f32("-NaN".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "NaN<f32>");
        let v = Tablecell::force_as_f32("value123".to_string());
        assert!(v.is_err());
    }

    #[test]
    fn test_force_as_f64() {
        let v = Tablecell::force_as_f64("123".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "123<f64>");
        let v = Tablecell::force_as_f64("123.456".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "123.456<f64>");
        let v = Tablecell::force_as_f64("-123".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "-123<f64>");
        let v = Tablecell::force_as_f64("inf".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "inf<f64>");
        let v = Tablecell::force_as_f64("NAN".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "NaN<f64>");
        let v = Tablecell::force_as_f64("-inf".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "-inf<f64>");
        let v = Tablecell::force_as_f64("-NaN".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "NaN<f64>");
        let v = Tablecell::force_as_f64("value123".to_string());
        assert!(v.is_err());
    }
}
