use crate::parsers::keypair_parser::keypair_parser;
use crate::types::{OKeyPair, OType};

use alloc::vec::Vec;

named!(
    multi_parser<Vec<OKeyPair>>,
    many1!(do_parse!(
        opt!(many0!(char!(' '))) >> keypair: keypair_parser >> opt!(tag!("\n")) >> (keypair)
    ))
);

/// Test section
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests some of the basic functionality of `multi_parser`.
    #[test]
    fn multi_parser_basic_functionality() {
        let input = b"\"Hello\": 1234; 35675: 34265; \"other:\": 6342;";

        let keypair_vector: Vec<OKeyPair> = vec![
            OKeyPair {
                name: OType::StringType(b"Hello"),
                data: OType::IntType(1234),
            },
            OKeyPair {
                name: OType::IntType(35675),
                data: OType::IntType(34265),
            },
            OKeyPair {
                name: OType::StringType(b"other"),
                data: OType::IntType(6342),
            },
        ];

        let content_left: &[u8] = &[];

        assert_eq!(Ok((content_left, keypair_vector)), multi_parser(input));
    }

    /// Tests multiline input for `multi_parser`.
    #[test]
    fn multi_parser_newlines() {
        let input = b"135542: \"Woot!\";\n\"Alright\": 73523;"; // 2x OKeyPair on seperate lines

        let keypair_vector: Vec<OKeyPair> = vec![
            OKeyPair {
                name: OType::IntType(135542),
                data: OType::StringType(b"Woot!"),
            },
            OKeyPair {
                name: OType::StringType(b"Alright"),
                data: OType::IntType(73523),
            },
        ];

        let content_left: &[u8] = &[];

        assert_eq!(Ok((content_left, keypair_vector)), multi_parser(input));
    }
}
