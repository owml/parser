extern crate alloc;

use alloc::vec::Vec;

use crate::parsers::o_keypair_parser::o_keypair_parser;
use crate::types::OType;

/// The main frontend parser for owml. This takes in an input of `&[u8]` and
/// returns a Vector with several keypairs (a tuple containing 2x [OType]) or a
/// corrosponding nom error.
named!(
    o_main_parser<Vec<(OType, OType)>>,
    many1!(do_parse!(
        many0!(char!(' ')) >> keypair: o_keypair_parser >> one_of!(";,") >> (keypair)
    ))
);

/// Test section
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the basic functionality of `o_main_parser`.
    #[test]
    fn basic_functionality() {
        let input_str = "(i) 625443 (i) 234535; (s) 'This should be 1234:' (i) 1234;\n";

        let first_keypair = (OType::IntType(625443), OType::IntType(234535));
        let second_keypair = (
            OType::StringType("This should be 1234:".as_bytes()),
            OType::IntType(1234),
        );

        let mut vec_to_assert = Vec::new();

        vec_to_assert.push(first_keypair);
        vec_to_assert.push(second_keypair);

        assert_eq!(
            Ok(("\n".as_bytes(), vec_to_assert)),
            o_main_parser(input_str.as_bytes())
        );
    }
}
