//! # Tablecellcore
//! Include enum called ```Tablecellcore``` represents a cell's value in a table,
//! with some useful methods

use ibig::{ibig, IBig};

/// # Tablecellcore
/// Store the value within a cell with its type, valid types are listed below
///     - String
///     - Int
///     - Float

#[derive(Clone)]
pub enum Tablecellcore {
    String(String),
    Int(IBig),
    Float(f64),
}

impl Tablecellcore {
    /// Create a new Tablecell with a string value, automatically decide its type
    pub fn auto_from(value: &String) -> Self {
        if let Ok(v) = IBig::from_str_with_radix_prefix(value.as_str()) {
            //TODO: find out a suitable constraint to aviod excessive memory use
            Self::Int(v)
        } else if let Ok(v) = value.parse::<f64>() {
            Self::Float(v)
            // let v_f32 = value.parse::<f32>();
            // if v_f32.is_err() {
            //     // If parse to f32 get error, it must be a string
            //     return Self::String(value.to_string());
            // }
            // let v_f32 = v_f32.unwrap();
            // let v_f64 = value.parse::<f64>().unwrap();
            // if v_f32.is_infinite() {
            //     return Self::F64(v_f64);
            // }
            // if v_f32.is_nan() {
            //     return Self::F32(v_f32);
            // }
            // // println!("{} {}", v_f32.to_string(), value);
            // if v_f32.to_string() == v_f64.to_string() && (v_f32 == 0.0 || v_f32 == -0.0) {
            //     let parts = value
            //         .split(|c| c == '.' || c == 'e' || c == 'E')
            //         .collect::<Vec<&str>>();
            //     for part in parts {
            //         if let Ok(v) = part.parse::<f64>() {
            //             if v != 0.0 {
            //                 return Self::F64(v_f64);
            //             }
            //         }
            //     }
            //     Self::F32(v_f32)
            // } else if v_f32.to_string() == v_f64.to_string() {
            //     Self::F32(v_f32)
            // } else {
            //     Self::F64(v_f64)
            // }
            // The code above is about deciding a string is f32 or f64, but is not necessary now
        } else {
            Self::String(value.to_string())
        }
    }
    /// Convert the value to a string
    pub fn to_string(&self) -> String {
        match self {
            Self::String(v) => v.clone(),
            Self::Int(v) => v.to_string(),
            Self::Float(v) => v.to_string(),
        }
    }
    /// Force to convert a string to a cell of int, return Err if the Conversion failed
    pub fn force_as_int(value: &String) -> Result<Self, ibig::error::ParseError> {
        let v = IBig::from_str_with_radix_prefix(value.as_str())?;
        Ok(Self::Int(v))
    }
    /// Force to convert a string to a cell of float, return Err if the Conversion failed
    pub fn force_as_float(value: &String) -> Result<Self, std::num::ParseFloatError> {
        let v = value.parse::<f64>()?;
        Ok(Self::Float(v))
    }
    /// Force to convert a string to a cell of string, won't fail
    pub fn force_as_string(value: &String) -> Self {
        Self::String(value.to_string())
    }
}

/* --------------------------------- Display -------------------------------- */

impl std::fmt::Display for Tablecellcore {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::fmt::Debug for Tablecellcore {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::String(v) => write!(f, "{}<str>", v),
            Self::Int(v) => write!(f, "{}<int>", v),
            Self::Float(v) => write!(f, "{}<float>", v),
        }
    }
}

