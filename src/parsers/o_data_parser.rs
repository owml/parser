use crate::types::OType;

use nom::number::streaming::be_i32;

/// Detects a string wrapped in `"` and returns an [OType::StringType].
named!(
    o_data_string_parser<OType>,
    map_res!(
        alt!(
            delimited!(char!('"'), is_not!("\""), char!('"'))
                | delimited!(char!('\''), is_not!("'"), char!('\''))
        ),
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
    map_res!(be_i32, build_o_data_int_parser)
);

/// Builds o_data_int_parser.
#[allow(dead_code)]
fn build_o_data_int_parser<'a>(input: i32) -> Result<OType<'a>, ()> {
    Ok(OType::IntType(input))
}

/// Finds the [OType] for given data.
named!(
    pub (crate) o_data_parser<OType>,
    do_parse!(
        many0!(char!(' ')) >>
        found_data: alt!(
            o_data_string_parser |
            o_data_int_parser
        ) >>
        many0!(char!(' ')) >>
        (found_data)
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
            Ok(("".as_bytes(), OType::StringType("Hello there!".as_bytes()))),
            o_data_string_parser("\"Hello there!\"".as_bytes())
        ); // Expects ok with no input left and `"Hello There"` in a [OType::StringType]
        assert_eq!(
            Ok(("".as_bytes(), OType::StringType("Hello there!".as_bytes()))),
            o_data_string_parser("\'Hello there!\'".as_bytes())
        ); // Expects ok with no input left and `'Hello There'` in a [OType::StringType]
    }

    /// Tests the o_data_int_parser parser
    #[test]
    fn o_data_int_parser_test() {
        assert_eq!(
            Ok(("".as_bytes(), OType::IntType(1234))),
            o_data_int_parser("1234".as_bytes())
        ); // Expects ok with no input left and `1234` in a [OType::IntType]
    }
}
