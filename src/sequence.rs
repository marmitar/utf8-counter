//! Infinite sequence of numbers.

use rug::Integer;

/// Something that generates an infinite sequence of numbers, starting from zero.
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

    /// Iterates until the element at position `n` on the sequence.
    ///
    /// Since sequences starts at zero, this will yield the first `n+1` elements.
    #[inline]
    fn until_n(&mut self, n: usize, mut apply: impl FnMut(usize, &Integer)) {
        for i in 0..=n {
            apply(i, self.current());
            if i < n {
                self.update();
            }
        }
    }

    /// Update `n` steps on the sequence, and return the result.
    #[inline]
    fn nth(&mut self, n: usize) -> &Integer {
        self.until_n(n, |_, _| {});
        self.current()
    }

    /// Iterator over cloned elements of a sequence.
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

    #[inline]
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

/// Produces the [factorial](https://en.wikipedia.org/wiki/Factorial) sequence.
#[inline]
#[must_use]
pub fn factorial() -> Factorial {
    Factorial {
        n: 0,
        f: Integer::from(1_u8),
    }
}

/// Produces the [factorial](https://en.wikipedia.org/wiki/Factorial) sequence.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Factorial {
    /// Current iteration.
    n: usize,
    /// Current result.
    f: Integer,
}

impl SequenceGenerator for Factorial {
    #[inline]
    fn current(&self) -> &Integer {
        &self.f
    }

    #[inline]
    fn update(&mut self) {
        self.n = self.n.checked_add(1).expect("is it even posible?");
        self.f *= self.n;
    }
}

/// Produces the [fibonacci sequence](https://en.wikipedia.org/wiki/Fibonacci_sequence).
#[inline]
#[must_use]
pub fn fibonacci() -> Fibonnaci {
    Fibonnaci {
        f0: Integer::from(0_u8),
        f1: Integer::from(1_u8),
    }
}

/// Produces the [fibonacci sequence](https://en.wikipedia.org/wiki/Fibonacci_sequence).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fibonnaci {
    /// Current element.
    f0: Integer,
    /// Last element.
    f1: Integer,
}

impl SequenceGenerator for Fibonnaci {
    #[inline]
    fn current(&self) -> &Integer {
        &self.f0
    }

    #[inline]
    fn update(&mut self) {
        std::mem::swap(&mut self.f0, &mut self.f1);
        self.f0 += &self.f1;
    }
}

/// Sum over all elements of another sequence.
#[inline]
#[must_use]
pub const fn cumulative<S: SequenceGenerator>(seq: S) -> Cumulative<S> {
    Cumulative {
        acc: Integer::ZERO,
        seq,
    }
}

/// Sum over all elements of another sequence.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cumulative<S> {
    /// Accumalated sum, and the next result.
    acc: Integer,
    /// Non-accumulated iterator.
    seq: S,
}

impl<S: SequenceGenerator> SequenceGenerator for Cumulative<S> {
    #[inline]
    fn current(&self) -> &Integer {
        &self.acc
    }

    #[inline]
    fn update(&mut self) {
        self.acc += self.seq.current();
        self.seq.update();
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    /// Useful for verifying the custom sequences above before any benchmark.
    fn validate_sequence<const N: usize>(seq: impl SequenceGenerator, expected: [usize; N]) {
        let name = std::any::type_name_of_val(&seq);
        let values: Vec<_> = seq.cloning().take(N).collect();
        assert_eq!(values, expected.map(Integer::from), "invalid sequence '{name}'");
    }

    #[test]
    fn factorial_sequence() {
        validate_sequence(factorial(), [1, 1, 2, 6, 24, 120, 720, 5_040, 40_320, 362_880]);
    }

    #[test]
    fn fibonnaci_sequence() {
        validate_sequence(fibonacci(), [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144]);
    }

    #[test]
    fn cumulative_sequence() {
        validate_sequence(cumulative(factorial()), [0, 1, 2, 4, 10, 34, 154, 874]);
    }
}
