use alloc::vec::Vec;

/// The main type enum for owml, containing the type along with the
/// corrosponding data.
#[derive(Debug, PartialEq)]
pub enum OType<'a> {
    StringType(&'a str),
    IntType(i32),
    ObjectType(Vec<OKeyPair<'a>>),
    ArrayType(Vec<OType<'a>>),
}

/// A wrapper for two OTypes. Used as a frontend for having a name and data.
#[derive(Debug, PartialEq)]
pub struct OKeyPair<'a> {
    pub name: OType<'a>,
    pub data: OType<'a>,
}
