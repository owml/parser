use crate::error::ErrorKind;
use crate::types::OTypeEncoded;

use core::str;
use nom::{bytes::complete::is_not, character::complete::char, sequence::delimited};

/// Parses `(s)` (owml datatypes) and returns an [OTypeEncoded] (does not know
/// data, only datatype).
named!(
    pub (crate) o_datatype_parser<OTypeEncoded>,
    map_res!(
        delimited(char('('), is_not(")"), char(')')),
        build_o_datatype_parser
    )
);

/// Builds [o_datatype_parser].
#[allow(dead_code)]
fn build_o_datatype_parser(input: &[u8]) -> Result<OTypeEncoded, ErrorKind> {
    let input_str = str::from_utf8(input).map_err(|e| ErrorKind::InvalidEncoding(e))?;

    match input_str {
        "s" => Ok(OTypeEncoded::StringType),
        "i" => Ok(OTypeEncoded::IntType),
        _ => Err(ErrorKind::UnknownType),
    }
}
