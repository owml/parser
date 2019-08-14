use crate::error::ErrorKind;
use crate::types::OType;

use core::str;
use nom::character::streaming::digit0;

/// Finds the [OType] for given data. This is a frontend parser for
/// `data_string_parser` and `data_int_parser`.
named!(
    pub (crate) data_parser<OType>,
    alt!(
        data_string_parser |
        data_int_parser
    )
);

/// Detects a string wrapped in `"` and returns an [OType::StringType].
named!(
    data_string_parser<OType>,
    map_res!(
        delimited!(one_of!("\"\'"), is_not!("\"\'"), one_of!("\"\'")),
        build_data_string_parser
    )
);

/// Builds data_string_parser.
#[allow(dead_code)]
fn build_data_string_parser(input: &[u8]) -> Result<OType, ()> {
    Ok(OType::StringType(input))
}

/// Detects an i32 and returns an [OType::IntType].
named!(
    data_int_parser<OType>,
    map_res!(digit0, build_data_int_parser)
);

/// Builds data_int_parser.
#[allow(dead_code)]
fn build_data_int_parser(input: &[u8]) -> Result<OType, ErrorKind> {
    let res_int = str::parse::<i32>(unsafe { str::from_utf8_unchecked(input) })
        .map_err(|_| ErrorKind::InvalidEncoding)?;

    Ok(OType::IntType(res_int))
}
