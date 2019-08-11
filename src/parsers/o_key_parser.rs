use crate::error::ErrorKind;
use crate::types::{OType, OTypeEncoded};

use crate::parsers::o_data_parser::o_data_parser;
use crate::parsers::o_datatype_parser::o_datatype_parser;

/// Adds together [o_datatype_parser] and [build_o_data_parser] to get one
/// entire `(s) "hello"` and return an [OType] for it.
named!(
    pub o_key_parser<OType>,
    map_res!(
        do_parse!(
            exp_dt: o_datatype_parser >>
            many0!(char!(' ')) >>
            found_data: o_data_parser >>
            (exp_dt, found_data)
        ),
        build_o_key_parser
    )
);

/// Build [o_key].
#[allow(dead_code)]
fn build_o_key_parser(input: (OTypeEncoded, OType)) -> Result<OType, ErrorKind> {
    if input.0.compare_otype(&input.1) {
        Ok(input.1)
    } else {
        Err(ErrorKind::DataTypesDontMatch)
    }
}

/// Test section
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests entire keypair (*String/`(s)` only*) for [o_key_parser] parser.
    #[test]
    fn string_key_parse() {
        let input_key = r#"(s) "hello""#; // Tests for `(s) "hello"`
        let input_key_nospace = r#"(s)"hello""#; // Same as input_key but without space
        let input_key_quotes = "(s) 'hello'"; // Tests with `'` instead of `"`

        let expected_result = Ok(("".as_bytes(), OType::StringType("hello".as_bytes())));

        // Tests `input_key`'s result
        assert_eq!(expected_result, o_key_parser(input_key.as_bytes()));
        // Tests `input_key_nospace`'s result
        assert_eq!(expected_result, o_key_parser(input_key_nospace.as_bytes()));
        // Tests `input_key_quotes`'s result
        assert_eq!(expected_result, o_key_parser(input_key_quotes.as_bytes()));
    }

    /// Tests an invalid string key for [o_key_parser] parser.
    #[test]
    fn incorrect_string_parse() {
        // let input_str_int = "(s) 1234"; // Tries to pair string with int;

        // TODO add mis-match for equivilant of [ErrorKind::DataTypesDontMatch]
    }

    /// Tests entire keypair (*Int/`(i)` only*) for [o_key_parser] parser.
    #[test]
    fn int_key_parse() {
        let input_key = "(i) 1234"; // Tests for an int of 1234
        let input_key_nospace = "(i)1234"; // Same as input_key but without space

        let expected_result = Ok(("".as_bytes(), OType::IntType(1234)));

        // Tests `input_key`'s result'
        assert_eq!(expected_result, o_key_parser(input_key.as_bytes()));
        // Tests `input_key_nospace`'s result'
        assert_eq!(expected_result, o_key_parser(input_key_nospace.as_bytes()));
    }
}
