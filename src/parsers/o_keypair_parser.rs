use crate::types::OType;

use crate::parsers::o_key_parser::o_key_parser;

/// Parses 2 [OType]'s into something valid, for example `(s) "Hello" (i) 23`
/// is now valid. This would look like the following in json:o_keypair_parser
///
/// ```json
/// "Hello": 23
/// ```
named!(
    o_keypair_parser<(OType, OType)>,
    do_parse!(
        first_key: o_key_parser
            >> many0!(char!(' '))
            >> second_key: o_key_parser
            >> (first_key, second_key)
    )
);

/// Test section
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests correct inputs for `o_keypair_parser` using different kinds of
    /// strings (`''` and `""` currently).
    #[test]
    fn o_keypair_parser_strint_test() {
        let expected_result = Ok((
            "\n".as_bytes(),
            (OType::StringType("Hello".as_bytes()), OType::IntType(1234)),
        )); // Expects a string of `Hello` and an int of `1234`.

        assert_eq!(
            expected_result,
            o_keypair_parser("(s) \"Hello\" (i) 1234\n".as_bytes())
        ); // Tests with `""`.
        assert_eq!(
            expected_result,
            o_keypair_parser("(s) 'Hello' (i) 1234\n".as_bytes())
        ); // Tests with `''`.
    }

    /// Tests correct inputs for `o_keypair_parser` using fully ints. For
    /// example, `(i) 42345 (i) 2342` would be a tuple of [OType::IntType].
    #[test]
    fn o_keypair_parser_intint_test() {
        assert_eq!(
            Ok(("\n".as_bytes(), (OType::IntType(1234), OType::IntType(5678)))),
            o_keypair_parser("(i) 1234 (i) 5678\n".as_bytes())
        );
    }
}
