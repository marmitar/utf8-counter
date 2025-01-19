//! Calculate how many possible UTF8 strings there are.

use rug::Integer;

/// Iterate over the number of possible UTF8 strings of each possible length.
///
/// ```
/// use rug::Integer;
/// use utf8_counter::utf8_counter;
///
/// let mut counter = utf8_counter();
/// assert_eq!(counter.next(), Some(Integer::from(1)));
/// assert_eq!(counter.next(), Some(Integer::from(128)));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Utf8Counter {
    /// The next result, $f(n)$.
    l0: Integer,
    /// The last result, $f(n-1)$.
    l1: Integer,
    /// $f(n-2)$.
    l2: Integer,
    /// $f(n-3)$.
    l3: Integer,
}

// Let's ensure Rust uses the right representation.
static_assertions::assert_eq_size!(Utf8Counter, [Integer; 4]);
static_assertions::assert_eq_size!(Option<Utf8Counter>, [Integer; 4]);

/// Calculates the number of elements in an inclusive range.
macro_rules! range_len {
    ($start:literal ..= $end:literal) => {
        if $end >= $start {
            $end - $start + 1
        } else {
            panic!("invalid range")
        }
    };
}

/// All 1-byte codepoints, `U+0000` to `U+007F`.
const C1: u8 = range_len!(0x00..=0x7F);
/// All 2-byte codepoints, `U+0080` to `U+07FF`.
const C2: u16 = range_len!(0x0080..=0x07FF);
/// All 3-byte codepoints, `U+0800` to `U+FFFF`, except for surrogates, `U+D800` to `U+DFFF`.
const C3: u16 = range_len!(0x0800..=0xFFFF) - range_len!(0xD800..=0xDFFF);
/// All 4-byte codepoints, `U+10000` to `U+10FFFF`.
const C4: u32 = range_len!(0x10000..=0x10_FFFF);

/// Iterate over the number of possible UTF8 strings of each possible length.
///
/// ```
/// use rug::Integer;
/// use utf8_counter::utf8_counter;
///
/// let mut counter = utf8_counter();
/// assert_eq!(counter.next(), Some(Integer::from(1)));
/// assert_eq!(counter.next(), Some(Integer::from(128)));
/// ```
#[inline]
#[must_use]
pub fn utf8_counter() -> Utf8Counter {
    Utf8Counter::new()
}

impl Utf8Counter {
    /// Start the iterator on the first result, $f(0)$.
    #[inline]
    #[must_use]
    fn new() -> Self {
        Self {
            l0: Integer::from(1_u8),
            l1: Integer::ZERO,
            l2: Integer::ZERO,
            l3: Integer::ZERO,
        }
    }

    /// Calculate the next value.
    ///
    /// This is a non-fallible version of [`Iterator::next`]. Only allocation failures might happen, which are not
    /// handled by [`Integer`].
    pub fn next_count(&mut self) -> Integer {
        // apply operations on L3
        self.l3 *= C4;
        self.l3 += C1 * &self.l0;
        self.l3 += C2 * &self.l1;
        self.l3 += C3 * &self.l2;
        // move L3 to L0
        std::mem::swap(&mut self.l2, &mut self.l3);
        std::mem::swap(&mut self.l1, &mut self.l2);
        std::mem::swap(&mut self.l0, &mut self.l1);

        self.l1.clone()
    }
}

impl Iterator for Utf8Counter {
    type Item = Integer;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.next_count())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }
}

impl Default for Utf8Counter {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const fn is_utf8<const N: usize>(bytes: &[u8; N]) -> bool {
        std::str::from_utf8(bytes).is_ok()
    }

    #[test]
    fn count_0_bytes() {
        let counted = std::iter::once([]).filter(is_utf8).count();
        #[expect(clippy::iter_nth_zero, reason = "Consitency across tests")]
        let calculated = utf8_counter().nth(0).unwrap();
        assert_eq!(calculated, counted);
    }

    #[test]
    fn count_1_byte() {
        let counted = (0x00..=0xFF).map(u8::to_be_bytes).filter(is_utf8).count();
        let calculated = utf8_counter().nth(1).unwrap();
        assert_eq!(calculated, counted);
    }

    #[test]
    fn count_2_bytes() {
        let counted = (0x0000..=0xFFFF).map(u16::to_be_bytes).filter(is_utf8).count();
        let calculated = utf8_counter().nth(2).unwrap();
        assert_eq!(calculated, counted);
    }

    #[test]
    fn count_3_bytes() {
        let counted = (0x0000_0000..=0x00FF_FFFF)
            .map(u32::to_be_bytes)
            .filter(is_utf8)
            .count();

        let calculated = utf8_counter().nth(3).unwrap();
        assert_eq!(calculated, counted);
    }

    #[test]
    fn count_4_bytes() {
        use rayon::iter::{IntoParallelIterator, ParallelIterator};

        let counted = (0x0000_0000..=0xFFFF_FFFF)
            .into_par_iter()
            .map(u32::to_be_bytes)
            .filter(is_utf8)
            .count();

        let calculated = utf8_counter().nth(4).unwrap();
        assert_eq!(calculated, counted);
    }
}
