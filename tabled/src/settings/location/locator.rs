use super::{ByColumnName, ByContent};

/// An abstract factory for locations, to be used to find different things on the table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Locator;

impl Locator {
    /// Constructs a new location searcher for a cells with a given content.
    pub fn content<S>(text: S) -> ByContent<S>
    where
        S: AsRef<str>,
    {
        ByContent::new(text)
    }

    /// Constructs a new location searcher for a column by its header.
    pub fn column<S>(text: S) -> ByColumnName<S>
    where
        S: AsRef<str>,
    {
        ByColumnName::new(text)
    }
}
