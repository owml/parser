// #![no_std]

#[macro_use]
extern crate nom;

use std::vec::Vec; // here for now until `core::alloc::Vec` works
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
#[derive(Debug, PartialEq)]
pub enum SyntaxError {
    InvalidDataType(str::Utf8Error),
    DataTypeNotFound,
}

/// Matches a raw, u8 slice of an str into valid datatypes or returns a
/// [SyntaxError] error.
fn match_datatypes(in_u8_slice: &[u8]) -> Result<DataType, SyntaxError> {
    let in_str = str::from_utf8(in_u8_slice)
        .map_err(|error| SyntaxError::InvalidDataType(error))?;

    match in_str {
        "s" => Ok(DataType::StringType),
        "i" => Ok(DataType::IntType),
        "o" => Ok(DataType::ObjectType),
        "a" => {
            let array_recursive = match arraytype(in_u8_slice) {
                Ok((_, x)) => x,
                Err(_) => return Err(SyntaxError::DataTypeNotFound),
            };

            Ok(DataType::ArrayType(&array_recursive))
        },
        _ => Err(SyntaxError::DataTypeNotFound)
    }
}

named!(
    arraytype<DataType>,
    map_res!(
        many_till!(tag!("a-"), alt!(char!('s') | char!('i') | char!('o') | char!('a'))),
        build_arraytype_parser
    )
);

/// Converts the unusable returns from `arraytype` into a parsed result.
fn build_arraytype_parser(in_vec: (Vec<&[u8]>, char)) -> Result<DataType, SyntaxError> {
    match_datatypes(in_vec.0) // TODO fix
}

named!(
    datatype<DataType>,
    map_res!(
        delimited(char('('), is_not(")"), char(')')),
        match_datatypes
    )
);
