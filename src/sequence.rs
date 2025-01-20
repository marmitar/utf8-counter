//! Infinite sequence of numbers.

use rug::Integer;

/// Something that generates an infinite sequence of numbers.
pub trait SequenceGenerator {
    /// Latest number on the sequence.
    #[must_use]
    fn current(&self) -> &Integer;

    /// Iterate one step of the sequence.
    fn update(&mut self);

    /// Update the sequence and return the next number.
    #[inline]
    #[must_use]
    fn next(&mut self) -> &Integer {
        self.update();
        self.current()
    }

    /// Update `n` steps on the sequence, and return the result.
    fn nth(&mut self, n: usize) -> &Integer {
        for _ in 0..n {
            self.update();
        }
        self.current()
    }

    #[inline]
    fn cloning(self) -> CloningIterator<Self>
    where
        Self: Sized,
    {
        CloningIterator { sequence: self }
    }
}

/// Iterator over cloned elements of a sequence.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct CloningIterator<S> {
    /// Internal sequence
    sequence: S,
}

impl<S: SequenceGenerator> Iterator for CloningIterator<S> {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        let next = self.sequence.current().clone();
        self.sequence.update();
        Some(next)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }
}