/* ---------------------------------- test ---------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_from() {
        let v = Tablecellcore::auto_from(&"123".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "123<int>");
        let v = Tablecellcore::auto_from(&"123456".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "123456<int>");
        let v = Tablecellcore::auto_from(&"123456789".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "123456789<int>");
        let v = Tablecellcore::auto_from(&"12345678901".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "12345678901<int>");
        let v = Tablecellcore::auto_from(&"123456789012345678901".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "123456789012345678901<int>");
        let v = Tablecellcore::auto_from(&"-123".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "-123<int>");
        let v = Tablecellcore::auto_from(&"-12345".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "-12345<int>");
        let v = Tablecellcore::auto_from(&"-123456789".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "-123456789<int>");
        let v = Tablecellcore::auto_from(&"-12345678901".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "-12345678901<int>");
        let v = Tablecellcore::auto_from(&"-123456789012345678901".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "-123456789012345678901<int>");
        let v = Tablecellcore::auto_from(&"123.456".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "123.456<float>");
        let v = Tablecellcore::auto_from(&"123.45678901234567890123456789".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "123.45678901234568<float>");
        let v = Tablecellcore::auto_from(&"Hello, world!".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "Hello, world!<str>");
    }

    #[test]
    fn test_auto_from_special() {
        let v = Tablecellcore::auto_from(&"inf".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "inf<float>");
        let v = Tablecellcore::auto_from(&"-inf".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "-inf<float>");
        let v = Tablecellcore::auto_from(&"NAN".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "NaN<float>");
        let v = Tablecellcore::auto_from(&"-NaN".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "NaN<float>");
        let v = Tablecellcore::auto_from(&"1e400".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "inf<float>");
        let v = Tablecellcore::auto_from(&"-1e400".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "-inf<float>");
        let v = Tablecellcore::auto_from(&"0.00".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "0<float>");
        let v = Tablecellcore::auto_from(&"1e-400".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "0<float>");
        let v = Tablecellcore::auto_from(&"-1e-400".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "-0<float>");
        let v = Tablecellcore::auto_from(&"0.2e-400".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "0<float>");
        let v = Tablecellcore::auto_from(&"1.00".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "1<float>");
        let v = Tablecellcore::auto_from(&"0.2e-10".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "0.00000000002<float>");
        let v = Tablecellcore::auto_from(&"1.00".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "1<float>");
        let v = Tablecellcore::auto_from(&"10_0".to_string());
        let output = format!("{:?}", v);
        assert_eq!(output, "10_0<str>");
    }

    #[test]
    fn test_to_string() {
        let v = Tablecellcore::Int(ibig!(123));
        assert_eq!(v.to_string(), "123");
        let v = Tablecellcore::Float(123.456);
        assert_eq!(v.to_string(), "123.456");
        let v = Tablecellcore::String("Hello, world!".to_string());
        assert_eq!(v.to_string(), "Hello, world!");
    }

    #[test]
    fn test_force_as_int() {
        let v = Tablecellcore::force_as_int(&"123".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "123<int>");
        let v = Tablecellcore::force_as_int(&"123456".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "123456<int>");
        let v = Tablecellcore::force_as_int(&"123456789".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "123456789<int>");
        let v = Tablecellcore::force_as_int(&"12345678901".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "12345678901<int>");
        let v = Tablecellcore::force_as_int(&"123456789012345678901".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "123456789012345678901<int>");
        let v = Tablecellcore::force_as_int(&"-123".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "-123<int>");
        let v = Tablecellcore::force_as_int(&"-12345".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "-12345<int>");
        let v = Tablecellcore::force_as_int(&"-123456789".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "-123456789<int>");
        let v = Tablecellcore::force_as_int(&"-12345678901".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "-12345678901<int>");
        let v = Tablecellcore::force_as_int(&"-123456789012345678901".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "-123456789012345678901<int>");
        let v = Tablecellcore::force_as_int(&"123.456".to_string());
        assert!(v.is_err());
        let v = Tablecellcore::force_as_int(&"Hello".to_string());
        assert!(v.is_err());
    }

    #[test]
    fn test_force_as_float() {
        // TODO: need more test
        let v = Tablecellcore::force_as_float(&"123.456".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "123.456<float>");
        let v =
            Tablecellcore::force_as_float(&"123.45678901234567890123456789".to_string()).unwrap();
        let output = format!("{:?}", v);
        assert_eq!(output, "123.45678901234568<float>");
        let v = Tablecellcore::force_as_float(&"Hello".to_string());
        assert!(v.is_err());
    }
}
