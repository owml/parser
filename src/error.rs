use std::str;

/// The main error enum for owml
///
/// # Error Types
///
/// - [ErrorKind::UnknownType]: When a given type (Example: `(s)` for a String) is
/// unknown.
/// - [ErrorKind::InvalidEncoding]: When an inputted string is encoded incorrectly
/// and the parser cannot understand it. *Your best bet for dealing with this
/// one is using UTF8 encoding*.
/// - [ErrorKind::DataTypesDontMatch]: When a given datatype (`(s)` for
/// example) and data `1234` are not of the same type. For example, `(i)
/// "hello"` would return this as `"hello"` is a string, not int.
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    UnknownType,
    InvalidEncoding(str::Utf8Error),
    DataTypesDontMatch,
}
