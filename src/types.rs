use alloc::vec::Vec;

/// The main type enum for owml, containing the type along with the corrosponding data.
///
/// *If you would like to not embed the data, you may use the private [OTypeEncoded]*.
#[derive(Debug, PartialEq)]
pub enum OType<'a> {
    StringType(&'a str),
    IntType(i32),
    ObjectType(Vec<OKeyPair<'a>>)
}

/// A wrapper for two OTypes. Used as a frontend for having a name and data.
#[derive(Debug, PartialEq)]
pub struct OKeyPair<'a> {
    pub name: OType<'a>,
    pub data: OType<'a>
}
