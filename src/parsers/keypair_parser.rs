use crate::error::ErrorKind;
use crate::types::{OType, OKeyPair};
use crate::parsers::key_parser::key_parser;

/// Allows `[key]: [data];` syntax.
named!(
    pub(crate) keypair_parser<OKeyPair>,
    map_res!(
        do_parse!(
            name: key_parser >>
            tag!(': ') >>
            data: key_parser >>
            char!(';') >>
            (name, data)
        ),
        build_keypair_parser
    )
);

/// Builds `keypair_parser`.
fn build_keypair_parser(input: (OType, OType)) -> Result<OKeyPair, ()> {
    Ok(OKeyPair {name: input.0, data: input.1})
}

/// Test section
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keypair_parser_basic_functionality() {
        let expected_result = OKeyPair {
            name: OType::StringType(b"Hello"),
            data: OType::IntType(1234)
        };

        assert_eq!(
            Ok((b"", expected_result)),
            keypair_parser(b"\"Hello\" 1234")
        );
    }
}
