//! Comparison with other numerical sequences.

use criterion::{AxisScale, BenchmarkId, Criterion, PlotConfiguration, Throughput, criterion_group, criterion_main};
use num_bigint::BigUint;

use utf8_counter::utf8_counter;

/// Produces the [factorial](https://en.wikipedia.org/wiki/Factorial) sequence.
fn factorial() -> impl Iterator<Item = BigUint> {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Factorial {
        /// Next iteration.
        n: usize,
        /// Next result.
        f: BigUint,
    }

    impl Iterator for Factorial {
        type Item = BigUint;

        fn next(&mut self) -> Option<BigUint> {
            self.n = self.n.checked_add(1)?;
            let next = &self.f * self.n;
            let output = std::mem::replace(&mut self.f, next);
            Some(output)
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            (usize::MAX - 1, Some(usize::MAX - 1))
        }
    }

    Factorial {
        n: 0,
        f: BigUint::from(1_u8),
    }
}

/// Produces the [fibonacci sequence](https://en.wikipedia.org/wiki/Fibonacci_sequence).
fn fibonacci() -> impl Iterator<Item = BigUint> {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Fibonnaci {
        /// Next element.
        f0: BigUint,
        /// The one after that.
        f1: BigUint,
    }

    impl Iterator for Fibonnaci {
        type Item = BigUint;

        fn next(&mut self) -> Option<BigUint> {
            let output = self.f0.clone();
            self.f1 += &self.f0;
            std::mem::swap(&mut self.f0, &mut self.f1);
            Some(output)
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            (usize::MAX, None)
        }
    }

    Fibonnaci {
        f0: BigUint::from(0_u8),
        f1: BigUint::from(1_u8),
    }
}

/// Sum over all elements of another sequence.
const fn cumulative(iter: impl Iterator<Item = BigUint>) -> impl Iterator<Item = BigUint> {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Cumulative<I> {
        /// Accumalated sum, and the next result.
        acc: BigUint,
        /// Non-accumulated iterator.
        iter: I,
    }

    impl<I: Iterator<Item = BigUint>> Iterator for Cumulative<I> {
        type Item = BigUint;

        fn next(&mut self) -> Option<BigUint> {
            let mut next = self.iter.next()?;
            next += &self.acc;

            let output = std::mem::replace(&mut self.acc, next);
            Some(output)
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            let (lo, hi) = self.iter.size_hint();
            (lo.saturating_add(1), hi.and_then(|hi| hi.checked_add(1)))
        }
    }

    Cumulative {
        acc: BigUint::ZERO,
        iter,
    }
}

/// Useful for verifying the custom sequences above before any benchmark.
fn validate_sequence<const N: usize>(iter: impl Iterator<Item = BigUint>, expected: [usize; N]) {
    let name = std::any::type_name_of_val(&iter);
    let values: Vec<_> = iter.take(N).collect();
    assert_eq!(values, expected.map(BigUint::from), "invalid sequence '{name}'");
}

/// Benchmark the extract of the `n`th element of a sequence.
///
/// Note: both construction and drop of the iteration are measured in each iteration of the benchmark. Additionaly,
/// all elements produced by the iterator are dropped inside the benchmark and are so considered in the measurement.
macro_rules! bench_iterator {
    ($bench:ident, $name:literal, $n:ident, $iter:expr) => {
        $bench.bench_with_input(BenchmarkId::new($name, $n), &$n, |b, &size| {
            b.iter(move || {
                $iter.nth(size).expect("not enough items");
            });
        });
    };
}

/// Compare different large sequence iterators.
///
/// # Panics
///
/// On memory allocation failures (and possibly on internal conversion errors, which shouldn't happen).
pub fn sequence_iter(c: &mut Criterion) {
    validate_sequence(factorial(), [1, 1, 2, 6, 24, 120, 720, 5_040, 40_320, 362_880]);
    validate_sequence(fibonacci(), [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144]);
    validate_sequence(cumulative(factorial()), [0, 1, 2, 4, 10, 34, 154, 874]);

    let mut group = c.benchmark_group("Sequence");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    for n in [64, 128, 256, 512, 768, 1024, 1536, 2048] {
        let size = n.try_into().expect("cannot fit into u64");
        group.throughput(Throughput::Elements(size));

        // direct sequences
        bench_iterator!(group, "Factorial", n, factorial());
        bench_iterator!(group, "Fibonacci", n, fibonacci());
        bench_iterator!(group, "UTF8 Counter", n, utf8_counter());

        // cumulative sequences
        bench_iterator!(group, "Cumulative Factorial", n, cumulative(factorial()));
        bench_iterator!(group, "Cumulative Fibonacci", n, cumulative(fibonacci()));
        bench_iterator!(group, "Cumulative UTF8 Counter", n, cumulative(utf8_counter()));
    }
    group.finish();
}

criterion_group!(benches, sequence_iter);
criterion_main!(benches);
