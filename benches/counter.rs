//! Comparison with other numerical sequences.

use criterion::{AxisScale, BenchmarkId, Criterion, PlotConfiguration, Throughput, criterion_group, criterion_main};
use rug::Integer;

use utf8_counter::{SequenceGenerator, utf8_counter};

/// Produces the [factorial](https://en.wikipedia.org/wiki/Factorial) sequence.
fn factorial() -> impl SequenceGenerator {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Factorial {
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

        fn update(&mut self) {
            self.n = self.n.checked_add(1).expect("is it even posible?");
            self.f *= self.n;
        }
    }

    Factorial {
        n: 0,
        f: Integer::from(1_u8),
    }
}

/// Produces the [fibonacci sequence](https://en.wikipedia.org/wiki/Fibonacci_sequence).
fn fibonacci() -> impl SequenceGenerator {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Fibonnaci {
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

        fn update(&mut self) {
            std::mem::swap(&mut self.f0, &mut self.f1);
            self.f0 += &self.f1;
        }
    }

    Fibonnaci {
        f0: Integer::from(0_u8),
        f1: Integer::from(1_u8),
    }
}

/// Sum over all elements of another sequence.
const fn cumulative(seq: impl SequenceGenerator) -> impl SequenceGenerator {
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct Cumulative<S> {
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

        fn update(&mut self) {
            self.acc += self.seq.current();
            self.seq.update();
        }
    }

    Cumulative {
        acc: Integer::ZERO,
        seq,
    }
}

/// Useful for verifying the custom sequences above before any benchmark.
fn validate_sequence<const N: usize>(seq: impl SequenceGenerator, expected: [usize; N]) {
    let name = std::any::type_name_of_val(&seq);
    let values: Vec<_> = seq.cloning().take(N).collect();
    assert_eq!(values, expected.map(Integer::from), "invalid sequence '{name}'");
}

/// Benchmark the extract of the `n`th element of a sequence.
///
/// Note: both construction and drop of the iteration are measured in each iteration of the benchmark. Additionaly,
/// all elements produced by the iterator are dropped inside the benchmark and are so considered in the measurement.
macro_rules! bench_iterator {
    ($bench:ident, $name:literal, $n:ident, $iter:expr) => {
        $bench.bench_with_input(BenchmarkId::new($name, $n), &$n, |b, &size| {
            b.iter(move || {
                $iter.nth(size);
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
