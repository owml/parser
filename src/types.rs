/// The main type enum for owml, containing the type along with the corrosponding data.
///
/// *If you would like to not embed the data, you may use the private [OTypeEncoded]*.
#[derive(Debug, PartialEq)]
pub enum OType<'a> {
    StringType(&'a [u8]),
    IntType(i32),
}

/// A wrapper for two OTypes. Used as a frontend for having a name and data.
pub struct OKeyPair<'a> {
    name: OType<'a>,
    data: OType<'a>
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

/// Test section
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the OTypeEncoded.compare_otype method.
    #[test]
    fn compare_otype_test() {
        assert_eq!(
            true,
            OTypeEncoded::StringType.compare_otype(&OType::StringType(&[]))
        ); // Tests string to be string. Should return true
        assert_eq!(
            true,
            OTypeEncoded::IntType.compare_otype(&OType::IntType(0))
        ); // Tests int to be int. Should return true
    }

    /// Tests incorrect types to make sure bad types don't match/
    #[test]
    fn compare_otype_test_incorrect() {
        assert_eq!(
            false,
            OTypeEncoded::StringType.compare_otype(&OType::IntType(0))
        ); // Tests string to be int. Should return false
        assert_eq!(
            false,
            OTypeEncoded::IntType.compare_otype(&OType::StringType(&[]))
        ); // Tests int to be string. Should return false
    }
}
