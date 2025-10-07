//! Trait indicating that an iterator is a set of lines.

use std::ops::Deref;

/// Indicates that the iterator is a succession of lines.
pub trait Lines: Iterator<Item: Deref<Target = str>> {
    /// Returns the text of the section with lines separated by `\n`.
    fn text(&mut self) -> String {
        let mut res = self.fold("".to_string(), |mut a, b| {
            a.push_str(&b);
            a.push('\n');
            a
        });

        res.pop();
        res
    }
}