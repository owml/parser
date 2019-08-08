#[macro_use]
extern crate nom;

use core::str;
use nom::{bytes::complete::is_not, character::complete::char, sequence::delimited};

/// The main type enum for owml, containing the type along with the corrosponding data.
///
/// *If you would like to not embed the data, you may use the private [OTypeEncoded]
#[derive(Debug, PartialEq)]
pub enum OType {
    StringType(String),
    IntType(i32),
}

/// The main error enum for owml
///
/// # Error Types
///
/// - [OError::UnknownType]: When a given type (Example: `(s)` for a String) is
/// unknown.
/// - [OError::InvalidEncoding]: When an inputted string is encoded incorrectly
/// and the parser cannot understand it. *Your best bet for dealing with this
/// one is using UTF8 encoding*.
#[derive(Debug, PartialEq)]
pub enum OError {
    UnknownType,
    InvalidEncoding(str::Utf8Error),
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
    fn compare_otype(&self, to_compare: OType) -> bool {
        match to_compare {
            OType::IntType(_) => self == &OTypeEncoded::IntType,
            OType::StringType(_) => self == &OTypeEncoded::StringType,
        }
    }
}

named!(
    o_datatype_parser<OTypeEncoded>,
    map_res!(
        delimited(char('('), is_not(")"), char(')')),
        build_o_datatype_parser
    )
);

fn build_o_datatype_parser(input: &[u8]) -> Result<OTypeEncoded, OError> {
    let input_str = str::from_utf8(input).map_err(|e| OError::InvalidEncoding(e))?;

    match input_str {
        "s" => Ok(OTypeEncoded::StringType),
        "i" => Ok(OTypeEncoded::IntType),
        _ => Err(OError::UnknownType),
    }
}

named!(
    pub o_data_parser<OType>,
    map_res!(
        char('s'), // testing, not real. should change dt
        build_o_data_parser
    )
);

fn build_o_data_parser(input: char) -> Result<OType, OError> {
    let (stripped_input, found_dt) = o_datatype_parser(&[input as u8]).unwrap(); // &[input as u8] for now

    // TODO parse the `input` into x and then match it to the datatypes of `found_dt` with if found_dt.compare_otype(x) and etc
    unimplemented!();
}
