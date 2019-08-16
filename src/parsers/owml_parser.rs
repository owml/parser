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
    alt((key_parser_int, key_parser_string, key_parser_object))(input)
}

/// Parses an object. This essentially recurses the `owml_parser` to find values inbetween `{}` tags.
///
/// *NOTE: This should not be used as a name of a value, only the data.*
fn key_parser_object(input: &str) -> IResult<&str, OType> {
    let (input, _) = tag("{")(input)?; // Open {
    let (input, _) = strip_whitespace(input)?; // Strip any whitespace between `{` and values
    let (input, found_vec) = owml_parser(input)?; // Get objects
    let (input, _) = strip_whitespace(input)?; // Strip any whitespace between `}` and values
    let (input, _) = tag("}")(input)?; // Close }

    Ok((input, OType::ObjectType(found_vec)))
}

/// Tries to parse and find ints for `key_parser`.
fn key_parser_int(input: &str) -> IResult<&str, OType> {
    let (input, found_neg) = opt(tag("-"))(input)?; // Finds neg number if avalible
    let (input, found_digits) = digit1(input)?; // Parses digits into builder
    let (_, found_otype) = build_key_parser_int(found_digits, found_neg.is_some())?;

    Ok((input, found_otype))
}

/// Parses recognised digits into a proper OType and returns.
fn build_key_parser_int(input: &str, is_neg_num: bool) -> IResult<&str, OType> {
    let mut input_as_int =
        str::parse::<i32>(input).map_err(|_| nom::Err::Error((input, ErrorKind::Digit)))?;

    if is_neg_num {
        input_as_int = -input_as_int;
    }

    Ok((input, OType::IntType(input_as_int)))
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

    /// Tests `key_parser_object`.
    ///
    /// *NOTE: The `}` on `Second value}` is purposeful.*
    #[test]
    fn key_parser_object_test() {
        let expected_result: Vec<OKeyPair> = vec![
            OKeyPair {
                name: OType::StringType("Object test"),
                data: OType::IntType(672342),
            },
            OKeyPair {
                name: OType::IntType(847624),
                data: OType::StringType("Second value}"),
            },
        ];

        assert_eq!(
            Ok(("", OType::ObjectType(expected_result))),
            key_parser_object("{'Object test': 672342; 847624: 'Second value}'; }")
        );
    }

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
        let input = "'This is a name': 'And this is data'; 63452123: { 'Inside an object!': 765234; 423457: 6823473; };";

        let inside_obj: Vec<OKeyPair> = vec![
            OKeyPair {
                name: OType::StringType("Inside an object!"),
                data: OType::IntType(765234),
            },
            OKeyPair {
                name: OType::IntType(423457),
                data: OType::IntType(6823473),
            },
        ];

        let expected_result: Vec<OKeyPair> = vec![
            OKeyPair {
                name: OType::StringType("This is a name"),
                data: OType::StringType("And this is data"),
            },
            OKeyPair {
                name: OType::IntType(63452123),
                data: OType::ObjectType(inside_obj),
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
            },
        ];

        assert_eq!(Ok(("", expected_result)), owml_parser(input));
    }
}
