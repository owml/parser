use std::collections::LinkedList;

/// Template for an array in `owens-ml`.
///
/// # Notes
///
/// - It is advised against making an array containing arrays with `(a-a)`
/// though it is not strictly forbidden inside of the parser.
///
/// # Examples
///
/// - `owens-ml`: data_type = `(s)`, used_data = `"hello!"`
/// - Rust: data_type = `DataType::ArrayType.data_type`,
/// used_data = `DataType::ArrayType.used_data`
pub struct ArrayType {
    pub data_type: DataType,
    pub used_data: DataType,
}

/// The main enum for identifying owens-ml datatypes. All written datatypes
/// are lower-cased versions of these options; for example the `(s)` in
/// owens-ml is `DataType::S` in this parser.
pub enum DataType {
    S(String),
    I(i32),
    A(Box<ArrayType>),
    O(LinkedList<DataType>),
}
