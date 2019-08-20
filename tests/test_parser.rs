///! This is a public-facing test of owml_parser
extern crate owml_parser;

#[test]
fn basic_test() {
    let input_str = "'Hello': { 1234: [ 'h'; 'e'; 'l'; 'l'; 'o'; ]; };";

    let expected_result =
        owml_parser::types::OType::ObjectType(vec![owml_parser::types::OKeyPair {
            name: owml_parser::types::OType::StringType("Hello"),
            data: owml_parser::types::OType::ObjectType(vec![owml_parser::types::OKeyPair {
                name: owml_parser::types::OType::IntType(1234),
                data: owml_parser::types::OType::ArrayType(vec![
                    owml_parser::types::OType::StringType("h"),
                    owml_parser::types::OType::StringType("e"),
                    owml_parser::types::OType::StringType("l"),
                    owml_parser::types::OType::StringType("l"),
                    owml_parser::types::OType::StringType("o"),
                ]),
            }]),
        }]);

    assert_eq!(
        Ok(("", expected_result)),
        owml_parser::parse_owml_str(input_str)
    );
}
