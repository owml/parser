use core::str;

/// The main error enum for owml
///
/// # Error Types
///
/// - [ErrorKind::UnknownType]: When a given type (Example: `(s)` for a String) is
/// unknown.
/// - [ErrorKind::InvalidEncoding]: When an inputted string is encoded incorrectly
/// and the parser cannot understand it. *Your best bet for dealing with this
/// one is using UTF8 encoding*.
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    UnknownType,
    InvalidEncoding(str::Utf8Error),
    DataTypesDontMatch,
}
