#[macro_use]
extern crate nom;

/// The main type enum for owmu, containing the type along with the corrosponding data.
/// 
/// *If you would like to not embed the data, you may use the private [OTypeEncoded]
pub enum OType {
    StringType(String),
    IntType(i32)
}

/// Similar to OType but doesn't come with mandatory data encoded. This is
/// usually used by a parser that does not contain data but still wants to
/// convey a certain OType to use in the future.
enum OTypeEncoded {
    StringType,
    IntType
}
