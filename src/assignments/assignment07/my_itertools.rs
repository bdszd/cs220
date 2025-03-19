//! Implement your own minimal `itertools` crate.

use std::collections::HashSet;
use std::hash::Hash;

/// Iterator that iterates over the given iterator and returns only unique elements.
#[derive(Debug)]
pub struct Unique<I: Iterator> {
    // TODO: remove `_marker` and add necessary fields as you want
    iter: I,
    unique: HashSet<I::Item>,
}

impl<I: Iterator> Iterator for Unique<I>
where
    I::Item: Eq + Hash + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.by_ref().next() {
            if self.unique.insert(item.clone()) {
                return Some(item);
            }
        }
        None
    }
}

/// Iterator that chains two iterators together.
#[derive(Debug)]
pub struct Chain<I1: Iterator, I2: Iterator> {
    // TODO: remove `_marker` and add necessary fields as you want
    iter1: I1,
    iter2: I2,
    is_first: bool,
}

impl<T: Eq + Hash + Clone, I1: Iterator<Item = T>, I2: Iterator<Item = T>> Iterator
    for Chain<I1, I2>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_first {
            match self.iter1.next() {
                Some(item) => Some(item),
                None => {
                    self.is_first = false;
                    self.iter2.next()
                }
            }
        } else {
            self.iter2.next()
        }
    }
}

/// Iterator that iterates over given iterator and enumerates each element.
#[derive(Debug)]
pub struct Enumerate<I: Iterator> {
    // TODO: remove `_marker` and add necessary fields as you want
    iter: I,
    index: usize,
}

impl<I: Iterator> Iterator for Enumerate<I> {
    type Item = (usize, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|item| {
            let ret = (self.index, item);
            self.index += 1;
            ret
        })
    }
}

/// Iterator that zips two iterators together.
///
/// If one iterator is longer than the other one, the remaining elements for the longer element
/// should be ignored.
#[derive(Debug)]
pub struct Zip<I1: Iterator, I2: Iterator> {
    // TODO: remove `_marker` and add necessary fields as you want
    iter1: I1,
    iter2: I2,
}

impl<I1: Iterator, I2: Iterator> Iterator for Zip<I1, I2> {
    type Item = (I1::Item, I2::Item);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.iter1.next(), self.iter2.next()) {
            (Some(item1), Some(item2)) => Some((item1, item2)),
            _ => None,
        }
    }
}

/// My Itertools trait.
pub trait MyIterTools: Iterator {
    /// Returns an iterator that iterates over the `self` and returns only unique elements.
    fn my_unique(self) -> Unique<Self>
    where
        Self: Sized,
    {
        Unique {
            iter: self,
            unique: HashSet::new(),
        }
    }

    /// Returns an iterator that chains `self` and `other` together.
    fn my_chain<I: Iterator>(self, other: I) -> Chain<Self, I>
    where
        Self: Sized,
    {
        Chain {
            iter1: self,
            iter2: other,
            is_first: true,
        }
    }

    /// Returns an iterator that iterates over `self` and enumerates each element.
    fn my_enumerate(self) -> Enumerate<Self>
    where
        Self: Sized,
    {
        Enumerate {
            iter: self,
            index: 0,
        }
    }

    /// Returns an iterator that zips `self` and `other` together.
    fn my_zip<I: Iterator>(self, other: I) -> Zip<Self, I>
    where
        Self: Sized,
    {
        Zip {
            iter1: self,
            iter2: other,
        }
    }

    /// Foldleft for `MyIterTools`
    fn my_fold<T, F>(mut self, init: T, mut f: F) -> T
    where
        Self: Sized,
        F: FnMut(Self::Item, T) -> T,
    {
        let mut fold = init;
        for item in self {
            fold = f(item, fold);
        }
        fold
    }
}

impl<T: ?Sized> MyIterTools for T where T: Iterator {}
