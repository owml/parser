#![no_std]

#[macro_use]
extern crate nom;

use core::str;
use nom::{bytes::complete::is_not, character::complete::char, sequence::delimited};

/// The main enum for identifying owens-ml datatypes. All written datatypes
/// are lower-cased versions of these options.
///
/// # Translated Types
///
/// - [DataType::StringType]: `(s)`
/// - [DataType::IntType]: `(i)`
/// - [DataType::ArrayType]: `(a-[type])` *where `[type]` is a type that's not an array*
/// - [DataType::ObjectType]: `(o)`
#[derive(Debug, PartialEq)]
pub enum DataType {
    StringType,
    IntType,
    ObjectType,
}

/// The main error enum for `owens-ml`.
///
/// # Error types
///
/// - [SyntaxError::InvalidDataType]: When the datatype is not found (does not
/// line up to any datatype inside of the [DataType] struct)
/// - [SyntaxError::DataTypeNotFound]: When there are an empty set of empty
/// parenthesis (no datatype given)
#[derive(Debug, PartialEq)]
pub enum SyntaxError {
    InvalidDataType(str::Utf8Error),
    DataTypeNotFound,
}

/// Matches a raw, u8 slice of an str into valid datatypes or returns a
/// [SyntaxError] error.
fn match_datatypes<'a>(in_u8_slice: &[u8]) -> Result<DataType, SyntaxError> {
    let in_str =
        str::from_utf8(in_u8_slice).map_err(|error| SyntaxError::InvalidDataType(error))?;

    match in_str {
        "s" => Ok(DataType::StringType),
        "i" => Ok(DataType::IntType),
        "o" => Ok(DataType::ObjectType),
        _ => Err(SyntaxError::DataTypeNotFound),
    }
}

named!(
    pub datatype_parser<DataType>,
    map_res!(
        delimited(char('('), is_not(")"), char(')')),
        match_datatypes
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_datatype_parser() {
        assert_eq!(
            Ok((" 3324".as_bytes(), DataType::IntType)),
            datatype_parser("(i) 3324".as_bytes())
        ); // See if it removes (i) and returns [DataType::IntType]
        assert_eq!(
            Ok((" (s)".as_bytes(), DataType::IntType)),
            datatype_parser("(i) (s)".as_bytes())
        ); // See if it parses 1 or two (should be just 1)
    }
}
