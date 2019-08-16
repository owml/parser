use crate::types::{OKeyPair, OType};

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{digit1, one_of},
    combinator::opt,
    error::ErrorKind,
    multi::{many0, many1},
    sequence::delimited,
    IResult,
};

use alloc::vec::Vec;
use core::{mem, str};

/// Reads input and returns a full vec of OTypes
pub fn get_vec_parser(input: &str) -> IResult<&str, Vec<OKeyPair>> {
    many1(build_get_vec_parser)(input)
}

/// Helper function for get_vec_parser. Gets keypairs and strips any whitespace
/// with `strip_whitespace`.
fn build_get_vec_parser(input: &str) -> IResult<&str, OKeyPair> {
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
    let (input, name) = keypair_name_disallow_parser(input)?; // Get first key
    let (input, _) = tag(": ")(input)?; // Makes sure has `: ` as a seperator
    let (input, data) = key_parser(input)?; // Get second key
    let (input, _) = tag(";")(input)?; // Force `;`

    Ok((input, OKeyPair { name, data }))
}

/// Removes OTypes from being used as a name for a keypair.
///
/// # Disallowed Types
///
/// - [OType::ObjectType]
fn keypair_name_disallow_parser(input: &str) -> IResult<&str, OType> {
    let (input, name) = key_parser(input)?; // Get name

    let issued_error = Err(nom::Err::Error((input, ErrorKind::Permutation)));

    match name {
        OType::ObjectType(_) | OType::ArrayType(_) => issued_error,
        _ => Ok((input, name)),
    }
}

/// Parses a key into an OType token. This is arguably the main logic behind
/// owml as it infers the types.
fn key_parser(input: &str) -> IResult<&str, OType> {
    alt((
        key_int_parser,    // Parses ints like `36624` or `-6456412`
        key_string_parser, // Parses strings like `"Hello!"` or `'Cool!'`
        key_object_parser, // Parses an object (recurses get_vec_parser) inbetween `{}`
        key_array_parser,  // Like key_object_parser but sticks to 1 type
    ))(input)
}

/// Similar to [OType::ObjectType] but infers 1 type and sticks to it,
/// otherwise returns an `ErrorKind::OneOf` error.
fn key_array_parser(input: &str) -> IResult<&str, OType> {
    let (input, _) = tag("[")(input)?; // Open [
    let (input, found_otype) = build_key_array_parser(input)?; // Capture arrays
    let (input, _) = tag("]")(input)?; // Close ]

    Ok((input, found_otype))
}

/// One cycle of strict types for `key_array_parser`.
fn key_array_parser_cycle(input: &str) -> IResult<&str, OType> {
    let (input, _) = strip_whitespace(input)?; // Strip any whitespace at start
    let (input, found_otype) = key_parser(input)?; // Get OType (can recur)
    let (input, _) = tag(";")(input)?; // Check for `;` on end
    let (input, _) = strip_whitespace(input)?; // Strip any whitespace at start

    Ok((input, found_otype))
}

/// Builds the main types inside of the `[]` for `key_array_parser`.
fn build_key_array_parser(input: &str) -> IResult<&str, OType> {
    let (input, found_otype) = many0(key_array_parser_cycle)(input)?; // Get array items

    let first_val = match found_otype.first() {
        Some(x) => x,
        None => return Err(nom::Err::Error((input, ErrorKind::NonEmpty))),
    };

    for found_value in found_otype.iter() {
        if mem::discriminant(first_val) != mem::discriminant(found_value) {
            return Err(nom::Err::Error((input, ErrorKind::OneOf)));
        }
    }

    Ok((input, OType::ArrayType(found_otype)))
}

/// Parses an object. This essentially recurses the `get_vec_parser` to find values inbetween `{}` tags.
///
/// *NOTE: This should not be used as a name of a value, only the data.*
fn key_object_parser(input: &str) -> IResult<&str, OType> {
    let (input, _) = tag("{")(input)?; // Open {
    let (input, _) = strip_whitespace(input)?; // Strip any whitespace between `{` and values
    let (input, found_vec) = get_vec_parser(input)?; // Get objects
    let (input, _) = strip_whitespace(input)?; // Strip any whitespace between `}` and values
    let (input, _) = tag("}")(input)?; // Close }

    Ok((input, OType::ObjectType(found_vec)))
}

