#![no_std]

#[macro_use]
extern crate nom;

use nom::{bytes::complete::is_not, character::complete::char, sequence::delimited};
use core::str;

/// The main enum for identifying owens-ml datatypes. All written datatypes
/// are lower-cased versions of these options.
///
/// # Translated Types
///
/// - [DataType::StringType]: `(s)`
/// - [DataType::IntType]: `(i)`
/// - [DataType::ArrayType]: `(a-[type])` *where `[type]` is a type that's not an array*
/// - [DataType::ObjectType]: `(o)`
#[derive(Debug)]
pub enum DataType {
    StringType,
    IntType,
    ArrayType,
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
pub enum SyntaxError {
    InvalidDataType,
    DataTypeNotFound,
}

named!(
    get_dt<DataType>,
    map_res!(delimited(char('('), is_not(")"), char(')')), build_dt)
);

fn build_dt(input: &[u8]) -> Result<DataType, SyntaxError> {
    let input_stringified = match str::from_utf8(input) {
        Ok(x) => x,
        Err(_) => return Err(SyntaxError::DataTypeNotFound),
    };

    match input_stringified {
        "s" => Ok(DataType::StringType),
        "i" => Ok(DataType::IntType),
        "o" => Ok(DataType::ObjectType),
        "a" => Ok(DataType::ArrayType),
        _ => Err(SyntaxError::InvalidDataType),
    }
}
