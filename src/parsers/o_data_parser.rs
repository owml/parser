use crate::error::ErrorKind;
use crate::types::OType;

use core::str;
use nom::character::streaming::digit0;

/// Detects a string wrapped in `"` and returns an [OType::StringType].
named!(
    o_data_string_parser<OType>,
    map_res!(
        delimited!(one_of!("\"\'"), is_not!("\"\'"), one_of!("\"\'")),
        build_o_data_string_parser
    )
);

/// Builds o_data_string_parser.
#[allow(dead_code)]
fn build_o_data_string_parser(input: &[u8]) -> Result<OType, ()> {
    Ok(OType::StringType(input))
}

/// Detects an i32 and returns an [OType::IntType].
named!(
    o_data_int_parser<OType>,
    map_res!(digit0, build_o_data_int_parser)
);

/// Builds o_data_int_parser.
#[allow(dead_code)]
fn build_o_data_int_parser(input: &[u8]) -> Result<OType, ErrorKind> {
    let res_int = str::parse::<i32>(unsafe { str::from_utf8_unchecked(input) })
        .map_err(|_| ErrorKind::InvalidEncoding)?;

    Ok(OType::IntType(res_int))
}

/// Finds the [OType] for given data. This is a frontend parser for
/// `o_data_string_parser` and `o_data_int_parser`.
named!(
    pub (crate) o_data_parser<OType>,
    alt!(
        o_data_string_parser |
        o_data_int_parser
    )
);

/// Test section
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the o_data_string_parser parser
    #[test]
    fn o_data_string_parser_test() {
        assert_eq!(
            Ok(("\n".as_bytes(), OType::StringType("Hello there!".as_bytes()))),
            o_data_string_parser("\"Hello there!\"\n".as_bytes())
        ); // Expects ok with no input left and `"Hello There"` in a [OType::StringType]
        assert_eq!(
            Ok(("\n".as_bytes(), OType::StringType("Hello there!".as_bytes()))),
            o_data_string_parser("\'Hello there!\'\n".as_bytes())
        ); // Expects ok with no input left and `'Hello There'` in a [OType::StringType]
    }

    /// Tests the o_data_int_parser parser
    #[test]
    fn o_data_int_parser_test() {
        assert_eq!(
            Ok(("\n".as_bytes(), OType::IntType(1234))),
            o_data_int_parser("1234\n".as_bytes())
        ); // Expects ok with no input left and `1234` in a [OType::IntType]
    }

    /// Tests o_data_parser as a whole with ints and strings. Basically testing
    /// if it infers correctly as it's a frontend parser.
    #[test]
    fn o_data_parser_test() {
        assert_eq!(
            Ok(("\n".as_bytes(), OType::IntType(5345))),
            o_data_parser("5345\n".as_bytes())
        ); // Tests for OType::IntType(5345) with correct (i) and 1234
        assert_eq!(
            Ok(("\n".as_bytes(), OType::StringType("Woot".as_bytes()))),
            o_data_parser("'Woot'\n".as_bytes())
        ); // Tests for OType::StringType("Woot") with correct (s) and "Woot"
    }
}
