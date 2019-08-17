#![no_std]

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;
extern crate nom;

mod parsers;
pub mod types;

use nom::IResult;
use types::OType;

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
pub fn parse_owml_str(input: &str) -> IResult<&str, OType> {
    let (input, found_vec) = parsers::owml_parser::get_vec_parser(input)?;

    Ok((input, OType::ObjectType(found_vec)))
}
