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
#[derive(Debug)]
pub enum DataType<'a> {
    StringType,
    IntType,
    ArrayType(&'a DataType<'a>),
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
    ArrayError,
}

fn u8_to_str(input: &[u8]) -> Result<&str, SyntaxError> {
    match str::from_utf8(input) {
        Ok(x) => Ok(x),
        Err(_) => Err(SyntaxError::DataTypeNotFound),
    }
}

fn dt_match(input: &[u8]) -> Result<DataType, SyntaxError> {
    let input_str = u8_to_str(input)?;

    match input_str {
        "s" => Ok(DataType::StringType),
        "i" => Ok(DataType::IntType),
        "o" => Ok(DataType::ObjectType),
        // "a" => {
        //     let array_inner = match get_array_type(input) {
        //         Ok((_, x)) => x,
        //         Err(x) => return Err(SyntaxError::ArrayError),
        //     };

        //     Ok(DataType::ArrayType(&array_inner))
        // },
        _ => Err(SyntaxError::InvalidDataType),
    }
}

// named!(get_array_type<DataType>,
//     map_res!(
//         .. // <-- right parse here
//         dt_match
//     )
// );

named!(
    get_dt<DataType>,
    map_res!(delimited(char('('), is_not(")"), char(')')), dt_match)
);
