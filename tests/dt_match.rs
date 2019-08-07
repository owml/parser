use owens_ml_parser::*;

#[test]
fn basic_right() {
    assert_eq!(Ok(DataType::StringType), dt_match("(s)".as_bytes()));
    assert_eq!(DataType::IntType, dt_match("(i)".as_bytes()));
    assert_eq!(DataType::ObjectType, dt_match("(o)".as_bytes()));
}

#[test]
fn basic_wrong() {
    assert_ne!(DataType::IntType, dt_match("(s)".as_bytes()));
}
