extern crate alloc;

use alloc::vec::Vec;

use crate::types::OType;
use crate::parsers::o_keypair_parser::o_keypair_parser;

/// The main frontend parser for owml. This takes in an input of `&[u8]` and
/// returns a Vector with several keypairs (a tuple containing 2x [OType]) or a
/// corrosponding nom error.
named!(
    o_main_parser<Vec<(OType, OType)>>,
    many0!(
        do_parse!(
            many0!(char!(' ')) >>
            keypair: o_keypair_parser >>
            tag!(";,") >>
            (keypair)
        )
    )
);
