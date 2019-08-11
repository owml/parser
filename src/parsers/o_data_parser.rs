use crate::types::OType;

use nom::number::streaming::be_i32;

named!(
    o_data_string_parser<OType>,
    map_res!(
        alt!(
            delimited!(char!('"'), is_not!("\""), char!('"')) |
            delimited!(char!('\''), is_not!("'"), char!('\''))
        ),
        build_o_data_string_parser
    )
);

fn build_o_data_string_parser(input: &[u8]) -> Result<OType, ()> {
    Ok(OType::StringType(input))
}

named!(
    o_data_int_parser<OType>,
    map_res!(
        be_i32,
        build_o_data_int_parser
    )
);

fn build_o_data_int_parser(input: i32) -> Result<OType, ()> { // TODO fix lifetime error
    Ok(OType::IntType(input))
}

named!(
    pub (crate) o_data_parser<OType>,
    do_parse!(
        many0!(char!(' ')) >>
        found_data: alt!(
            o_data_string_parser |
            o_data_int_parser
        ) >>
        (found_data)
    )
);
