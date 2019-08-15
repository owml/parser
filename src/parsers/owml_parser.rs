use crate::types::{OKeyPair, OType};

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{digit1, one_of},
    combinator::{map_res, opt},
    error::ErrorKind,
    multi::{many0, many1},
    sequence::delimited,
    IResult,
};

use alloc::vec::Vec;
use core::str;

/// This is the main frontend parser for Owen's Markup Language.
/// 
/// # Language specification
/// 
/// The language specification for Owen's Markup Language can be found
/// [here](https://owml.gitlab.io/owml-website/docs/lang-spec/).
/// 
/// # Using the parser
/// 
/// All documentation for using this parser can be found
/// [here](https://owml.gitlab.io/owml-website/docs/parser/).
pub fn owml_parser(input: &str) -> IResult<&str, Vec<OKeyPair>> {
    many1(build_owml_parser_keypairs)(input)
}

/// Helper function for owml_parser. Gets keypairs and strips any whitespace
/// with `strip_whitespace`.
fn build_owml_parser_keypairs(input: &str) -> IResult<&str, OKeyPair> {
    let (input, found_keypair) = keypair_parser(input)?; // Get keypair
    let (input, _) = strip_whitespace(input)?;

    Ok((input, found_keypair))
}

/// Strips any whitespace like "`[space][space]\n[space]`".
fn strip_whitespace(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(tag(" "))(input)?; // Allow optional spaces
    let (input, _) = opt(tag("\n"))(input)?; // Strip \n if there
    let (input, _) = many0(tag(" "))(input)?; // Allow optional spaces

    Ok((input, ()))
}

/// Parses 2 keys/values to make a proper key. Example syntax: `'hello': 1234`.
fn keypair_parser(input: &str) -> IResult<&str, OKeyPair> {
    let (input, name) = key_parser(input)?; // Get first key
    let (input, _) = tag(": ")(input)?; // Makes sure has `: ` as a seperator
    let (input, data) = key_parser(input)?; // Get second key
    let (input, _) = tag(";")(input)?; // Force `;`

    Ok((input, OKeyPair { name, data }))
}

/// Parses a key into an OType token. This is arguably the main logic behind
/// owml as it infers the types.
fn key_parser(input: &str) -> IResult<&str, OType> {
    alt((key_parser_int, key_parser_string))(input)
}

/// Tries to parse and find ints for `key_parser`.
fn key_parser_int(input: &str) -> IResult<&str, OType> {
    let (input, (_, found_otype)) = map_res(digit1, build_key_parser_int)(input)?;

    Ok((input, found_otype))
}

/// Parses recognised digits into a proper OType and returns.
fn build_key_parser_int(input: &str) -> IResult<&str, OType> {
    match str::parse::<i32>(input) {
        Ok(x) => Ok((input, OType::IntType(x))),
        Err(_) => Err(nom::Err::Error((input, ErrorKind::Digit))),
    }
}

/// Tries to parse strings for `key_parser`.
fn key_parser_string(input: &str) -> IResult<&str, OType> {
    let (input, removed_quotes) = delimited(one_of("\"'"), is_not("\"'"), one_of("\"'"))(input)?;

    Ok((input, OType::StringType(removed_quotes)))
}

/// Test section
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests `keypair_parser`.
    #[test]
    fn keypair_parser_basic_test() {
        let expected_keypair = OKeyPair {
            name: OType::StringType("Testing keypairs.."),
            data: OType::IntType(8678234),
        };

        assert_eq!(
            Ok(("", expected_keypair)),
            keypair_parser("'Testing keypairs..': 8678234;")
        ); // With `'`
    }

    /// Tests `key_parser_int` & `build_key_parser_int`.
    #[test]
    fn key_parser_int_test() {
        assert_eq!(Ok(("", OType::IntType(1234))), key_parser_int("1234")); // Small num
        assert_eq!(Ok(("", OType::IntType(6356234))), key_parser_int("6356234")); // Larger num
        assert_eq!(Ok(("", OType::IntType(-46234))), key_parser_int("-46234")) // Neg number
    }

    /// Tests `key_parser_string`.
    #[test]
    fn key_parser_string_test() {
        assert_eq!(
            Ok(("", OType::StringType("test"))),
            key_parser_string("'test'")
        ); // With `'`
        assert_eq!(
            Ok(("", OType::StringType("test"))),
            key_parser_string("\"test\"")
        ); // With `"`
        assert_ne!(
            Ok(("", OType::StringType("224521"))),
            key_parser_string("224521")
        ); // Make sure it doesn't parse int
        assert_eq!(
            Ok(("", OType::StringType("1234"))),
            key_parser_string("'1234'")
        ); // Make sure it *does* parse int if wrapped in string
    }

    /// Some general parsing tests on [parsers::owml_parser].
    #[test]
    fn owml_parser_basic_test() {
        let input = "'This is a name': 'And this is data'; '2 main types, int and str': 1234; 63452123: 'Can also be ints as you can see';";

        let expected_result: Vec<OKeyPair> = vec![
            OKeyPair {
                name: OType::StringType("This is a name"),
                data: OType::StringType("And this is data"),
            },
            OKeyPair {
                name: OType::StringType("2 main types, int and str"),
                data: OType::IntType(1234),
            },
            OKeyPair {
                name: OType::IntType(63452123),
                data: OType::StringType("Can also be ints as you can see"),
            },
        ];

        assert_eq!(Ok(("", expected_result)), owml_parser(input));
    }

    /// Same as owml_parser_test but checks multiline.
    #[test]
    fn owml_parser_multiline_test() {
        let input = "'first line': 324325;\n'second line': 'woo!';";

        let expected_result: Vec<OKeyPair> = vec![
            OKeyPair {
                name: OType::StringType("first line"),
                data: OType::IntType(324325),
            },
            OKeyPair {
                name: OType::StringType("second line"),
                data: OType::StringType("woo!"),
            }
        ];

        assert_eq!(Ok(("", expected_result)), owml_parser(input));
    }
}
