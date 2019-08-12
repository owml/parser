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
    fn o_key_parser_string_test() {
        let expected_result = Ok(("".as_bytes(), OType::StringType("hello".as_bytes())));

        assert_eq!(expected_result, o_key_parser("(s) \"hello\"".as_bytes())); // Tests for `""` with a space
        assert_eq!(expected_result, o_key_parser("(s)\"hello\"".as_bytes())); // Tests for `""` without a space

        assert_eq!(expected_result, o_key_parser("(s) 'hello'".as_bytes())); // Tests `''` with a space
        assert_eq!(expected_result, o_key_parser("(s)'hello'".as_bytes())); // Tests `''` without a space
    }

    /// Tests a mis-match for equivilant of [ErrorKind::DataTypesDontMatch]. An example of this is `(s) 1234`
    /// where 1234 is not a string but is instead an int.
    #[test]
    fn o_key_parser_incorrect_string_test() {
        assert_ne!(
            Ok(("\n".as_bytes(), OType::StringType("1234".as_bytes()))),
            o_key_parser("(s) 1234\n".as_bytes())
        ); // Make sure it doesn't return a string with 1234 inside as chars
    }

    /// Tests entire keypair (*Int/`(i)` only*) for [o_key_parser] parser.
    #[test]
    fn o_key_parser_int_test() {
        let input_key = "(i) 1234\n"; // Tests for an int of 1234
        let input_key_nospace = "(i)1234\n"; // Same as input_key but without space

        let expected_result = Ok(("\n".as_bytes(), OType::IntType(1234)));

        // Tests `input_key`'s result'
        assert_eq!(expected_result, o_key_parser(input_key.as_bytes()));
        // Tests `input_key_nospace`'s result'
        assert_eq!(expected_result, o_key_parser(input_key_nospace.as_bytes()));
    }
}
