//! Provides a function and associated structs to read from an iterator of lines
//! in which some lines are separators marking the beginning of a new section

use std::ops::Deref;

use crate::lines;

/// The iterator returned by [get_sections].
///
/// It yields the lines found in a section.
///
/// Once a section is over, call [next_section](Iter::next_section) to make the iterator now iterate on the next section.
pub struct Iter<'a, Section, Lines, F> {
    found: Option<Section>,
    lines: Lines,
    section_finder: &'a F,
}

impl<'a, Section, Lines, F> Iter<'a, Section, Lines, F>
where
    Lines: Iterator<Item: Deref<Target = str> + 'a>,
    F: Fn(&str) -> Option<Section>,
{
    /// Makes the iterator go to the next section.
    pub fn next_section(&mut self) -> Option<Section> {
        self.found
            .take()
            .or_else(|| find_section(&mut self.lines, &self.section_finder))
    }
}

/// Reads the provided lines and finds lines outputing Some with given function.
///
/// Then returns text found between matches.
pub fn get_sections<'a, Section: 'a, Lines: 'a, F>(
    lines: Lines,
    section_finder: &'a F,
) -> Iter<'a, Section, Lines, F>
where
    Lines: Iterator<Item: Deref<Target = str> + 'a>,
    F: Fn(&str) -> Option<Section>,
{
    Iter {
        found: None,
        lines,
        section_finder,
    }
}

/// Auxiliary function that finds the next section delimiter if any.
fn find_section<'a, Section, Lines, F>(lines: &mut Lines, section_finder: &'a F) -> Option<Section>
where
    Lines: Iterator<Item: Deref<Target = str> + 'a>,
    F: Fn(&str) -> Option<Section>,
{
    for line in lines {
        match section_finder(line.deref()) {
            Some(section) => return Some(section),
            None => {}
        }
    }

    None
}

impl<'a, Section, Lines, F> Iterator for Iter<'a, Section, Lines, F>
where
    Lines: Iterator<Item: Deref<Target = str>>,
    F: Fn(&str) -> Option<Section>,
    <Lines as Iterator>::Item: 'a,
{
    type Item = <Lines as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.found.is_some() {
            return None;
        }

        let next_line = self.lines.next()?;

        match (self.section_finder)(next_line.deref()) {
            Some(section) => {
                self.found = Some(section);
                None
            }
            None => Some(next_line),
        }
    }
}

impl<'a, Section, Lines, F> lines::Lines for Iter<'a, Section, Lines, F>
where
    Lines: Iterator<Item: Deref<Target = str>>,
    F: Fn(&str) -> Option<Section>,
    <Lines as Iterator>::Item: 'a,
{
}
