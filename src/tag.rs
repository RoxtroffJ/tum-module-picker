//! Tags to indicate that an object has some properties or belongs to a certain group.

use std::{collections::HashSet, iter::{empty, Empty}};

/// A tag
pub type Tag = String;

/// A tag bank. It stores known tags, and is used to produce new tags.
pub struct TagBank {
    tags: HashSet<Tag>,
}

impl TagBank {
    /// Creates a new empty [TagBank].
    pub fn new() -> Self {
        let iter: Empty<String> = empty();
        Self::from_iter(iter)
    }

    /// Adds a tag to the [TagBank] and returns it.
    pub fn add_tag<T: Into<String>>(&mut self, tag: T) -> &Tag {
        match self.add_tag_checked(tag) {
            Ok(res) | Err(res) => res,
        }
    }

    /// Same as [add_tag] but returns [Ok]`(tag)` if the tag wasn't in the [TagBank],
    /// and [Err]`(tag)` if it already was.
    /// In this case, the [TagBank] is unchanged.
    pub fn add_tag_checked<T: Into<String>>(&mut self, tag: T) -> Result<&Tag, &Tag> {
        // TODO use get_or_insert if stablised
        let tag = tag.into();
        let present = self.tags.insert(tag.clone());
        let result = self.tags.get(&tag).unwrap();

        if present {
            Result::Err(result)
        } else {
            Result::Ok(result)
        }
    }
}

impl<T: Into<String>> FromIterator<T> for TagBank {
    fn from_iter<Iter: IntoIterator<Item = T>>(iter: Iter) -> Self {
        Self {
            tags: HashSet::from_iter(iter.into_iter().map(Into::into)),
        }
    }
}
