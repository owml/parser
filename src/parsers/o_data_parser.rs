use crate::error::ErrorKind;
use crate::types::OType;

/// Parses a key (for example: `(s) "Hello"`) and returns a full [OType] with
/// the matching data or throws an error from [ErrorKind].
named!(
    pub (crate) o_data_parser<OType>,
    map_res!(
        char!('g'), // TODO add proper data parsing.
        build_o_data_parser
    )
);

/// Build [o_data_parser].
#[allow(dead_code)]
fn build_o_data_parser(_input: char) -> Result<OType, ErrorKind> {
    // TODO parse the `input` and return as OType
    unimplemented!();
}
