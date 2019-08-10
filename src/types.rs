use std::string::String;

/// The main type enum for owml, containing the type along with the corrosponding data.
///
/// *If you would like to not embed the data, you may use the private [OTypeEncoded]*.
#[derive(Debug, PartialEq)]
pub enum OType {
    StringType(String),
    IntType(i32),
}

/// Similar to OType but doesn't come with mandatory data encoded. This is
/// usually used by a parser that does not contain data but still wants to
/// convey a certain OType to use in the future.
#[derive(Debug, PartialEq)]
pub enum OTypeEncoded {
    StringType,
    IntType,
}

impl OTypeEncoded {
    /// Compares [OTypeEncoded] and [OType] to see if they have matching
    /// parameters and return a bool if they do (true) or don't (false).
    pub fn compare_otype(&self, to_compare: &OType) -> bool {
        match to_compare {
            OType::IntType(_) => self == &OTypeEncoded::IntType,
            OType::StringType(_) => self == &OTypeEncoded::StringType,
        }
    }
}
