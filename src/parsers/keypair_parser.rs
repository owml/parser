use crate::types::{OType, OKeyPair};
use crate::parsers::key_parser::key_parser;

/// Allows `[key]: [data];` syntax.
named!(
    pub(crate) keypair_parser<OKeyPair>,
    map_res!(
        do_parse!(
            name: key_parser >>
            char!(':') >>
            opt!(char!(' ')) >>
            data: key_parser >>
            char!(';') >>
            (name, data)
        ),
        build_keypair_parser
    )
);

/// Builds `keypair_parser`.
#[allow(dead_code)]
fn build_keypair_parser<'a>(input: (OType<'a>, OType<'a>)) -> Result<OKeyPair<'a>, ()> {
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

        let content_left: &[u8] = &[];

        assert_eq!(
            Ok((content_left, expected_result)),
            keypair_parser(b"\"Hello\": 1234;")
        );
    }
}