/// Tries to parse and find ints for `key_parser`.
fn key_int_parser(input: &str) -> IResult<&str, OType> {
    let (input, found_neg) = opt(tag("-"))(input)?; // Finds neg number if avalible
    let (input, found_digits) = digit1(input)?; // Parses digits into builder
    let (_, found_otype) = build_key_int_parser(found_digits, found_neg.is_some())?;

    Ok((input, found_otype))
}

/// Parses recognised digits into a proper OType and returns.
fn build_key_int_parser(input: &str, is_neg_num: bool) -> IResult<&str, OType> {
    let mut input_as_int =
        str::parse::<i32>(input).map_err(|_| nom::Err::Error((input, ErrorKind::Digit)))?;

    if is_neg_num {
        input_as_int = -input_as_int;
    }

    Ok((input, OType::IntType(input_as_int)))
}

/// Tries to parse strings for `key_parser`.
fn key_string_parser(input: &str) -> IResult<&str, OType> {
    let (input, removed_quotes) = delimited(one_of("\"'"), is_not("\"'"), one_of("\"'"))(input)?;

    Ok((input, OType::StringType(removed_quotes)))
}

/// Test section
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests `key_array_parser`.
    #[test]
    fn key_array_parser_test() {
        assert_eq!(
            Ok((
                "",
                OType::ArrayType(vec![
                    OType::IntType(53234),
                    OType::IntType(365),
                    OType::IntType(-59823)
                ])
            )),
            key_array_parser("[ 53234; 365; -59823; ]")
        ); // Runs a passing test with only ints
        assert_ne!(
            Ok((
                "",
                OType::ArrayType(vec![
                    OType::IntType(53234),
                    OType::StringType("Shouldn't work")
                ])
            )),
            key_array_parser("[ 53234; 'Shouldn't work'; ]")
        ); // Should **NOT** succeed, makes sure array is strict
    }

    /// Tests various disallowed types that are included in
    /// `keypair_name_disallow_parser`.
    #[test]
    fn keypair_name_disallow_parser_test() {
        assert_eq!(
            Err(nom::Err::Error((": 73892;", ErrorKind::Permutation))),
            get_vec_parser("{ 45223: 'adfgoj'; }: 73892;")
        ); // Tests for objects
        assert_eq!(
            Err(nom::Err::Error((": 73892;", ErrorKind::Permutation))),
            get_vec_parser("[ 1234; 4632; 2523; ]: 73892;")
        ) // Tests for arrays
    }

    /// Tests `key_object_parser`.
    ///
    /// *NOTE: The `}` on `Second value}` is purposeful.*
    #[test]
    fn key_object_parser_test() {
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
            key_object_parser("{'Object test': 672342; 847624: 'Second value}'; }")
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

    /// Tests `key_int_parser` & `build_key_int_parser`.
    #[test]
    fn key_int_parser_test() {
        assert_eq!(Ok(("", OType::IntType(1234))), key_int_parser("1234")); // Small num
        assert_eq!(Ok(("", OType::IntType(6356234))), key_int_parser("6356234")); // Larger num
        assert_eq!(Ok(("", OType::IntType(-46234))), key_int_parser("-46234")) // Neg number
    }

    /// Tests `key_string_parser`.
    #[test]
    fn key_string_parser_test() {
        assert_eq!(
            Ok(("", OType::StringType("test"))),
            key_string_parser("'test'")
        ); // With `'`
        assert_eq!(
            Ok(("", OType::StringType("test"))),
            key_string_parser("\"test\"")
        ); // With `"`
        assert_ne!(
            Ok(("", OType::StringType("224521"))),
            key_string_parser("224521")
        ); // Make sure it doesn't parse int
        assert_eq!(
            Ok(("", OType::StringType("1234"))),
            key_string_parser("'1234'")
        ); // Make sure it *does* parse int if wrapped in string
    }

    /// Some general parsing tests on [parsers::get_vec_parser].
    #[test]
    fn get_vec_parser_basic_test() {
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

        assert_eq!(Ok(("", expected_result)), get_vec_parser(input));
    }

    /// Same as get_vec_parser_test but checks multiline.
    #[test]
    fn get_vec_parser_multiline_test() {
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

        assert_eq!(Ok(("", expected_result)), get_vec_parser(input));
    }
}
