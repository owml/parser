#![allow(unused_doc_comments)]

#[macro_use]
extern crate nom;

use core::str;
use nom::{bytes::complete::is_not, character::complete::char, sequence::delimited};

/// The main type enum for owml, containing the type along with the corrosponding data.
///
/// *If you would like to not embed the data, you may use the private [OTypeEncoded]*
#[derive(Debug, PartialEq)]
pub enum OType {
    StringType(String),
    IntType(i32),
}

/// The main error enum for owml
///
/// # Error Types
///
/// - [ErrorKind::UnknownType]: When a given type (Example: `(s)` for a String) is
/// unknown.
/// - [ErrorKind::InvalidEncoding]: When an inputted string is encoded incorrectly
/// and the parser cannot understand it. *Your best bet for dealing with this
/// one is using UTF8 encoding*.
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    UnknownType,
    InvalidEncoding(str::Utf8Error),
    DataTypesDontMatch,
}

/// Similar to OType but doesn't come with mandatory data encoded. This is
/// usually used by a parser that does not contain data but still wants to
/// convey a certain OType to use in the future.
#[derive(Debug, PartialEq)]
enum OTypeEncoded {
    StringType,
    IntType,
}

impl OTypeEncoded {
    /// Compares [OTypeEncoded] and [OType] to see if they have matching
    /// parameters and return a bool if they do (true) or don't (false).
    fn compare_otype(&self, to_compare: &OType) -> bool {
        match to_compare {
            OType::IntType(_) => self == &OTypeEncoded::IntType,
            OType::StringType(_) => self == &OTypeEncoded::StringType,
        }
    }
}

/// Parses `(s)` (owml datatypes) and returns an [OTypeEncoded] (does not know
/// data, only datatype).
named!(
    o_datatype_parser<OTypeEncoded>,
    map_res!(
        delimited(char('('), is_not(")"), char(')')),
        build_o_datatype_parser
    )
);

fn build_o_datatype_parser(input: &[u8]) -> Result<OTypeEncoded, ErrorKind> {
    let input_str = str::from_utf8(input).map_err(|e| ErrorKind::InvalidEncoding(e))?;

    match input_str {
        "s" => Ok(OTypeEncoded::StringType),
        "i" => Ok(OTypeEncoded::IntType),
        _ => Err(ErrorKind::UnknownType),
    }
}

/// Parses a key (for example: `(s) "Hello"`) and returns a full [OType] with
/// the matching data or throws an error from [ErrorKind].
named!(
    o_data_parser<OType>,
    map_res!(
        char('g'), // TODO add proper data parsing.
        build_o_data_parser
    )
);

fn build_o_data_parser(_input: char) -> Result<OType, ErrorKind> {
    // TODO parse the `input` and return as OType
    unimplemented!();
}

/// Adds together [o_datatype_parser] and [build_o_data_parser] to get one
/// entire `(s) "hello"` and return an [OType] for it.
named!(
    pub o_key<OType>,
    map_res!(
        do_parse!(
            exp_dt: o_datatype_parser >>
            char!(' ') >> // TODO make optional until
            found_data: o_data_parser >>
            (exp_dt, found_data)
        ),
        build_o_key_parser
    )
);

fn build_o_key_parser(input: (OTypeEncoded, OType)) -> Result<OType, ErrorKind> {
    if input.0.compare_otype(&input.1) {
        Ok(input.1)
    } else {
        Err(ErrorKind::DataTypesDontMatch)
    }
}
