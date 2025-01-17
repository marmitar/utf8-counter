//! Iterate over how many UTF8 strings of a specific length are possible.

use num_bigint::BigUint;

/// Iterate over the number of possible UTF8 strings of each possible length.
///
/// ```
/// let counter = utf8_counter();
/// assert_eq!(counter.next(), 1.to_biguint());
/// assert_eq!(counter.next(), 128.to_biguint());
/// ```
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Utf8Counter {
    /// Private implementation.
    inner: Inner,
}

macro_rules! range_len {
    ($start:literal, $end:literal) => {
        $end - $start + 1
    };
}

/// All 1-byte codepoints, `U+0000` to `U+007F`.
const C1: u8 = range_len!(0x00, 0x7F);
/// All 2-byte codepoints, `U+0080` to `U+07FF`.
const C2: u16 = range_len!(0x0080, 0x07FF);
/// All 3-byte codepoints, `U+0800` to `U+FFFF`, except for surrogates, `U+D800` to `U+DFFF`.
const C3: u16 = range_len!(0x0800, 0xFFFF) - range_len!(0xD800, 0xDFFF);
/// All 4-byte codepoints, `U+10000` to `U+10FFFF`.
const C4: u32 = range_len!(0x10000, 0x10_FFFF);

// Let's ensure Rust uses the right representation.
static_assertions::assert_eq_size!(Utf8Counter, [BigUint; 4]);

/// The last 4 results, used for iterative calculation.
///
/// Calculated as:
/// - $f(n) = C1 f(n-1) + C2 f(n-2) + C3 f(n-3) + C4 f(n-4)$
///
/// Should be binary compatible with `[Option<BigUint>; 4]`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Inner {
    /// Next is $f(0)$, no last results.
    F0,
    /// Next is $f(1)$, with $f(0)$.
    F1(BigUint),
    /// Next is $f(2)$, with $f(1)$ and $f(0)$.
    F2(BigUint, BigUint),
    /// Next is $f(3)$, with $f(2)$, $f(1)$ and $f(0)$.
    F3(BigUint, BigUint, BigUint),
    /// Next is $f(n)$, with $f(n-1)$, $f(n-2)$, $f(n-3)$ and $f(n-4)$.
    Fn(BigUint, BigUint, BigUint, BigUint),
}

/// Iterate over the number of possible UTF8 strings of each possible length.
///
/// ```
/// let counter = utf8_counter();
/// assert_eq!(counter.next(), 1.to_biguint());
/// assert_eq!(counter.next(), 128.to_biguint());
/// ```
#[inline]
#[must_use]
pub const fn utf8_counter() -> Utf8Counter {
    Utf8Counter::new()
}

impl Utf8Counter {
    /// Start the iterator on the first result, $f(0)$.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { inner: Inner::F0 }
    }

    /// Calculate the next value.
    ///
    /// This is a non-fallible version of [`Iterator::next`]. Only allocation failures might happen, which are not
    /// handled by [`BigUint`].
    pub fn next_count(&mut self) -> BigUint {
        let inner = std::mem::replace(&mut self.inner, Inner::F0);
        match inner {
            Inner::Fn(l0, l1, l2, l3) => {
                let ln = C1 * &l0 + C2 * &l1 + C3 * &l2 + C4 * l3;
                self.inner = Inner::Fn(ln.clone(), l0, l1, l2);
                ln
            }
            Inner::F3(f2, f1, f0) => {
                let f3 = C1 * &f2 + C2 * &f1 + C3 * &f0;
                self.inner = Inner::Fn(f3.clone(), f2, f1, f0);
                f3
            }
            Inner::F2(f1, f0) => {
                let f2 = C1 * &f1 + C2 * &f0;
                self.inner = Inner::F3(f2.clone(), f1, f0);
                f2
            }
            Inner::F1(f0) => {
                let f1 = C1 * &f0;
                self.inner = Inner::F2(f1.clone(), f0);
                f1
            }
            Inner::F0 => {
                let f0 = BigUint::from(1_u8);
                self.inner = Inner::F1(f0.clone());
                f0
            }
        }
    }
}

impl Iterator for Utf8Counter {
    type Item = BigUint;

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
    use num_bigint::ToBigUint;
    use pretty_assertions::assert_eq;

    use super::*;

    const fn is_utf8<const N: usize>(bytes: &[u8; N]) -> bool {
        std::str::from_utf8(bytes).is_ok()
    }

    #[test]
    fn count_0_bytes() {
        let counted = std::iter::once([]).filter(is_utf8).count();
        #[expect(clippy::iter_nth_zero, reason = "Consitency across tests")]
        let calculated = utf8_counter().nth(0);
        assert_eq!(calculated, counted.to_biguint());
    }

    #[test]
    fn count_1_byte() {
        let counted = (0x00..=0xFF).map(u8::to_be_bytes).filter(is_utf8).count();
        let calculated = utf8_counter().nth(1);
        assert_eq!(calculated, counted.to_biguint());
    }

    #[test]
    fn count_2_bytes() {
        let counted = (0x0000..=0xFFFF).map(u16::to_be_bytes).filter(is_utf8).count();
        let calculated = utf8_counter().nth(2);
        assert_eq!(calculated, counted.to_biguint());
    }

    #[test]
    fn count_3_bytes() {
        let counted = (0x0000_0000..=0x00FF_FFFF).map(u32::to_be_bytes).filter(is_utf8).count();
        let calculated = utf8_counter().nth(3);
        assert_eq!(calculated, counted.to_biguint());
    }

    #[test]
    fn count_4_bytes() {
        use rayon::iter::{IntoParallelIterator, ParallelIterator};

        let counted = (0x0000_0000..=0xFFFF_FFFF)
            .into_par_iter()
            .map(u32::to_be_bytes)
            .filter(is_utf8)
            .count();

        let calculated = utf8_counter().nth(4);
        assert_eq!(calculated, counted.to_biguint());
    }
}
