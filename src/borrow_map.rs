//! Adds the function [borrow_map] to all iterators.

use std::iter::FusedIterator;

/// The return type of [borrow_map].
#[derive(Debug)]
pub struct Map<'a, I, F> {
    iter: &'a mut I,
    f: F,
}

impl<'a, I, F, T> Iterator for Map<'a, I, F>
where
    I: Iterator,
    F: FnMut(<I as Iterator>::Item) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|x| (self.f)(x))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, I, F, T> DoubleEndedIterator for Map<'a, I, F>
where
    I: DoubleEndedIterator,
    F: FnMut(<I as Iterator>::Item) -> T,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back().map(|x| (self.f)(x))
    }
}

impl<'a, I, F, T> ExactSizeIterator for Map<'a, I, F>
where
    I: ExactSizeIterator,
    F: FnMut(<I as Iterator>::Item) -> T,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, I, F, T> FusedIterator for Map<'a, I, F>
where
    I: FusedIterator,
    F: FnMut(<I as Iterator>::Item) -> T,
{
}

/// Provides the function [borrow_map]
pub trait BorrowMap: Iterator {
    /// Same as [map](Iterator::map) but borrows the iterator.
    fn borrow_map<B, F>(&mut self, f: F) -> Map<'_, Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        Map { iter: self, f }
    }
}

impl<I: Iterator> BorrowMap for I {}
