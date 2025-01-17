use num_bigint::BigUint;

static_assertions::assert_eq_size!(Utf8Counter, [BigUint; 4]);

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Utf8Counter {
    inner: Inner
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Inner {
    F0,
    F1(BigUint),
    F2(BigUint, BigUint),
    F3(BigUint, BigUint, BigUint),
    Fn(BigUint, BigUint, BigUint, BigUint),
}

#[inline]
#[must_use]
pub const fn utf8_counter() -> Utf8Counter {
    Utf8Counter::new()
}

impl Utf8Counter {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self { inner: Inner::F0 }
    }

    pub fn next_count(&mut self) -> BigUint {
        use Inner::*;

        let a1 = 128u8;
        let a2 = 1920u16;
        let a3 = 61_440u16;
        let a4 = 1_048_576u32;

        let inner = std::mem::replace(&mut self.inner, F0);
        match inner {
            Fn(l0, l1, l2, l3) => {
                let ln = a1 * &l0 + a2 * &l1 + a3 * &l2 + a4 * l3;
                self.inner = Fn(ln.clone(), l0, l1, l2);
                ln
            },
            F3(f2, f1, f0) => {
                let f3 = a1 * &f2 + a2 * &f1 + a3 * &f0;
                self.inner = Fn(f3.clone(), f2, f1, f0);
                f3
            },
            F2(f1, f0) => {
                let f2 = a1 * &f1 + a2 * &f0;
                self.inner = F3(f2.clone(), f1, f0);
                f2
            },
            F1(f0) => {
                let f1 = a1 * &f0;
                self.inner = F2(f1.clone(), f0);
                f1
            },
            F0 => {
                let f0 = BigUint::from(1u8);
                self.inner = F1(f0.clone());
                f0
            },
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
